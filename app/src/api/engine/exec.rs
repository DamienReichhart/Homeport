//! Interactive shells inside a container.

use homeport_core::error::AppResult;
use homeport_engine::containers::exec::{self, ExecEvent, ExecOptions, SessionId};

use crate::app::AppState;

pub async fn exec_start(
    state: &AppState,
    host_id: String,
    container_id: String,
    options: ExecOptions,
    on_event: impl Fn(ExecEvent) -> bool + Send + Sync + 'static,
) -> AppResult<SessionId> {
    let docker = state.hosts.registry.docker(&host_id)?;
    let (session, output) = exec::start(&docker, &container_id, options).await?;
    Ok(state.terminals.engine.open(session, output, on_event))
}

pub async fn exec_input(state: &AppState, session: SessionId, data: String) -> AppResult<()> {
    state
        .terminals
        .engine
        .get(session)?
        .write(data.as_bytes())
        .await
}

pub async fn exec_resize(
    state: &AppState,
    session: SessionId,
    cols: u16,
    rows: u16,
) -> AppResult<()> {
    state
        .terminals
        .engine
        .get(session)?
        .resize(cols, rows)
        .await
}

pub fn exec_close(state: &AppState, session: SessionId) {
    state.terminals.engine.close(session);
}

#[cfg(test)]
mod tests {
    use homeport_core::error::ErrorCode;

    use super::*;
    use crate::api::engine::testing::{code, host};
    use crate::app::testing::app;

    #[tokio::test]
    async fn a_terminal_needs_a_connected_host() {
        let app = app();
        let options = ExecOptions::default();
        let started = exec_start(&app.state, host(), "c1".into(), options, |_| true).await;
        assert_eq!(code(started), Some(ErrorCode::HostNotConnected));
    }

    #[tokio::test]
    async fn a_session_that_ended_takes_no_input() {
        let app = app();
        let typed = exec_input(&app.state, 7, "ls\n".into()).await;
        assert_eq!(code(typed), Some(ErrorCode::NotFound));
        let resized = exec_resize(&app.state, 7, 80, 24).await;
        assert_eq!(code(resized), Some(ErrorCode::NotFound));
    }

    #[tokio::test]
    async fn closing_a_session_that_ended_does_nothing() {
        let app = app();
        exec_close(&app.state, 7);
        exec_close(&app.state, 7);
    }
}
