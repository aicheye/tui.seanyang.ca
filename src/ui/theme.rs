//! Minimal theme — 3 colors, no borders, terminal.shop-inspired.
//!
//! Palette:
//!   PRIMARY (coral)   — headings, selected items, emphasis
//!   MUTED             — descriptions, hints, metadata
//!   HI                — body text, active content

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

// ---------------------------------------------------------------------------
// Palette — exactly 3 colors
// ---------------------------------------------------------------------------

/// Primary — warm coral. Headings, accents, selected items.
pub const PRIMARY: Color = Color::Rgb(0xFF, 0x6B, 0x6B);

/// Secondary — cool muted gray. Hints, labels, metadata.
pub const MUTED: Color = Color::Rgb(0x6B, 0x7B, 0x8D);

/// High — light gray. Body text, values, active content.
pub const HI: Color = Color::Rgb(0xE0, 0xE0, 0xE0);

// ---------------------------------------------------------------------------
// Style helpers
// ---------------------------------------------------------------------------

/// Primary color.
pub fn primary() -> Style {
    Style::default().fg(PRIMARY)
}

/// Primary + bold — headings, selected items.
pub fn primary_bold() -> Style {
    Style::default().fg(PRIMARY).add_modifier(Modifier::BOLD)
}

/// Muted / secondary text.
pub fn secondary() -> Style {
    Style::default().fg(MUTED)
}

/// Body text.
pub fn body() -> Style {
    Style::default().fg(HI)
}

// ---------------------------------------------------------------------------
// Layout helpers
// ---------------------------------------------------------------------------

/// Returns a `── title ──` divider line, filling `width` columns.
pub fn divider(title: &str, width: u16) -> Line<'_> {
    let label = format!(" {} ", title);
    let label_len = label.len() as u16;
    let remaining = width.saturating_sub(label_len);
    let left = remaining / 2;
    let right = remaining.saturating_sub(left);

    Line::from(vec![
        Span::styled("─".repeat(left as usize), secondary()),
        Span::styled(label, primary_bold()),
        Span::styled("─".repeat(right as usize), secondary()),
    ])
}
