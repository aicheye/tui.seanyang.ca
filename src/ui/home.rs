use rand::seq::SliceRandom;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
};

use super::theme;
use crate::{
    data::{
        portfolio::BUILDING,
        quotes::{QUOTES, Quote},
    },
    input::Key,
    section::SectionView,
};

/// ASCII art wordmark — rendered at the top of the home screen.
const WORDMARK: &str = r#"
 ,---.  ,---.  ,--,--.,--,--, ,--. ,--.,--,--.,--,--,  ,---.
(  .-' | .-. :' ,-.  ||      \ \  '  /' ,-.  ||      \| .-. |
.-'  `)\   --.\ '-'  ||  ||  |  \   ' \ '-'  ||  ||  |' '-' '
`----'  `----' `--`--'`--''--'.-'  /   `--`--'`--''--'.`-  /
                              `---'                   `---'
"#;

pub struct HomeSection {
    order: Vec<usize>,
    cursor: usize,
}

impl HomeSection {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut order: Vec<usize> = (0..QUOTES.len()).collect();
        order.shuffle(&mut rng);
        Self { order, cursor: 0 }
    }

    fn current_quote(&self) -> &'static Quote {
        &QUOTES[self.order[self.cursor]]
    }
}

impl SectionView for HomeSection {
    fn label(&self) -> &'static str {
        "Home"
    }

    fn handle_key(&mut self, key: Key) {
        match key {
            Key::Right | Key::Char('l') | Key::Char('n') => {
                self.cursor = (self.cursor + 1) % self.order.len();
            }
            Key::Left | Key::Char('h') | Key::Char('p') => {
                self.cursor = self.cursor.checked_sub(1).unwrap_or(self.order.len() - 1);
            }
            _ => {}
        }
    }

    fn render(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),                                   // top margin
                Constraint::Length(WORDMARK.lines().count() as u16 + 1), // wordmark
                Constraint::Length(2),                                   // tagline
                Constraint::Length(1),                                   // spacer
                Constraint::Min(8),                                      // bio + building
                Constraint::Length(1),                                   // space above quote
                Constraint::Length(6),                                   // quote
                Constraint::Fill(1),                                     // bottom fill
            ])
            .split(area);

        render_wordmark(f, chunks[1]);
        render_tagline(f, chunks[2]);
        render_bio_and_building(f, chunks[4]);
        render_quote(f, self, chunks[6]);
    }
}

fn render_wordmark(f: &mut Frame, area: Rect) {
    let p = Paragraph::new(WORDMARK)
        .style(theme::primary_bold())
        .alignment(Alignment::Center);
    f.render_widget(p, area);
}

fn render_tagline(f: &mut Frame, area: Rect) {
    let line = Line::from(vec![
        Span::styled("coder", theme::body()),
        Span::styled("  ·  ", theme::secondary()),
        Span::styled("sustainable urbanist", theme::body()),
        Span::styled("  ·  ", theme::secondary()),
        Span::styled("democratic socialist", theme::body()),
        Span::styled("  ·  ", theme::secondary()),
        Span::styled("SE @ UWaterloo", theme::body()),
    ]);
    let p = Paragraph::new(line).alignment(Alignment::Center);
    f.render_widget(p, area);
}

fn render_bio_and_building(f: &mut Frame, area: Rect) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(54),
            Constraint::Length(2),
            Constraint::Percentage(44),
        ])
        .split(area);

    // Left: greeting + bio
    let bio_text = vec![
        Line::from(Span::styled("Hello, World!", theme::primary_bold())),
        Line::from(""),
        Line::from(Span::styled(
            "I'm a coder, sustainable urbanist, and advocate for economic justice \
             studying Software Engineering at the University of Waterloo.",
            theme::body(),
        )),
    ];
    let bio = Paragraph::new(bio_text).wrap(Wrap { trim: true });
    f.render_widget(bio, cols[0]);

    // Right: currently building
    render_building(f, cols[2]);
}

fn render_building(f: &mut Frame, area: Rect) {
    let mut lines: Vec<Line> = vec![
        theme::divider("currently building", area.width),
        Line::from(""),
    ];

    for b in BUILDING.iter() {
        lines.push(Line::from(vec![
            Span::styled("▸ ", theme::primary()),
            Span::styled(
                b.name,
                Style::default()
                    .fg(theme::PRIMARY)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(b.description, theme::secondary()),
        ]));
        lines.push(Line::from(""));
    }

    let p = Paragraph::new(lines).wrap(Wrap { trim: true });
    f.render_widget(p, area);
}

fn render_quote(f: &mut Frame, section: &HomeSection, area: Rect) {
    let quote = section.current_quote();
    let index_label = format!("{} / {}", section.cursor + 1, section.order.len());

    let tui_lines = vec![
        theme::divider("quote", area.width),
        Line::from(""),
        Line::from(Span::styled(
            format!("❝ {} ❞", quote.text),
            Style::default()
                .fg(theme::HI)
                .add_modifier(Modifier::ITALIC),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("— ", theme::secondary()),
            Span::styled(quote.author, theme::primary_bold()),
            Span::styled(format!("  ({index_label})  ",), theme::secondary()),
            Span::styled("←/→", theme::primary()),
            Span::styled(" prev/next", theme::secondary()),
        ]),
    ];

    let p = Paragraph::new(tui_lines)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(p, area);
}
