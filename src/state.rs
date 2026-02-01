use niri_ipc::{Window, Workspace, WindowLayout};
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct DockState {
    pub windows: Arc<[Window]>,
    pub workspaces: Arc<[Workspace]>,
    focused: Option<u64>,
    layout_cache: Arc<HashMap<u64, WindowLayout>>,
}

impl DockState {
    pub fn set_windows(&mut self, windows: Vec<Window>) {
        self.windows = windows.into();
    }

    pub fn set_workspaces(&mut self, workspaces: Vec<Workspace>) {
        self.workspaces = workspaces.into();
    }

    pub fn set_focused_window(&mut self, id: Option<u64>) {
        self.focused = id;
    }

    pub fn apply_layout_changes(&mut self, changes: &[(u64, WindowLayout)]) {
        let mut cache = (*self.layout_cache).clone();
        for (id, layout) in changes {
            cache.insert(*id, layout.clone());
        }
        self.layout_cache = cache.into();
    }

    /// Iterator of tiles organized by (column, tile_index)
    pub fn tiles(&self) -> impl Iterator<Item = TileView> {
        self.windows.iter().filter_map(|w| {
            let (col, tile) = w.layout.pos_in_scrolling_layout?;
            Some(TileView {
                id: w.id,
                app_id: w.app_id.as_deref().unwrap_or("?"),
                title: w.title.as_deref().unwrap_or(""),
                column: col,
                tile: tile,
                is_focused: w.is_focused,
            })
        })
    }
}

pub struct TileView<'a> {
    pub id: u64,
    pub app_id: &'a str,
    pub title: &'a str,
    pub column: usize,
    pub tile: usize,
    pub is_focused: bool,
}

use std::sync::Arc;