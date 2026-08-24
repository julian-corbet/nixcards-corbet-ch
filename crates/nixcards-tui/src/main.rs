mod app;
mod progress_store;

use app::App;
use crossterm::event::{self, Event};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use nixcards_core::bundled_catalog;
use nixcards_store::{CatalogStore, OFFICIAL_REPOSITORY, default_store_path};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::env;
use std::io::{self, IsTerminal};
use std::path::Path;
use std::time::Duration;

fn main() {
    if let Err(error) = run() {
        eprintln!("nixcards: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let catalog = bundled_catalog().map_err(|error| error.to_string())?;
    let mut arguments: Vec<String> = env::args().skip(1).collect();
    let store_path = extract_store_path(&mut arguments)?;
    let store = CatalogStore::new(store_path);
    match arguments.as_slice() {
        [] => {
            let catalog = if store.is_initialized() {
                store.load_selected_catalog()?
            } else {
                catalog
            };
            run_tui(catalog, store)
        }
        [command] if command == "validate" => {
            println!(
                "valid: {} set(s), {} card(s)",
                catalog.sets.len(),
                catalog.card_count()
            );
            for set in catalog.sets {
                println!("{}\t{}\t{} cards", set.id, set.language, set.cards.len());
            }
            Ok(())
        }
        [command] if command == "list" => {
            for set in catalog.sets {
                println!("{}\t{}\t{}", set.id, set.language, set.title);
            }
            Ok(())
        }
        [command, action] if command == "catalog" && action == "init" => {
            store.initialize(OFFICIAL_REPOSITORY)?;
            println!("initialized catalogue at {}", store.root().display());
            Ok(())
        }
        [command, action, repository] if command == "catalog" && action == "init" => {
            store.initialize(repository)?;
            println!("initialized catalogue at {}", store.root().display());
            Ok(())
        }
        [command, action] if command == "catalog" && action == "status" => {
            store.validate_checkout()?;
            let selected = store.selected_set_ids()?;
            let filter = store.promisor_filter()?.unwrap_or_else(|| "none".into());
            println!("store: {}", store.root().display());
            println!("partial clone filter: {filter}");
            println!("selected sets: {}", selected.len());
            for id in selected {
                println!("{id}");
            }
            Ok(())
        }
        [command, action] if command == "catalog" && action == "list" => {
            let selected: std::collections::BTreeSet<_> =
                store.selected_set_ids()?.into_iter().collect();
            for set in store.catalog_index()?.sets {
                let marker = if selected.contains(&set.id) { "x" } else { " " };
                println!("[{marker}]\t{}\t{}\t{}", set.id, set.language, set.title);
            }
            Ok(())
        }
        [command, action, selectors @ ..] if command == "catalog" && action == "select" => {
            let selected = store.resolve_selectors(selectors)?;
            store.select(&selected)?;
            println!("selected {} set(s)", selected.len());
            Ok(())
        }
        [command, action] if command == "catalog" && action == "sync" => {
            store.sync()?;
            println!("synchronized catalogue at {}", store.root().display());
            Ok(())
        }
        [command] if command == "catalog-index" => {
            println!(
                "{}",
                serde_json::to_string_pretty(&catalog.index()).map_err(|error| error.to_string())?
            );
            Ok(())
        }
        [command, path] if command == "check-catalog-index" => {
            let expected = format!(
                "{}\n",
                serde_json::to_string_pretty(&catalog.index()).map_err(|error| error.to_string())?
            );
            let actual = std::fs::read_to_string(path)
                .map_err(|error| format!("cannot read {path}: {error}"))?;
            if actual != expected {
                return Err(format!(
                    "{path} is stale; regenerate it with `nixcards catalog-index`"
                ));
            }
            println!("valid catalogue index: {path}");
            Ok(())
        }
        [command, path] if command == "export-progress" => {
            progress_store::export(&progress_store::default_path()?, Path::new(path))?;
            println!("exported progress to {path}");
            Ok(())
        }
        [command, path] if command == "import-progress" => {
            let destination = progress_store::default_path()?;
            let progress = progress_store::import(Path::new(path), &destination)?;
            println!(
                "imported {} review event(s) into {}",
                progress.events.len(),
                destination.display()
            );
            Ok(())
        }
        [command] if command == "--help" || command == "-h" || command == "help" => {
            print_help();
            Ok(())
        }
        _ => {
            print_help();
            Err("invalid command line".into())
        }
    }
}

fn print_help() {
    println!(
        "nixcards\n\nUSAGE:\n  nixcards [--store PATH]\n  nixcards validate\n  nixcards list\n  nixcards [--store PATH] catalog init [REPOSITORY]\n  nixcards [--store PATH] catalog status\n  nixcards [--store PATH] catalog list\n  nixcards [--store PATH] catalog select [DOTTED-PREFIX ...]\n  nixcards [--store PATH] catalog sync\n  nixcards export-progress <file>\n  nixcards import-progress <file>"
    );
}

fn extract_store_path(arguments: &mut Vec<String>) -> Result<std::path::PathBuf, String> {
    if arguments
        .first()
        .is_some_and(|argument| argument == "--store")
    {
        if arguments.len() < 2 {
            return Err("--store requires a path".into());
        }
        let path = std::path::PathBuf::from(arguments.remove(1));
        arguments.remove(0);
        Ok(path)
    } else {
        default_store_path()
    }
}

fn run_tui(catalog: nixcards_core::Catalog, store: CatalogStore) -> Result<(), String> {
    if !io::stdin().is_terminal() || !io::stdout().is_terminal() {
        return Err("the interactive interface requires a terminal; try `nixcards list`".into());
    }

    let progress_path = progress_store::default_path()?;
    let progress = progress_store::load(&progress_path)?;
    let mut app = App::new(catalog, progress, progress_path, store);

    enable_raw_mode().map_err(|error| error.to_string())?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).map_err(|error| error.to_string())?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).map_err(|error| error.to_string())?;

    let result = (|| -> Result<(), String> {
        while !app.should_quit() {
            terminal
                .draw(|frame| app.draw(frame))
                .map_err(|error| error.to_string())?;
            if event::poll(Duration::from_millis(250)).map_err(|error| error.to_string())?
                && let Event::Key(key) = event::read().map_err(|error| error.to_string())?
            {
                app.handle_key(key);
            }
        }
        Ok(())
    })();

    disable_raw_mode().map_err(|error| error.to_string())?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen).map_err(|error| error.to_string())?;
    terminal.show_cursor().map_err(|error| error.to_string())?;
    result
}
