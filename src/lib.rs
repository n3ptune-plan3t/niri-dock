#![warn(missing_docs)]
//! Resource-efficient event-driven dock for Niri window manager

pub mod ipc;
pub mod state;
pub mod ui;
pub mod wayland;

pub use state::{DockState, TileView};
pub use ipc::IpcStream;
pub use wayland::DockSurface;
