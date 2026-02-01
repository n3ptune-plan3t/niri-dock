//! Wayland layer-shell + fractional-scale integration (stub)

use wayland_client::Connection;

pub struct DockSurface {
    conn: Connection,
    scale: f64,
}

impl DockSurface {
    pub fn new() -> anyhow::Result<Self> {
        let conn = Connection::connect_to_env()?;
        Ok(Self { conn, scale: 1.0 })
    }

    pub fn scale(&self) -> f64 {
        self.scale
    }
}