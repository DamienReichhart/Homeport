//! The streams every `*_subscribe` command starts.

use homeport_core::streams::StreamId;
use tauri::State;

use crate::state::AppState;

/// Stops a stream started by any `*_subscribe` command.
#[tauri::command]
#[specta::specta]
pub fn stream_cancel(state: State<'_, AppState>, id: StreamId) {
    state.streams.cancel(id);
}
