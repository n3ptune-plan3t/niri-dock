//! Efficient immutable dock state

use niri_ipc::{Window, Workspace, WindowLayout};
use std::collections::HashMap;
use std::sync::Arc;

/// Immutable dock state snapshot—safe to clone cheap (Arc semantics)
#[derive(Clone, Default)]
pub struct DockState {
    /// All windows, Arc-backed for O(1) clones
    pub windows: Arc<[Window]>,
    /// All workspaces, Arc-backed
    pub workspaces: Arc<[Workspace]>,
    /// Focused window ID
    focused: Option<u64>,
    /// Cached layout by window ID
    layout_cache: Arc<HashMap<u64, WindowLayout>>,
}

impl DockState {
    /// Update windows list
    pub fn set_windows(&mut self, windows: Vec<Window>) {
        self.windows = windows.into();
    }

    /// Update workspaces list
    pub fn set_workspaces(&mut self, workspaces: Vec<Workspace>) {
        self.workspaces = workspaces.into();
    }

    /// Set focused window
    pub fn set_focused_window(&mut self, id: Option<u64>) {
        self.focused = id;
    }

    /// Apply layout changes
    pub fn apply_layout_changes(&mut self, changes: &[(u64, WindowLayout)]) {
        let mut cache = (*self.layout_cache).clone();
        for &(id, ref layout) in changes {
            cache.insert(id, layout.clone());
        }
        self.layout_cache = cache.into();
    }

    /// Iterator of tiles organized by (column, tile_index)
    pub fn tiles(&self) -> impl Iterator<Item = TileView> + '_ {
        self.windows.iter().filter_map(|w| {
            let (col, tile) = w.layout.pos_in_scrolling_layout?;
            Some(TileView {
                id: w.id,
                app_id: w.app_id.as_deref().unwrap_or("?"),
                title: w.title.as_deref().unwrap_or(""),
                column: col,
                tile,
                is_focused: w.is_focused,
            })
        })
    }
}

/// Borrowing view of a window tile
#[derive(Debug, Clone, Copy)]
pub struct TileView<'a> {
    /// Window ID
    pub id: u64,
    /// Application ID
    pub app_id: &'a str,
    /// Window title
    pub title: &'a str,
    /// Column index (1-based)
    pub column: usize,
    /// Tile index in column (1-based)
    pub tile: usize,
    /// Is this window focused?
    pub is_focused: bool,
}
