//! Networks and the containers attached to them.

use std::collections::HashMap;

use bollard::{
    models::{
        Ipam, IpamConfig, NetworkConnectRequest, NetworkCreateRequest, NetworkDisconnectRequest,
    },
    query_parameters::{
        InspectNetworkOptions, ListContainersOptionsBuilder, ListNetworksOptions,
        PruneNetworksOptions,
    },
    Docker,
};
use serde::{Deserialize, Serialize};

use homeport_core::error::{AppError, AppResult};

use crate::messages as msg;

use crate::{images::PruneReport, COMPOSE_PROJECT};

/// Networks every Engine creates; they can't be removed.
pub const BUILT_IN: [&str; 3] = ["bridge", "host", "none"];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct NetworkView {
    pub id: String,
    pub name: String,
    pub driver: String,
    pub scope: String,
    pub internal: bool,
    pub attachable: bool,
    pub subnets: Vec<String>,
    pub created: Option<String>,
    pub labels: HashMap<String, String>,
    pub stack: Option<String>,
    pub built_in: bool,
    /// Names of attached containers.
    pub containers: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct NetworkInput {
    pub name: String,
    /// Defaults to `bridge`.
    pub driver: Option<String>,
    pub subnet: Option<String>,
    pub gateway: Option<String>,
    pub internal: bool,
    pub attachable: bool,
}

pub async fn list(docker: &Docker) -> AppResult<Vec<NetworkView>> {
    let mut attached: HashMap<String, Vec<String>> = HashMap::new();
    for c in docker
        .list_containers(Some(
            ListContainersOptionsBuilder::default().all(true).build(),
        ))
        .await?
    {
        let name = c
            .names
            .and_then(|n| n.into_iter().next())
            .map(|n| n.trim_start_matches('/').to_owned())
            .unwrap_or_default();
        // Keyed by network name: containers that never started have no network id yet.
        for network in c
            .network_settings
            .and_then(|n| n.networks)
            .unwrap_or_default()
            .into_keys()
        {
            attached.entry(network).or_default().push(name.clone());
        }
    }
    let mut networks: Vec<NetworkView> = docker
        .list_networks(None::<ListNetworksOptions>)
        .await?
        .into_iter()
        .map(|n| {
            let id = n.id.unwrap_or_default();
            let name = n.name.unwrap_or_default();
            let labels = n.labels.unwrap_or_default();
            NetworkView {
                containers: attached.remove(&name).unwrap_or_default(),
                built_in: BUILT_IN.contains(&name.as_str()),
                stack: labels.get(COMPOSE_PROJECT).cloned(),
                subnets: n
                    .ipam
                    .and_then(|i| i.config)
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|c| c.subnet)
                    .collect(),
                driver: n.driver.unwrap_or_default(),
                scope: n.scope.unwrap_or_default(),
                internal: n.internal.unwrap_or(false),
                attachable: n.attachable.unwrap_or(false),
                created: n.created.map(|d| d.to_string()),
                id,
                name,
                labels,
            }
        })
        .collect();
    networks.sort_by(|a, b| (!a.built_in, &a.name).cmp(&(!b.built_in, &b.name)));
    Ok(networks)
}

pub async fn inspect(docker: &Docker, id: &str) -> AppResult<serde_json::Value> {
    Ok(serde_json::to_value(
        docker
            .inspect_network(id, None::<InspectNetworkOptions>)
            .await?,
    )?)
}

pub async fn create(docker: &Docker, input: NetworkInput) -> AppResult<String> {
    let name = input.name.trim();
    if name.is_empty() {
        return Err(AppError::validation(msg::NETWORK_NAME));
    }
    let non_empty = |s: Option<String>| s.map(|s| s.trim().to_owned()).filter(|s| !s.is_empty());
    let subnet = non_empty(input.subnet);
    let gateway = non_empty(input.gateway);
    let ipam = (subnet.is_some() || gateway.is_some()).then(|| Ipam {
        config: Some(vec![IpamConfig {
            subnet,
            gateway,
            ..Default::default()
        }]),
        ..Default::default()
    });
    let created = docker
        .create_network(NetworkCreateRequest {
            name: name.to_owned(),
            driver: non_empty(input.driver),
            internal: Some(input.internal),
            attachable: Some(input.attachable),
            ipam,
            ..Default::default()
        })
        .await?;
    Ok(created.id)
}

pub async fn remove(docker: &Docker, id: &str) -> AppResult<()> {
    Ok(docker.remove_network(id).await?)
}

pub async fn connect(docker: &Docker, network: &str, container: &str) -> AppResult<()> {
    Ok(docker
        .connect_network(
            network,
            NetworkConnectRequest {
                container: container.to_owned(),
                endpoint_config: None,
            },
        )
        .await?)
}

pub async fn disconnect(
    docker: &Docker,
    network: &str,
    container: &str,
    force: bool,
) -> AppResult<()> {
    Ok(docker
        .disconnect_network(
            network,
            NetworkDisconnectRequest {
                container: container.to_owned(),
                force: Some(force),
            },
        )
        .await?)
}

/// Removes every network no container uses.
pub async fn prune(docker: &Docker) -> AppResult<PruneReport> {
    let report = docker.prune_networks(None::<PruneNetworksOptions>).await?;
    Ok(PruneReport {
        removed: report.networks_deleted.unwrap_or_default(),
        space_reclaimed: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing;

    #[tokio::test]
    #[ignore = "needs a local Docker Engine"]
    async fn containers_can_join_and_leave_networks() {
        let docker = testing::docker();
        let name = testing::unique("net");
        let id = create(
            &docker,
            NetworkInput {
                name: name.clone(),
                driver: None,
                subnet: Some("10.231.0.0/24".into()),
                gateway: None,
                internal: true,
                attachable: false,
            },
        )
        .await
        .unwrap();
        let container_name = testing::unique("net-user");
        let _c = testing::container(&docker, &container_name, &["sleep", "60"]).await;

        connect(&docker, &id, &container_name).await.unwrap();
        let listed = list(&docker).await.unwrap();
        let view = listed.iter().find(|n| n.name == name).unwrap();
        assert!(view.internal && !view.built_in);
        assert_eq!(view.subnets, ["10.231.0.0/24"]);
        assert_eq!(view.containers, std::slice::from_ref(&container_name));
        assert!(listed.iter().any(|n| n.name == "bridge" && n.built_in));

        disconnect(&docker, &id, &container_name, false)
            .await
            .unwrap();
        remove(&docker, &id).await.unwrap();
    }
}
