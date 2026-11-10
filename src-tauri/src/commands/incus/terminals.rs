//! Terminals on Incus Instances: exec sessions and consoles.

use tauri::{ipc::Channel, State};

use homeport_core::terminal::ExecEvent;
use homeport_incus::terminal::{self, IncusExecOptions, IncusTerminal, SessionId};

use crate::{app::AppState, error::AppResult};

/// Runs a command (a shell by default) in a running Instance, with a
/// terminal, through Incus's exec websockets.
#[tauri::command]
#[specta::specta]
pub async fn incus_exec_start(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    name: String,
    options: IncusExecOptions,
    on_event: Channel<ExecEvent>,
) -> AppResult<SessionId> {
    let incus = state.hosts.registry.incus(&host_id)?;
    let (session, output) = IncusTerminal::exec(&incus, &project, &name, &options).await?;
    session.resize(options.cols, options.rows).await.ok();
    Ok(state
        .terminals
        .incus
        .open(session, output, move |e| on_event.send(e).is_ok()))
}

/// Attaches to a running Instance's text console.
#[tauri::command]
#[specta::specta]
pub async fn incus_console_start(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    name: String,
    cols: u16,
    rows: u16,
    on_event: Channel<ExecEvent>,
) -> AppResult<SessionId> {
    let incus = state.hosts.registry.incus(&host_id)?;
    let (session, output) = IncusTerminal::console(&incus, &project, &name, cols, rows).await?;
    Ok(state
        .terminals
        .incus
        .open(session, output, move |e| on_event.send(e).is_ok()))
}

/// What an Instance printed on its console, if Incus kept it.
#[tauri::command]
#[specta::specta]
pub async fn incus_console_log(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    name: String,
) -> AppResult<String> {
    let incus = state.hosts.registry.incus(&host_id)?;
    Ok(terminal::console_log(&incus, &project, &name).await)
}

#[tauri::command]
#[specta::specta]
pub async fn incus_terminal_input(
    state: State<'_, AppState>,
    session: SessionId,
    data: String,
) -> AppResult<()> {
    state
        .terminals
        .incus
        .get(session)?
        .write(data.as_bytes())
        .await
}

#[tauri::command]
#[specta::specta]
pub async fn incus_terminal_resize(
    state: State<'_, AppState>,
    session: SessionId,
    cols: u16,
    rows: u16,
) -> AppResult<()> {
    state.terminals.incus.get(session)?.resize(cols, rows).await
}

/// Closes a terminal: hangs up an exec's command, detaches from a console.
#[tauri::command]
#[specta::specta]
pub async fn incus_terminal_close(state: State<'_, AppState>, session: SessionId) -> AppResult<()> {
    state.terminals.incus.close(session);
    Ok(())
}
