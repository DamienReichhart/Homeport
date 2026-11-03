//! Lifecycle of any Stack through the Engine API, so External Stacks can be
//! started, stopped and removed without their compose files.

use std::collections::HashMap;

use bollard::{
    query_parameters::{ListContainersOptionsBuilder, ListNetworksOptionsBuilder},
    Docker,
};
use serde::Deserialize;

use homeport_engine::{
    containers::{self, Container, ContainerAction},
    COMPOSE_PROJECT,
};

use crate::error::AppResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum StackAction {
    Start,
    Stop,
    Restart,
}

fn project_filter(name: &str) -> HashMap<&'static str, Vec<String>> {
    HashMap::from([("label", vec![format!("{COMPOSE_PROJECT}={name}")])])
}

/// The containers of the Stack named `name`.
pub async fn containers_of(docker: &Docker, name: &str) -> AppResult<Vec<Container>> {
    let options = ListContainersOptionsBuilder::default()
        .all(true)
        .filters(&project_filter(name))
        .build();
    Ok(docker
        .list_containers(Some(options))
        .await?
        .into_iter()
        .map(Container::from)
        .collect())
}

pub async fn act(docker: &Docker, name: &str, action: StackAction) -> AppResult<()> {
    for c in containers_of(docker, name).await? {
        let running = c.state == "running";
        match action {
            StackAction::Start if !running => {
                containers::act(docker, &c.id, ContainerAction::Start).await?
            }
            StackAction::Stop if running => {
                containers::act(docker, &c.id, ContainerAction::Stop).await?
            }
            StackAction::Restart => {
                containers::act(docker, &c.id, ContainerAction::Restart).await?
            }
            _ => {}
        }
    }
    Ok(())
}

/// Removes the Stack's containers and the networks Compose created for it
/// that nothing else uses. Volumes are kept.
pub async fn remove(docker: &Docker, name: &str) -> AppResult<()> {
    for c in containers_of(docker, name).await? {
        containers::remove(docker, &c.id, true, false).await?;
    }
    let options = ListNetworksOptionsBuilder::default()
        .filters(&project_filter(name))
        .build();
    for network in docker.list_networks(Some(options)).await? {
        // Best effort: a network another container still uses stays behind.
        if let Some(id) = network.id {
            if let Err(e) = docker.remove_network(&id).await {
                log::warn!("kept network {id} of stack {name}: {e}");
            }
        }
    }
    Ok(())
}
