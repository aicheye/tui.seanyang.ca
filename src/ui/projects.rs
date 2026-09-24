use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use super::{hyperlink::Hyperlink, scroll::Scroll, theme};
use crate::{
    data::{Project, snapshot},
    input::Key,
    section::SectionView,
};

const COLS: usize = 2;
const CARD_H: u16 = 5; // 4 content lines + 1 padding

pub struct ProjectsSection {
    scroll: Scroll,
}

impl ProjectsSection {
    pub fn new() -> Self {
        Self {
            scroll: Scroll::default(),
        }
    }
}

impl SectionView for ProjectsSection {
    fn label(&self) -> &'static str {
        "Projects"
    }

    fn handle_key(&mut self, key: Key) {
        self.scroll.handle_key(&key);
    }

    fn scrollable(&self) -> bool {
        self.scroll.scrollable()
    }

    fn render(&self, f: &mut Frame, area: Rect) {
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // top margin
                Constraint::Min(1),    // grid
            ])
            .split(area);

        let data = snapshot();
        let projects = &data.projects;
        // The last row needs no padding line below it.
        let height = (projects.len().div_ceil(COLS) as u16 * CARD_H).saturating_sub(1);
        self.scroll.render(f, rows[1], height, |buf, area| {
            render_grid(buf, area, projects)
        });
    }
}

fn render_grid(buf: &mut Buffer, area: Rect, projects: &[Project]) {
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
            render_card(buf, project, h_cols[col_idx * 2]);
        }
    }
}

fn render_card(buf: &mut Buffer, p: &Project, area: Rect) {
    let tech_str = p.technologies.join(" · ");
    let url = p.github.as_deref().map(|u| u.trim_end_matches('/'));

    let lines: Vec<Line<'static>> = vec![
        Line::from(vec![Span::styled(p.title.clone(), theme::green_bold())]),
        Line::from(vec![Span::styled(p.description.clone(), theme::body())]),
        Line::from(vec![Span::styled(tech_str, theme::secondary())]),
    ];
    Paragraph::new(lines).render(area, buf);

    if let Some(url) = url {
        let display = url
            .trim_start_matches("https://")
            .trim_start_matches("http://");
        let style = Style::default()
            .fg(theme::MUTED)
            .add_modifier(Modifier::UNDERLINED);
        Hyperlink::new(Span::styled(display, style), url)
            .render(Rect::new(area.x, area.y + 3, area.width, 1), buf);
    }
}
