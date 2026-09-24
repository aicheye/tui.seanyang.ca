//! Vertical scrolling for sections whose content can outgrow the fixed
//! 80x22 frame. The content is drawn into an offscreen buffer at its full
//! height, and the visible window is copied into the frame next to a
//! scrollbar. Content that fits is drawn in place with no scrollbar.

use std::cell::Cell;

use ratatui::{Frame, buffer::Buffer, layout::Rect};

use super::theme;
use crate::input::Key;

/// Columns taken from the content when it scrolls: two blank columns plus
/// the scrollbar track.
const BAR_W: u16 = 3;

#[derive(Default)]
pub struct Scroll {
    /// First content row shown at the top of the viewport.
    offset: u16,
    /// Largest valid offset, recorded by the last render so key handling can
    /// clamp without knowing the content height.
    max: Cell<u16>,
    /// Viewport height from the last render, used for page up/down.
    page: Cell<u16>,
}

impl Scroll {
    /// True when the last render had content outside the viewport.
    pub fn scrollable(&self) -> bool {
        self.max.get() > 0
    }

    /// Apply a scroll key. Returns true if the key was a scroll key.
    pub fn handle_key(&mut self, key: &Key) -> bool {
        let max = self.max.get();
        let page = self.page.get().saturating_sub(1).max(1);
        self.offset = match key {
            Key::Up | Key::Char('k') => self.offset.saturating_sub(1),
            Key::Down | Key::Char('j') => self.offset.saturating_add(1),
            Key::PageUp | Key::Char('u') => self.offset.saturating_sub(page),
            Key::PageDown | Key::Char('d') | Key::Char(' ') => self.offset.saturating_add(page),
            Key::Home | Key::Char('g') => 0,
            Key::End | Key::Char('G') => max,
            _ => return false,
        }
        .min(max);
        true
    }

    /// Draw `content_height` rows of content into `area`. `draw` receives a
    /// buffer and the rect to draw the full content into.
    pub fn render(
        &self,
        f: &mut Frame,
        area: Rect,
        content_height: u16,
        draw: impl FnOnce(&mut Buffer, Rect),
    ) {
        self.page.set(area.height);

        if content_height <= area.height || area.width <= BAR_W {
            self.max.set(0);
            draw(f.buffer_mut(), area);
            return;
        }

        let max = content_height - area.height;
        self.max.set(max);
        let offset = self.offset.min(max);

        let content = Rect::new(0, 0, area.width - BAR_W, content_height);
        let mut off = Buffer::empty(content);
        draw(&mut off, content);

        let buf = f.buffer_mut();
        for row in 0..area.height {
            for col in 0..content.width {
                if let (Some(src), Some(dst)) = (
                    off.cell((col, offset + row)),
                    buf.cell_mut((area.x + col, area.y + row)),
                ) {
                    *dst = src.clone();
                }
            }
        }

        render_bar(
            buf,
            Rect::new(area.x + area.width - 1, area.y, 1, area.height),
            offset,
            max,
            content_height,
        );
    }
}

/// Scrollbar thumb sized to the visible fraction of the content, with
/// arrows in the end rows when there is more content in that direction. The
/// thumb spans the full height, so at either end it takes the row the absent
/// arrow would use. The rest of the track is left blank.
fn render_bar(buf: &mut Buffer, bar: Rect, offset: u16, max: u16, content_height: u16) {
    let mut put = |row: u16, symbol: &str| {
        if let Some(cell) = buf.cell_mut((bar.x, bar.y + row)) {
            cell.set_symbol(symbol).set_style(theme::secondary());
        }
    };

    let h = bar.height;
    let thumb_h = (u32::from(h) * u32::from(h) / u32::from(content_height)).max(1) as u16;
    let travel = h - thumb_h;
    let thumb_top =
        ((u32::from(offset) * u32::from(travel) + u32::from(max) / 2) / u32::from(max)) as u16;
    for row in thumb_top..thumb_top + thumb_h {
        put(row, "┃");
    }

    if offset > 0 {
        put(0, "▲");
    }
    if offset < max {
        put(h - 1, "▼");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{Terminal, backend::TestBackend, text::Line, widgets::Widget};

    fn draw_numbered(buf: &mut Buffer, area: Rect) {
        for i in 0..area.height {
            Line::from(format!("row {i}"))
                .render(Rect::new(area.x, area.y + i, area.width, 1), buf);
        }
    }

    fn row(term: &Terminal<TestBackend>, y: u16) -> String {
        let buf = term.backend().buffer();
        (0..buf.area.width)
            .map(|x| buf.cell((x, y)).unwrap().symbol().to_string())
            .collect()
    }

    #[test]
    fn fits_without_scrollbar() {
        let mut term = Terminal::new(TestBackend::new(20, 5)).unwrap();
        let s = Scroll::default();
        term.draw(|f| s.render(f, f.area(), 3, draw_numbered))
            .unwrap();
        assert!(!s.scrollable());
        assert_eq!(row(&term, 0).trim_end(), "row 0");
    }

    #[test]
    fn scrolls_and_clamps() {
        let mut term = Terminal::new(TestBackend::new(20, 4)).unwrap();
        let mut s = Scroll::default();
        term.draw(|f| s.render(f, f.area(), 10, draw_numbered))
            .unwrap();
        assert!(s.scrollable());
        assert!(row(&term, 0).ends_with('┃'));
        assert!(row(&term, 3).ends_with('▼'));

        s.handle_key(&Key::End);
        term.draw(|f| s.render(f, f.area(), 10, draw_numbered))
            .unwrap();
        assert!(row(&term, 0).starts_with("row 6"));
        assert!(row(&term, 0).ends_with('▲'));
        assert!(row(&term, 3).ends_with('┃'));

        // Overscrolling stays at the bottom, so one Up moves immediately.
        s.handle_key(&Key::Down);
        s.handle_key(&Key::Up);
        term.draw(|f| s.render(f, f.area(), 10, draw_numbered))
            .unwrap();
        assert!(row(&term, 0).starts_with("row 5"));
    }
}
