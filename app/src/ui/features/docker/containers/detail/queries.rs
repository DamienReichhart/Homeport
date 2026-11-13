//! What only a container's page reads (`containerQueries`), filed under
//! its Host's containers as in React: whatever makes the list out of
//! date (an action, an Engine event) makes these out of date too.

use homeport_engine::containers::{files::FileEntry, health::Health, Processes};

use super::model::inspect::Inspected;
use crate::{
    api::engine::{
        containers::{containers_health, containers_inspect, containers_processes},
        files::files_list,
    },
    data::{keys, Key, Query},
};

/// Everything of one container (`containerKey`).
fn container(host_id: &str, container_id: &str) -> Key {
    keys::host::containers(host_id).with(container_id)
}

pub fn inspect_key(host_id: &str, container_id: &str, state: &str) -> Key {
    container(host_id, container_id).with("inspect").with(state)
}

/// The inspect document, read again when the container's state changes:
/// the state is part of its key. The document is read into the tab's
/// sections and its tree by the read itself, off the window's thread.
pub fn inspect(host_id: &str, container_id: &str, state: &str) -> Query<Inspected> {
    let (host, id) = (host_id.to_owned(), container_id.to_owned());
    Query::new(inspect_key(host_id, container_id, state), move |app| {
        let (host, id) = (host.clone(), id.clone());
        async move {
            let document = containers_inspect(&app, host, id).await?;
            tokio::task::spawn_blocking(move || Inspected::of(&document))
                .await
                .map_err(|died| std::io::Error::other(died).into())
        }
    })
}

pub fn health_key(host_id: &str, container_id: &str, status: Option<&str>) -> Key {
    container(host_id, container_id)
        .with("health")
        .with_optional(status)
}

/// The healthcheck and its last results, read again when the status the
/// list shows changes: the status is part of its key.
pub fn health(host_id: &str, container_id: &str, status: Option<&str>) -> Query<Option<Health>> {
    let (host, id) = (host_id.to_owned(), container_id.to_owned());
    Query::new(health_key(host_id, container_id, status), move |app| {
        let (host, id) = (host.clone(), id.clone());
        async move { containers_health(&app, host, id).await }
    })
}

pub fn files_key(host_id: &str, container_id: &str, path: &str) -> Key {
    container(host_id, container_id).with("files").with(path)
}

/// What a folder of the container holds.
pub fn files(host_id: &str, container_id: &str, path: &str) -> Query<Vec<FileEntry>> {
    let (host, id, folder) = (host_id.to_owned(), container_id.to_owned(), path.to_owned());
    Query::new(files_key(host_id, container_id, path), move |app| {
        let (host, id, folder) = (host.clone(), id.clone(), folder.clone());
        async move { files_list(&app, host, id, folder).await }
    })
}

pub fn processes_key(host_id: &str, container_id: &str) -> Key {
    container(host_id, container_id).with("processes")
}

/// What runs in the container (`docker top`).
pub fn processes(host_id: &str, container_id: &str) -> Query<Processes> {
    let (host, id) = (host_id.to_owned(), container_id.to_owned());
    Query::new(processes_key(host_id, container_id), move |app| {
        let (host, id) = (host.clone(), id.clone());
        async move { containers_processes(&app, host, id).await }
    })
}
