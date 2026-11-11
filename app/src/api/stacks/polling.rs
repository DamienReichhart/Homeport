//! Polling Synced Stacks while Homeport runs: the poller lives in
//! `homeport-stacks`; this tells the UI what it finds, and deploys new
//! commits for it through the same path as Sync, recorded as automatic.

use std::{future::Future, pin::Pin, sync::Arc};

use homeport_core::error::{AppError, ErrorCode};
use homeport_hosts::registry::HostStatus;
use homeport_stacks::{
    compose::{ComposeAction, ComposeEvent},
    model::definition,
    runtime::{self, SourceKind},
    synced::{
        poller::{AutoDeploy, PollEvents, Poller},
        SyncedStack,
    },
    StackOwner,
};

use super::target;
use crate::{
    app::{events::Event, AppState},
    messages as msg,
};

pub struct AppPollEvents(pub Arc<AppState>);

impl PollEvents for AppPollEvents {
    fn update_available(&self, stack: &SyncedStack) {
        self.0.events.send(Event::SyncedUpdateAvailable {
            owner: stack.owner.clone(),
            stack_id: stack.id.clone(),
            name: stack.name.clone(),
        });
    }

    fn auto_deploy(
        &self,
        stack: SyncedStack,
        commit: String,
    ) -> Pin<Box<dyn Future<Output = AutoDeploy> + Send + '_>> {
        Box::pin(async move {
            let state = &self.0;
            // Paused while the Host, or every manager of the Swarm, can't be
            // reached; the next round tries again.
            let reachable = match &stack.owner {
                StackOwner::Host(id) => {
                    matches!(
                        state.hosts.registry.status(id),
                        HostStatus::Connected { .. }
                    )
                }
                StackOwner::Swarm(id) => state.hosts.registry.manager_host(id).is_ok(),
            };
            if !reachable {
                return AutoDeploy::Skipped;
            }
            match deploy(state, &stack, &commit).await {
                Ok(()) => AutoDeploy::Deployed,
                Err(e) => AutoDeploy::Failed(e.logged("auto-deploying a Synced Stack")),
            }
        })
    }

    fn auto_deployed(&self, stack: &SyncedStack, commit: &str, error: Option<&AppError>) {
        self.0.events.send(Event::SyncedAutoDeployed {
            owner: stack.owner.clone(),
            stack_id: stack.id.clone(),
            name: stack.name.clone(),
            commit: commit.to_owned(),
            error: error.cloned().map(Box::new),
        });
    }
}

/// Deploys `commit` of `stack` as Sync would: rebuilding when a service is
/// built, and noting the deployed commit once it succeeds.
async fn deploy(state: &AppState, stack: &SyncedStack, commit: &str) -> Result<(), AppError> {
    let cli = homeport_git::cli::detect().await?;
    let compose = state
        .stacks
        .synced
        .compose_file(&cli, &stack.repository_id, commit, &stack.compose_path)
        .await?;
    let action = if definition::builds(&[&compose]) {
        ComposeAction::Rebuild
    } else {
        ComposeAction::Up
    };
    let (job, _) = target::job(
        state,
        SourceKind::Synced,
        &stack.id,
        action,
        Some(commit),
        true,
    )
    .await?;
    let outcome = std::sync::Mutex::new(None);
    runtime::run(job, |e| {
        match e {
            ComposeEvent::Exit { success, error, .. } => {
                *outcome.lock().expect("outcome poisoned") = Some(if success {
                    Ok(())
                } else {
                    Err(error.unwrap_or_else(|| {
                        AppError::new(ErrorCode::ComposeFailed, msg::AUTO_DEPLOY_FAILED)
                    }))
                });
            }
            ComposeEvent::Error { error } => {
                *outcome.lock().expect("outcome poisoned") = Some(Err(error));
            }
            ComposeEvent::Line { .. } => {}
        }
        true
    })
    .await;
    let outcome = outcome
        .into_inner()
        .expect("outcome poisoned")
        .unwrap_or_else(|| {
            Err(AppError::new(
                ErrorCode::ComposeFailed,
                msg::AUTO_DEPLOY_FAILED,
            ))
        });
    outcome?;
    state.stacks.synced.store.set_deployed(&stack.id, commit)?;
    Ok(())
}

/// Polls Synced Stacks for as long as it is awaited, on the interval set in
/// Settings. The shell spawns it once the app's state is built.
pub async fn run(state: Arc<AppState>) {
    let poller = Arc::new(Poller::new(
        state.stacks.synced.clone(),
        Arc::new(AppPollEvents(state.clone())),
    ));
    poller
        .run(move || {
            state
                .settings
                .get()
                .map(|s| s.sync_interval())
                .unwrap_or(std::time::Duration::from_secs(300))
        })
        .await;
}

#[cfg(test)]
mod tests {
    use std::{sync::Arc, time::Duration};

    use homeport_core::error::{AppError, ErrorCode};
    use homeport_stacks::{
        synced::poller::{AutoDeploy, PollEvents},
        StackOwner,
    };

    use super::{super::fixtures, AppPollEvents};
    use crate::app::{events::Event, testing};

    #[tokio::test]
    async fn the_ui_hears_of_updates_and_automatic_deploys() {
        let mut app = testing::app();
        let owner = fixtures::host(&app.state);
        let stack = fixtures::synced(&app.state, &owner);
        let events = AppPollEvents(Arc::new(app.state));

        events.update_available(&stack);
        let Some(Event::SyncedUpdateAvailable {
            owner: of,
            stack_id,
            name,
        }) = app.events.try_recv().ok()
        else {
            panic!("no update was published");
        };
        assert_eq!(
            (of, stack_id, name.as_str()),
            (owner, stack.id.clone(), "synced")
        );

        let failure = AppError::new(
            ErrorCode::ComposeFailed,
            crate::messages::AUTO_DEPLOY_FAILED,
        );
        events.auto_deployed(&stack, "0123abc", Some(&failure));
        let Some(Event::SyncedAutoDeployed {
            stack_id,
            commit,
            error,
            ..
        }) = app.events.try_recv().ok()
        else {
            panic!("no automatic deploy was published");
        };
        assert_eq!((stack_id, commit.as_str()), (stack.id.clone(), "0123abc"));
        assert_eq!(error.map(|e| e.code), Some(ErrorCode::ComposeFailed));
        events.auto_deployed(&stack, "0123abc", None);
        assert!(matches!(
            app.events.try_recv(),
            Ok(Event::SyncedAutoDeployed { error: None, .. })
        ));
    }

    #[tokio::test]
    async fn an_automatic_deploy_waits_for_the_host_or_a_manager() {
        let app = testing::app();
        let mut stack = fixtures::synced(&app.state, &fixtures::host(&app.state));
        let events = AppPollEvents(Arc::new(app.state));
        let outcome = events.auto_deploy(stack.clone(), "0123abc".into()).await;
        assert!(matches!(outcome, AutoDeploy::Skipped), "{outcome:?}");
        stack.owner = StackOwner::Swarm("gone".into());
        let outcome = events.auto_deploy(stack.clone(), "0123abc".into()).await;
        assert!(matches!(outcome, AutoDeploy::Skipped), "{outcome:?}");

        // Its repository was never cloned, so there's no compose file to read.
        assert!(super::deploy(&events.0, &stack, "0123abc").await.is_err());
        let stored = events.0.stacks.synced.store.get(&stack.id).unwrap();
        assert_eq!(stored.deployed_commit, None);
    }

    #[tokio::test]
    async fn polling_runs_until_it_is_dropped() {
        let app = testing::app();
        let polling = super::run(Arc::new(app.state));
        let ended = tokio::time::timeout(Duration::from_millis(50), polling).await;
        assert!(ended.is_err(), "the poller stopped by itself");
    }
}
