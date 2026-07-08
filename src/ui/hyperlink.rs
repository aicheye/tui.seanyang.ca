//! A clickable-in-terminal link widget using the OSC 8 escape sequence
//! (supported by iTerm2, kitty, WezTerm, Windows Terminal, and others).
//!
//! Ratatui's buffer is cell-based and doesn't understand escape sequences, so
//! the visible text is rendered normally and the OSC 8 codes are smuggled back
//! in by rewriting cell symbols. Unlike ratatui's `hyperlink` example (which
//! splits the link into two-character chunks, each with its own escape
//! sequence), the whole link goes into the *first* cell as a single OSC 8
//! region and the remaining cells are flagged with `skip`:
//!
//! - One OSC 8 region per link means terminals treat it as one link (separate
//!   regions per chunk show up as several adjacent links with broken hover
//!   highlighting).
//! - `Buffer::diff` measures a cell by the unicode width of its symbol, and
//!   the escape sequence's URL bytes are printable, so a rewritten cell looks
//!   dozens of columns wide. With chunks, that inflated width suppressed the
//!   update of whichever cell followed a chunk, leaving stale characters from
//!   the previous frame on screen after the end of odd-length links. Skipped
//!   cells are never diffed or emitted, so the single rewritten cell is the
//!   only place the accounting has to be right.
//!
//! The text is expected to be styled uniformly: the terminal renders all of it
//! with the first cell's style.

use ratatui::{buffer::Buffer, layout::Rect, text::Line, widgets::Widget};
use unicode_width::UnicodeWidthChar;

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
        // Lay the styled text down normally so the first cell picks up the
        // link's style (and the buffer stays inspectable in tests).
        let rendered = self.text.to_string();
        self.text.render(area, buf);

        // The portion of the text that actually fit in the area.
        let mut visible = String::new();
        let mut width: u16 = 0;
        for ch in rendered.chars() {
            let w = ch.width().unwrap_or(0) as u16;
            if width + w > area.width {
                break;
            }
            visible.push(ch);
            width += w;
        }
        if width == 0 {
            return;
        }

        let hyperlink = format!("\x1b]8;;{}\x07{visible}\x1b]8;;\x07", self.url);
        if let Some(cell) = buf.cell_mut((area.x, area.y)) {
            cell.set_symbol(&hyperlink);
        }
        // The first cell prints every column of the link, so the cells it
        // covers must never be emitted on their own.
        for x in (area.x + 1)..(area.x + width) {
            if let Some(cell) = buf.cell_mut((x, area.y)) {
                cell.set_skip(true);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{Terminal, backend::TestBackend};

    #[test]
    fn packs_link_into_first_cell_and_skips_the_rest() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 30, 1));
        let area = Rect::new(0, 0, 25, 1);
        Hyperlink::new("github.com", "https://github.com").render(area, &mut buf);

        // The first cell carries the whole link as a single OSC 8 region —
        // this is exactly what gets sent to a real terminal.
        assert_eq!(
            buf.cell((0, 0)).unwrap().symbol(),
            "\x1b]8;;https://github.com\x07github.com\x1b]8;;\x07"
        );

        // The cells covered by the link's text are skipped so Buffer::diff
        // never emits them ("github.com" is 10 columns -> cells 1..=9).
        for x in 1..10u16 {
            let cell = buf.cell((x, 0)).unwrap();
            assert!(cell.skip, "cell {x} should be skipped");
        }

        // Nothing past the visible text was touched.
        let after = buf.cell((10, 0)).unwrap();
        assert!(!after.skip);
        assert_eq!(after.symbol(), " ");
    }

    #[test]
    fn truncates_to_the_area() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 1));
        let area = Rect::new(0, 0, 6, 1);
        Hyperlink::new("github.com", "https://github.com").render(area, &mut buf);

        assert_eq!(
            buf.cell((0, 0)).unwrap().symbol(),
            "\x1b]8;;https://github.com\x07github\x1b]8;;\x07"
        );
        for x in 1..6u16 {
            assert!(buf.cell((x, 0)).unwrap().skip);
        }
        assert!(!buf.cell((6, 0)).unwrap().skip);
    }

    #[test]
    fn clears_stale_cells_after_the_link_on_redraw() {
        // Regression test: with per-chunk OSC 8 rewriting, Buffer::diff's
        // width accounting suppressed the update of the cell following an
        // odd-length link, so switching tabs left stray characters from the
        // previous frame right after the link (e.g. "…apexblun").
        let area = Rect::new(0, 0, 40, 1);
        let mut prev = Buffer::empty(area);
        prev.set_string(
            0,
            0,
            "instagram.com/seanyang_esports_gaming",
            ratatui::style::Style::default(),
        );

        let mut next = Buffer::empty(area);
        // 29 columns (odd) — the old chunked code failed to clear x = 29.
        Hyperlink::new(
            "open.spotify.com/user/apexblu",
            "https://open.spotify.com/user/apexblu",
        )
        .render(area, &mut next);

        let updates = prev.diff(&next);
        for x in 29..37u16 {
            assert!(
                updates
                    .iter()
                    .any(|(ux, _, cell)| *ux == x && cell.symbol() == " "),
                "stale cell {x} after the link should be cleared"
            );
        }
    }

    #[test]
    fn emits_only_the_link_cell_to_the_terminal() {
        let backend = TestBackend::new(30, 1);
        let mut term = Terminal::new(backend).unwrap();
        term.draw(|f| {
            let area = Rect::new(0, 0, 25, 1);
            f.render_widget(Hyperlink::new("github.com", "https://github.com"), area);
        })
        .unwrap();

        // Only the first cell reaches the backend; the covered cells are
        // never emitted (the terminal fills them when it prints the link).
        let buf = term.backend().buffer();
        assert_eq!(
            buf.cell((0, 0)).unwrap().symbol(),
            "\x1b]8;;https://github.com\x07github.com\x1b]8;;\x07"
        );
        for x in 1..10u16 {
            assert_eq!(buf.cell((x, 0)).unwrap().symbol(), " ");
        }
    }
}
