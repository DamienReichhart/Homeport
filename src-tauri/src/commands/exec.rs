//! Interactive shells inside a container.

use tauri::{ipc::Channel, State};

use homeport_engine::containers::exec::{self, ExecEvent, ExecOptions, SessionId};

use crate::{app::AppState, error::AppResult};

#[tauri::command]
#[specta::specta]
pub async fn exec_start(
    state: State<'_, AppState>,
    host_id: String,
    container_id: String,
    options: ExecOptions,
    on_event: Channel<ExecEvent>,
) -> AppResult<SessionId> {
    let docker = state.hosts.registry.docker(&host_id)?;
    let (session, output) = exec::start(&docker, &container_id, options).await?;
    Ok(state
        .exec
        .open(session, output, move |e| on_event.send(e).is_ok()))
}

#[tauri::command]
#[specta::specta]
pub async fn exec_input(
    state: State<'_, AppState>,
    session: SessionId,
    data: String,
) -> AppResult<()> {
    state.exec.get(session)?.write(data.as_bytes()).await
}

#[tauri::command]
#[specta::specta]
pub async fn exec_resize(
    state: State<'_, AppState>,
    session: SessionId,
    cols: u16,
    rows: u16,
) -> AppResult<()> {
    state.exec.get(session)?.resize(cols, rows).await
}

#[tauri::command]
#[specta::specta]
pub fn exec_close(state: State<'_, AppState>, session: SessionId) {
    state.exec.close(session);
}
