//! Incus images: the Host's, a remote's catalogue, copies and aliases.

use homeport_core::{error::AppResult, pull::PullEvent, streams::StreamId};
use homeport_incus::images::{
    self,
    simplestreams::{self, RemoteImage},
    ImageCopy, IncusImage,
};

use crate::app::AppState;

/// The images of `project`, newest first.
pub async fn incus_images(
    state: &AppState,
    host_id: String,
    project: String,
) -> AppResult<Vec<IncusImage>> {
    images::list(&state.hosts.registry.incus(&host_id)?, &project).await
}

/// What `server` (a simplestreams image server) publishes for the Host's
/// architecture, read from this computer and kept a few minutes.
pub async fn incus_remote_images(
    state: &AppState,
    host_id: String,
    server: String,
) -> AppResult<Vec<RemoteImage>> {
    let incus = state.hosts.registry.incus(&host_id)?;
    simplestreams::catalogue(&server, &incus.server().arch).await
}

/// Copies an image from a remote server into `project`, streaming the
/// download's progress.
pub async fn incus_image_copy_subscribe(
    state: &AppState,
    host_id: String,
    project: String,
    copy: ImageCopy,
    on_event: impl Fn(PullEvent) -> bool + Send + Sync + 'static,
) -> AppResult<StreamId> {
    let incus = state.hosts.registry.incus(&host_id)?;
    Ok(state.streams.spawn(async move {
        images::copy(&incus, &project, &copy, on_event).await;
    }))
}

/// Fetches a newer build of an image from where it came from.
pub async fn incus_image_refresh(
    state: &AppState,
    host_id: String,
    project: String,
    fingerprint: String,
) -> AppResult<()> {
    images::refresh(
        &state.hosts.registry.incus(&host_id)?,
        &project,
        &fingerprint,
    )
    .await
}

/// Deletes an image; Instances made from it keep their own copy.
pub async fn incus_image_delete(
    state: &AppState,
    host_id: String,
    project: String,
    fingerprint: String,
) -> AppResult<()> {
    images::delete(
        &state.hosts.registry.incus(&host_id)?,
        &project,
        &fingerprint,
    )
    .await
}

pub async fn incus_image_alias_add(
    state: &AppState,
    host_id: String,
    project: String,
    fingerprint: String,
    name: String,
) -> AppResult<()> {
    let incus = state.hosts.registry.incus(&host_id)?;
    images::add_alias(&incus, &project, &fingerprint, &name).await
}

pub async fn incus_image_alias_remove(
    state: &AppState,
    host_id: String,
    project: String,
    name: String,
) -> AppResult<()> {
    images::remove_alias(&state.hosts.registry.incus(&host_id)?, &project, &name).await
}

#[cfg(test)]
mod tests {
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
    async fn images_need_a_connected_host() {
        let app = testing::app();
        let state = &app.state;
        let fingerprint = || "8d6a1f0a3c".to_owned();
        not_connected(incus_images(state, host(), project()).await);
        let server = "https://images.linuxcontainers.org".to_owned();
        not_connected(incus_remote_images(state, host(), server).await);
        not_connected(incus_image_refresh(state, host(), project(), fingerprint()).await);
        not_connected(incus_image_delete(state, host(), project(), fingerprint()).await);
        let alias = || "debian/12".to_owned();
        not_connected(
            incus_image_alias_add(state, host(), project(), fingerprint(), alias()).await,
        );
        not_connected(incus_image_alias_remove(state, host(), project(), alias()).await);
    }

    #[tokio::test]
    async fn a_copy_needs_a_connected_host_and_starts_no_stream() {
        let app = testing::app();
        let copy: ImageCopy = serde_json::from_value(serde_json::json!({
            "server": "https://images.linuxcontainers.org",
            "alias": "debian/12",
            "type": "container",
            "autoUpdate": true,
            "addAlias": true,
        }))
        .unwrap();
        let copied = incus_image_copy_subscribe(&app.state, host(), project(), copy, |_| {
            panic!("no progress may come from an unknown Host")
        })
        .await;
        not_connected(copied);
    }
}
