//! Incus Instances: listing, acting on, creating and deleting them.

use tauri::{ipc::Channel, State};

use homeport_core::{pull::PullEvent, streams::StreamId};
use homeport_incus::instances::{
    self, IncusInstance, IncusInstanceDetail, InstanceAction, InstanceSpec,
};

use crate::{app::AppState, error::AppResult};

/// The Instances of `project`, with their state.
#[tauri::command]
#[specta::specta]
pub async fn incus_instances(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
) -> AppResult<Vec<IncusInstance>> {
    instances::list(&state.hosts.registry.incus(&host_id)?, &project).await
}

/// One Instance with its configuration, devices and snapshots' names.
#[tauri::command]
#[specta::specta]
pub async fn incus_instance(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    name: String,
) -> AppResult<IncusInstanceDetail> {
    instances::get(&state.hosts.registry.incus(&host_id)?, &project, &name).await
}

/// Starts, stops, restarts, freezes or unfreezes an Instance; `force` stops
/// or restarts it without waiting for its system to shut down.
#[tauri::command]
#[specta::specta]
pub async fn incus_instance_act(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    name: String,
    action: InstanceAction,
    force: bool,
) -> AppResult<()> {
    let incus = state.hosts.registry.incus(&host_id)?;
    instances::act(&incus, &project, &name, action, force).await
}

/// Makes an Instance from an image, streaming the image download's
/// progress until it's made (and started, if asked).
#[tauri::command]
#[specta::specta]
pub async fn incus_instance_create_subscribe(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    spec: InstanceSpec,
    on_event: Channel<PullEvent>,
) -> AppResult<StreamId> {
    let incus = state.hosts.registry.incus(&host_id)?;
    instances::validate_name(&spec.name)?;
    Ok(state.streams.spawn(async move {
        instances::create(&incus, &project, &spec, move |e| on_event.send(e).is_ok()).await;
    }))
}

/// Deletes an Instance with its snapshots; `stop_first` stops a running one.
#[tauri::command]
#[specta::specta]
pub async fn incus_instance_delete(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    name: String,
    stop_first: bool,
) -> AppResult<()> {
    let incus = state.hosts.registry.incus(&host_id)?;
    instances::delete(&incus, &project, &name, stop_first).await
}
