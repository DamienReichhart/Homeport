//! Volumes on a Host.

use std::{collections::HashMap, path::Path};

use homeport_core::{error::AppResult, streams::StreamId};
use homeport_engine::{
    containers::files::FileEntry,
    images::{self, PruneReport, PullEvent},
    transfer::TransferEvent,
    volumes::{
        self,
        archive::{self, RestoreSpec},
        browse,
        helper::{self, HelperImage},
        VolumeInput, VolumeView,
    },
};

use crate::api::hosts::registries;
use crate::app::AppState;

pub async fn volumes_list(state: &AppState, host_id: String) -> AppResult<Vec<VolumeView>> {
    volumes::list(&state.hosts.registry.docker(&host_id)?).await
}

/// Each volume's size in bytes, `null` when its driver doesn't report one.
/// Apart from the list, because the Engine walks every volume to answer.
pub async fn volumes_sizes(
    state: &AppState,
    host_id: String,
) -> AppResult<HashMap<String, Option<u64>>> {
    volumes::sizes(&state.hosts.registry.docker(&host_id)?).await
}

pub async fn volumes_inspect(
    state: &AppState,
    host_id: String,
    name: String,
) -> AppResult<serde_json::Value> {
    volumes::inspect(&state.hosts.registry.docker(&host_id)?, &name).await
}

pub async fn volumes_create(
    state: &AppState,
    host_id: String,
    input: VolumeInput,
) -> AppResult<String> {
    volumes::create(&state.hosts.registry.docker(&host_id)?, input).await
}

pub async fn volumes_remove(
    state: &AppState,
    host_id: String,
    name: String,
    force: bool,
) -> AppResult<()> {
    volumes::remove(&state.hosts.registry.docker(&host_id)?, &name, force).await
}

pub async fn volumes_prune(state: &AppState, host_id: String, all: bool) -> AppResult<PruneReport> {
    volumes::prune(&state.hosts.registry.docker(&host_id)?, all).await
}

/// The image of the helper container that opens volumes (ADR 0015), and
/// whether the Host has it.
pub async fn volumes_helper_image(state: &AppState, host_id: String) -> AppResult<HelperImage> {
    helper::image(&state.hosts.registry.docker(&host_id)?).await
}

/// Pulls the helper's image onto a Host, once the user agreed, streaming
/// progress. A failure says how to get the image there by hand.
pub async fn volumes_helper_pull_subscribe(
    state: &AppState,
    host_id: String,
    on_event: impl Fn(PullEvent) -> bool + Send + Sync + 'static,
) -> AppResult<StreamId> {
    let docker = state.hosts.registry.docker(&host_id)?;
    let auth = registries::auth_for(state, helper::IMAGE)?;
    Ok(state.streams.spawn(images::pull(
        docker,
        helper::IMAGE.into(),
        auth,
        move |event| {
            on_event(match event {
                PullEvent::Error { error } => PullEvent::Error {
                    error: helper::image_unavailable(Some(&error)),
                },
                e => e,
            })
        },
    )))
}

/// Opens `volume` read-only in a helper; returns the session's id. With
/// `pulled_image`, ending the session also removes the image Homeport pulled.
pub async fn volumes_browse_start(
    state: &AppState,
    host_id: String,
    volume: String,
    pulled_image: bool,
) -> AppResult<String> {
    let docker = state.hosts.registry.docker(&host_id)?;
    state
        .volume_browser
        .start(&docker, &volume, pulled_image)
        .await
}

/// Lists a folder of a browsed volume; paths are relative to the volume.
pub async fn volumes_browse_list(
    state: &AppState,
    host_id: String,
    session: String,
    path: String,
) -> AppResult<Vec<FileEntry>> {
    browse::list(&state.hosts.registry.docker(&host_id)?, &session, &path).await
}

/// Saves a file, or a folder as a `.tar`, of a browsed volume to `destination`.
pub async fn volumes_browse_download(
    state: &AppState,
    host_id: String,
    session: String,
    path: String,
    destination: String,
) -> AppResult<bool> {
    let docker = state.hosts.registry.docker(&host_id)?;
    browse::download(&docker, &session, &path, Path::new(&destination)).await
}

/// Ends a browse session, removing its helper.
pub async fn volumes_browse_end(state: &AppState, session: String) -> AppResult<()> {
    state.volume_browser.end(&session).await;
    Ok(())
}

/// The running containers mounting a volume: a backup of it may be
/// inconsistent, and a restore into it is refused until they stop.
pub async fn volumes_running_users(
    state: &AppState,
    host_id: String,
    volume: String,
) -> AppResult<Vec<String>> {
    archive::running_users(&state.hosts.registry.docker(&host_id)?, &volume).await
}

/// Backs a volume up to a `.tar` at `destination` on this machine,
/// streaming progress. With `pulled_image`, the helper's image is removed
/// afterwards if nothing else uses it.
pub async fn volumes_backup_subscribe(
    state: &AppState,
    host_id: String,
    volume: String,
    destination: String,
    pulled_image: bool,
    on_event: impl Fn(TransferEvent) -> bool + Send + Sync + 'static,
) -> AppResult<StreamId> {
    let docker = state.hosts.registry.docker(&host_id)?;
    Ok(state.streams.spawn(archive::backup(
        docker,
        volume,
        destination.into(),
        pulled_image,
        on_event,
    )))
}

/// Restores a volume, new or existing, from a `.tar` at `source` on this
/// machine, streaming progress.
pub async fn volumes_restore_subscribe(
    state: &AppState,
    host_id: String,
    source: String,
    spec: RestoreSpec,
    pulled_image: bool,
    on_event: impl Fn(TransferEvent) -> bool + Send + Sync + 'static,
) -> AppResult<StreamId> {
    let docker = state.hosts.registry.docker(&host_id)?;
    Ok(state.streams.spawn(archive::restore(
        docker,
        source.into(),
        spec,
        pulled_image,
        on_event,
    )))
}

/// Copies a volume into a new one on the same Host; returns its name. With
/// `pulled_image`, the helper's image is removed afterwards if unused.
pub async fn volumes_clone(
    state: &AppState,
    host_id: String,
    source: String,
    target: String,
    pulled_image: bool,
) -> AppResult<String> {
    let docker = state.hosts.registry.docker(&host_id)?;
    volumes::clone(&docker, &source, &target, pulled_image).await
}

#[cfg(test)]
mod tests {
    use homeport_core::error::ErrorCode;

    use super::*;
    use crate::api::engine::testing::{assert_all, code, host};
    use crate::app::testing::app;

    #[tokio::test]
    async fn a_host_that_isnt_connected_is_refused() {
        let app = app();
        let s = &app.state;
        let vol = || "data".to_owned();
        let tar = || app.dir.path().join("data.tar").display().to_string();
        let input = VolumeInput {
            name: vol(),
            driver: None,
            labels: HashMap::new(),
        };
        let restore = RestoreSpec {
            volume: vol(),
            create: true,
            replace: false,
        };
        let results = [
            ("list", code(volumes_list(s, host()).await)),
            ("sizes", code(volumes_sizes(s, host()).await)),
            ("inspect", code(volumes_inspect(s, host(), vol()).await)),
            ("create", code(volumes_create(s, host(), input).await)),
            (
                "remove",
                code(volumes_remove(s, host(), vol(), false).await),
            ),
            ("prune", code(volumes_prune(s, host(), true).await)),
            ("helper_image", code(volumes_helper_image(s, host()).await)),
            (
                "helper_pull",
                code(volumes_helper_pull_subscribe(s, host(), |_| true).await),
            ),
            (
                "browse_start",
                code(volumes_browse_start(s, host(), vol(), false).await),
            ),
            (
                "browse_list",
                code(volumes_browse_list(s, host(), "s1".into(), "/".into()).await),
            ),
            (
                "browse_download",
                code(volumes_browse_download(s, host(), "s1".into(), "/".into(), tar()).await),
            ),
            (
                "running_users",
                code(volumes_running_users(s, host(), vol()).await),
            ),
            (
                "backup",
                code(volumes_backup_subscribe(s, host(), vol(), tar(), false, |_| true).await),
            ),
            (
                "restore",
                code(volumes_restore_subscribe(s, host(), tar(), restore, false, |_| true).await),
            ),
            (
                "clone",
                code(volumes_clone(s, host(), vol(), "copy".into(), false).await),
            ),
        ];
        assert_all(ErrorCode::HostNotConnected, &results);
        assert!(!std::path::Path::new(&tar()).exists());
    }

    #[tokio::test]
    async fn ending_a_browse_session_that_is_gone_does_nothing() {
        let app = app();
        assert_eq!(
            code(volumes_browse_end(&app.state, "s1".into()).await),
            None
        );
    }
}
