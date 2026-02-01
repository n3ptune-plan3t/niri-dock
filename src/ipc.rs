use niri_ipc::{Event, Request, Socket, Window, Workspace};
use std::time::Duration;
use tokio::sync::watch;
use tracing::{debug, error, warn};

use crate::state::DockState;

pub struct IpcStream;

impl IpcStream {
    pub fn new() -> Self {
        Self
    }

    /// Blocking event loop—runs on dedicated thread
    pub fn run(self, state_tx: watch::Sender<DockState>) -> anyhow::Result<()> {
        let socket = self.connect_with_retry()?;
        self.event_loop(socket, state_tx)
    }

    fn connect_with_retry(&self) -> anyhow::Result<Socket> {
        let mut attempts = 0;
        loop {
            match Socket::connect() {
                Ok(socket) => {
                    debug!("connected to niri socket");
                    return Ok(socket);
                }
                Err(e) if attempts < 5 => {
                    warn!("connection attempt {}: {}", attempts + 1, e);
                    std::thread::sleep(Duration::from_millis(100));
                    attempts += 1;
                }
                Err(e) => return Err(e.into()),
            }
        }
    }

    fn event_loop(
        &self,
        mut socket: Socket,
        state_tx: watch::Sender<DockState>,
    ) -> anyhow::Result<()> {
        socket.send(Request::EventStream)?;
        let mut read_event = socket.read_events();

        loop {
            match read_event() {
                Ok(event) => self.handle_event(&event, &state_tx),
                Err(e) => {
                    error!("ipc read error: {}", e);
                    return Err(e.into());
                }
            }
        }
    }

    fn handle_event(&self, event: &Event, state_tx: &watch::Sender<DockState>) {
        use niri_ipc::Event::*;

        match event {
            WindowsChanged { windows } => {
                let mut state = state_tx.borrow_mut();
                state.set_windows(windows.clone());
            }
            WorkspacesChanged { workspaces } => {
                let mut state = state_tx.borrow_mut();
                state.set_workspaces(workspaces.clone());
            }
            WindowLayoutsChanged { changes } => {
                let mut state = state_tx.borrow_mut();
                state.apply_layout_changes(changes);
            }
            WindowFocusChanged { id } => {
                let mut state = state_tx.borrow_mut();
                state.set_focused_window(*id);
            }
            _ => {} // Ignore irrelevant events
        }

        debug!("state updated: {} windows", state_tx.borrow().windows.len());
    }
}
