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
