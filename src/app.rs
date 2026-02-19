use rand::seq::SliceRandom;

use crate::data::{
    portfolio::PROJECTS,
    quotes::{QUOTES, Quote},
    songs::{SONGS, Song},
};
use crate::input::Key;

/// Which top-level section is active. Order matches the nav tabs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Section {
    Home,
    About,
    Portfolio,
    Contact,
}

impl Section {
    pub const ALL: &'static [Section] = &[
        Section::Home,
        Section::About,
        Section::Portfolio,
        Section::Contact,
    ];

    pub fn index(&self) -> usize {
        Self::ALL.iter().position(|s| s == self).unwrap_or(0)
    }

    pub fn label(&self) -> &'static str {
        match self {
            Section::Home => "Home",
            Section::About => "About",
            Section::Portfolio => "Portfolio",
            Section::Contact => "Contact",
        }
    }
}

/// The full application state, fully owned (no async, no I/O).
pub struct App {
    pub section: Section,
    pub should_quit: bool,

    // --- Portfolio state ---
    /// Index of the hovered project in the list.
    pub portfolio_cursor: usize,
    /// Scroll offset for the details view.
    pub portfolio_scroll: u16,

    // --- Quotes state ---
    pub quote_order: Vec<usize>,
    pub quote_cursor: usize,

    // --- About / song state ---
    pub song_order: Vec<usize>,
    pub song_cursor: usize,

    // --- Terminal size ---
    pub cols: u16,
    pub rows: u16,
}

impl App {
    pub fn new(cols: u16, rows: u16) -> Self {
        let mut rng = rand::thread_rng();
        let mut quote_order: Vec<usize> = (0..QUOTES.len()).collect();
        quote_order.shuffle(&mut rng);
        let mut song_order: Vec<usize> = (0..SONGS.len()).collect();
        song_order.shuffle(&mut rng);

        Self {
            section: Section::Home,
            should_quit: false,
            portfolio_cursor: 0,
            portfolio_scroll: 0,
            quote_order,
            quote_cursor: 0,
            song_order,
            song_cursor: 0,
            cols,
            rows,
        }
    }

    pub fn current_quote(&self) -> &'static Quote {
        &QUOTES[self.quote_order[self.quote_cursor]]
    }

    pub fn current_song(&self) -> &'static Song {
        &SONGS[self.song_order[self.song_cursor]]
    }

    /// Handle a key press. Returns true if the UI needs to be redrawn.
    pub fn handle_key(&mut self, key: Key) -> bool {
        match &key {
            // Global quit
            Key::Char('q') | Key::Char('Q') => {
                self.should_quit = true;
                return true;
            }
            // Global section jump (1-4)
            Key::Char('1') => {
                self.set_section(Section::Home);
                return true;
            }
            Key::Char('2') => {
                self.set_section(Section::About);
                return true;
            }
            Key::Char('3') => {
                self.set_section(Section::Portfolio);
                return true;
            }
            Key::Char('4') => {
                self.set_section(Section::Contact);
                return true;
            }
            // Tab / Shift-Tab cycle through sections
            Key::Tab => {
                let next = (self.section.index() + 1) % Section::ALL.len();
                self.section = Section::ALL[next].clone();
                return true;
            }
            Key::BackTab => {
                let prev = self
                    .section
                    .index()
                    .checked_sub(1)
                    .unwrap_or(Section::ALL.len() - 1);
                self.section = Section::ALL[prev].clone();
                return true;
            }
            _ => {}
        }

        // Section-local key handling
        match &self.section {
            Section::Home => self.handle_home(key),
            Section::About => self.handle_about(key),
            Section::Portfolio => self.handle_portfolio(key),
            Section::Contact => self.handle_contact(key),
        }
    }

    fn set_section(&mut self, s: Section) {
        self.section = s;
    }

    // ------------------------------------------------------------------
    // Section handlers
    // ------------------------------------------------------------------

    fn handle_home(&mut self, key: Key) -> bool {
        match key {
            Key::Right | Key::Char('l') | Key::Char('n') => {
                self.quote_cursor = (self.quote_cursor + 1) % self.quote_order.len();
                true
            }
            Key::Left | Key::Char('h') | Key::Char('p') => {
                self.quote_cursor = self
                    .quote_cursor
                    .checked_sub(1)
                    .unwrap_or(self.quote_order.len() - 1);
                true
            }
            _ => false,
        }
    }

    fn handle_about(&mut self, key: Key) -> bool {
        match key {
            Key::Right | Key::Char('l') => {
                self.song_cursor = (self.song_cursor + 1) % self.song_order.len();
                true
            }
            Key::Left | Key::Char('h') => {
                self.song_cursor = self
                    .song_cursor
                    .checked_sub(1)
                    .unwrap_or(self.song_order.len() - 1);
                true
            }
            Key::Escape | Key::Backspace => {
                self.set_section(Section::Home);
                true
            }
            _ => false,
        }
    }

    fn handle_portfolio(&mut self, key: Key) -> bool {
        match key {
            Key::Left | Key::Char('h') => {
                if self.portfolio_cursor > 0 {
                    self.portfolio_cursor -= 1;
                    self.portfolio_scroll = 0;
                }
                true
            }
            Key::Right | Key::Char('l') => {
                if self.portfolio_cursor + 1 < PROJECTS.len() {
                    self.portfolio_cursor += 1;
                    self.portfolio_scroll = 0;
                }
                true
            }
            Key::Up | Key::Char('k') => {
                self.portfolio_scroll = self.portfolio_scroll.saturating_sub(1);
                true
            }
            Key::Down | Key::Char('j') => {
                self.portfolio_scroll = self.portfolio_scroll.saturating_add(1);
                true
            }
            Key::Escape | Key::Backspace => {
                self.set_section(Section::Home);
                true
            }
            _ => false,
        }
    }

    fn handle_contact(&mut self, _key: Key) -> bool {
        false
    }

    /// Called when the terminal is resized.
    pub fn resize(&mut self, cols: u16, rows: u16) {
        self.cols = cols;
        self.rows = rows;
    }
}
