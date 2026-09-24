use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Widget,
};

use super::{hyperlink::Hyperlink, scroll::Scroll, theme};
use crate::{
    data::{Social, snapshot},
    input::Key,
    section::SectionView,
};

pub struct LinksSection {
    scroll: Scroll,
}

impl LinksSection {
    pub fn new() -> Self {
        Self {
            scroll: Scroll::default(),
        }
    }
}

impl SectionView for LinksSection {
    fn label(&self) -> &'static str {
        "Links"
    }

    fn handle_key(&mut self, key: Key) {
        self.scroll.handle_key(&key);
    }

    fn scrollable(&self) -> bool {
        self.scroll.scrollable()
    }

    fn render(&self, f: &mut Frame, area: Rect) {
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // top margin
                Constraint::Min(1),    // links
            ])
            .split(area);

        let data = snapshot();
        let socials = &data.socials;
        // The last entry needs no blank gap below it.
        let height = (socials.len().div_ceil(2) as u16 * ENTRY_HEIGHT).saturating_sub(2);
        self.scroll.render(f, rows[1], height, |buf, area| {
            render_links(buf, area, socials)
        });
    }
}

fn render_links(buf: &mut Buffer, area: Rect, socials: &[Social]) {
    let mid = socials.len().div_ceil(2);
    let h = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(3), // gap
            Constraint::Fill(1),
        ])
        .split(area);

    render_column(buf, h[0], &socials[..mid]);
    render_column(buf, h[2], &socials[mid..]);
}

/// Height in rows of one social entry: title/handle, url, and a blank gap.
const ENTRY_HEIGHT: u16 = 4;

fn render_column(buf: &mut Buffer, area: Rect, socials: &[Social]) {
    for (i, s) in socials.iter().enumerate() {
        let y = area.y + i as u16 * ENTRY_HEIGHT;
        if y >= area.y + area.height {
            break;
        }

        Line::from(vec![
            Span::styled(s.label.clone(), theme::green_bold()),
            Span::styled("  ·  ", theme::secondary()),
            Span::styled(s.handle.clone(), theme::body()),
        ])
        .render(Rect::new(area.x, y, area.width, 1), buf);

        if y + 1 < area.y + area.height {
            let display = s
                .url
                .trim_start_matches("https://")
                .trim_start_matches("http://")
                .trim_start_matches("mailto:");
            let style = Style::default()
                .fg(theme::MUTED)
                .add_modifier(Modifier::UNDERLINED);
            Hyperlink::new(Span::styled(display, style), &s.url)
                .render(Rect::new(area.x, y + 1, area.width, 1), buf);
        }
    }
}
