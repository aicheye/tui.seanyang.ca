pub mod about;
pub mod contact;
pub mod home;
pub mod portfolio;
pub mod theme;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs},
};

use crate::app::App;

/// Top-level render: draws the chrome (nav bar + footer) then delegates to
/// the active section renderer.
pub fn render(f: &mut Frame, app: &App) {
    let area = f.area();

    // Constrain and centre the entire chrome within configurable bounds.
    let max_width: u16 = 80;
    let min_width: u16 = 60;
    let max_height: u16 = 40;

    let width = area.width.min(max_width).max(min_width);
    let height = area.height.min(max_height);
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    let padded = Rect::new(x, y, width.min(area.width), height.min(area.height));

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4), // top padding
            Constraint::Length(3), // nav bar (tabs + border)
            Constraint::Min(1),    // section content
            Constraint::Length(1), // footer
            Constraint::Length(4), // bottom padding
        ])
        .split(padded);

    render_nav(f, app, chunks[1]);
    app.sections[app.active].render(f, chunks[2]);
    render_footer(f, chunks[3]);
}

// ---------------------------------------------------------------------------
// Chrome
// ---------------------------------------------------------------------------

fn render_nav(f: &mut Frame, app: &App, area: Rect) {
    let tab_titles: Vec<Line> = app
        .sections
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
        .title(Span::styled(" seanyang.me ", theme::primary()));

    let tabs = Tabs::new(tab_titles)
        .block(block)
        .select(app.active)
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

fn render_footer(f: &mut Frame, area: Rect) {
    let key = Style::default()
        .fg(theme::PRIMARY)
        .add_modifier(Modifier::BOLD);
    let label = Style::default().fg(theme::HI);

    // "<q> quit" = 8 chars
    let bar_width: u16 = 8;

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(bar_width.min(area.width)),
        ])
        .split(area);

    let made_with = Line::from(vec![Span::styled(
        "Made with ❤︎  in Rust",
        theme::secondary(),
    )]);
    f.render_widget(Paragraph::new(made_with), cols[0]);

    let text = Line::from(vec![Span::styled("<q>", key), Span::styled(" quit", label)]);
    let footer = Paragraph::new(text).style(Style::default());
    f.render_widget(footer, cols[1]);
}
