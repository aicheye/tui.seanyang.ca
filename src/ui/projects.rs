use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use super::theme;
use crate::{data::projects::PROJECTS, section::SectionView};

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
    let row_count = PROJECTS.len().div_ceil(COLS);

    let mut v_constraints = vec![];
    for _ in 0..row_count {
        v_constraints.push(Constraint::Length(CARD_H));
    }
    v_constraints.push(Constraint::Fill(1));

    let v_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(v_constraints)
        .split(area);

    for (row_idx, chunk) in PROJECTS.chunks(COLS).enumerate() {
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

fn render_card(f: &mut Frame, p: &crate::data::projects::Project, area: Rect) {
    let lang_str = p.languages.join(" · ");
    let url = p.github.map(|u| u.trim_end_matches('/')).unwrap_or("");

    let lines: Vec<Line<'static>> = vec![
        Line::from(vec![Span::styled(p.title, theme::green_bold())]),
        Line::from(vec![Span::styled(p.description, theme::body())]),
        Line::from(vec![Span::styled(lang_str, theme::secondary())]),
        Line::from(vec![Span::styled(
            url,
            Style::default()
                .fg(theme::MUTED)
                .add_modifier(Modifier::UNDERLINED),
        )]),
        Line::from(""),
    ];

    f.render_widget(Paragraph::new(lines), area);
}
