use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
};

use super::{hyperlink::Hyperlink, theme};
use crate::{
    data::{Social, snapshot},
    section::SectionView,
};

pub struct LinksSection;

impl LinksSection {
    pub fn new() -> Self {
        Self
    }
}

impl SectionView for LinksSection {
    fn label(&self) -> &'static str {
        "Links"
    }

    fn render(&self, f: &mut Frame, area: Rect) {
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // top margin
                Constraint::Min(1),    // links
            ])
            .split(area);

        render_links(f, rows[1]);
    }
}

fn render_links(f: &mut Frame, area: Rect) {
    let data = snapshot();
    let socials = &data.socials;
    let mid = socials.len().div_ceil(2);
    let h = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(3), // gap
            Constraint::Fill(1),
        ])
        .split(area);

    render_column(f, h[0], &socials[..mid]);
    render_column(f, h[2], &socials[mid..]);
}

/// Height in rows of one social entry: title/handle, url, and a blank gap.
const ENTRY_HEIGHT: u16 = 4;

fn render_column(f: &mut Frame, area: Rect, socials: &[Social]) {
    for (i, s) in socials.iter().enumerate() {
        let y = area.y + i as u16 * ENTRY_HEIGHT;
        if y >= area.y + area.height {
            break;
        }

        f.render_widget(
            Line::from(vec![
                Span::styled(s.label.clone(), theme::green_bold()),
                Span::styled("  ·  ", theme::secondary()),
                Span::styled(s.handle.clone(), theme::body()),
            ]),
            Rect::new(area.x, y, area.width, 1),
        );

        if y + 1 < area.y + area.height {
            let display = s
                .url
                .trim_start_matches("https://")
                .trim_start_matches("http://")
                .trim_start_matches("mailto:");
            let style = Style::default()
                .fg(theme::MUTED)
                .add_modifier(Modifier::UNDERLINED);
            f.render_widget(
                Hyperlink::new(Span::styled(display, style), &s.url),
                Rect::new(area.x, y + 1, area.width, 1),
            );
        }
    }
}
