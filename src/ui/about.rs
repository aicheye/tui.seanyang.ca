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
            Constraint::Length(6), // song
            Constraint::Min(1),    // about + poke
        ])
        .split(area);

    render_song_card(f, app, rows[0]);

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(54),
            Constraint::Length(2),
            Constraint::Percentage(44),
        ])
        .split(rows[1]);

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
            "I'm always open to new opportunities, especially in non-profit or \
            ESG sectors.",
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
    text.push(fact("Hometown", "Toronto"));
    text.push(fact("Pets", "2 dogs"));
    text.push(fact(
        "Fav Games",
        "Subway Builder, Cities Skylines II, Minecraft",
    ));
    text.push(fact("Fav Artists", "beabadoobee, Laufey, keshi"));
    text.push(fact("Dream Location", "The Moon"));

    let p = Paragraph::new(text).wrap(Wrap { trim: true });
    f.render_widget(p, area);
}

fn fact(key: &'static str, value: &'static str) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{key:<16}"), theme::secondary()),
        Span::styled(value, theme::body()),
    ])
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
