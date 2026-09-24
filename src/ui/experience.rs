use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Widget,
};

use super::{hyperlink::Hyperlink, scroll::Scroll, theme};
use crate::{
    data::{Job, snapshot},
    input::Key,
    section::SectionView,
};

/// Rows between two jobs, drawn as a continuing timeline.
const GAP_ROWS: u16 = 2;

pub struct ExperienceSection {
    scroll: Scroll,
}

impl ExperienceSection {
    pub fn new() -> Self {
        Self {
            scroll: Scroll::default(),
        }
    }
}

impl SectionView for ExperienceSection {
    fn label(&self) -> &'static str {
        "Exp"
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
                Constraint::Min(1),    // timeline
            ])
            .split(area);

        let data = snapshot();
        let jobs = &data.jobs;
        let height: u16 = jobs.iter().map(job_height).sum::<u16>()
            + GAP_ROWS * (jobs.len() as u16).saturating_sub(1);

        self.scroll.render(f, rows[1], height, |buf, area| {
            render_timeline(buf, area, jobs)
        });
    }
}

/// Rows one job takes, not counting the gap after it.
fn job_height(job: &Job) -> u16 {
    if job.technologies.is_empty() { 3 } else { 4 }
}

fn render_timeline(buf: &mut Buffer, area: Rect, jobs: &[Job]) {
    let pipe = Span::styled("│", theme::secondary());
    let mut y = area.y;
    let row = |buf: &mut Buffer, y: &mut u16, line: Line<'static>| {
        line.render(Rect::new(area.x, *y, area.width, 1), buf);
        *y += 1;
    };

    for (i, job) in jobs.iter().enumerate() {
        if i > 0 {
            for _ in 0..GAP_ROWS {
                row(buf, &mut y, Line::from(pipe.clone()));
            }
        }

        let dot = if job.current { "●" } else { "○" };
        let dot_style = if job.current {
            theme::green_bold()
        } else {
            theme::secondary()
        };

        // ●  dates                    location
        let dates = job.dates.join(" → ");
        let header_left_w = 1 + 2 + dates.chars().count();
        let header_pad =
            (area.width as usize).saturating_sub(header_left_w + job.location.chars().count());
        row(
            buf,
            &mut y,
            Line::from(vec![
                Span::styled(dot, dot_style),
                Span::raw("  "),
                Span::styled(dates, theme::secondary()),
                Span::raw(" ".repeat(header_pad)),
                Span::styled(job.location.clone(), theme::body()),
            ]),
        );

        // │  title @ company                         host
        let website = job.website.trim_end_matches('/');
        let host = website
            .trim_start_matches("https://")
            .trim_start_matches("http://")
            .split('/')
            .next()
            .unwrap_or_default();
        let left_w = 1 + 2 + job.title.chars().count() + 5 + job.company.chars().count();
        let pad = (area.width as usize).saturating_sub(left_w + host.chars().count());
        Line::from(vec![
            pipe.clone(),
            Span::raw("  "),
            Span::styled(job.title.clone(), theme::green_bold()),
            Span::styled("  @  ", theme::secondary()),
            Span::styled(job.company.clone(), theme::green_bold()),
        ])
        .render(Rect::new(area.x, y, area.width, 1), buf);
        let link_x = area.x + (left_w + pad).min(area.width as usize) as u16;
        let link_w = area.width.saturating_sub(link_x - area.x);
        let style = Style::default()
            .fg(theme::MUTED)
            .add_modifier(Modifier::UNDERLINED);
        Hyperlink::new(Span::styled(host, style), website)
            .render(Rect::new(link_x, y, link_w, 1), buf);
        y += 1;

        // │  description
        row(
            buf,
            &mut y,
            Line::from(vec![
                pipe.clone(),
                Span::raw("  "),
                Span::styled(job.description.clone(), theme::body()),
            ]),
        );

        // │  tech stack (if any)
        if !job.technologies.is_empty() {
            row(
                buf,
                &mut y,
                Line::from(vec![
                    pipe.clone(),
                    Span::raw("  "),
                    Span::styled(job.technologies.join(" · "), theme::secondary()),
                ]),
            );
        }
    }
}
