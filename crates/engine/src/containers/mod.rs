//! Containers: listing, inspection and lifecycle here; what runs inside one
//! (exec, files, logs, stats) and creating one (`run`) in submodules.

pub mod commit;
pub mod exec;
pub mod files;
pub mod health;
pub mod logs;
pub mod recreate;
pub mod run;
pub mod stats;

use std::collections::HashMap;

use bollard::{
    models::{ContainerSummary as EngineSummary, PortSummaryTypeEnum},
    query_parameters::{
        KillContainerOptions, ListContainersOptionsBuilder, RemoveContainerOptionsBuilder,
        RenameContainerOptionsBuilder, RestartContainerOptions, StartContainerOptions,
        StopContainerOptions, TopOptionsBuilder,
    },
    Docker,
};
use serde::{Deserialize, Serialize};

use homeport_core::error::{AppError, AppResult, ErrorCode};

use crate::messages as msg;

use crate::{COMPOSE_PROJECT, COMPOSE_SERVICE};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Container {
    pub id: String,
    pub name: String,
    pub image: String,
    pub image_id: String,
    pub command: String,
    /// Unix time in seconds.
    pub created: i64,
    /// `created`, `running`, `paused`, `restarting`, `exited`, `removing` or `dead`.
    pub state: String,
    /// Human status from the Engine, e.g. "Up 3 hours (healthy)".
    pub status: String,
    pub health: Option<String>,
    pub ports: Vec<PortBinding>,
    pub labels: HashMap<String, String>,
    pub stack: Option<String>,
    pub service: Option<String>,
    pub networks: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct PortBinding {
    pub ip: Option<String>,
    pub private_port: u16,
    pub public_port: Option<u16>,
    pub protocol: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum ContainerAction {
    Start,
    Stop,
    Restart,
    Pause,
    Unpause,
    Kill,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
pub struct Processes {
    pub titles: Vec<String>,
    pub processes: Vec<Vec<String>>,
}

impl From<EngineSummary> for Container {
    fn from(c: EngineSummary) -> Self {
        let labels = c.labels.unwrap_or_default();
        let mut ports: Vec<PortBinding> = c
            .ports
            .unwrap_or_default()
            .into_iter()
            .map(|p| PortBinding {
                ip: p.ip.filter(|ip| !ip.is_empty()),
                private_port: p.private_port,
                public_port: p.public_port,
                protocol: p.typ.unwrap_or(PortSummaryTypeEnum::TCP).to_string(),
            })
            .collect();
        ports.sort_by_key(|p| (p.private_port, p.public_port, p.ip.clone()));
        // The Engine lists IPv4 and IPv6 bindings separately; show each port once.
        ports.dedup_by(|a, b| {
            a.private_port == b.private_port
                && a.public_port == b.public_port
                && a.protocol == b.protocol
        });
        let mut networks: Vec<String> = c
            .network_settings
            .and_then(|n| n.networks)
            .map(|n| n.into_keys().collect())
            .unwrap_or_default();
        networks.sort();
        Self {
            id: c.id.unwrap_or_default(),
            name: c
                .names
                .and_then(|n| n.into_iter().next())
                .map(|n| n.trim_start_matches('/').to_owned())
                .unwrap_or_default(),
            image: c.image.unwrap_or_default(),
            image_id: c.image_id.unwrap_or_default(),
            command: c.command.unwrap_or_default(),
            created: c.created.unwrap_or_default(),
            state: c.state.map(|s| s.to_string()).unwrap_or_default(),
            status: c.status.unwrap_or_default(),
            health: c
                .health
                .and_then(|h| h.status)
                .map(|s| s.to_string())
                .filter(|s| !s.is_empty() && s != "none"),
            ports,
            stack: labels.get(COMPOSE_PROJECT).cloned(),
            service: labels.get(COMPOSE_SERVICE).cloned(),
            labels,
            networks,
        }
    }
}

pub async fn list(docker: &Docker) -> AppResult<Vec<Container>> {
    let options = ListContainersOptionsBuilder::default().all(true).build();
    let mut containers: Vec<Container> = docker
        .list_containers(Some(options))
        .await?
        .into_iter()
        .map(Container::from)
        .collect();
    containers.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(containers)
}

/// The Engine's full inspect document, passed through untouched.
pub async fn inspect(docker: &Docker, id: &str) -> AppResult<serde_json::Value> {
    Ok(serde_json::to_value(
        docker.inspect_container(id, None).await?,
    )?)
}

/// Already being in the requested state (Engine status 304) counts as success.
pub async fn act(docker: &Docker, id: &str, action: ContainerAction) -> AppResult<()> {
    let result = match action {
        ContainerAction::Start => {
            docker
                .start_container(id, None::<StartContainerOptions>)
                .await
        }
        ContainerAction::Stop => {
            docker
                .stop_container(id, None::<StopContainerOptions>)
                .await
        }
        ContainerAction::Restart => {
            docker
                .restart_container(id, None::<RestartContainerOptions>)
                .await
        }
        ContainerAction::Pause => docker.pause_container(id).await,
        ContainerAction::Unpause => docker.unpause_container(id).await,
        ContainerAction::Kill => {
            docker
                .kill_container(id, None::<KillContainerOptions>)
                .await
        }
    };
    match result.map_err(AppError::from) {
        Err(e) if e.is(ErrorCode::AlreadyDone) => Ok(()),
        other => other,
    }
}

/// Removes a container; `volumes` also removes its anonymous volumes.
pub async fn remove(docker: &Docker, id: &str, force: bool, volumes: bool) -> AppResult<()> {
    let options = RemoveContainerOptionsBuilder::default()
        .force(force)
        .v(volumes)
        .build();
    Ok(docker.remove_container(id, Some(options)).await?)
}

pub async fn rename(docker: &Docker, id: &str, name: &str) -> AppResult<()> {
    let name = name.trim().trim_start_matches('/');
    if name.is_empty() {
        return Err(AppError::validation(msg::RENAME_EMPTY));
    }
    let options = RenameContainerOptionsBuilder::default().name(name).build();
    Ok(docker.rename_container(id, options).await?)
}

pub async fn processes(docker: &Docker, id: &str) -> AppResult<Processes> {
    // `ps aux`-style columns are richer than the default, but not every ps supports them.
    let detailed = TopOptionsBuilder::default().ps_args("aux").build();
    let top = match docker.top_processes(id, Some(detailed)).await {
        Ok(top) => top,
        Err(_) => docker.top_processes(id, None).await?,
    };
    Ok(Processes {
        titles: top.titles.unwrap_or_default(),
        processes: top.processes.unwrap_or_default(),
    })
}

#[cfg(test)]
mod tests {
    use bollard::models::{ContainerSummaryStateEnum, PortSummary};

    use super::*;
    use crate::testing;

    #[test]
    fn summaries_are_flattened_for_display() {
        let summary = EngineSummary {
            id: Some("abc".into()),
            names: Some(vec!["/web-1".into()]),
            state: Some(ContainerSummaryStateEnum::RUNNING),
            labels: Some(
                [
                    (COMPOSE_PROJECT.to_string(), "shop".to_string()),
                    (COMPOSE_SERVICE.to_string(), "web".to_string()),
                ]
                .into(),
            ),
            ports: Some(vec![
                PortSummary {
                    ip: Some("0.0.0.0".into()),
                    private_port: 80,
                    public_port: Some(8080),
                    typ: Some(PortSummaryTypeEnum::TCP),
                },
                PortSummary {
                    ip: Some("::".into()),
                    private_port: 80,
                    public_port: Some(8080),
                    typ: Some(PortSummaryTypeEnum::TCP),
                },
            ]),
            ..Default::default()
        };
        let c = Container::from(summary);
        assert_eq!(c.name, "web-1");
        assert_eq!(c.state, "running");
        assert_eq!(c.stack.as_deref(), Some("shop"));
        assert_eq!(c.service.as_deref(), Some("web"));
        assert_eq!(c.ports.len(), 1);
        assert_eq!(c.ports[0].protocol, "tcp");
        assert_eq!(c.health, None);
    }

    #[tokio::test]
    #[ignore = "needs a local Docker Engine"]
    async fn containers_go_through_their_lifecycle() {
        let docker = testing::docker();
        let name = testing::unique("lifecycle");
        let guard = testing::container(&docker, &name, &["sleep", "300"]).await;
        let state = |name: String| {
            let docker = docker.clone();
            async move {
                list(&docker)
                    .await
                    .unwrap()
                    .into_iter()
                    .find(|c| c.name == name)
                    .map(|c| c.state)
            }
        };

        assert_eq!(state(name.clone()).await.as_deref(), Some("created"));
        act(&docker, &guard.id, ContainerAction::Start)
            .await
            .unwrap();
        assert_eq!(state(name.clone()).await.as_deref(), Some("running"));
        act(&docker, &guard.id, ContainerAction::Pause)
            .await
            .unwrap();
        assert_eq!(state(name.clone()).await.as_deref(), Some("paused"));
        act(&docker, &guard.id, ContainerAction::Unpause)
            .await
            .unwrap();

        let top = processes(&docker, &guard.id).await.unwrap();
        assert!(top
            .processes
            .iter()
            .flatten()
            .any(|cell| cell.contains("sleep")));

        let renamed = testing::unique("renamed");
        rename(&docker, &guard.id, &renamed).await.unwrap();
        assert_eq!(state(renamed.clone()).await.as_deref(), Some("running"));

        let inspected = inspect(&docker, &guard.id).await.unwrap();
        assert_eq!(inspected["Name"], format!("/{renamed}"));

        act(&docker, &guard.id, ContainerAction::Kill)
            .await
            .unwrap();
        remove(&docker, &guard.id, false, true).await.unwrap();
        assert_eq!(state(renamed).await, None);
    }
}
