//! Volumes and which containers use them.

use std::collections::HashMap;

use bollard::{
    models::VolumeCreateRequest,
    query_parameters::{
        ListContainersOptionsBuilder, ListVolumesOptions, PruneVolumesOptionsBuilder,
        RemoveVolumeOptionsBuilder,
    },
    Docker,
};
use serde::{Deserialize, Serialize};

use homeport_core::error::{AppError, AppResult};

use crate::{images::PruneReport, COMPOSE_PROJECT};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct VolumeView {
    pub name: String,
    pub driver: String,
    pub mountpoint: String,
    pub created_at: Option<String>,
    pub labels: HashMap<String, String>,
    pub stack: Option<String>,
    /// Names of containers mounting this volume.
    pub used_by: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct VolumeInput {
    pub name: String,
    /// Defaults to `local`.
    pub driver: Option<String>,
    pub labels: HashMap<String, String>,
}

pub async fn list(docker: &Docker) -> AppResult<Vec<VolumeView>> {
    let mut used_by: HashMap<String, Vec<String>> = HashMap::new();
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
        for mount in c.mounts.unwrap_or_default() {
            if let Some(volume) = mount.name {
                used_by.entry(volume).or_default().push(name.clone());
            }
        }
    }
    let mut volumes: Vec<VolumeView> = docker
        .list_volumes(None::<ListVolumesOptions>)
        .await?
        .volumes
        .unwrap_or_default()
        .into_iter()
        .map(|v| VolumeView {
            used_by: used_by.remove(&v.name).unwrap_or_default(),
            stack: v.labels.get(COMPOSE_PROJECT).cloned(),
            created_at: v.created_at.map(|d| d.to_string()),
            name: v.name,
            driver: v.driver,
            mountpoint: v.mountpoint,
            labels: v.labels,
        })
        .collect();
    volumes.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(volumes)
}

pub async fn inspect(docker: &Docker, name: &str) -> AppResult<serde_json::Value> {
    Ok(serde_json::to_value(docker.inspect_volume(name).await?)?)
}

pub async fn create(docker: &Docker, input: VolumeInput) -> AppResult<String> {
    let name = input.name.trim();
    if name.is_empty() {
        return Err(AppError::validation("Enter a volume name."));
    }
    let created = docker
        .create_volume(VolumeCreateRequest {
            name: Some(name.to_owned()),
            driver: input.driver.filter(|d| !d.trim().is_empty()),
            labels: Some(input.labels),
            ..Default::default()
        })
        .await?;
    Ok(created.name)
}

pub async fn remove(docker: &Docker, name: &str, force: bool) -> AppResult<()> {
    let options = RemoveVolumeOptionsBuilder::default().force(force).build();
    Ok(docker.remove_volume(name, Some(options)).await?)
}

/// Removes unused anonymous volumes, or every unused volume with `all`.
pub async fn prune(docker: &Docker, all: bool) -> AppResult<PruneReport> {
    let filters = if all {
        HashMap::from([("all", vec!["true"])])
    } else {
        HashMap::new()
    };
    let report = docker
        .prune_volumes(Some(
            PruneVolumesOptionsBuilder::default()
                .filters(&filters)
                .build(),
        ))
        .await?;
    Ok(PruneReport {
        removed: report.volumes_deleted.unwrap_or_default(),
        space_reclaimed: report.space_reclaimed.unwrap_or(0),
    })
}

#[cfg(test)]
mod tests {
    use bollard::{
        models::{ContainerCreateBody, HostConfig},
        query_parameters::{CreateContainerOptionsBuilder, RemoveContainerOptionsBuilder},
    };

    use super::*;
    use crate::testing;

    #[tokio::test]
    #[ignore = "needs a local Docker Engine"]
    async fn volumes_show_the_containers_using_them() {
        let docker = testing::docker();
        testing::ensure_image(&docker).await;
        let volume = create(
            &docker,
            VolumeInput {
                name: testing::unique("vol"),
                driver: None,
                labels: HashMap::new(),
            },
        )
        .await
        .unwrap();
        let container = testing::unique("vol-user");
        docker
            .create_container(
                Some(
                    CreateContainerOptionsBuilder::default()
                        .name(&container)
                        .build(),
                ),
                ContainerCreateBody {
                    image: Some(testing::IMAGE.into()),
                    host_config: Some(HostConfig {
                        binds: Some(vec![format!("{volume}:/data")]),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            )
            .await
            .unwrap();

        let listed = list(&docker).await.unwrap();
        let view = listed.iter().find(|v| v.name == volume).unwrap();
        assert_eq!(view.used_by, std::slice::from_ref(&container));
        assert!(remove(&docker, &volume, false).await.is_err(), "in use");

        docker
            .remove_container(
                &container,
                Some(RemoveContainerOptionsBuilder::default().force(true).build()),
            )
            .await
            .unwrap();
        remove(&docker, &volume, false).await.unwrap();
    }
}
