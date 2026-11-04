//! Resource usage of running containers, computed the way `docker stats` does.

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use bollard::{
    models::ContainerStatsResponse,
    query_parameters::{ListContainersOptionsBuilder, StatsOptionsBuilder},
    Docker,
};
use futures_util::StreamExt;
use serde::Serialize;
use tokio::task::JoinHandle;

#[derive(Debug, Clone, PartialEq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct StatsSample {
    /// Percent of one CPU; 200 means two CPUs fully used.
    pub cpu_percent: f64,
    /// Bytes in use, excluding reclaimable page cache.
    pub memory_usage: u64,
    pub memory_limit: u64,
    pub net_rx: u64,
    pub net_tx: u64,
    pub block_read: u64,
    pub block_write: u64,
    pub pids: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ContainerUsage {
    pub id: String,
    pub cpu_percent: f64,
    pub memory_usage: u64,
    pub memory_limit: u64,
}

/// Reduces one Engine stats report to the numbers the UI shows.
pub fn sample(stats: &ContainerStatsResponse) -> StatsSample {
    let cpu = stats.cpu_stats.as_ref();
    let pre = stats.precpu_stats.as_ref();
    let total = |c: Option<&bollard::models::ContainerCpuStats>| {
        c.and_then(|c| c.cpu_usage.as_ref())
            .and_then(|u| u.total_usage)
            .unwrap_or(0)
    };
    let cpu_delta = total(cpu).saturating_sub(total(pre)) as f64;
    let system_delta =
        cpu.and_then(|c| c.system_cpu_usage)
            .unwrap_or(0)
            .saturating_sub(pre.and_then(|c| c.system_cpu_usage).unwrap_or(0)) as f64;
    let online_cpus = cpu
        .and_then(|c| c.online_cpus.map(u64::from))
        .or_else(|| {
            cpu.and_then(|c| c.cpu_usage.as_ref())
                .and_then(|u| u.percpu_usage.as_ref())
                .map(|p| p.len() as u64)
        })
        .unwrap_or(1) as f64;
    // The first report of a stream has no previous reading to compare with.
    let has_history = pre.and_then(|c| c.system_cpu_usage).is_some_and(|s| s > 0);
    let cpu_percent = if has_history && system_delta > 0.0 && cpu_delta > 0.0 {
        cpu_delta / system_delta * online_cpus * 100.0
    } else {
        0.0
    };

    let memory = stats.memory_stats.as_ref();
    let usage = memory.and_then(|m| m.usage).unwrap_or(0);
    // cgroup v2 reports `inactive_file`, v1 `total_inactive_file`; both are page cache.
    let cache = memory
        .and_then(|m| m.stats.as_ref())
        .and_then(|s| {
            s.get("inactive_file")
                .or_else(|| s.get("total_inactive_file"))
        })
        .copied()
        .filter(|&c| c < usage)
        .unwrap_or(0);

    let (net_rx, net_tx) = stats
        .networks
        .iter()
        .flat_map(|n| n.values())
        .fold((0, 0), |(rx, tx), n| {
            (rx + n.rx_bytes.unwrap_or(0), tx + n.tx_bytes.unwrap_or(0))
        });

    let (block_read, block_write) = stats
        .blkio_stats
        .as_ref()
        .and_then(|b| b.io_service_bytes_recursive.as_ref())
        .into_iter()
        .flatten()
        .fold((0, 0), |(r, w), e| {
            match e.op.as_deref().map(str::to_ascii_lowercase).as_deref() {
                Some("read") => (r + e.value.unwrap_or(0), w),
                Some("write") => (r, w + e.value.unwrap_or(0)),
                _ => (r, w),
            }
        });

    StatsSample {
        cpu_percent,
        memory_usage: usage - cache,
        memory_limit: memory.and_then(|m| m.limit).unwrap_or(0),
        net_rx,
        net_tx,
        block_read,
        block_write,
        pids: stats
            .pids_stats
            .as_ref()
            .and_then(|p| p.current)
            .unwrap_or(0),
    }
}

/// Streams samples for one container (about one per second) until it stops.
pub async fn stream(docker: Docker, id: String, sink: impl Fn(StatsSample) -> bool) {
    let options = StatsOptionsBuilder::default().stream(true).build();
    let mut stats = docker.stats(&id, Some(options));
    while let Some(Ok(report)) = stats.next().await {
        if !sink(sample(&report)) {
            return;
        }
    }
}

/// Per-container stats tasks, aborted when the overview stream ends or is cancelled.
struct Watchers(HashMap<String, JoinHandle<()>>);

impl Drop for Watchers {
    fn drop(&mut self) {
        self.0.values().for_each(JoinHandle::abort);
    }
}

type Latest = Arc<Mutex<HashMap<String, ContainerUsage>>>;

/// Every two seconds, sends the latest usage of every running container.
pub async fn overview(docker: Docker, sink: impl Fn(Vec<ContainerUsage>) -> bool) {
    let latest: Latest = Arc::default();
    let mut watchers = Watchers(HashMap::new());
    let mut tick = tokio::time::interval(Duration::from_secs(2));
    for n in 0u64.. {
        tick.tick().await;
        // Pick up started and stopped containers every third tick.
        if n % 3 == 0 {
            watch_running(&docker, &mut watchers.0, &latest).await;
        }
        let snapshot = latest
            .lock()
            .expect("lock poisoned")
            .values()
            .cloned()
            .collect();
        if !sink(snapshot) {
            return;
        }
    }
}

async fn watch_running(
    docker: &Docker,
    watchers: &mut HashMap<String, JoinHandle<()>>,
    latest: &Latest,
) {
    let Ok(list) = docker
        .list_containers(Some(ListContainersOptionsBuilder::default().build()))
        .await
    else {
        return;
    };
    let running: Vec<String> = list.into_iter().filter_map(|c| c.id).collect();
    watchers.retain(|id, task| {
        let keep = running.contains(id) && !task.is_finished();
        if !keep {
            task.abort();
            latest.lock().expect("lock poisoned").remove(id);
        }
        keep
    });
    for id in running {
        watchers.entry(id.clone()).or_insert_with(|| {
            let latest = latest.clone();
            tokio::spawn(stream(docker.clone(), id.clone(), move |s| {
                latest.lock().expect("lock poisoned").insert(
                    id.clone(),
                    ContainerUsage {
                        id: id.clone(),
                        cpu_percent: s.cpu_percent,
                        memory_usage: s.memory_usage,
                        memory_limit: s.memory_limit,
                    },
                );
                true
            }))
        });
    }
}

#[cfg(test)]
mod tests {
    use bollard::models::{
        ContainerBlkioStatEntry, ContainerBlkioStats, ContainerCpuStats, ContainerCpuUsage,
        ContainerMemoryStats, ContainerNetworkStats, ContainerPidsStats,
    };

    use super::*;
    use crate::{containers, testing};

    fn cpu(total: u64, system: u64) -> ContainerCpuStats {
        ContainerCpuStats {
            cpu_usage: Some(ContainerCpuUsage {
                total_usage: Some(total),
                ..Default::default()
            }),
            system_cpu_usage: Some(system),
            online_cpus: Some(4),
            ..Default::default()
        }
    }

    #[test]
    fn computes_the_same_numbers_as_docker_stats() {
        let report = ContainerStatsResponse {
            precpu_stats: Some(cpu(1_000, 100_000)),
            // 500 of 10_000 system ticks on 4 CPUs = 20 % of one CPU.
            cpu_stats: Some(cpu(1_500, 110_000)),
            memory_stats: Some(ContainerMemoryStats {
                usage: Some(300),
                limit: Some(1_000),
                stats: Some([("inactive_file".to_string(), 100)].into()),
                ..Default::default()
            }),
            networks: Some(
                [
                    (
                        "eth0".to_string(),
                        ContainerNetworkStats {
                            rx_bytes: Some(10),
                            tx_bytes: Some(1),
                            ..Default::default()
                        },
                    ),
                    (
                        "eth1".to_string(),
                        ContainerNetworkStats {
                            rx_bytes: Some(5),
                            tx_bytes: Some(2),
                            ..Default::default()
                        },
                    ),
                ]
                .into(),
            ),
            blkio_stats: Some(ContainerBlkioStats {
                io_service_bytes_recursive: Some(vec![
                    ContainerBlkioStatEntry {
                        op: Some("read".into()),
                        value: Some(7),
                        ..Default::default()
                    },
                    ContainerBlkioStatEntry {
                        op: Some("Write".into()),
                        value: Some(9),
                        ..Default::default()
                    },
                ]),
                ..Default::default()
            }),
            pids_stats: Some(ContainerPidsStats {
                current: Some(3),
                ..Default::default()
            }),
            ..Default::default()
        };
        let s = sample(&report);
        assert!((s.cpu_percent - 20.0).abs() < 1e-9, "{}", s.cpu_percent);
        assert_eq!((s.memory_usage, s.memory_limit), (200, 1_000));
        assert_eq!((s.net_rx, s.net_tx), (15, 3));
        assert_eq!((s.block_read, s.block_write), (7, 9));
        assert_eq!(s.pids, 3);
    }

    #[test]
    fn first_sample_without_history_reports_zero_cpu() {
        let s = sample(&ContainerStatsResponse {
            cpu_stats: Some(cpu(1_500, 110_000)),
            ..Default::default()
        });
        assert_eq!(s.cpu_percent, 0.0);
    }

    #[tokio::test]
    #[ignore = "needs a local Docker Engine"]
    async fn running_containers_report_usage() {
        let docker = testing::docker();
        let c = testing::container(&docker, &testing::unique("stats"), &["sleep", "60"]).await;
        containers::act(&docker, &c.id, containers::ContainerAction::Start)
            .await
            .unwrap();
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        let task = tokio::spawn(stream(docker.clone(), c.id.clone(), move |s| {
            tx.send(s).is_ok()
        }));
        let first = rx.recv().await.unwrap();
        assert!(first.memory_usage > 0);
        assert!(first.pids >= 1);
        task.abort();
    }
}
