//! A clickable-in-terminal link widget using the OSC 8 escape sequence
//! (supported by iTerm2, kitty, WezTerm, Windows Terminal, and others).
//!
//! Ratatui's buffer is cell-based and doesn't understand escape sequences, so
//! this follows ratatui's own `hyperlink` example: render the visible text
//! normally, then smuggle the OSC 8 codes back in by rewriting cell symbols.

use ratatui::{buffer::Buffer, layout::Rect, text::Line, widgets::Widget};

pub struct Hyperlink<'a> {
    text: Line<'a>,
    url: &'a str,
}

impl<'a> Hyperlink<'a> {
    pub fn new(text: impl Into<Line<'a>>, url: &'a str) -> Self {
        Self {
            text: text.into(),
            url,
        }
    }
}

impl Widget for Hyperlink<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let rendered = self.text.to_string();
        self.text.render(area, buf);

        let max_x = area.x + area.width;
        let chars: Vec<char> = rendered.chars().collect();
        for (i, pair) in chars.chunks(2).enumerate() {
            let x = area.x + i as u16 * 2;
            if x >= max_x {
                break;
            }
            let chunk: String = pair.iter().collect();
            let hyperlink = format!("\x1b]8;;{}\x07{chunk}\x1b]8;;\x07", self.url);
            if let Some(cell) = buf.cell_mut((x, area.y)) {
                cell.set_symbol(&hyperlink);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{Terminal, backend::TestBackend};

    #[test]
    fn packs_two_visible_chars_per_rewritten_cell() {
        let backend = TestBackend::new(30, 1);
        let mut term = Terminal::new(backend).unwrap();
        term.draw(|f| {
            let area = Rect::new(0, 0, 25, 1);
            f.render_widget(Hyperlink::new("github.com", "https://github.com"), area);
        })
        .unwrap();

        let buf = term.backend().buffer();
        // Even cells (0, 2, 4, ...) carry two real chars wrapped in OSC 8 —
        // this is exactly what gets sent to a real terminal.
        assert_eq!(
            buf.cell((0, 0)).unwrap().symbol(),
            "\x1b]8;;https://github.com\x07gi\x1b]8;;\x07"
        );
        assert_eq!(
            buf.cell((2, 0)).unwrap().symbol(),
            "\x1b]8;;https://github.com\x07th\x1b]8;;\x07"
        );
        // "github.com" is 10 chars -> 5 pairs; the last covers "o","m".
        assert_eq!(
            buf.cell((8, 0)).unwrap().symbol(),
            "\x1b]8;;https://github.com\x07om\x1b]8;;\x07"
        );

        // Odd cells are never sent by Buffer::diff (the preceding wide cell
        // covers them), so they stay at TestBackend's initial blank state
        assert_eq!(buf.cell((1, 0)).unwrap().symbol(), " ");

        // Nothing past the visible text was touched.
        let after = buf.cell((10, 0)).unwrap().symbol();
        assert_eq!(after, " ");
    }
}
