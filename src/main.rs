mod app;
mod data;
mod input;
mod section;
mod ui;

use chai_framework::russh::{MethodKind, MethodSet, server::Config};
use chai_framework::{ChaiServer, load_host_keys};

use app::App;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    let key_path = std::env::var("SSH_HOST_KEY").unwrap_or_else(|_| "host_key".to_string());
    let host_key = load_host_keys(&key_path).expect("Failed to load host keys");

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

    tracing::info!("Listening on 0.0.0.0:{port}");
    let mut server = ChaiServer::<App>::new(port);
    server.run(config).await.expect("Server failed");
}
