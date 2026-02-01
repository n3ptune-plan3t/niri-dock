//! niri-dock: Event-driven dock for Niri window manager
//! 
//! Architecture:
//! - IPC thread: blocking on niri event stream (no polling)
//! - Dispatch task: applies state mutations async
//! - UI subscribers: read via watch::channel (zero-copy Arc slices)

mod ipc;
mod state;
mod wayland;
mod ui;

use ipc::IpcStream;
use state::DockState;
use std::sync::Arc;
use tokio::sync::watch;
use tracing::info;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    info!("niri-dock v{}", env!("CARGO_PKG_VERSION"));

    let (state_tx, state_rx) = watch::channel(DockState::default());

    let ipc_stream = IpcStream::new();
    let ipc_handle = {
        let tx = state_tx.clone();
        tokio::task::spawn_blocking(move || ipc_stream.run(tx))
    };

    // UI event loop (Qt/other)
    let ui_handle = {
        let rx = state_rx.clone();
        tokio::spawn(ui::run_ui(rx))
    };

    // Graceful shutdown
    tokio::signal::ctrl_c().await?;
    drop(state_tx);

    let _ = tokio::time::timeout(std::time::Duration::from_secs(5), ipc_handle).await;
    let _ = ui_handle.await;

    Ok(())
}

fn init_tracing() {
    let filter = std::env::var("RUST_LOG")
        .ok()
        .map(|l| tracing_subscriber::EnvFilter::new(l))
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("niri_dock=info"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .compact()
        .init();
}