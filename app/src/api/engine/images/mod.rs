//! Images on a Host.

use homeport_core::{
    error::{AppError, AppResult},
    streams::StreamId,
};
use homeport_engine::{
    build::{self, BuildEvent, BuildSpec},
    containers,
    images::{
        self,
        freshness::{FreshnessResult, FreshnessTarget},
        Image, ImageLayer, PruneReport, PullEvent, SearchResult,
    },
    transfer::{self, ImportSpec, TransferEvent},
};

use crate::api::hosts::registries;
use crate::app::AppState;

pub async fn images_list(state: &AppState, host_id: String) -> AppResult<Vec<Image>> {
    images::list(&state.hosts.registry.docker(&host_id)?).await
}

pub async fn images_inspect(
    state: &AppState,
    host_id: String,
    id: String,
) -> AppResult<serde_json::Value> {
    images::inspect(&state.hosts.registry.docker(&host_id)?, &id).await
}

pub async fn images_history(
    state: &AppState,
    host_id: String,
    id: String,
) -> AppResult<Vec<ImageLayer>> {
    images::history(&state.hosts.registry.docker(&host_id)?, &id).await
}

/// Searches Docker Hub from the Host, as its pulls would reach it.
pub async fn images_search(
    state: &AppState,
    host_id: String,
    term: String,
) -> AppResult<Vec<SearchResult>> {
    images::search(&state.hosts.registry.docker(&host_id)?, &term, 25).await
}

pub async fn images_tag(
    state: &AppState,
    host_id: String,
    id: String,
    target: String,
) -> AppResult<()> {
    images::tag(&state.hosts.registry.docker(&host_id)?, &id, &target).await
}

pub async fn images_remove(
    state: &AppState,
    host_id: String,
    id: String,
    force: bool,
) -> AppResult<Vec<String>> {
    images::remove(&state.hosts.registry.docker(&host_id)?, &id, force).await
}

pub async fn images_prune(state: &AppState, host_id: String, all: bool) -> AppResult<PruneReport> {
    images::prune(&state.hosts.registry.docker(&host_id)?, all).await
}

pub async fn images_pull_subscribe(
    state: &AppState,
    host_id: String,
    reference: String,
    on_event: impl Fn(PullEvent) -> bool + Send + Sync + 'static,
) -> AppResult<StreamId> {
    let docker = state.hosts.registry.docker(&host_id)?;
    let auth = registries::auth_for(state, &reference)?;
    let explained = registries::explainer(state, &reference);
    Ok(state
        .streams
        .spawn(images::pull(docker, reference, auth, move |e| {
            on_event(explain_event(&explained, e))
        })))
}

/// Pushes an image name to its registry with the stored login, streaming progress.
pub async fn images_push_subscribe(
    state: &AppState,
    host_id: String,
    reference: String,
    on_event: impl Fn(PullEvent) -> bool + Send + Sync + 'static,
) -> AppResult<StreamId> {
    let docker = state.hosts.registry.docker(&host_id)?;
    let auth = registries::auth_for(state, &reference)?;
    let explained = registries::push_explainer(state, &reference);
    Ok(state
        .streams
        .spawn(images::push(docker, reference, auth, move |e| {
            on_event(explain_event(&explained, e))
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
pub async fn images_build_subscribe(
    state: &AppState,
    host_id: String,
    spec: BuildSpec,
    on_event: impl Fn(BuildEvent) -> bool + Send + Sync + 'static,
) -> AppResult<StreamId> {
    build::validate(&spec)?;
    let docker = state.hosts.registry.docker(&host_id)?;
    let logins = registries::logins(state)?.all().to_vec();
    Ok(state
        .streams
        .spawn(build::build(docker, spec, logins, on_event)))
}

/// Saves images into one tar file on this machine, streaming progress.
pub async fn images_save_subscribe(
    state: &AppState,
    host_id: String,
    names: Vec<String>,
    destination: String,
    on_event: impl Fn(TransferEvent) -> bool + Send + Sync + 'static,
) -> AppResult<StreamId> {
    let docker = state.hosts.registry.docker(&host_id)?;
    Ok(state
        .streams
        .spawn(transfer::save(docker, names, destination.into(), on_event)))
}

/// Loads the images in a tar file on this machine into a Host, streaming progress.
pub async fn images_load_subscribe(
    state: &AppState,
    host_id: String,
    source: String,
    on_event: impl Fn(TransferEvent) -> bool + Send + Sync + 'static,
) -> AppResult<StreamId> {
    let docker = state.hosts.registry.docker(&host_id)?;
    Ok(state
        .streams
        .spawn(transfer::load(docker, source.into(), on_event)))
}

/// Creates an image from a root filesystem tar on this machine (`docker import`), streaming progress.
pub async fn images_import_subscribe(
    state: &AppState,
    host_id: String,
    source: String,
    spec: ImportSpec,
    on_event: impl Fn(TransferEvent) -> bool + Send + Sync + 'static,
) -> AppResult<StreamId> {
    let docker = state.hosts.registry.docker(&host_id)?;
    Ok(state
        .streams
        .spawn(transfer::import(docker, source.into(), spec, on_event)))
}

/// Saves a container's filesystem to a tar on this machine (`docker export`), streaming progress.
pub async fn containers_export_subscribe(
    state: &AppState,
    host_id: String,
    id: String,
    destination: String,
    on_event: impl Fn(TransferEvent) -> bool + Send + Sync + 'static,
) -> AppResult<StreamId> {
    let docker = state.hosts.registry.docker(&host_id)?;
    Ok(state
        .streams
        .spawn(transfer::export(docker, id, destination.into(), on_event)))
}

/// Whether a newer image is published for each container's image and each
/// tagged image on a Host. Registry answers are cached (`force` skips the
/// cache); a registry that can't be asked leaves its images `unknown`.
pub async fn images_freshness(
    state: &AppState,
    host_id: String,
    force: bool,
) -> AppResult<Vec<FreshnessResult>> {
    let docker = state.hosts.registry.docker(&host_id)?;
    let mut targets: Vec<FreshnessTarget> = containers::list(&docker)
        .await?
        .into_iter()
        .map(|c| FreshnessTarget {
            reference: c.image,
            image_id: c.image_id,
        })
        .collect();
    for image in images::list(&docker).await? {
        targets.extend(image.tags.into_iter().map(|reference| FreshnessTarget {
            reference,
            image_id: image.id.clone(),
        }));
    }
    targets.sort_by(|a, b| (&a.reference, &a.image_id).cmp(&(&b.reference, &b.image_id)));
    targets.dedup();
    let logins = registries::logins(state)?;
    Ok(state
        .freshness
        .check(&docker, &host_id, targets, &logins, force)
        .await)
}

#[cfg(test)]
mod tests;
