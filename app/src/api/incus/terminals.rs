//! Terminals on Incus Instances: exec sessions and consoles.

use homeport_core::{error::AppResult, terminal::ExecEvent};
use homeport_incus::instances::terminal::{self, IncusExecOptions, IncusTerminal, SessionId};

use crate::app::AppState;

/// Runs a command (a shell by default) in a running Instance, with a
/// terminal, through Incus's exec websockets.
pub async fn incus_exec_start(
    state: &AppState,
    host_id: String,
    project: String,
    name: String,
    options: IncusExecOptions,
    on_event: impl Fn(ExecEvent) -> bool + Send + Sync + 'static,
) -> AppResult<SessionId> {
    let incus = state.hosts.registry.incus(&host_id)?;
    let (session, output) = IncusTerminal::exec(&incus, &project, &name, &options).await?;
    session.resize(options.cols, options.rows).await.ok();
    Ok(state.terminals.incus.open(session, output, on_event))
}

/// Attaches to a running Instance's text console.
pub async fn incus_console_start(
    state: &AppState,
    host_id: String,
    project: String,
    name: String,
    cols: u16,
    rows: u16,
    on_event: impl Fn(ExecEvent) -> bool + Send + Sync + 'static,
) -> AppResult<SessionId> {
    let incus = state.hosts.registry.incus(&host_id)?;
    let (session, output) = IncusTerminal::console(&incus, &project, &name, cols, rows).await?;
    Ok(state.terminals.incus.open(session, output, on_event))
}

/// What an Instance printed on its console, if Incus kept it.
pub async fn incus_console_log(
    state: &AppState,
    host_id: String,
    project: String,
    name: String,
) -> AppResult<String> {
    let incus = state.hosts.registry.incus(&host_id)?;
    Ok(terminal::console_log(&incus, &project, &name).await)
}

pub async fn incus_terminal_input(
    state: &AppState,
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

pub async fn incus_terminal_resize(
    state: &AppState,
    session: SessionId,
    cols: u16,
    rows: u16,
) -> AppResult<()> {
    state.terminals.incus.get(session)?.resize(cols, rows).await
}

/// Closes a terminal: hangs up an exec's command, detaches from a console.
pub async fn incus_terminal_close(state: &AppState, session: SessionId) -> AppResult<()> {
    state.terminals.incus.close(session);
    Ok(())
}

#[cfg(test)]
mod tests {
    use homeport_core::error::ErrorCode;

    use super::*;
    use crate::{
        api::incus::tests::{not_connected, NO_HOST},
        app::testing,
    };

    fn host() -> String {
        NO_HOST.to_owned()
    }

    fn project() -> String {
        "default".to_owned()
    }

    #[tokio::test]
    async fn terminals_need_a_connected_host() {
        let app = testing::app();
        let state = &app.state;
        let name = || "web".to_owned();
        let options = IncusExecOptions {
            command: None,
            user: None,
            cwd: None,
            cols: 80,
            rows: 24,
        };
        let exec = incus_exec_start(state, host(), project(), name(), options, |_| {
            panic!("no output may come from an unknown Host")
        })
        .await;
        not_connected(exec);
        let console = incus_console_start(state, host(), project(), name(), 80, 24, |_| {
            panic!("no output may come from an unknown Host")
        })
        .await;
        not_connected(console);
        not_connected(incus_console_log(state, host(), project(), name()).await);
    }

    #[tokio::test]
    async fn a_terminal_that_ended_refuses_input_and_closes_quietly() {
        let app = testing::app();
        let state = &app.state;
        let typed = incus_terminal_input(state, 7, "ls\n".into()).await;
        assert_eq!(typed.unwrap_err().code, ErrorCode::NotFound);
        let resized = incus_terminal_resize(state, 7, 120, 40).await;
        assert_eq!(resized.unwrap_err().code, ErrorCode::NotFound);
        assert!(incus_terminal_close(state, 7).await.is_ok());
    }
}
