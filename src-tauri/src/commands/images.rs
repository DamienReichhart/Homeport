//! Images on a Host.

use tauri::{ipc::Channel, State};

use homeport_core::{error::AppError, streams::StreamId};
use homeport_engine::{
    build::{self, BuildEvent, BuildSpec},
    images::{self, Image, ImageLayer, PruneReport, PullEvent},
    transfer::{self, TransferEvent},
};

use crate::{app::AppState, commands::registries, error::AppResult};

#[tauri::command]
#[specta::specta]
pub async fn images_list(state: State<'_, AppState>, host_id: String) -> AppResult<Vec<Image>> {
    images::list(&state.hosts.registry.docker(&host_id)?).await
}

#[tauri::command]
#[specta::specta]
pub async fn images_inspect(
    state: State<'_, AppState>,
    host_id: String,
    id: String,
) -> AppResult<serde_json::Value> {
    images::inspect(&state.hosts.registry.docker(&host_id)?, &id).await
}

#[tauri::command]
#[specta::specta]
pub async fn images_history(
    state: State<'_, AppState>,
    host_id: String,
    id: String,
) -> AppResult<Vec<ImageLayer>> {
    images::history(&state.hosts.registry.docker(&host_id)?, &id).await
}

#[tauri::command]
#[specta::specta]
pub async fn images_tag(
    state: State<'_, AppState>,
    host_id: String,
    id: String,
    target: String,
) -> AppResult<()> {
    images::tag(&state.hosts.registry.docker(&host_id)?, &id, &target).await
}

#[tauri::command]
#[specta::specta]
pub async fn images_remove(
    state: State<'_, AppState>,
    host_id: String,
    id: String,
    force: bool,
) -> AppResult<Vec<String>> {
    images::remove(&state.hosts.registry.docker(&host_id)?, &id, force).await
}

#[tauri::command]
#[specta::specta]
pub async fn images_prune(
    state: State<'_, AppState>,
    host_id: String,
    all: bool,
) -> AppResult<PruneReport> {
    images::prune(&state.hosts.registry.docker(&host_id)?, all).await
}

#[tauri::command]
#[specta::specta]
pub async fn images_pull_subscribe(
    state: State<'_, AppState>,
    host_id: String,
    reference: String,
    on_event: Channel<PullEvent>,
) -> AppResult<StreamId> {
    let docker = state.hosts.registry.docker(&host_id)?;
    let auth = registries::auth_for(&state, &reference)?;
    let explained = registries::explainer(&state, &reference);
    Ok(state
        .streams
        .spawn(images::pull(docker, reference, auth, move |e| {
            on_event.send(explain_event(&explained, e)).is_ok()
        })))
}

/// Pushes an image name to its registry with the stored login, streaming progress.
#[tauri::command]
#[specta::specta]
pub async fn images_push_subscribe(
    state: State<'_, AppState>,
    host_id: String,
    reference: String,
    on_event: Channel<PullEvent>,
) -> AppResult<StreamId> {
    let docker = state.hosts.registry.docker(&host_id)?;
    let auth = registries::auth_for(&state, &reference)?;
    let explained = registries::push_explainer(&state, &reference);
    Ok(state
        .streams
        .spawn(images::push(docker, reference, auth, move |e| {
            on_event.send(explain_event(&explained, e)).is_ok()
        })))
}

/// Applies a registry hint to a pull or push failure.
fn explain_event(explain: &impl Fn(AppError) -> AppError, event: PullEvent) -> PullEvent {
    match event {
        PullEvent::Error { error } => PullEvent::Error {
            error: explain(error),
        },
        e => e,
    }
}

/// Builds an image on a Host from a folder on this machine, streaming the
/// output. Every stored login is sent, for private `FROM` images.
#[tauri::command]
#[specta::specta]
pub async fn images_build_subscribe(
    state: State<'_, AppState>,
    host_id: String,
    spec: BuildSpec,
    on_event: Channel<BuildEvent>,
) -> AppResult<StreamId> {
    build::validate(&spec)?;
    let docker = state.hosts.registry.docker(&host_id)?;
    let logins = registries::logins(&state)?.all().to_vec();
    Ok(state
        .streams
        .spawn(build::build(docker, spec, logins, move |e| {
            on_event.send(e).is_ok()
        })))
}

/// Saves images into one tar file on this machine, streaming progress.
#[tauri::command]
#[specta::specta]
pub async fn images_save_subscribe(
    state: State<'_, AppState>,
    host_id: String,
    names: Vec<String>,
    destination: String,
    on_event: Channel<TransferEvent>,
) -> AppResult<StreamId> {
    let docker = state.hosts.registry.docker(&host_id)?;
    Ok(state.streams.spawn(transfer::save(
        docker,
        names,
        destination.into(),
        move |e| on_event.send(e).is_ok(),
    )))
}

/// Loads the images in a tar file on this machine into a Host, streaming progress.
#[tauri::command]
#[specta::specta]
pub async fn images_load_subscribe(
    state: State<'_, AppState>,
    host_id: String,
    source: String,
    on_event: Channel<TransferEvent>,
) -> AppResult<StreamId> {
    let docker = state.hosts.registry.docker(&host_id)?;
    Ok(state
        .streams
        .spawn(transfer::load(docker, source.into(), move |e| {
            on_event.send(e).is_ok()
        })))
}
