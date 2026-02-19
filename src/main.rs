mod app;
mod backend;
mod data;
mod input;
mod server;
mod session;
mod ui;

use std::{net::SocketAddr, path::PathBuf};

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Structured logging via RUST_LOG.  Default: info.
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    let addr: SocketAddr = std::env::var("SSH_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:2222".to_string())
        .parse()?;

    let key_path =
        PathBuf::from(std::env::var("SSH_HOST_KEY").unwrap_or_else(|_| "host_key".to_string()));

    let key = server::load_or_generate_host_key(&key_path).await?;

    server::run(addr, key).await
}
