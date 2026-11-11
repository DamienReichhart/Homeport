//! Deploying a Stack through the runtime, its deploy history, rolling back and
//! what a redeploy would change.

use homeport_core::{error::AppResult, streams::StreamId};
use homeport_stacks::{
    compose::{ComposeAction, ComposeEvent},
    model::diff::{Candidate, DeployDiff},
    runtime::{self, SourceKind},
    store::history::{self, DeployDetail, DeploySummary},
};

use super::target;
use crate::app::AppState;

/// Runs a compose command for the Managed, Linked or Synced Stack `id`,
/// streaming its output. A Synced Stack's deploy that succeeds is noted as
/// its deployed commit.
pub async fn stacks_compose_subscribe(
    state: &AppState,
    kind: SourceKind,
    id: String,
    action: ComposeAction,
    on_event: impl Fn(ComposeEvent) -> bool + Send + Sync + 'static,
) -> AppResult<StreamId> {
    spawn(state, kind, id, action, None, on_event).await
}

/// Starts the compose job for `action` on the Stack `id`, `commit` for a
/// Synced Stack's reviewed one; a Synced Stack's deploy that succeeds is
/// noted as its deployed commit.
pub(super) async fn spawn(
    state: &AppState,
    kind: SourceKind,
    id: String,
    action: ComposeAction,
    commit: Option<String>,
    on_event: impl Fn(ComposeEvent) -> bool + Send + Sync + 'static,
) -> AppResult<StreamId> {
    let (job, commit) = target::job(state, kind, &id, action, commit.as_deref(), false).await?;
    let synced = state.stacks.synced.clone();
    Ok(state.streams.spawn(runtime::run(job, move |e| {
        if let (Some(commit), ComposeEvent::Exit { success: true, .. }) = (&commit, &e) {
            if action.deploys() {
                if let Err(e) = synced.store.set_deployed(&id, commit) {
                    e.logged("noting a Synced Stack's deployed commit");
                }
            }
        }
        on_event(e)
    })))
}

/// What redeploying the Managed, Linked or Synced Stack `id` would change:
/// its latest successful deploy against `candidate`, or against its stored
/// definition (a Linked Stack's files as they are now, a Synced Stack's
/// compose file at its fetched commit) when there's none. Environment values
/// are compared here and never sent back.
pub async fn stacks_diff(
    state: &AppState,
    kind: SourceKind,
    id: String,
    candidate: Option<Candidate>,
) -> AppResult<DeployDiff> {
    state.stacks.diff(kind, &id, candidate).await
}

/// A Stack's recorded deploys, newest first.
pub fn stacks_history(state: &AppState, id: String) -> AppResult<Vec<DeploySummary>> {
    state.stacks.history.list(&id)
}

/// One recorded deploy, with its definition and the names of its variables.
pub fn stacks_deploy(state: &AppState, id: String) -> AppResult<DeployDetail> {
    Ok(DeployDetail {
        deploy: state.stacks.history.get(&id)?,
        env_keys: state.stacks.history.env_keys(&id)?,
    })
}

/// Rolls a Managed Stack back to the recorded deploy `deploy_id`: puts its
/// definition, environment and profiles back, then redeploys, streaming the
/// output.
pub async fn stacks_rollback_subscribe(
    state: &AppState,
    deploy_id: String,
    on_event: impl Fn(ComposeEvent) -> bool + Send + Sync + 'static,
) -> AppResult<StreamId> {
    let stack = history::restore(&state.stacks.history, &state.stacks.managed, &deploy_id)?;
    let (job, _) = target::job(
        state,
        SourceKind::Managed,
        &stack.id,
        ComposeAction::Up,
        None,
        false,
    )
    .await?;
    Ok(state.streams.spawn(runtime::run(job, on_event)))
}

#[cfg(test)]
mod tests;
