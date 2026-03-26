use chai_framework::ChaiApp;
use ratatui::Frame;

use crate::input::{Key, parse_keys};
use crate::section::SectionView;
use crate::ui;
use crate::ui::{
    experience::ExperienceSection, home::HomeSection, links::LinksSection,
    projects::ProjectsSection,
};

/// Number of sections. Determines valid 1-N jump keys.
pub const NUM_SECTIONS: usize = 4;

/// The full application state.
pub struct App {
    /// Ordered sections — index determines nav position and 1-N shortcut.
    pub sections: [Box<dyn SectionView>; NUM_SECTIONS],
    /// Index of the active section in `sections`.
    pub active: usize,
    pub quit: bool,
}

impl ChaiApp for App {
    fn new() -> Self {
        Self {
            sections: [
                Box::new(HomeSection::new()),
                Box::new(ExperienceSection::new()),
                Box::new(ProjectsSection::new()),
                Box::new(LinksSection::new()),
            ],
            active: 0,
            quit: false,
        }
    }

    fn update(&mut self) {
        self.sections[self.active].update();
    }

    fn draw(&mut self, f: &mut Frame) {
        ui::render(f, self);
    }

    fn handle_input(&mut self, data: &[u8]) {
        for key in parse_keys(data) {
            self.handle_key(key);
        }
    }

    fn should_quit(&self) -> bool {
        self.quit
    }
}

impl App {
    /// Handle a key press. Returns true if the UI needs to be redrawn.
    pub fn handle_key(&mut self, key: Key) {
        match &key {
            // Global quit
            Key::Char('q') | Key::Char('Q') => {
                self.quit = true;
            }
            // Global section jump (1-based)
            Key::Char(c @ '1'..='9') => {
                let idx = (*c as usize) - ('1' as usize);
                if idx < NUM_SECTIONS {
                    self.active = idx;
                }
            }
            // Tab / Shift-Tab cycle through sections
            Key::Tab => {
                self.active = (self.active + 1) % NUM_SECTIONS;
            }
            Key::BackTab => {
                self.active = self.active.checked_sub(1).unwrap_or(NUM_SECTIONS - 1);
            }
            _ => {}
        }

        // Delegate to the active section
        self.sections[self.active].handle_key(key);
    }
}
