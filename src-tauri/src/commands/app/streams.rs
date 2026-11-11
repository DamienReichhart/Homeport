//! The streams every `*_subscribe` command starts.

use tauri::State;

use homeport_core::streams::StreamId;

use crate::app::AppState;

/// Stops a stream started by any `*_subscribe` command.
#[tauri::command]
#[specta::specta]
pub fn stream_cancel(state: State<'_, AppState>, id: StreamId) {
    state.streams.cancel(id);
}
