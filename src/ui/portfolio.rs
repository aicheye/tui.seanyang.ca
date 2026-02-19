use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{List, ListItem, ListState, Paragraph, Wrap},
};

use super::theme;
use crate::{app::App, data::portfolio::PROJECTS};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
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
            Constraint::Percentage(39),
            Constraint::Length(2),
            Constraint::Percentage(59),
        ])
        .split(rows[1]);

    render_list(f, app, chunks[0]);
    render_preview(f, app, chunks[2]);
}

fn render_list(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = PROJECTS
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let is_selected = i == app.portfolio_cursor;
            if is_selected {
                ListItem::new(Line::from(vec![
                    Span::styled("▸ ", theme::primary_bold()),
                    Span::styled(p.title, theme::primary_bold()),
                    Span::styled(format!("  {}", p.year), theme::primary()),
                ]))
            } else {
                ListItem::new(Line::from(vec![
                    Span::styled("  ", theme::secondary()),
                    Span::styled(p.title, theme::body()),
                    Span::styled(format!("  {}", p.year), theme::secondary()),
                ]))
            }
        })
        .collect();

    let list_height = PROJECTS.len() as u16 + 2;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),           // divider
            Constraint::Length(1),           // spacer
            Constraint::Length(list_height), // list
            Constraint::Length(1),           // hint
            Constraint::Min(0),              // spacer
        ])
        .split(area);

    f.render_widget(
        Paragraph::new(theme::divider("selected works", area.width)),
        chunks[0],
    );

    let list = List::new(items);
    let mut state = ListState::default();
    state.select(Some(app.portfolio_cursor));
    f.render_stateful_widget(list, chunks[2], &mut state);

    let hint = Line::from(vec![
        Span::styled("←/→ ", theme::primary()),
        Span::styled("select", theme::secondary()),
        Span::styled("   ↑/↓ ", theme::primary()),
        Span::styled("scroll", theme::secondary()),
    ]);
    f.render_widget(
        Paragraph::new(hint).alignment(ratatui::layout::Alignment::Center),
        chunks[3],
    );
}

fn render_preview(f: &mut Frame, app: &App, area: Rect) {
    let p = &PROJECTS[app.portfolio_cursor];

    let lang_str = p.languages.join("  ·  ");
    let feature_lines: Vec<Line> = p
        .features
        .iter()
        .map(|feat| {
            Line::from(vec![
                Span::styled("  ✓ ", theme::primary()),
                Span::styled(*feat, theme::body()),
            ])
        })
        .collect();

    let mut text = vec![
        theme::divider("details", area.width),
        Line::from(""),
        Line::from(vec![
            Span::styled(p.title, theme::primary_bold()),
            Span::styled(
                format!("  {}  ·  {}", p.year, p.category),
                theme::secondary(),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(p.description, theme::body())),
        Line::from(""),
        Line::from(Span::styled(p.long_description, theme::body())),
        Line::from(""),
        Line::from(Span::styled(lang_str, theme::secondary())),
        Line::from(""),
    ];
    text.extend(feature_lines);

    if let Some(url) = p.github {
        text.push(Line::from(""));
        text.push(Line::from(vec![
            Span::styled("github  ", theme::secondary()),
            Span::styled(
                url,
                Style::default()
                    .fg(theme::PRIMARY)
                    .add_modifier(ratatui::style::Modifier::UNDERLINED),
            ),
        ]));
    }

    // Calculate max scroll
    let content_lines = text.len() as u16;
    let max_scroll = content_lines.saturating_sub(area.height);
    let clamped_scroll = app.portfolio_scroll.min(max_scroll);

    let para = Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .scroll((clamped_scroll, 0));
    f.render_widget(para, area);
}
