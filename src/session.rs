/// Per-client TUI session.
///
/// Each SSH connection that requests a PTY + shell gets one of these. It
/// owns the app state and drives the ratatui render loop.
use std::io::Write;
use std::time::Duration;

use anyhow::Result;
use crossterm::{
    ExecutableCommand, cursor,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::Terminal;
use russh::{ChannelId, CryptoVec, server::Handle};
use tokio::sync::mpsc;
use tracing::warn;

use crate::{
    app::App,
    backend::{ChannelWriter, SshBackend},
    input::parse_keys,
    ui,
};

/// Messages sent from the SSH handler to the session task.
pub enum SessionMsg {
    /// Raw bytes received from the SSH client (keyboard input).
    Input(Vec<u8>),
    /// Terminal was resized to (cols, rows).
    Resize(u16, u16),
    /// Client disconnected.
    Shutdown,
}

/// Spawn a TUI session for a new SSH client.
///
/// Returns the sender half of the message channel. The SSH handler pushes
/// [`SessionMsg`]s through this channel.
pub fn spawn(handle: Handle, channel: ChannelId, cols: u32, rows: u32) -> mpsc::Sender<SessionMsg> {
    let (msg_tx, msg_rx) = mpsc::channel::<SessionMsg>(64);
    let (write_tx, write_rx) = mpsc::channel::<Vec<u8>>(32);

    // Writer task: drains rendered ANSI bytes → SSH channel data.
    let handle_clone = handle.clone();
    let writer_handle = tokio::spawn(async move {
        writer_task(handle_clone, channel, write_rx).await;
    });

    // TUI task: runs the ratatui event loop.
    tokio::spawn(async move {
        if let Err(e) = tui_task(write_tx, msg_rx, cols as u16, rows as u16).await {
            warn!("TUI task ended: {e}");
        }
        // Wait for the writer task to finish draining all pending bytes
        // (including the cleanup escape sequences) before closing the channel.
        let _ = writer_handle.await;
        // Now it's safe to close — the client has received everything.
        let _ = handle.close(channel).await;
    });

    msg_tx
}

// ---------------------------------------------------------------------------
// Writer task
// ---------------------------------------------------------------------------

async fn writer_task(handle: Handle, channel: ChannelId, mut rx: mpsc::Receiver<Vec<u8>>) {
    while let Some(data) = rx.recv().await {
        if handle.data(channel, CryptoVec::from(data)).await.is_err() {
            break;
        }
    }
}

// ---------------------------------------------------------------------------
// TUI task
// ---------------------------------------------------------------------------

async fn tui_task(
    write_tx: mpsc::Sender<Vec<u8>>,
    mut msg_rx: mpsc::Receiver<SessionMsg>,
    cols: u16,
    rows: u16,
) -> Result<()> {
    let mut writer = ChannelWriter::new(write_tx);
    writer.execute(EnterAlternateScreen)?;

    let backend = SshBackend::new(writer, cols, rows);
    let mut terminal = Terminal::new(backend)?;

    // Hide cursor and clear screen on entry.
    terminal.hide_cursor()?;
    terminal.clear()?;

    let mut app = App::new(cols, rows);

    // Initial render.
    terminal.draw(|f| ui::render(f, &app))?;

    // Render-tick interval — ensures we redraw even when there's no input
    // (e.g. for future animations). 60 ms ≈ 16 fps.
    let mut tick = tokio::time::interval(Duration::from_millis(60));

    // Coalesce rapid resize events: store the latest resize and apply on tick.
    let mut pending_resize: Option<(u16, u16)> = None;
    let mut dirty = false;

    loop {
        tokio::select! {
            _ = tick.tick() => {
                // Apply a pending resize (if any) once per tick to avoid
                // re-rendering continuously during a drag/resize.
                if let Some((w, h)) = pending_resize.take() {
                    app.resize(w, h);
                    terminal.backend_mut().resize(w, h);
                    terminal.resize(ratatui::layout::Rect::new(0, 0, w, h))?;
                    terminal.draw(|f| ui::render(f, &app))?;
                } else if dirty {
                    terminal.draw(|f| ui::render(f, &app))?;
                    dirty = false;
                }
            }

            msg = msg_rx.recv() => {
                match msg {
                    None | Some(SessionMsg::Shutdown) => break,

                    Some(SessionMsg::Input(bytes)) => {
                        for key in parse_keys(&bytes) {
                            dirty |= app.handle_key(key);
                        }
                        if app.should_quit {
                            break;
                        }
                        if dirty {
                            terminal.draw(|f| ui::render(f, &app))?;
                            dirty = false;
                        }
                    }

                    Some(SessionMsg::Resize(w, h)) => {
                        // Coalesce rapid resize events; apply on next tick.
                        pending_resize = Some((w, h));
                    }
                }
            }
        }
    }

    // Cleanup: reset styles, show cursor, leave alternate screen.
    // Flushed through the terminal's own backend so everything goes through
    // one writer in the correct order.
    let backend = terminal.backend_mut();
    backend.write_all(b"\x1b[0m")?;
    backend.flush()?;

    let writer = backend.writer_mut();
    writer.execute(cursor::Show)?;
    writer.execute(LeaveAlternateScreen)?;
    writer.flush()?;

    // Drop the terminal (and its write_tx sender) so the writer task sees
    // the channel close and can finish draining.
    drop(terminal);

    Ok(())
}
