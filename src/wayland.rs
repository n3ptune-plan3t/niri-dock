//! Wayland layer-shell + fractional-scale integration

use wayland_client::Connection;

/// Wayland dock surface manager
pub struct DockSurface {
    conn: Connection,
    scale: f64,
}

impl DockSurface {
    /// Create dock surface
    pub fn new() -> anyhow::Result<Self> {
        let conn = Connection::connect_to_env()?;
        Ok(Self { conn, scale: 1.0 })
    }

    /// Get current DPI scale
    pub fn scale(&self) -> f64 {
        self.scale
    }
}

impl Default for DockSurface {
    fn default() -> Self {
        Self::new().expect("wayland connection required")
    }
}
