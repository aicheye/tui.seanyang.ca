use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use super::theme;
use crate::{
    data::socials::{SOCIALS, Social},
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
    let mid = SOCIALS.len().div_ceil(2);
    let h = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(3), // gap
            Constraint::Length(45),
        ])
        .split(area);

    f.render_widget(Paragraph::new(link_lines(&SOCIALS[..mid])), h[0]);
    f.render_widget(Paragraph::new(link_lines(&SOCIALS[mid..])), h[2]);
}

fn link_lines(socials: &'static [Social]) -> Vec<Line<'static>> {
    let mut lines: Vec<Line<'static>> = Vec::new();
    for s in socials {
        let url = s.url.trim_start_matches("mailto:");
        lines.push(Line::from(vec![
            Span::styled(s.name, theme::green_bold()),
            Span::styled("  ·  ", theme::secondary()),
            Span::styled(s.handle, theme::body()),
        ]));
        lines.push(Line::from(Span::styled(
            url,
            Style::default()
                .fg(theme::MUTED)
                .add_modifier(Modifier::UNDERLINED),
        )));
        lines.push(Line::from(""));
        lines.push(Line::from(""));
    }
    lines
}
