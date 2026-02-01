use std::sync::Arc;
use tokio::sync::RwLock;
use niri_ipc::{Window, Workspace};

mod ipc;
mod wayland;
mod fractional_scale;

pub use ipc::IpcClient;
pub use wayland::WaylandDock;
pub use fractional_scale::FractionalScaleHandler;

#[derive(Debug, Clone)]
pub struct DockState {
    pub windows: Arc<RwLock<Vec<Window>>>,
    pub workspaces: Arc<RwLock<Vec<Workspace>>>,
    pub scale: Arc<RwLock<f64>>,
}

impl DockState {
    pub fn new() -> Self {
        Self {
            windows: Arc::new(RwLock::new(Vec::new())),
            workspaces: Arc::new(RwLock::new(Vec::new())),
            scale: Arc::new(RwLock::new(1.0)),
        }
    }
}