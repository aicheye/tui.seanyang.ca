use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
};

use super::theme;
use crate::app::App;

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // top margin
            Constraint::Length(6), // song
            Constraint::Length(1), // spacer
            Constraint::Min(1),    // about + poke
        ])
        .split(area);

    render_song_card(f, app, rows[1]);

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(54),
            Constraint::Length(2),
            Constraint::Percentage(44),
        ])
        .split(rows[3]);

    render_about_card(f, cols[0]);
    render_poke_card(f, cols[2]);
}

fn render_about_card(f: &mut Frame, area: Rect) {
    let mut text = vec![
        theme::divider("about", area.width),
        Line::from(""),
        Line::from(Span::styled("Nice to meet you!", theme::primary_bold())),
        Line::from(""),
        Line::from(Span::styled(
            "I believe in the power of technology to create positive change. I'm \
            passionate about using my skills to contribute to a more equitable \
            and sustainable future.",
            theme::body(),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "A few things about me",
            Style::default()
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
                .fg(theme::PRIMARY),
        )),
        Line::from(""),
    ];
    text.extend(fact("Hometown", "Toronto", area.width));
    text.extend(fact("Pets", "2 dogs", area.width));
    text.extend(fact(
        "Fav Games",
        "Subway Builder, Cities Skylines II, Minecraft",
        area.width,
    ));
    text.extend(fact(
        "Fav Artists",
        "beabadoobee, Laufey, keshi",
        area.width,
    ));
    text.extend(fact("Dream Location", "The Moon", area.width));

    let p = Paragraph::new(text).wrap(Wrap { trim: false });
    f.render_widget(p, area);
}

fn fact(key: &'static str, value: &'static str, col_width: u16) -> Vec<Line<'static>> {
    const KEY_WIDTH: usize = 16;
    let val_width = (col_width as usize).saturating_sub(KEY_WIDTH);

    let dots = ".".repeat(KEY_WIDTH.saturating_sub(key.len()));
    let key_label = format!("{key}{dots}");

    if val_width == 0 || value.len() <= val_width {
        return vec![Line::from(vec![
            Span::styled(key_label, theme::secondary()),
            Span::styled(value, theme::body()),
        ])];
    }

    let mut lines = Vec::new();
    let mut remaining = value;
    let mut first = true;

    while !remaining.is_empty() {
        let chunk = if remaining.len() <= val_width {
            remaining
        } else {
            match remaining[..val_width].rfind(' ') {
                Some(i) => &remaining[..i],
                None => &remaining[..val_width],
            }
        };

        let key_span = if first {
            Span::styled(key_label.clone(), theme::secondary())
        } else {
            Span::raw("                ") // 16 spaces
        };

        lines.push(Line::from(vec![
            key_span,
            Span::styled(chunk, theme::body()),
        ]));

        remaining = remaining[chunk.len()..].trim_start_matches(' ');
        first = false;
    }

    lines
}

fn render_song_card(f: &mut Frame, app: &App, area: Rect) {
    let song = app.current_song();
    let index_label = format!("{} / {}", app.song_cursor + 1, app.song_order.len());
    let spotify_url = format!("https://open.spotify.com/track/{}", song.id);

    let text = vec![
        theme::divider("song recommendations", area.width),
        Line::from(""),
        Line::from(Span::styled(
            format!("♫⋆｡♪ ₊˚♬ ﾟ. {}  ♫ ⋆♪ ₊˚♬ ﾟ.", song.desc),
            Style::default()
                .fg(theme::HI)
                .add_modifier(Modifier::ITALIC),
        )),
        Line::from(Span::styled(
            &spotify_url,
            Style::default()
                .fg(theme::PRIMARY)
                .add_modifier(Modifier::UNDERLINED),
        )),
        Line::from(vec![
            Span::styled(format!("({index_label})  "), theme::secondary()),
            Span::styled("h/l", theme::primary()),
            Span::styled(" prev/next", theme::secondary()),
        ]),
    ];

    let p = Paragraph::new(text).alignment(Alignment::Center);
    f.render_widget(p, area);
}

fn render_poke_card(f: &mut Frame, area: Rect) {
    let text = vec![
        theme::divider("poke me", area.width),
        Line::from(""),
        Line::from(Span::styled("Ping my phone!", theme::primary_bold())),
        Line::from(""),
        Line::from(Span::styled(
            "Send me a message directly to my phone via the poke API.",
            theme::body(),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "(not yet implemented — coming soon)",
            Style::default()
                .fg(theme::MUTED)
                .add_modifier(Modifier::ITALIC),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("In the meantime: {}", crate::data::socials::PRIMARY_EMAIL),
            Style::default()
                .fg(theme::PRIMARY)
                .add_modifier(Modifier::UNDERLINED),
        )),
    ];
    let p = Paragraph::new(text).wrap(Wrap { trim: true });
    f.render_widget(p, area);
}
