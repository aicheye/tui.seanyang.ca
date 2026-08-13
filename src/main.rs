mod app;
mod data;
mod input;
mod section;
mod ui;

use std::path::Path;

use chai_framework::russh::keys::ssh_key::LineEnding;
use chai_framework::russh::keys::{Algorithm, PrivateKey};
use chai_framework::russh::{MethodKind, MethodSet, server::Config};
use chai_framework::{ChaiServer, load_host_keys};

use app::App;

/// Load the Ed25519 host key, generating (and persisting) one on first run.
///
/// The framework's `load_host_keys` only reads an existing key, so a fresh
/// deployment -- an empty `/data` volume, a new `ReadWritePaths` directory --
/// has nothing to load. Generating here keeps the key stable across restarts
/// as long as the path is on persistent storage; visitors would otherwise see
/// a host key mismatch warning.
fn load_or_generate_host_key(path: &str) -> anyhow::Result<PrivateKey> {
    let key_path = Path::new(path);

    if key_path.exists() {
        return load_host_keys(path);
    }

    if let Some(parent) = key_path.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent)?;
    }

    let key = PrivateKey::random(&mut rand::rngs::OsRng, Algorithm::Ed25519)?;
    // Written with mode 0600 by ssh-key itself.
    key.write_openssh_file(key_path, LineEnding::LF)?;
    tracing::info!("generated new ed25519 host key at {}", key_path.display());

    Ok(key)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    // Pull the latest site content before accepting connections. Falls back to
    // the bundled snapshot if the website is unreachable.
    data::init().await;

    let key_path = std::env::var("SSH_HOST_KEY").unwrap_or_else(|_| "host_key".to_string());
    let host_key = load_or_generate_host_key(&key_path).expect("Failed to load host keys");

    let port: u16 = std::env::var("SSH_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(2222);

    let mut methods = MethodSet::empty();
    methods.push(MethodKind::None);

    let config = Config {
        inactivity_timeout: Some(std::time::Duration::from_secs(3600)),
        auth_rejection_time: std::time::Duration::from_secs(1),
        auth_rejection_time_initial: Some(std::time::Duration::ZERO),
        keys: vec![host_key],
        methods,
        ..Default::default()
    };

    let mut server = ChaiServer::<App>::new(port);
    server.run(config).await.expect("Server failed");
}
