//! Engine-wide information: host details, disk usage and pruning.

use bollard::{
    query_parameters::{DataUsageOptions, PruneBuildOptionsBuilder, PruneContainersOptions},
    Docker,
};
use serde::{Deserialize, Serialize};

use super::{images, networks, volumes};
use crate::error::AppResult;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct EngineInfo {
    pub name: String,
    pub server_version: String,
    pub operating_system: String,
    pub kernel_version: String,
    pub architecture: String,
    pub cpus: i64,
    pub memory: i64,
    pub root_dir: String,
    pub storage_driver: String,
    pub containers: i64,
    pub running: i64,
    pub paused: i64,
    pub stopped: i64,
    pub images: i64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct UsageLine {
    pub total: i64,
    pub active: i64,
    pub size: i64,
    pub reclaimable: i64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct DiskUsage {
    pub images: UsageLine,
    pub containers: UsageLine,
    pub volumes: UsageLine,
    pub build_cache: UsageLine,
}

#[derive(Debug, Clone, Copy, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct PruneSelection {
    pub containers: bool,
    /// Dangling images, or every unused image with `all_images`.
    pub images: bool,
    pub all_images: bool,
    pub networks: bool,
    /// Unused volumes, named ones included. Destroys data.
    pub volumes: bool,
    pub build_cache: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SystemPruneReport {
    pub containers: u32,
    pub images: u32,
    pub networks: u32,
    pub volumes: u32,
    pub space_reclaimed: i64,
}

pub async fn info(docker: &Docker) -> AppResult<EngineInfo> {
    let i = docker.info().await?;
    Ok(EngineInfo {
        name: i.name.unwrap_or_default(),
        server_version: i.server_version.unwrap_or_default(),
        operating_system: i.operating_system.unwrap_or_default(),
        kernel_version: i.kernel_version.unwrap_or_default(),
        architecture: i.architecture.unwrap_or_default(),
        cpus: i.ncpu.unwrap_or_default(),
        memory: i.mem_total.unwrap_or_default(),
        root_dir: i.docker_root_dir.unwrap_or_default(),
        storage_driver: i.driver.unwrap_or_default(),
        containers: i.containers.unwrap_or_default(),
        running: i.containers_running.unwrap_or_default(),
        paused: i.containers_paused.unwrap_or_default(),
        stopped: i.containers_stopped.unwrap_or_default(),
        images: i.images.unwrap_or_default(),
    })
}

pub async fn disk_usage(docker: &Docker) -> AppResult<DiskUsage> {
    let df = docker.df(None::<DataUsageOptions>).await?;
    let line = |total: Option<i64>,
                active: Option<i64>,
                size: Option<i64>,
                reclaimable: Option<i64>| UsageLine {
        total: total.unwrap_or(0),
        active: active.unwrap_or(0),
        size: size.unwrap_or(0),
        reclaimable: reclaimable.unwrap_or(0),
    };
    Ok(DiskUsage {
        images: df
            .image_usage
            .map(|u| line(u.total_count, u.active_count, u.total_size, u.reclaimable))
            .unwrap_or_default(),
        containers: df
            .container_usage
            .map(|u| line(u.total_count, u.active_count, u.total_size, u.reclaimable))
            .unwrap_or_default(),
        volumes: df
            .volume_usage
            .map(|u| line(u.total_count, u.active_count, u.total_size, u.reclaimable))
            .unwrap_or_default(),
        build_cache: df
            .build_cache_usage
            .map(|u| line(u.total_count, u.active_count, u.total_size, u.reclaimable))
            .unwrap_or_default(),
    })
}

/// Prunes the selected kinds of unused resources, containers first so the
/// images, networks and volumes they held become unused too.
pub async fn prune(docker: &Docker, selection: PruneSelection) -> AppResult<SystemPruneReport> {
    let mut report = SystemPruneReport::default();
    if selection.containers {
        let r = docker
            .prune_containers(None::<PruneContainersOptions>)
            .await?;
        report.containers = r.containers_deleted.map_or(0, |d| d.len() as u32);
        report.space_reclaimed += r.space_reclaimed.unwrap_or(0);
    }
    if selection.images {
        let r = images::prune(docker, selection.all_images).await?;
        report.images = r.removed.len() as u32;
        report.space_reclaimed += r.space_reclaimed;
    }
    if selection.networks {
        report.networks = networks::prune(docker).await?.removed.len() as u32;
    }
    if selection.volumes {
        let r = volumes::prune(docker, true).await?;
        report.volumes = r.removed.len() as u32;
        report.space_reclaimed += r.space_reclaimed;
    }
    if selection.build_cache {
        let r = docker
            .prune_build(Some(PruneBuildOptionsBuilder::default().build()))
            .await?;
        report.space_reclaimed += r.space_reclaimed.unwrap_or(0);
    }
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::docker::testing;

    #[tokio::test]
    #[ignore = "needs a local Docker Engine"]
    async fn reports_engine_details_and_disk_usage() {
        let docker = testing::docker();
        testing::ensure_image(&docker).await;
        let info = info(&docker).await.unwrap();
        assert!(info.cpus > 0 && info.memory > 0);
        assert!(!info.server_version.is_empty());
        let usage = disk_usage(&docker).await.unwrap();
        assert!(usage.images.total >= 1 && usage.images.size > 0);
    }
}
