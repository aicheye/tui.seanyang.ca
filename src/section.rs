use ratatui::{Frame, layout::Rect};

use crate::input::Key;

/// Common interface for all navigable sections.
pub trait SectionView: Send {
    /// Label shown in the nav bar (e.g. "Home", "About").
    fn label(&self) -> &'static str;

    /// Handle a section-local key press.
    fn handle_key(&mut self, key: Key) {
        // By default, sections don't handle any keys.
        let _ = key;
    }

    /// Render this section into the given area.
    fn render(&self, f: &mut Frame, area: Rect);

    /// Optional periodic update logic.
    fn update(&mut self) {
        // No periodic update logic needed by default.
    }
}
