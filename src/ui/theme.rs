//! Theme — inspired by seanyang.me's solarized-warm palette.
//!
//! Palette:
//!   PRIMARY (orange-red)  — accents, selected items      (#d16d3e, --accent)
//!   GREEN   (olive green) — current indicators, progress (#77934d, --secondary)
//!   MUTED   (warm gray)   — hints, secondary labels
//!   HI      (warm cream)  — body text, active content

use ratatui::style::{Color, Modifier, Style};

// ---------------------------------------------------------------------------
// Palette
// ---------------------------------------------------------------------------

/// Orange-red accent — headings, selected items. (#d16d3e)
pub const PRIMARY: Color = Color::Rgb(0xD1, 0x6D, 0x3E);

/// Olive green — current indicators, progress bar fill. (#77934d)
pub const GREEN: Color = Color::Rgb(0x77, 0x93, 0x4D);

/// Warm muted — hints, metadata, secondary labels.
pub const MUTED: Color = Color::Rgb(0xB0, 0xA8, 0x90);

/// Warm cream — body text, values, active content.
pub const HI: Color = Color::Rgb(0xE8, 0xDD, 0xD0);

// ---------------------------------------------------------------------------
// Style helpers
// ---------------------------------------------------------------------------

pub fn primary() -> Style {
    Style::default().fg(PRIMARY)
}

pub fn primary_bold() -> Style {
    Style::default().fg(PRIMARY).add_modifier(Modifier::BOLD)
}

pub fn green() -> Style {
    Style::default().fg(GREEN)
}

pub fn green_bold() -> Style {
    Style::default().fg(GREEN).add_modifier(Modifier::BOLD)
}

pub fn secondary() -> Style {
    Style::default().fg(MUTED)
}

pub fn body() -> Style {
    Style::default().fg(HI)
}
