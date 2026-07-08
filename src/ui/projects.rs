use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use super::{hyperlink::Hyperlink, theme};
use crate::{
    data::{Project, snapshot},
    section::SectionView,
};

const COLS: usize = 2;
const CARD_H: u16 = 5; // 4 content lines + 1 padding

pub struct ProjectsSection;

impl ProjectsSection {
    pub fn new() -> Self {
        Self
    }
}

impl SectionView for ProjectsSection {
    fn label(&self) -> &'static str {
        "Projects"
    }

    fn render(&self, f: &mut Frame, area: Rect) {
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // top margin
                Constraint::Min(1),    // grid
            ])
            .split(area);

        render_grid(f, rows[1]);
    }
}

fn render_grid(f: &mut Frame, area: Rect) {
    let data = snapshot();
    let projects = &data.projects;
    let row_count = projects.len().div_ceil(COLS);

    let mut v_constraints = vec![];
    for _ in 0..row_count {
        v_constraints.push(Constraint::Length(CARD_H));
    }
    v_constraints.push(Constraint::Fill(1));

    let v_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(v_constraints)
        .split(area);

    for (row_idx, chunk) in projects.chunks(COLS).enumerate() {
        let row_area = v_rows[row_idx];
        let h_cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(3), // gap
                Constraint::Fill(1),
            ])
            .split(row_area);

        for (col_idx, project) in chunk.iter().enumerate() {
            render_card(f, project, h_cols[col_idx * 2]);
        }
    }
}

fn render_card(f: &mut Frame, p: &Project, area: Rect) {
    let tech_str = p.technologies.join(" · ");
    let url = p.github.as_deref().map(|u| u.trim_end_matches('/'));

    let lines: Vec<Line<'static>> = vec![
        Line::from(vec![Span::styled(p.title.clone(), theme::green_bold())]),
        Line::from(vec![Span::styled(p.description.clone(), theme::body())]),
        Line::from(vec![Span::styled(tech_str, theme::secondary())]),
    ];
    f.render_widget(Paragraph::new(lines), area);

    if let Some(url) = url {
        let display = url
            .trim_start_matches("https://")
            .trim_start_matches("http://");
        let style = Style::default()
            .fg(theme::MUTED)
            .add_modifier(Modifier::UNDERLINED);
        f.render_widget(
            Hyperlink::new(Span::styled(display, style), url),
            Rect::new(area.x, area.y + 3, area.width, 1),
        );
    }
}
