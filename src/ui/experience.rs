use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
};

use super::theme;
use crate::{data::snapshot, section::SectionView};

pub struct ExperienceSection;

impl ExperienceSection {
    pub fn new() -> Self {
        Self
    }
}

impl SectionView for ExperienceSection {
    fn label(&self) -> &'static str {
        "Exp"
    }

    fn render(&self, f: &mut Frame, area: Rect) {
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // top margin
                Constraint::Min(1),    // timeline
            ])
            .split(area);

        render_timeline(f, rows[1]);
    }
}

fn render_timeline(f: &mut Frame, area: Rect) {
    let data = snapshot();
    let mut lines: Vec<Line<'static>> = vec![];

    for job in data.jobs.iter() {
        let dot = if job.current { "●" } else { "○" };
        let dot_style = if job.current {
            theme::green_bold()
        } else {
            theme::secondary()
        };
        let pipe = Span::styled("│", theme::secondary());

        let dates = job.dates.join(" → ");

        // ●  dates                    location
        let header_left_w = 1 + 2 + dates.chars().count();
        let header_pad =
            (area.width as usize).saturating_sub(header_left_w + job.location.chars().count());
        lines.push(Line::from(vec![
            Span::styled(dot, dot_style),
            Span::raw("  "),
            Span::styled(dates, theme::secondary()),
            Span::raw(" ".repeat(header_pad)),
            Span::styled(job.location.clone(), theme::body()),
        ]));

        // │ title @ company                         website
        let website = job.website.trim_end_matches('/').to_string();
        let left_w = 1 + 2 + job.title.chars().count() + 5 + job.company.chars().count();
        let pad = (area.width as usize).saturating_sub(left_w + website.chars().count());
        lines.push(Line::from(vec![
            pipe.clone(),
            Span::raw("  "),
            Span::styled(job.title.clone(), theme::green_bold()),
            Span::styled("  @  ", theme::secondary()),
            Span::styled(job.company.clone(), theme::green_bold()),
            Span::raw(" ".repeat(pad)),
            Span::styled(
                website,
                Style::default()
                    .fg(theme::MUTED)
                    .add_modifier(Modifier::UNDERLINED),
            ),
        ]));

        // │  description
        lines.push(Line::from(vec![
            pipe.clone(),
            Span::raw("  "),
            Span::styled(job.description.clone(), theme::body()),
        ]));

        // │  tech stack (if any)
        if !job.technologies.is_empty() {
            let tech = job.technologies.join(" · ");
            lines.push(Line::from(vec![
                pipe.clone(),
                Span::raw("  "),
                Span::styled(tech, theme::secondary()),
            ]));
        }

        // trailing pipes
        lines.push(Line::from(vec![pipe.clone()]));
        lines.push(Line::from(vec![pipe.clone()]));
    }

    let p = Paragraph::new(lines).wrap(Wrap { trim: true });
    f.render_widget(p, area);
}
