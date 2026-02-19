/// SSH server: accepts connections, handles auth and PTY negotiation,
/// and spawns a [`crate::session`] for each connected client.
use std::{net::SocketAddr, sync::Arc};

use anyhow::Result;
use async_trait::async_trait;
use russh::{
    Channel, ChannelId,
    server::{self, Auth, Msg, Server as _, Session},
};
use russh_keys::key::KeyPair;
use tokio::sync::{OwnedSemaphorePermit, Semaphore, mpsc};
use tracing::{info, warn};

use crate::session::{self, SessionMsg};

/// Maximum number of concurrent SSH sessions.
const MAX_CONNECTIONS: usize = 100;

// ---------------------------------------------------------------------------
// Server factory
// ---------------------------------------------------------------------------

/// Implements [`server::Server`]: creates a new [`Client`] for each
/// incoming SSH connection.
pub struct AppServer {
    semaphore: Arc<Semaphore>,
}

impl AppServer {
    pub fn new() -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(MAX_CONNECTIONS)),
        }
    }
}

impl server::Server for AppServer {
    type Handler = Client;

    fn new_client(&mut self, peer: Option<SocketAddr>) -> Client {
        info!(
            "new connection from {:?}",
            peer.unwrap_or_else(|| "<unknown>".parse().unwrap())
        );
        Client::new(self.semaphore.clone())
    }

    fn handle_session_error(&mut self, _err: <Self::Handler as server::Handler>::Error) {
        // errors are already traced inside the handler
    }
}

// ---------------------------------------------------------------------------
// Per-connection handler
// ---------------------------------------------------------------------------

/// State for a single SSH client connection.
pub struct Client {
    /// PTY dimensions from the last `pty_request` / `window_change_request`.
    cols: u32,
    rows: u32,
    /// Sender to the active TUI session (set after `shell_request`).
    session_tx: Option<mpsc::Sender<SessionMsg>>,
    /// Shared semaphore for connection limiting.
    semaphore: Arc<Semaphore>,
    /// Held permit — dropped automatically when this Client is dropped.
    _permit: Option<OwnedSemaphorePermit>,
}

impl Client {
    fn new(semaphore: Arc<Semaphore>) -> Self {
        Self {
            cols: 80,
            rows: 24,
            session_tx: None,
            semaphore,
            _permit: None,
        }
    }

    fn send(&self, msg: SessionMsg) {
        if let Some(tx) = &self.session_tx {
            // Use try_send to avoid blocking; drop the message if full.
            let _ = tx.try_send(msg);
        }
    }
}

#[async_trait]
impl server::Handler for Client {
    type Error = anyhow::Error;

    // ------------------------------------------------------------------
    // Auth: accept everything — this is a public showcase SSH app.
    // ------------------------------------------------------------------

    async fn auth_none(&mut self, _user: &str) -> Result<Auth, Self::Error> {
        // Try to acquire a connection permit.
        match self.semaphore.clone().try_acquire_owned() {
            Ok(permit) => {
                self._permit = Some(permit);
                Ok(Auth::Accept)
            }
            Err(_) => {
                warn!("connection rejected: at capacity ({MAX_CONNECTIONS})");
                Ok(Auth::Reject {
                    proceed_with_methods: None,
                })
            }
        }
    }

    async fn auth_password(&mut self, _user: &str, _pw: &str) -> Result<Auth, Self::Error> {
        if self._permit.is_some() {
            Ok(Auth::Accept)
        } else {
            Ok(Auth::Reject {
                proceed_with_methods: None,
            })
        }
    }

    async fn auth_publickey(
        &mut self,
        _user: &str,
        _key: &russh_keys::key::PublicKey,
    ) -> Result<Auth, Self::Error> {
        if self._permit.is_some() {
            Ok(Auth::Accept)
        } else {
            Ok(Auth::Reject {
                proceed_with_methods: None,
            })
        }
    }

    // ------------------------------------------------------------------
    // Channel open
    // ------------------------------------------------------------------

    async fn channel_open_session(
        &mut self,
        _channel: Channel<Msg>,
        _session: &mut Session,
    ) -> Result<bool, Self::Error> {
        Ok(true)
    }

    // ------------------------------------------------------------------
    // PTY + shell setup
    // ------------------------------------------------------------------

    async fn pty_request(
        &mut self,
        _channel: ChannelId,
        _term: &str,
        col_width: u32,
        row_height: u32,
        _pix_width: u32,
        _pix_height: u32,
        _modes: &[(russh::Pty, u32)],
        _session: &mut Session,
    ) -> Result<(), Self::Error> {
        self.cols = col_width;
        self.rows = row_height;
        Ok(())
    }

    async fn shell_request(
        &mut self,
        channel: ChannelId,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        let handle = session.handle();
        let tx = session::spawn(handle, channel, self.cols, self.rows);
        self.session_tx = Some(tx);
        Ok(())
    }

    // ------------------------------------------------------------------
    // Input forwarding
    // ------------------------------------------------------------------

    async fn data(
        &mut self,
        _channel: ChannelId,
        data: &[u8],
        _session: &mut Session,
    ) -> Result<(), Self::Error> {
        self.send(SessionMsg::Input(data.to_vec()));
        Ok(())
    }

    async fn window_change_request(
        &mut self,
        _channel: ChannelId,
        col_width: u32,
        row_height: u32,
        _pix_width: u32,
        _pix_height: u32,
        _session: &mut Session,
    ) -> Result<(), Self::Error> {
        self.cols = col_width;
        self.rows = row_height;
        self.send(SessionMsg::Resize(col_width as u16, row_height as u16));
        Ok(())
    }

    // ------------------------------------------------------------------
    // Disconnect
    // ------------------------------------------------------------------

    async fn channel_close(
        &mut self,
        _channel: ChannelId,
        _session: &mut Session,
    ) -> Result<(), Self::Error> {
        self.send(SessionMsg::Shutdown);
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Key management
// ---------------------------------------------------------------------------

/// Load an Ed25519 host key from `path`, or generate (and save) a new one.
pub async fn load_or_generate_host_key(path: &std::path::Path) -> Result<KeyPair> {
    if path.exists() {
        let pem = tokio::fs::read_to_string(path).await?;
        let kp = russh_keys::decode_secret_key(&pem, None)?;
        info!("Loaded host key from {}", path.display());
        Ok(kp)
    } else {
        let kp = KeyPair::generate_ed25519().ok_or_else(|| anyhow::anyhow!("key gen failed"))?;
        let mut pem_buf = Vec::new();
        russh_keys::encode_pkcs8_pem(&kp, &mut pem_buf)?;
        tokio::fs::write(path, &pem_buf).await?;
        info!("Generated new host key -> {}", path.display());
        Ok(kp)
    }
}

// ---------------------------------------------------------------------------
// Server runner
// ---------------------------------------------------------------------------

pub async fn run(addr: SocketAddr, key: KeyPair) -> Result<()> {
    let config = Arc::new(server::Config {
        inactivity_timeout: Some(std::time::Duration::from_secs(3600)),
        auth_rejection_time: std::time::Duration::from_secs(1),
        auth_rejection_time_initial: Some(std::time::Duration::ZERO),
        keys: vec![key],
        ..Default::default()
    });

    info!("Listening on {addr} (max {MAX_CONNECTIONS} connections)");
    AppServer::new().run_on_address(config, addr).await?;
    Ok(())
}
