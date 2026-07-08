//! A clickable-in-terminal link widget using the OSC 8 escape sequence
//! (supported by iTerm2, kitty, WezTerm, Windows Terminal, and others).
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

        // Without an explicit id, terminals guess which cells form one link:
        // VTE assigns a fresh id per OSC 8 run, while iTerm2 connects
        // adjacent same-URI cells on the (soft-wrap-joined) logical line.
        // Both heuristics misjudge the link's extent under ratatui's partial
        // redraws, so pin it down with a stable id.
        let id = link_id(area.x, area.y, self.url);
        let hyperlink = format!("\x1b]8;id={id};{}\x07{visible}\x1b]8;;\x07", self.url);
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

        // Ghostty's default Ctrl/Cmd-hover URL matcher folds the trailing
        // blank run into its match when URL-ish text is the last thing on
        // the row (its path regex branches end with `(?: +(?= *$))?`),
        // underlining from the link to the row edge. A no-break space right
        // after the link renders as the same blank but is neither a space
        // nor a path character, so the match stops at the link text. When
        // the link fills its area the guard lands one cell outside it —
        // that cell is blank margin in every current layout, the write is
        // bounds-checked, and cells holding real content are left alone.
        if let Some(cell) = buf.cell_mut((area.x.saturating_add(width), area.y))
            && cell.symbol() == " "
        {
            cell.set_symbol("\u{a0}");
        }
    }
}

/// A stable identifier for the link at a given screen position: the same
/// link keeps the same id across redraws, and two on-screen links never
/// share one (FNV-1a over the URL, mixed with the position).
fn link_id(x: u16, y: u16, url: &str) -> String {
    const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in url.as_bytes() {
        h ^= u64::from(*b);
        h = h.wrapping_mul(FNV_PRIME);
    }
    h ^= u64::from(x) | (u64::from(y) << 16);
    h = h.wrapping_mul(FNV_PRIME);
    format!("{:08x}", (h ^ (h >> 32)) as u32)
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
        let id = link_id(0, 0, "https://github.com");
        assert_eq!(
            buf.cell((0, 0)).unwrap().symbol(),
            format!("\x1b]8;id={id};https://github.com\x07github.com\x1b]8;;\x07")
        );

        // The cells covered by the link's text are skipped so Buffer::diff
        // never emits them ("github.com" is 10 columns -> cells 1..=9).
        for x in 1..10u16 {
            let cell = buf.cell((x, 0)).unwrap();
            assert!(cell.skip, "cell {x} should be skipped");
        }

        // The cell after the link holds the no-break-space guard that stops
        // terminal-side URL matchers from running into the trailing blanks.
        let after = buf.cell((10, 0)).unwrap();
        assert!(!after.skip);
        assert_eq!(after.symbol(), "\u{a0}");

        // Cells past the guard stay untouched.
        let past = buf.cell((11, 0)).unwrap();
        assert!(!past.skip);
        assert_eq!(past.symbol(), " ");
    }

    #[test]
    fn guard_never_clobbers_real_content() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 30, 1));
        buf.set_string(10, 0, "x", ratatui::style::Style::default());
        let area = Rect::new(0, 0, 10, 1);
        Hyperlink::new("github.com", "https://github.com").render(area, &mut buf);

        // "github.com" fills the area, so the guard cell falls just past it;
        // it must not overwrite the neighbouring content.
        assert_eq!(buf.cell((10, 0)).unwrap().symbol(), "x");
    }

    #[test]
    fn truncates_to_the_area() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 1));
        let area = Rect::new(0, 0, 6, 1);
        Hyperlink::new("github.com", "https://github.com").render(area, &mut buf);

        let id = link_id(0, 0, "https://github.com");
        assert_eq!(
            buf.cell((0, 0)).unwrap().symbol(),
            format!("\x1b]8;id={id};https://github.com\x07github\x1b]8;;\x07")
        );
        for x in 1..6u16 {
            assert!(buf.cell((x, 0)).unwrap().skip);
        }
        let after = buf.cell((6, 0)).unwrap();
        assert!(!after.skip);
        assert_eq!(after.symbol(), "\u{a0}");
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
            // x = 29 holds the no-break-space guard; the rest are blanks.
            let expected = if x == 29 { "\u{a0}" } else { " " };
            assert!(
                updates
                    .iter()
                    .any(|(ux, _, cell)| *ux == x && cell.symbol() == expected),
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
        let id = link_id(0, 0, "https://github.com");
        assert_eq!(
            buf.cell((0, 0)).unwrap().symbol(),
            format!("\x1b]8;id={id};https://github.com\x07github.com\x1b]8;;\x07")
        );
        for x in 1..10u16 {
            assert_eq!(buf.cell((x, 0)).unwrap().symbol(), " ");
        }
    }
}
