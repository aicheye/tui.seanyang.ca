//! Diagnostic probe (test-only): renders the real UI through the crossterm
//! backend into a byte sink, then replays the emitted escape stream through a
//! tiny terminal-emulator model that tracks the active OSC 8 hyperlink, so we
//! can see exactly which screen cells end up carrying which URL.

#![allow(dead_code)]

use std::io::Write;
use std::sync::{Arc, Mutex};

use ratatui::backend::CrosstermBackend;
use ratatui::layout::Rect;
use ratatui::{Terminal, TerminalOptions, Viewport};
use unicode_width::UnicodeWidthChar;

#[derive(Clone, Default)]
pub struct Sink(pub Arc<Mutex<Vec<u8>>>);

impl Write for Sink {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[derive(Clone, Default, PartialEq)]
pub struct EmuCell {
    pub ch: char,
    /// (id, url) of the OSC 8 hyperlink covering this cell, if any.
    pub url: Option<(String, String)>,
}

pub struct Emu {
    pub w: usize,
    pub h: usize,
    pub grid: Vec<EmuCell>,
    cx: usize,
    cy: usize,
    url: Option<(String, String)>,
}

impl Emu {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            w,
            h,
            grid: vec![
                EmuCell {
                    ch: ' ',
                    url: None
                };
                w * h
            ],
            cx: 0,
            cy: 0,
            url: None,
        }
    }

    /// Feed raw bytes (assumed valid UTF-8) into the emulator.
    pub fn feed(&mut self, bytes: &[u8]) {
        let s = String::from_utf8_lossy(bytes).into_owned();
        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '\x1b' => match chars.next() {
                    // CSI
                    Some('[') => {
                        let mut params = String::new();
                        for p in chars.by_ref() {
                            if p.is_ascii_alphabetic() {
                                if p == 'H' {
                                    let mut it = params.split(';');
                                    let row: usize =
                                        it.next().unwrap_or("1").parse().unwrap_or(1);
                                    let col: usize =
                                        it.next().unwrap_or("1").parse().unwrap_or(1);
                                    self.cy = row.saturating_sub(1);
                                    self.cx = col.saturating_sub(1);
                                }
                                // SGR / cursor-visibility / etc: no cell effect
                                break;
                            }
                            params.push(p);
                        }
                    }
                    // OSC ... (BEL or ESC \ terminated)
                    Some(']') => {
                        let mut body = String::new();
                        loop {
                            match chars.next() {
                                Some('\x07') | None => break,
                                Some('\x1b') => {
                                    if chars.peek() == Some(&'\\') {
                                        chars.next();
                                    }
                                    break;
                                }
                                Some(other) => body.push(other),
                            }
                        }
                        if let Some(rest) = body.strip_prefix("8;") {
                            // rest = "params;URI", params may hold "id=..."
                            let (params, uri) = rest.split_once(';').unwrap_or(("", ""));
                            let id = params.strip_prefix("id=").unwrap_or("");
                            self.url = if uri.is_empty() {
                                None
                            } else {
                                Some((id.to_string(), uri.to_string()))
                            };
                        }
                    }
                    _ => {}
                },
                '\r' => self.cx = 0,
                '\n' => self.cy = (self.cy + 1).min(self.h - 1),
                '\x07' => {}
                c => {
                    let cw = c.width().unwrap_or(0);
                    if cw == 0 {
                        continue;
                    }
                    if self.cx < self.w && self.cy < self.h {
                        let idx = self.cy * self.w + self.cx;
                        self.grid[idx] = EmuCell {
                            ch: c,
                            url: self.url.clone(),
                        };
                        // wide char: occupy trailing cells too
                        for k in 1..cw {
                            if self.cx + k < self.w {
                                self.grid[idx + k] = EmuCell {
                                    ch: '\0',
                                    url: self.url.clone(),
                                };
                            }
                        }
                    }
                    self.cx += cw; // may run past w: pending-wrap approximation
                }
            }
        }
    }

    /// Rows that contain at least one hyperlinked cell, as
    /// (row, Vec<(x_start, x_end_inclusive, (id, url), text)>).
    #[allow(clippy::type_complexity)]
    pub fn link_runs(&self) -> Vec<(usize, Vec<(usize, usize, (String, String), String)>)> {
        let mut out = vec![];
        for y in 0..self.h {
            let mut runs: Vec<(usize, usize, (String, String), String)> = vec![];
            let mut x = 0;
            while x < self.w {
                let cell = &self.grid[y * self.w + x];
                if let Some(link) = &cell.url {
                    let start = x;
                    let mut text = String::new();
                    while x < self.w {
                        let c = &self.grid[y * self.w + x];
                        if c.url.as_ref() != Some(link) {
                            break;
                        }
                        if c.ch != '\0' {
                            text.push(c.ch);
                        }
                        x += 1;
                    }
                    runs.push((start, x - 1, link.clone(), text));
                } else {
                    x += 1;
                }
            }
            if !runs.is_empty() {
                out.push((y, runs));
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::App;
    use chai_framework::ChaiApp;

    const W: u16 = 154;
    const H: u16 = 40;

    fn dump_frame(label: &str, emu: &Emu) {
        println!("== {label} ==");
        for (y, runs) in emu.link_runs() {
            for (x0, x1, (id, url), text) in runs {
                println!("  row {y:2} cols {x0:3}..{x1:3} id={id} url={url} text={text:?}");
            }
        }
    }

    /// Every hyperlinked cell run must cover exactly a link's text: no run
    /// may bleed into surrounding whitespace or run to the screen edge.
    fn assert_runs_clean(label: &str, emu: &Emu) {
        for (y, runs) in emu.link_runs() {
            for (x0, x1, (id, url), text) in runs {
                assert!(
                    !text.contains(' '),
                    "{label}: link run at row {y} cols {x0}..{x1} (id={id}, url={url}) \
                     bled into whitespace: {text:?}"
                );
                assert!(
                    x1 < emu.w - 1,
                    "{label}: link run at row {y} (url={url}) runs to the screen edge"
                );
                assert!(!id.is_empty(), "{label}: link at row {y} has no OSC 8 id");
            }
        }
    }

    #[test]
    fn probe_hyperlink_cells() {
        let sink = Sink::default();
        let backend = CrosstermBackend::new(sink.clone());
        let mut term = Terminal::with_options(
            backend,
            TerminalOptions {
                viewport: Viewport::Fixed(Rect::new(0, 0, W, H)),
            },
        )
        .unwrap();

        let mut app = App::new();
        let mut emu = Emu::new(W as usize, H as usize);

        let mut frame_no = 0;
        let mut draw = |term: &mut Terminal<CrosstermBackend<Sink>>,
                        app: &App,
                        emu: &mut Emu,
                        label: &str| {
            term.draw(|f| crate::ui::render(f, app)).unwrap();
            let bytes = std::mem::take(&mut *sink.0.lock().unwrap());
            emu.feed(&bytes);
            frame_no += 1;
            let label = format!("frame {frame_no}: {label}");
            dump_frame(&label, emu);
            assert_runs_clean(&label, emu);
            if std::env::var("DUMP_BYTES").is_ok() {
                let pretty = String::from_utf8_lossy(&bytes)
                    .replace('\x1b', "\n\\e")
                    .replace('\x07', "<BEL>");
                println!("--- raw frame {frame_no} ({label}) ---{pretty}\n--- end ---");
            }
        };

        draw(&mut term, &app, &mut emu, "home, first draw");
        draw(&mut term, &app, &mut emu, "home, redraw");
        app.active = 3;
        draw(&mut term, &app, &mut emu, "links, first draw");
        let links_first = emu.link_runs();
        draw(&mut term, &app, &mut emu, "links, redraw");
        assert_eq!(
            links_first,
            emu.link_runs(),
            "link runs (incl. ids) must be identical across redraws"
        );
        app.active = 2;
        draw(&mut term, &app, &mut emu, "projects, first draw");
        draw(&mut term, &app, &mut emu, "projects, redraw");
        app.active = 1;
        draw(&mut term, &app, &mut emu, "experience, first draw");
        draw(&mut term, &app, &mut emu, "experience, redraw");
        app.active = 3;
        draw(&mut term, &app, &mut emu, "links again");
        draw(&mut term, &app, &mut emu, "links again, redraw");
        assert_eq!(
            links_first,
            emu.link_runs(),
            "returning to a page must reproduce the same link runs and ids"
        );
    }
}
