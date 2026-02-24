/// A ratatui [`Backend`] that writes ANSI-escape-coded output to any
/// [`std::io::Write`] sink (i.e. a channel whose receiver forwards bytes to
/// the SSH client).
///
/// It delegates rendering to a [`crossterm`]-backed inner backend, but
/// overrides [`size()`] to return the terminal dimensions reported by the
/// SSH PTY request / window-change request, instead of querying the local
/// controlling terminal.
use std::io::{self, Write};

use crossterm::{
    QueueableCommand, cursor,
    style::{self, Attribute, Attributes, Color, Colors, SetAttribute},
    terminal,
};
use ratatui::{
    backend::Backend,
    buffer::Cell,
    layout::{Position, Size},
    style::{Color as RColor, Modifier},
};

/// Buffered writer: accumulates bytes and sends in one flush to avoid SSH packet overhead.
pub struct ChannelWriter {
    tx: tokio::sync::mpsc::Sender<Vec<u8>>,
    buffer: Vec<u8>,
}

impl ChannelWriter {
    pub fn new(tx: tokio::sync::mpsc::Sender<Vec<u8>>) -> Self {
        Self {
            tx,
            buffer: Vec::with_capacity(4096),
        }
    }
}

impl Write for ChannelWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buffer.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        if !self.buffer.is_empty() {
            self.tx
                .try_send(self.buffer.drain(..).collect())
                .map_err(|e| io::Error::new(io::ErrorKind::BrokenPipe, e))?;
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------

/// The SSH-aware ratatui backend.
pub struct SshBackend {
    writer: ChannelWriter,
    cols: u16,
    rows: u16,
}

impl SshBackend {
    pub fn new(writer: ChannelWriter, cols: u16, rows: u16) -> Self {
        Self { writer, cols, rows }
    }

    pub fn resize(&mut self, cols: u16, rows: u16) {
        self.cols = cols;
        self.rows = rows;
    }

    pub fn writer_mut(&mut self) -> &mut ChannelWriter {
        &mut self.writer
    }

    fn map_color(color: RColor) -> Color {
        match color {
            RColor::Reset => Color::Reset,
            RColor::Black => Color::Black,
            RColor::Red => Color::Red,         // Use bright red directly
            RColor::Green => Color::Green,     // Use bright green directly
            RColor::Yellow => Color::Yellow,   // Use bright yellow directly
            RColor::Blue => Color::Blue,       // Use bright blue directly
            RColor::Magenta => Color::Magenta, // Use bright magenta directly
            RColor::Cyan => Color::Cyan,       // Use bright cyan directly
            RColor::Gray => Color::Grey,
            RColor::DarkGray => Color::DarkGrey,
            RColor::LightRed => Color::Red,
            RColor::LightGreen => Color::Green,
            RColor::LightYellow => Color::Yellow,
            RColor::LightBlue => Color::Blue,
            RColor::LightMagenta => Color::Magenta,
            RColor::LightCyan => Color::Cyan,
            RColor::White => Color::White,
            RColor::Rgb(r, g, b) => Color::Rgb { r, g, b },
            RColor::Indexed(i) => Color::AnsiValue(i),
        }
    }
}

impl Backend for SshBackend {
    fn draw<'a, I>(&mut self, content: I) -> io::Result<()>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        let mut prev_fg = RColor::Reset;
        let mut prev_bg = RColor::Reset;
        let mut prev_mods = Modifier::empty();

        for (x, y, cell) in content {
            // Move cursor
            self.writer
                .queue(cursor::MoveTo(x, y))
                .map_err(io::Error::other)?;

            let mods_changed = cell.modifier != prev_mods;
            let colors_changed = cell.fg != prev_fg || cell.bg != prev_bg;

            // When modifiers change we must reset first, then re-apply
            // everything atomically (no intermediate state for the terminal
            // to render if a packet boundary lands here).
            if mods_changed {
                self.writer
                    .queue(SetAttribute(Attribute::Reset))
                    .map_err(io::Error::other)?;

                // Colors must be re-sent after a reset regardless of whether
                // they changed from the previous cell.
                self.writer
                    .queue(style::SetColors(Colors::new(
                        Self::map_color(cell.fg),
                        Self::map_color(cell.bg),
                    )))
                    .map_err(io::Error::other)?;

                // Set only the attributes this cell needs.
                let mut attrs = Attributes::default();
                if cell.modifier.contains(Modifier::BOLD) {
                    attrs.set(Attribute::Bold);
                }
                if cell.modifier.contains(Modifier::DIM) {
                    attrs.set(Attribute::Dim);
                }
                if cell.modifier.contains(Modifier::ITALIC) {
                    attrs.set(Attribute::Italic);
                }
                if cell.modifier.contains(Modifier::UNDERLINED) {
                    attrs.set(Attribute::Underlined);
                }
                if cell.modifier.contains(Modifier::REVERSED) {
                    attrs.set(Attribute::Reverse);
                }
                if !cell.modifier.is_empty() {
                    self.writer
                        .queue(style::SetAttributes(attrs))
                        .map_err(io::Error::other)?;
                }

                prev_mods = cell.modifier;
                prev_fg = cell.fg;
                prev_bg = cell.bg;
            } else if colors_changed {
                // Modifiers unchanged — just update colors.
                self.writer
                    .queue(style::SetColors(Colors::new(
                        Self::map_color(cell.fg),
                        Self::map_color(cell.bg),
                    )))
                    .map_err(io::Error::other)?;
                prev_fg = cell.fg;
                prev_bg = cell.bg;
            }

            // Write the symbol
            self.writer
                .queue(style::Print(&cell.symbol()))
                .map_err(io::Error::other)?;
        }

        // Reset all attributes so no SGR state leaks beyond this frame.
        self.writer
            .queue(SetAttribute(Attribute::Reset))
            .map_err(io::Error::other)?;

        self.writer.flush()
    }

    fn hide_cursor(&mut self) -> io::Result<()> {
        self.writer.queue(cursor::Hide).map_err(io::Error::other)?;
        Ok(())
    }

    fn show_cursor(&mut self) -> io::Result<()> {
        self.writer.queue(cursor::Show).map_err(io::Error::other)?;
        Ok(())
    }

    fn get_cursor_position(&mut self) -> io::Result<Position> {
        Ok(Position::new(0, 0)) // not tracked — we control cursor via draw()
    }

    fn set_cursor_position<P: Into<ratatui::layout::Position>>(
        &mut self,
        position: P,
    ) -> io::Result<()> {
        let pos = position.into();
        self.writer
            .queue(cursor::MoveTo(pos.x, pos.y))
            .map_err(io::Error::other)?;
        Ok(())
    }

    fn clear(&mut self) -> io::Result<()> {
        self.writer
            .queue(terminal::Clear(terminal::ClearType::All))
            .map_err(io::Error::other)?;
        Ok(())
    }

    fn size(&self) -> io::Result<Size> {
        Ok(Size::new(self.cols, self.rows))
    }

    fn window_size(&mut self) -> io::Result<ratatui::backend::WindowSize> {
        Ok(ratatui::backend::WindowSize {
            columns_rows: Size::new(self.cols, self.rows),
            pixels: Size::new(0, 0),
        })
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl Write for SshBackend {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}
