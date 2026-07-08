//! A clickable-in-terminal link widget using the OSC 8 escape sequence
//! (supported by iTerm2, kitty, WezTerm, Windows Terminal, and others).
//!
//! No extra crate is needed: a `Cell`'s symbol can hold an arbitrary string,
//! so the escape codes ride along on the first and last visible cell without
//! changing how many cells the widget occupies — the displayed text can
//! differ from (and be shorter than) the URL it links to.
//!
//! Terminated with BEL (`\x07`) rather than ST (`ESC \`): several terminals
//! (and SSH clients relaying to them) mis-parse the two-byte ST form and leak
//! or drop characters right at the boundary, which BEL avoids.

use ratatui::{buffer::Buffer, layout::Rect, text::Span, widgets::Widget};

pub struct Hyperlink<'a> {
    text: Span<'a>,
    url: &'a str,
}

impl<'a> Hyperlink<'a> {
    pub fn new(text: impl Into<Span<'a>>, url: &'a str) -> Self {
        Self {
            text: text.into(),
            url,
        }
    }
}

impl Widget for Hyperlink<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let width = (self.text.width() as u16).min(area.width);
        if width == 0 {
            return;
        }
        buf.set_span(area.x, area.y, &self.text, width);

        let open = format!("\x1b]8;;{}\x07", self.url);
        let close = "\x1b]8;;\x07";
        let last_x = area.x + width - 1;

        if last_x == area.x {
            if let Some(cell) = buf.cell_mut((area.x, area.y)) {
                let sym = format!("{open}{}{close}", cell.symbol());
                cell.set_symbol(&sym);
            }
        } else {
            if let Some(cell) = buf.cell_mut((area.x, area.y)) {
                let sym = format!("{open}{}", cell.symbol());
                cell.set_symbol(&sym);
            }
            if let Some(cell) = buf.cell_mut((last_x, area.y)) {
                let sym = format!("{}{close}", cell.symbol());
                cell.set_symbol(&sym);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{Terminal, backend::TestBackend};

    #[test]
    fn wraps_visible_text_without_shifting_layout() {
        let backend = TestBackend::new(30, 1);
        let mut term = Terminal::new(backend).unwrap();
        term.draw(|f| {
            let area = Rect::new(0, 0, 25, 1);
            f.render_widget(
                Hyperlink::new("github.com/aicheye", "https://github.com/aicheye"),
                area,
            );
        })
        .unwrap();

        let buf = term.backend().buffer();
        let text = "github.com/aicheye";
        let first = buf.cell((0, 0)).unwrap().symbol();
        let last = buf
            .cell((text.chars().count() as u16 - 1, 0))
            .unwrap()
            .symbol();

        assert!(first.starts_with("\x1b]8;;https://github.com/aicheye\x07g"));
        assert_eq!(last, "e\x1b]8;;\x07");

        // The cell right after the visible text must be untouched (still a
        // plain space), proving the widget didn't widen past its real width.
        let after = buf.cell((text.chars().count() as u16, 0)).unwrap().symbol();
        assert_eq!(after, " ");
    }
}
