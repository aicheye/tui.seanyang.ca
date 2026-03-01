use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{List, ListItem, Paragraph, Wrap},
};

use super::theme;
use crate::{data::socials::SOCIALS, section::SectionView};

pub struct ContactSection;

impl ContactSection {
    pub fn new() -> Self {
        Self
    }
}

impl SectionView for ContactSection {
    fn label(&self) -> &'static str {
        "Contact"
    }

    fn render(&self, f: &mut Frame, area: Rect) {
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // top margin
                Constraint::Min(1),    // content
            ])
            .split(area);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(44),
                Constraint::Length(2),
                Constraint::Percentage(56),
            ])
            .split(rows[1]);

        render_blurb(f, chunks[0]);
        render_socials(f, chunks[2]);
    }
}

fn render_blurb(f: &mut Frame, area: Rect) {
    let text = vec![
        theme::divider("contact", area.width),
        Line::from(""),
        Line::from(Span::styled("Get in touch", theme::primary_bold())),
        Line::from(""),
        Line::from(Span::styled(
            "The fastest way to reach me is via Instagram DMs. \
             I try to respond within a day.",
            theme::body(),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "For professional enquiries, LinkedIn works best, \
            or shoot me an email if you prefer.",
            theme::body(),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "I'm always happy to chat about:",
            theme::primary_bold(),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("  · ", theme::primary()),
            Span::styled("Software projects", theme::body()),
        ]),
        Line::from(vec![
            Span::styled("  · ", theme::primary()),
            Span::styled("Internship opportunities", theme::body()),
        ]),
        Line::from(vec![
            Span::styled("  · ", theme::primary()),
            Span::styled("Urbanism, transit, cities ", theme::body()),
        ]),
        Line::from(vec![
            Span::styled("  · ", theme::primary()),
            Span::styled("Music recommendations", theme::body()),
        ]),
        Line::from(vec![
            Span::styled("  · ", theme::primary()),
            Span::styled("Films", theme::body()),
        ]),
    ];

    let para = Paragraph::new(text).wrap(Wrap { trim: true });
    f.render_widget(para, area);
}

fn render_socials(f: &mut Frame, area: Rect) {
    let mut lines: Vec<ListItem> = Vec::new();

    // Divider as first item
    lines.push(ListItem::new(vec![
        theme::divider("socials", area.width),
        Line::from(""),
    ]));

    for s in SOCIALS.iter() {
        lines.push(ListItem::new(vec![
            Line::from(vec![
                Span::styled(s.name, theme::primary_bold()),
                Span::styled(" :: ", theme::secondary()),
                Span::styled(s.handle, theme::body()),
            ]),
            Line::from(Span::styled(
                s.url,
                Style::default()
                    .fg(theme::MUTED)
                    .add_modifier(Modifier::UNDERLINED),
            )),
            Line::from(""),
        ]));
    }

    let list = List::new(lines);
    f.render_widget(list, area);
}
