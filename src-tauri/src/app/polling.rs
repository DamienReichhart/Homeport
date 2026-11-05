//! Polling Synced Stacks while Homeport runs: the poller lives in
//! `homeport-stacks`; this tells the webview what it finds, and deploys new
//! commits for it through the same path as Sync, recorded as automatic.

use std::{future::Future, pin::Pin, sync::Arc};

use tauri::{AppHandle, Manager};
use tauri_specta::Event;

use homeport_core::error::{AppError, ErrorCode};
use homeport_hosts::registry::HostStatus;
use homeport_stacks::{
    compose::{ComposeAction, ComposeEvent},
    definition,
    runtime::{self, SourceKind},
    synced::{
        poller::{AutoDeploy, PollEvents, Poller},
        SyncedStack,
    },
};

use super::{
    events::{SyncedAutoDeployed, SyncedUpdateAvailable},
    AppState,
};
use crate::{commands::stacks::recorded_job, messages as msg};

pub struct TauriPollEvents(pub AppHandle);

impl PollEvents for TauriPollEvents {
    fn update_available(&self, stack: &SyncedStack) {
        let event = SyncedUpdateAvailable {
            host_id: stack.host_id.clone(),
            stack_id: stack.id.clone(),
            name: stack.name.clone(),
        };
        if let Err(e) = event.emit(&self.0) {
            log::warn!("failed to emit a Synced Stack's update: {e}");
        }
    }

    fn auto_deploy(
        &self,
        stack: SyncedStack,
        commit: String,
    ) -> Pin<Box<dyn Future<Output = AutoDeploy> + Send + '_>> {
        Box::pin(async move {
            let state = self.0.state::<AppState>();
            // Paused while the Host can't be reached; the next round tries again.
            if !matches!(
                state.hosts.registry.status(&stack.host_id),
                HostStatus::Connected { .. }
            ) {
                return AutoDeploy::Skipped;
            }
            match deploy(&state, &stack, &commit).await {
                Ok(()) => AutoDeploy::Deployed,
                Err(e) => AutoDeploy::Failed(e.logged("auto-deploying a Synced Stack")),
            }
        })
    }

    fn auto_deployed(&self, stack: &SyncedStack, commit: &str, error: Option<&AppError>) {
        let event = SyncedAutoDeployed {
            host_id: stack.host_id.clone(),
            stack_id: stack.id.clone(),
            name: stack.name.clone(),
            commit: commit.to_owned(),
            error: error.cloned(),
        };
        if let Err(e) = event.emit(&self.0) {
            log::warn!("failed to emit a Synced Stack's auto-deploy: {e}");
        }
    }
}

/// Deploys `commit` of `stack` as Sync would: rebuilding when a service is
/// built, and noting the deployed commit once it succeeds.
async fn deploy(state: &AppState, stack: &SyncedStack, commit: &str) -> Result<(), AppError> {
    let cli = homeport_git::cli::detect().await?;
    let compose = state
        .synced
        .compose_file(&cli, &stack.repository_id, commit, &stack.compose_path)
        .await?;
    let action = if definition::builds(&[&compose]) {
        ComposeAction::Rebuild
    } else {
        ComposeAction::Up
    };
    let (job, _) = recorded_job(
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
    state.synced.store.set_deployed(&stack.id, commit)?;
    Ok(())
}

/// Starts polling Synced Stacks for as long as the app runs, on the interval
/// set in Settings.
pub fn start(app: &AppHandle) {
    let state = app.state::<AppState>();
    let poller = Arc::new(Poller::new(
        state.synced.clone(),
        Arc::new(TauriPollEvents(app.clone())),
    ));
    let handle = app.clone();
    tauri::async_runtime::spawn(poller.run(move || {
        handle
            .state::<AppState>()
            .settings
            .get()
            .map(|s| s.sync_interval())
            .unwrap_or(std::time::Duration::from_secs(300))
    }));
}
