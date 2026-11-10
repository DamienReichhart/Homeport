//! Incus's Instances, system containers and virtual machines: listing
//! them with their state, and starting, stopping, restarting, freezing,
//! unfreezing and deleting them.

mod create;
mod model;
pub mod snapshots;
pub mod terminal;
pub mod usage;
mod wire;

use homeport_core::error::{AppError, AppResult, ErrorCode};
use homeport_transport::http::Request;

pub(crate) use self::wire::RawInstance;
use self::wire::RawState;
pub use self::{
    create::{create, validate_name},
    model::{
        IncusInstance, IncusInstanceDetail, InstanceAction, InstanceSource, InstanceSpec,
        InstanceStatus, InstanceType,
    },
};
use crate::{
    api::{
        client::{scoped, segment, Incus},
        operations,
    },
    messages as msg,
};

/// The API path of the Instance `name`.
pub(crate) fn path(name: &str) -> String {
    format!("/1.0/instances/{}", segment(name))
}

/// Every Instance of `project`, with its state.
pub async fn list(incus: &Incus, project: &str) -> AppResult<Vec<IncusInstance>> {
    let raw: Vec<RawInstance> = incus
        .sync(Request::get(scoped("/1.0/instances?recursion=2", project)))
        .await?;
    let mut instances: Vec<IncusInstance> = raw.iter().map(RawInstance::summary).collect();
    instances.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(instances)
}

/// The Instance `name` of `project`, with its configuration and devices.
pub async fn get(incus: &Incus, project: &str, name: &str) -> AppResult<IncusInstanceDetail> {
    let value: serde_json::Value = incus
        .sync(Request::get(scoped(
            &format!("{}?recursion=1", path(name)),
            project,
        )))
        .await
        .map_err(|e| not_found(e, name))?;
    let mut raw: RawInstance = crate::api::client::metadata(value.clone())?;
    let state: RawState = incus
        .sync(Request::get(scoped(
            &format!("{}/state", path(name)),
            project,
        )))
        .await
        .unwrap_or_default();
    raw.state = Some(state);
    let snapshot_names = raw
        .snapshots
        .as_ref()
        .map(|s| s.iter().map(|s| s.name.clone()).collect())
        .unwrap_or_default();
    Ok(IncusInstanceDetail {
        instance: raw.summary(),
        stateful_snapshots: raw
            .expanded_config
            .get("migration.stateful")
            .is_some_and(|v| v == "true"),
        config: raw.config,
        expanded_config: raw.expanded_config,
        devices: raw.devices,
        expanded_devices: raw.expanded_devices,
        snapshot_names,
        raw: value,
    })
}

/// A missing Instance, named.
fn not_found(error: AppError, name: &str) -> AppError {
    if error.is(ErrorCode::NotFound) {
        AppError::new(
            ErrorCode::NotFound,
            msg::INSTANCE_NOT_FOUND.with("name", name),
        )
        .with_hint(msg::INSTANCE_NOT_FOUND_HINT)
        .with_detail(error.detail.unwrap_or(error.message))
    } else {
        error
    }
}

/// The body of a state change.
pub(crate) fn state_body(
    action: InstanceAction,
    force: bool,
    timeout: Option<u32>,
) -> serde_json::Value {
    let mut body = serde_json::json!({ "action": action.as_str(), "force": force });
    if let Some(timeout) = timeout {
        body["timeout"] = serde_json::json!(timeout);
    }
    body
}

/// How long a graceful stop or restart may take before Incus gives up.
pub const STOP_TIMEOUT_SECS: u32 = 30;

/// Asks the Instance `name` to change state and waits for it. A graceful
/// stop or restart that doesn't finish in time says to force it.
pub async fn act(
    incus: &Incus,
    project: &str,
    name: &str,
    action: InstanceAction,
    force: bool,
) -> AppResult<()> {
    let graceful = matches!(action, InstanceAction::Stop | InstanceAction::Restart) && !force;
    let body = state_body(action, force, graceful.then_some(STOP_TIMEOUT_SECS));
    let request = Request::put(scoped(&format!("{}/state", path(name)), project)).json(&body)?;
    match incus.run(project, request, operations::SHORT).await {
        Ok(_) => Ok(()),
        Err(e) if graceful && looks_stuck(&e) => Err(AppError::new(
            ErrorCode::Timeout,
            msg::STOP_TIMED_OUT
                .with("name", name)
                .with("seconds", STOP_TIMEOUT_SECS),
        )
        .with_hint(msg::STOP_TIMED_OUT_HINT)
        .with_detail(e.detail.unwrap_or(e.message))),
        Err(e) => Err(not_found(e, name)),
    }
}

/// Whether a graceful stop failed because the guest didn't shut down.
fn looks_stuck(error: &AppError) -> bool {
    error.is(ErrorCode::Timeout)
        || error
            .detail
            .as_deref()
            .is_some_and(|d| d.to_lowercase().contains("timed out") || d.contains("deadline"))
}

/// Deletes the Instance `name` with its snapshots; `stop_first` stops a
/// running one by force first, since Incus refuses to delete it otherwise.
pub async fn delete(incus: &Incus, project: &str, name: &str, stop_first: bool) -> AppResult<()> {
    if stop_first {
        match act(incus, project, name, InstanceAction::Stop, true).await {
            Ok(()) => {}
            // Already stopped.
            Err(e) if e.is(ErrorCode::ContainerNotRunning) || e.is(ErrorCode::AlreadyDone) => {}
            Err(e) => return Err(e),
        }
    }
    incus
        .run(
            project,
            Request::delete(scoped(&path(name), project)),
            operations::SHORT,
        )
        .await
        .map(drop)
        .map_err(|e| {
            if e.is(ErrorCode::Conflict) || e.is(ErrorCode::BadRequest) {
                let running = e
                    .detail
                    .as_deref()
                    .is_some_and(|d| d.to_lowercase().contains("running"));
                if running {
                    return AppError::new(
                        ErrorCode::ContainerRunning,
                        msg::INSTANCE_RUNNING.with("name", name),
                    )
                    .with_hint(msg::INSTANCE_RUNNING_HINT)
                    .with_detail(e.detail.unwrap_or(e.message));
                }
            }
            not_found(e, name)
        })
}

#[cfg(test)]
pub(crate) mod tests;
