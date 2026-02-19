pub mod about;
pub mod contact;
pub mod home;
pub mod portfolio;
pub mod theme;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs},
};

use crate::app::{App, Section};

/// Top-level render: draws the chrome (nav bar + footer) then delegates to
/// the active section renderer.
pub fn render(f: &mut Frame, app: &App) {
    let area = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // nav bar (tabs + border)
            Constraint::Min(1),    // section content
            Constraint::Length(1), // footer
        ])
        .split(area);

    render_nav(f, app, chunks[0]);
    render_section(f, app, chunks[1]);
    render_footer(f, app, chunks[2]);
}

// ---------------------------------------------------------------------------
// Chrome
// ---------------------------------------------------------------------------

fn render_nav(f: &mut Frame, app: &App, area: Rect) {
    let tab_titles: Vec<Line> = Section::ALL
        .iter()
        .enumerate()
        .map(|(i, s)| {
            Line::from(vec![
                Span::styled(format!("{}", i + 1), theme::secondary()),
                Span::raw(" "),
                Span::styled(s.label(), theme::body()),
            ])
        })
        .collect();

    // "1 Home · 2 About · 3 Portfolio · 4 Contact" + borders + padding
    let tab_box_width = 52u16.min(area.width);
    let resume_url = "https://seanyang.me/resume";
    let resume_box_width = (resume_url.len() as u16).min(area.width);

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(tab_box_width),
            Constraint::Min(1),
            Constraint::Length(resume_box_width),
        ])
        .split(area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme::secondary())
        .title(Span::styled(" tui.seanyang.me ", theme::primary()));

    let tabs = Tabs::new(tab_titles)
        .block(block)
        .select(app.section.index())
        .style(theme::secondary())
        .highlight_style(theme::primary_bold())
        .divider(Span::styled(" · ", theme::secondary()));

    f.render_widget(tabs, cols[0]);

    let resume_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(cols[2]);
    let resume = Paragraph::new(Line::from(vec![Span::styled(
        resume_url,
        theme::secondary().add_modifier(Modifier::UNDERLINED),
    )]));
    f.render_widget(resume, resume_rows[1]);
}

fn render_footer(f: &mut Frame, _app: &App, area: Rect) {
    let text = Line::from(vec![
        Span::styled("tab", theme::primary()),
        Span::styled(" next  ", theme::secondary()),
        Span::styled("1-4", theme::primary()),
        Span::styled(" jump  ", theme::secondary()),
        Span::styled("q", theme::primary()),
        Span::styled(" quit", theme::secondary()),
    ]);
    let footer = Paragraph::new(text);
    f.render_widget(footer, area);
}

// ---------------------------------------------------------------------------
// Section dispatch
// ---------------------------------------------------------------------------

fn render_section(f: &mut Frame, app: &App, area: Rect) {
    let inner = centered_rect(area, 100, 32, 60, 20);

    match &app.section {
        Section::Home => home::render(f, app, inner),
        Section::About => about::render(f, app, inner),
        Section::Portfolio => portfolio::render(f, app, inner),
        Section::Contact => contact::render(f, app, inner),
    }
}

/// Returns a centered Rect with max width/height, enforcing minimums.
fn centered_rect(
    area: Rect,
    max_width: u16,
    max_height: u16,
    min_width: u16,
    min_height: u16,
) -> Rect {
    let width = area.width.min(max_width).max(min_width);
    let height = area.height.min(max_height).max(min_height);

    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;

    Rect::new(x, y, width.min(area.width), height.min(area.height))
}
