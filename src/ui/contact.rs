use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{List, ListItem, Paragraph, Wrap},
};

use super::theme;
use crate::{app::App, data::socials::SOCIALS};

pub fn render(f: &mut Frame, _app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(44),
            Constraint::Length(2),
            Constraint::Percentage(54),
        ])
        .split(area);

    render_blurb(f, chunks[0]);
    render_socials(f, chunks[2]);
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
            "For professional enquiries, LinkedIn works best — \
             or fire off an email if you prefer.",
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
            Span::styled("Software projects & collaboration", theme::body()),
        ]),
        Line::from(vec![
            Span::styled("  · ", theme::primary()),
            Span::styled("Internship / co-op opportunities", theme::body()),
        ]),
        Line::from(vec![
            Span::styled("  · ", theme::primary()),
            Span::styled("Urbanism, transit, city design", theme::body()),
        ]),
        Line::from(vec![
            Span::styled("  · ", theme::primary()),
            Span::styled("Music recommendations", theme::body()),
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
