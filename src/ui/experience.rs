use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
};

use super::{hyperlink::Hyperlink, theme};
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
    let bottom = area.y + area.height;
    let mut y = area.y;

    let render_row = |f: &mut Frame, y: u16, line: Line<'static>| {
        if y < bottom {
            f.render_widget(line, Rect::new(area.x, y, area.width, 1));
        }
    };

    for job in data.jobs.iter() {
        if y >= bottom {
            break;
        }

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
        render_row(
            f,
            y,
            Line::from(vec![
                Span::styled(dot, dot_style),
                Span::raw("  "),
                Span::styled(dates, theme::secondary()),
                Span::raw(" ".repeat(header_pad)),
                Span::styled(job.location.clone(), theme::body()),
            ]),
        );
        y += 1;

        // │ title @ company                         website
        let website = job.website.trim_end_matches('/').to_string();
        let display = website
            .trim_start_matches("https://")
            .trim_start_matches("http://");
        let left_w = 1 + 2 + job.title.chars().count() + 5 + job.company.chars().count();
        let pad = (area.width as usize).saturating_sub(left_w + display.chars().count());
        if y < bottom {
            f.render_widget(
                Line::from(vec![
                    pipe.clone(),
                    Span::raw("  "),
                    Span::styled(job.title.clone(), theme::green_bold()),
                    Span::styled("  @  ", theme::secondary()),
                    Span::styled(job.company.clone(), theme::green_bold()),
                    Span::raw(" ".repeat(pad)),
                ]),
                Rect::new(area.x, y, area.width, 1),
            );
            let link_x = area.x + (left_w + pad).min(area.width as usize) as u16;
            let link_w = area.width.saturating_sub(link_x - area.x);
            let style = Style::default()
                .fg(theme::MUTED)
                .add_modifier(Modifier::UNDERLINED);
            f.render_widget(
                Hyperlink::new(Span::styled(display, style), &website),
                Rect::new(link_x, y, link_w, 1),
            );
        }
        y += 1;

        // │  description
        render_row(
            f,
            y,
            Line::from(vec![
                pipe.clone(),
                Span::raw("  "),
                Span::styled(job.description.clone(), theme::body()),
            ]),
        );
        y += 1;

        // │  tech stack (if any)
        if !job.technologies.is_empty() {
            let tech = job.technologies.join(" · ");
            render_row(
                f,
                y,
                Line::from(vec![
                    pipe.clone(),
                    Span::raw("  "),
                    Span::styled(tech, theme::secondary()),
                ]),
            );
            y += 1;
        }

        // trailing pipes
        render_row(f, y, Line::from(vec![pipe.clone()]));
        y += 1;
        render_row(f, y, Line::from(vec![pipe.clone()]));
        y += 1;
    }
}
