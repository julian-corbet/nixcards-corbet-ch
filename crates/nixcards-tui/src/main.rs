mod app;
mod progress_store;

use app::App;
use crossterm::event::{self, Event};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use nixcards_core::bundled_catalog;
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
    let arguments: Vec<String> = env::args().skip(1).collect();
    match arguments.as_slice() {
        [] => run_tui(catalog),
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
        "nixcards\n\nUSAGE:\n  nixcards\n  nixcards validate\n  nixcards list\n  nixcards export-progress <file>\n  nixcards import-progress <file>"
    );
}

fn run_tui(catalog: nixcards_core::Catalog) -> Result<(), String> {
    if !io::stdin().is_terminal() || !io::stdout().is_terminal() {
        return Err("the interactive interface requires a terminal; try `nixcards list`".into());
    }

    let progress_path = progress_store::default_path()?;
    let progress = progress_store::load(&progress_path)?;
    let mut app = App::new(catalog, progress, progress_path);

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
