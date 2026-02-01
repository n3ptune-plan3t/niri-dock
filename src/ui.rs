use crate::state::DockState;
use tokio::sync::watch;
use tracing::debug;

pub async fn run_ui(mut state_rx: watch::Receiver<DockState>) -> anyhow::Result<()> {
    // Placeholder: integrate Qt/wayland here
    loop {
        if state_rx.changed().await.is_err() {
            break;
        }

        let state = state_rx.borrow();
        let tiles: Vec<_> = state.tiles().collect();
        debug!("rendering {} tiles", tiles.len());

        for tile in &tiles {
            debug!(
                "  [{},{}] {} ({})",
                tile.column, tile.tile, tile.app_id, if tile.is_focused { "focused" } else { "" }
            );
        }
    }

    Ok(())
}