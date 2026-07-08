pub mod experience;
pub mod home;
mod hyperlink;
pub mod links;
#[cfg(test)]
mod osc8_probe;
pub mod projects;
pub mod theme;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs},
};

use crate::app::App;
use hyperlink::Hyperlink;

/// Top-level render: draws the chrome (nav bar + footer) then delegates to
/// the active section renderer.
pub fn render(f: &mut Frame, app: &App) {
    let area = f.area();

    let target_width: u16 = 80;
    let target_height: u16 = 22;

    if area.width < target_width || area.height < target_height {
        render_too_small(f, area);
        return;
    }

    let x = area.x + (area.width - target_width) / 2;
    let y = area.y + (area.height - target_height) / 2;
    let padded = Rect::new(x, y, target_width, target_height);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // nav bar (tabs + border)
            Constraint::Min(1),    // section content
            Constraint::Length(1), // footer
        ])
        .split(padded);

    render_nav(f, app, chunks[0]);
    app.sections[app.active].render(f, chunks[1]);
    render_footer(f, chunks[2]);
}

// ---------------------------------------------------------------------------
// Too-small fallback
// ---------------------------------------------------------------------------

fn render_too_small(f: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme::secondary());
    let inner = block.inner(area);
    f.render_widget(block, area);

    if inner.height == 0 || inner.width == 0 {
        return;
    }

    // Fill every cell with ░
    let fill_row = "░".repeat(inner.width as usize);
    let fill_lines: Vec<Line> = (0..inner.height)
        .map(|_| Line::from(Span::styled(fill_row.clone(), theme::secondary())))
        .collect();
    f.render_widget(Paragraph::new(fill_lines), inner);

    // Overlay "not enough space" centered vertically with 1-row margins
    let msg_text = "  not enough space  ";
    let blank = Line::from(" ".repeat(msg_text.len()));
    let msg = Line::from(Span::styled(msg_text, theme::body()));
    let text_y = (inner.y + inner.height / 2).saturating_sub(1);
    let text_area = Rect::new(inner.x, text_y, inner.width, 3);
    f.render_widget(
        Paragraph::new(vec![blank.clone(), msg, blank]).alignment(Alignment::Center),
        text_area,
    );
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

    let resume_url = "https://seanyang.me/resume";
    let resume_display = resume_url
        .trim_start_matches("https://")
        .trim_start_matches("http://");

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(47), Constraint::Fill(1)])
        .split(area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme::secondary())
        .title(Span::styled(" seanyang.me ", theme::secondary()));

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
        .split(cols[1]);
    let link_area = resume_rows[1];
    let link_w = (resume_display.chars().count() as u16).min(link_area.width);
    let link_x = link_area.x + link_area.width - link_w;
    f.render_widget(
        Hyperlink::new(
            Span::styled(
                resume_display,
                theme::secondary().add_modifier(Modifier::UNDERLINED),
            ),
            resume_url,
        ),
        Rect::new(link_x, link_area.y, link_w, 1),
    );
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
