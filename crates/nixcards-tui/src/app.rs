use crate::progress_store;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use nixcards_core::{
    Card, CardSet, Catalog, CatalogIndex, CramSession, ProgressFile, ReviewRating, SearchHit,
    markdown_to_plain_text,
};
use nixcards_store::{CatalogStore, OFFICIAL_REPOSITORY};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct App {
    catalog: Catalog,
    progress: ProgressFile,
    progress_path: PathBuf,
    set_index: usize,
    card_index: usize,
    focus_sets: bool,
    revealed: bool,
    search_mode: bool,
    query: String,
    search_results: Vec<SearchHit>,
    search_index: usize,
    cram: Option<CramSession>,
    message: String,
    quit: bool,
    store: CatalogStore,
    managing_catalog: bool,
    catalog_rows: Vec<CatalogRow>,
    catalog_cursor: usize,
    pending_sets: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CatalogRow {
    path: String,
    depth: usize,
    title: Option<String>,
    descendants: Vec<String>,
}

impl App {
    pub fn new(
        catalog: Catalog,
        progress: ProgressFile,
        progress_path: PathBuf,
        store: CatalogStore,
    ) -> Self {
        Self {
            catalog,
            progress,
            progress_path,
            set_index: 0,
            card_index: 0,
            focus_sets: true,
            revealed: false,
            search_mode: false,
            query: String::new(),
            search_results: Vec::new(),
            search_index: 0,
            cram: None,
            message: "Browse freely. Press c to cram the selected set.".into(),
            quit: false,
            store,
            managing_catalog: false,
            catalog_rows: Vec::new(),
            catalog_cursor: 0,
            pending_sets: BTreeSet::new(),
        }
    }

    pub fn should_quit(&self) -> bool {
        self.quit
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if self.managing_catalog {
            self.handle_catalog_key(key);
            return;
        }
        if self.search_mode {
            self.handle_search_key(key);
            return;
        }
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            self.quit = true;
            return;
        }
        match key.code {
            KeyCode::Char('q') => self.quit = true,
            KeyCode::Char('/') => {
                self.search_mode = true;
                self.query.clear();
                self.search_results = self.catalog.search("");
                self.search_index = 0;
                self.message = "Type to search the complete catalogue.".into();
            }
            KeyCode::Char('m') if self.cram.is_none() => self.open_catalog_manager(),
            KeyCode::Esc => {
                if self.cram.take().is_some() {
                    self.message = "Cram session closed. Browse progress was not changed.".into();
                } else if !self.query.is_empty() {
                    self.query.clear();
                    self.search_results.clear();
                    self.search_index = 0;
                }
                self.revealed = false;
            }
            KeyCode::Tab | KeyCode::Left | KeyCode::Right
                if self.cram.is_none() && self.query.is_empty() =>
            {
                self.focus_sets = !self.focus_sets;
            }
            KeyCode::Up | KeyCode::Char('k') if self.cram.is_none() => self.move_selection(-1),
            KeyCode::Down | KeyCode::Char('j') if self.cram.is_none() => self.move_selection(1),
            KeyCode::Enter | KeyCode::Char(' ') => self.revealed = !self.revealed,
            KeyCode::Char('c') if self.cram.is_none() => self.start_cram(),
            KeyCode::Char('a') | KeyCode::Char('1') if self.cram.is_some() => {
                self.review(ReviewRating::Again)
            }
            KeyCode::Char('k') | KeyCode::Char('2') if self.cram.is_some() => {
                self.review(ReviewRating::Known)
            }
            _ => {}
        }
    }

    fn open_catalog_manager(&mut self) {
        if !self.store.is_initialized()
            && let Err(error) = self.store.initialize(OFFICIAL_REPOSITORY)
        {
            self.message = error;
            return;
        }
        match (self.store.catalog_index(), self.store.selected_set_ids()) {
            (Ok(index), Ok(selected)) => {
                self.catalog_rows = catalog_rows(&index);
                self.pending_sets = selected.into_iter().collect();
                self.catalog_cursor = self
                    .catalog_cursor
                    .min(self.catalog_rows.len().saturating_sub(1));
                self.managing_catalog = true;
                self.message = "Choose local sets. Enter applies the sparse checkout.".into();
            }
            (Err(error), _) | (_, Err(error)) => self.message = error,
        }
    }

    fn handle_catalog_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc | KeyCode::Char('m') => {
                self.managing_catalog = false;
                self.message = "Catalogue selection left unchanged.".into();
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.catalog_cursor = shifted(self.catalog_cursor, -1, self.catalog_rows.len());
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.catalog_cursor = shifted(self.catalog_cursor, 1, self.catalog_rows.len());
            }
            KeyCode::Char(' ') => self.toggle_catalog_row(),
            KeyCode::Char('r') => match self.store.sync() {
                Ok(()) => self.open_catalog_manager(),
                Err(error) => self.message = error,
            },
            KeyCode::Enter => self.apply_catalog_selection(),
            KeyCode::Char('q') => self.quit = true,
            _ => {}
        }
    }

    fn toggle_catalog_row(&mut self) {
        let Some(row) = self.catalog_rows.get(self.catalog_cursor) else {
            return;
        };
        let all_selected = row
            .descendants
            .iter()
            .all(|id| self.pending_sets.contains(id));
        if all_selected {
            for id in &row.descendants {
                self.pending_sets.remove(id);
            }
        } else {
            self.pending_sets.extend(row.descendants.iter().cloned());
        }
    }

    fn apply_catalog_selection(&mut self) {
        let selected: Vec<_> = self.pending_sets.iter().cloned().collect();
        if let Err(error) = self.store.select(&selected) {
            self.message = error;
            return;
        }
        match self.store.load_selected_catalog() {
            Ok(catalog) => {
                self.catalog = catalog;
                self.set_index = 0;
                self.card_index = 0;
                self.query.clear();
                self.search_results.clear();
                self.managing_catalog = false;
                self.message = format!("Applied {} local card set(s).", selected.len());
            }
            Err(error) => self.message = error,
        }
    }

    fn handle_search_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.search_mode = false;
                self.query.clear();
                self.search_results.clear();
            }
            KeyCode::Enter => {
                self.search_mode = false;
                self.revealed = false;
            }
            KeyCode::Backspace => {
                self.query.pop();
                self.refresh_search();
            }
            KeyCode::Char(character)
                if !key
                    .modifiers
                    .intersects(KeyModifiers::CONTROL | KeyModifiers::ALT) =>
            {
                self.query.push(character);
                self.refresh_search();
            }
            _ => {}
        }
    }

    fn refresh_search(&mut self) {
        self.search_results = self.catalog.search(&self.query);
        self.search_index = self
            .search_index
            .min(self.search_results.len().saturating_sub(1));
    }

    fn move_selection(&mut self, delta: isize) {
        if !self.query.is_empty() {
            self.search_index = shifted(self.search_index, delta, self.search_results.len());
        } else if self.focus_sets {
            self.set_index = shifted(self.set_index, delta, self.catalog.sets.len());
            self.card_index = 0;
        } else {
            let card_count = self
                .catalog
                .sets
                .get(self.set_index)
                .map_or(0, |set| set.cards.len());
            self.card_index = shifted(self.card_index, delta, card_count);
        }
        self.revealed = false;
    }

    fn start_cram(&mut self) {
        let Some(set) = self.catalog.sets.get(self.set_index) else {
            return;
        };
        let seed = now() as u32;
        self.cram = Some(CramSession::new(set, seed));
        self.revealed = false;
        self.message = format!(
            "Cram started: {} cards. Reveal, then press a or k.",
            set.cards.len()
        );
    }

    fn review(&mut self, rating: ReviewRating) {
        if !self.revealed {
            self.message = "Reveal the answer before rating it.".into();
            return;
        }
        let Some(session) = self.cram.as_mut() else {
            return;
        };
        let Some(card_id) = session.answer(rating) else {
            return;
        };
        self.progress.record(card_id, rating, now());
        if let Err(error) = progress_store::save(&self.progress_path, &self.progress) {
            self.message = error;
            return;
        }
        self.revealed = false;
        if session.is_complete() {
            let reviews = session.reviews();
            self.cram = None;
            self.message = format!("Cram complete after {reviews} reviews.");
        } else {
            self.message = format!("{} card(s) remaining.", session.remaining());
        }
    }

    fn current(&self) -> Option<(&CardSet, &Card)> {
        if let Some(session) = &self.cram {
            return session
                .current()
                .and_then(|id| self.catalog.card_by_canonical_id(id));
        }
        if !self.query.is_empty() {
            return self.search_results.get(self.search_index).and_then(|hit| {
                self.catalog
                    .set(&hit.set_id)
                    .zip(self.catalog.card(&hit.set_id, &hit.card_id))
            });
        }
        self.catalog
            .sets
            .get(self.set_index)
            .and_then(|set| set.cards.get(self.card_index).map(|card| (set, card)))
    }

    pub fn draw(&self, frame: &mut Frame<'_>) {
        if self.managing_catalog {
            self.draw_catalog_manager(frame);
            return;
        }
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(12),
                Constraint::Length(3),
            ])
            .split(frame.area());

        let title = if self.search_mode {
            format!(" nixcards /{} ", self.query)
        } else if let Some(session) = &self.cram {
            format!(
                " nixcards · cram {}/{} ",
                session.remaining(),
                session.initial_count()
            )
        } else {
            " nixcards · browse ".into()
        };
        frame.render_widget(
            Paragraph::new(Line::from(vec![
                Span::styled(
                    "nixcards",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("  local-first Markdown flashcards"),
            ]))
            .block(Block::default().borders(Borders::ALL).title(title)),
            vertical[0],
        );

        let horizontal = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(34), Constraint::Percentage(66)])
            .split(vertical[1]);

        let set_items: Vec<_> = self
            .catalog
            .sets
            .iter()
            .map(|set| ListItem::new(format!("{}  ({})", set.title, set.cards.len())))
            .collect();
        let mut set_state = ListState::default().with_selected(Some(self.set_index));
        let set_style = if self.focus_sets && self.cram.is_none() && self.query.is_empty() {
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        frame.render_stateful_widget(
            List::new(set_items)
                .block(Block::default().borders(Borders::ALL).title(" Sets "))
                .highlight_symbol("› ")
                .highlight_style(set_style),
            horizontal[0],
            &mut set_state,
        );

        let card_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(5), Constraint::Min(6)])
            .split(horizontal[1]);
        if let Some((set, card)) = self.current() {
            frame.render_widget(
                Paragraph::new(vec![
                    Line::styled(&set.id, Style::default().fg(Color::DarkGray)),
                    Line::from(Span::styled(
                        &card.question,
                        Style::default().add_modifier(Modifier::BOLD),
                    )),
                ])
                .block(Block::default().borders(Borders::ALL).title(" Card "))
                .wrap(Wrap { trim: true }),
                card_area[0],
            );
            let answer = if self.revealed {
                markdown_to_plain_text(&card.answer)
            } else {
                "Press Space or Enter to reveal the answer.".into()
            };
            frame.render_widget(
                Paragraph::new(answer)
                    .block(Block::default().borders(Borders::ALL).title(" Answer "))
                    .wrap(Wrap { trim: false }),
                card_area[1],
            );
        }

        let help = if self.search_mode {
            "Type · Enter keep result · Esc cancel"
        } else if self.cram.is_some() {
            "Space reveal · a/1 again · k/2 known · Esc end · q quit"
        } else {
            "Tab pane · ↑↓/jk move · Space reveal · / search · c cram · m catalogue · q quit"
        };
        frame.render_widget(
            Paragraph::new(vec![
                Line::from(self.message.as_str()),
                Line::styled(help, Style::default().fg(Color::DarkGray)),
            ])
            .block(Block::default().borders(Borders::ALL)),
            vertical[2],
        );
    }

    fn draw_catalog_manager(&self, frame: &mut Frame<'_>) {
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(8),
                Constraint::Length(3),
            ])
            .split(frame.area());
        frame.render_widget(
            Paragraph::new(Line::from(vec![
                Span::styled(
                    "nixcards",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("  local catalogue selection"),
            ]))
            .block(Block::default().borders(Borders::ALL).title(" Catalogue ")),
            vertical[0],
        );

        let items: Vec<_> = self
            .catalog_rows
            .iter()
            .map(|row| {
                let selected = row
                    .descendants
                    .iter()
                    .filter(|id| self.pending_sets.contains(*id))
                    .count();
                let marker = if selected == 0 {
                    "[ ]"
                } else if selected == row.descendants.len() {
                    "[x]"
                } else {
                    "[-]"
                };
                let label = row
                    .title
                    .as_deref()
                    .unwrap_or_else(|| row.path.rsplit('.').next().unwrap_or(&row.path));
                ListItem::new(format!(
                    "{}{} {}",
                    "  ".repeat(row.depth.saturating_sub(1)),
                    marker,
                    label
                ))
            })
            .collect();
        let mut state = ListState::default().with_selected(Some(self.catalog_cursor));
        frame.render_stateful_widget(
            List::new(items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Available sets "),
                )
                .highlight_symbol("› ")
                .highlight_style(
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
            vertical[1],
            &mut state,
        );
        frame.render_widget(
            Paragraph::new(vec![
                Line::from(self.message.as_str()),
                Line::styled(
                    "↑↓/jk move · Space toggle branch · Enter apply · r sync · Esc cancel",
                    Style::default().fg(Color::DarkGray),
                ),
            ])
            .block(Block::default().borders(Borders::ALL)),
            vertical[2],
        );
    }
}

fn catalog_rows(index: &CatalogIndex) -> Vec<CatalogRow> {
    let mut rows: BTreeMap<String, CatalogRow> = BTreeMap::new();
    for set in &index.sets {
        let segments: Vec<_> = set.id.split('.').collect();
        for depth in 1..=segments.len() {
            let path = segments[..depth].join(".");
            let row = rows.entry(path.clone()).or_insert_with(|| CatalogRow {
                path,
                depth,
                title: None,
                descendants: Vec::new(),
            });
            row.descendants.push(set.id.clone());
            if depth == segments.len() {
                row.title = Some(set.title.clone());
            }
        }
    }
    for row in rows.values_mut() {
        row.descendants.sort();
        row.descendants.dedup();
    }
    rows.into_values().collect()
}

fn shifted(current: usize, delta: isize, length: usize) -> usize {
    if length == 0 {
        return 0;
    }
    (current as isize + delta).rem_euclid(length as isize) as usize
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use nixcards_core::{CATALOG_INDEX_SCHEMA_VERSION, CatalogIndexSet};

    #[test]
    fn hierarchy_rows_group_sets_by_dotted_prefix() {
        let index = CatalogIndex {
            schema_version: CATALOG_INDEX_SCHEMA_VERSION,
            sets: vec![
                CatalogIndexSet {
                    id: "cloud.bearingpoint.interview".into(),
                    title: "Interview".into(),
                    language: "en".into(),
                    tags: Vec::new(),
                    path: "cloud/bearingpoint/interview".into(),
                    card_count: 1,
                },
                CatalogIndexSet {
                    id: "cloud.certificates.databricks.introduction".into(),
                    title: "Databricks".into(),
                    language: "en".into(),
                    tags: Vec::new(),
                    path: "cloud/certificates/databricks/introduction".into(),
                    card_count: 1,
                },
            ],
        };
        let rows = catalog_rows(&index);
        let certificates = rows
            .iter()
            .find(|row| row.path == "cloud.certificates")
            .expect("certificate branch");
        assert_eq!(
            certificates.descendants,
            ["cloud.certificates.databricks.introduction"]
        );
        assert_eq!(rows[0].path, "cloud");
    }
}
