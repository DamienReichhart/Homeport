//! The Run dialog: create and start a container from an image.
//!
//! [`RunSpec`] is what the dialog shows; [`create_body`] turns it into the
//! Engine's request.

mod body;
mod checks;

use bollard::{query_parameters::CreateContainerOptionsBuilder, Docker};
use serde::{Deserialize, Serialize};

use homeport_core::error::AppResult;

use crate::{auth::RegistryAuth, containers, images};

pub use body::create_body;
pub use checks::CAPABILITIES;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct RunSpec {
    pub image: String,
    pub name: Option<String>,
    /// Overrides the image command; parsed with shell quoting rules.
    pub command: Option<String>,
    /// `KEY=value` entries.
    pub env: Vec<String>,
    pub ports: Vec<PortSpec>,
    pub mounts: Vec<MountSpec>,
    pub network: Option<String>,
    pub restart: RestartPolicy,
    /// Remove the container when it exits (`--rm`).
    pub auto_remove: bool,
    /// CPUs the container may use, such as `0.5` (`--cpus`).
    pub cpus: Option<f64>,
    /// Relative CPU weight against other containers; 1024 is the default (`--cpu-shares`).
    pub cpu_shares: Option<i64>,
    /// Memory limit in bytes (`--memory`).
    pub memory: Option<i64>,
    /// Soft memory limit in bytes (`--memory-reservation`).
    pub memory_reservation: Option<i64>,
    /// Memory plus swap in bytes, `-1` for unlimited swap (`--memory-swap`).
    pub memory_swap: Option<i64>,
    /// `key=value` entries.
    pub labels: Vec<String>,
    /// Capabilities, with or without their `CAP_` prefix, or `ALL`.
    pub cap_add: Vec<String>,
    pub cap_drop: Vec<String>,
    /// Gives the container full access to the Host (`--privileged`).
    pub privileged: bool,
    pub devices: Vec<DeviceSpec>,
    pub gpus: Option<GpuSpec>,
    pub healthcheck: Option<HealthcheckSpec>,
    pub hostname: Option<String>,
    pub domainname: Option<String>,
    /// DNS server addresses.
    pub dns: Vec<String>,
    pub dns_search: Vec<String>,
    /// `name:address` entries for `/etc/hosts`; `host-gateway` is the Host's address.
    pub extra_hosts: Vec<String>,
    pub log: Option<LogSpec>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct PortSpec {
    pub host_ip: Option<String>,
    /// `None` lets the Engine pick a free port.
    pub host_port: Option<u16>,
    pub container_port: u16,
    /// `tcp` or `udp`.
    pub protocol: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct MountSpec {
    /// An absolute path on the Host (bind mount) or a volume name.
    pub source: String,
    pub target: String,
    pub read_only: bool,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "kebab-case")]
pub enum RestartPolicy {
    #[default]
    No,
    Always,
    UnlessStopped,
    OnFailure,
}

/// A device of the Host made available inside the container (`--device`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct DeviceSpec {
    /// An absolute path under the Host's `/dev`.
    pub host_path: String,
    /// Where it appears in the container; the same path when empty.
    pub container_path: Option<String>,
    /// Some of `r`, `w` and `m`.
    pub permissions: String,
}

/// GPUs requested from the vendor's container toolkit (`--gpus`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum GpuSpec {
    All,
    Count { count: u32 },
    Ids { ids: Vec<String> },
}

/// A healthcheck run in the container's shell (`--health-cmd` and friends).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct HealthcheckSpec {
    /// Run with `/bin/sh -c`; ignored when `disabled`.
    pub command: String,
    pub interval_s: Option<u32>,
    pub timeout_s: Option<u32>,
    pub retries: Option<u32>,
    pub start_period_s: Option<u32>,
    /// Turns off the image's own healthcheck (`--no-healthcheck`).
    pub disabled: bool,
}

/// Where the container's output goes (`--log-driver`, `--log-opt`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct LogSpec {
    pub driver: String,
    /// `key=value` driver options, such as `max-size=10m`.
    pub options: Vec<String>,
}

/// Creates and starts the container, pulling its image first (with `auth`
/// when given) if the Engine doesn't have it. A container that can't start is
/// removed again, so the dialog can be retried with the same name.
pub async fn run(docker: &Docker, spec: RunSpec, auth: Option<RegistryAuth>) -> AppResult<String> {
    let body = create_body(&spec)?;
    let image = spec.image.trim();
    if docker.inspect_image(image).await.is_err() {
        pull(docker, image, auth).await?;
    }
    let mut options = CreateContainerOptionsBuilder::default();
    if let Some(name) = spec
        .name
        .as_deref()
        .map(str::trim)
        .filter(|n| !n.is_empty())
    {
        options = options.name(name);
    }
    let id = docker
        .create_container(Some(options.build()), body)
        .await?
        .id;
    if let Err(e) = containers::act(docker, &id, containers::ContainerAction::Start).await {
        let _ = containers::remove(docker, &id, true, true).await;
        return Err(e);
    }
    Ok(id)
}

async fn pull(docker: &Docker, image: &str, auth: Option<RegistryAuth>) -> AppResult<()> {
    let failure = std::sync::Mutex::new(None);
    images::pull(docker.clone(), image.to_owned(), auth, |e| {
        if let images::PullEvent::Error { error } = e {
            *failure.lock().expect("lock poisoned") = Some(error);
        }
        true
    })
    .await;
    match failure.into_inner().expect("lock poisoned") {
        Some(error) => Err(error),
        None => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing;

    #[tokio::test]
    #[ignore = "needs a local Docker Engine"]
    async fn runs_a_container_from_an_image() {
        let docker = testing::docker();
        testing::ensure_image(&docker).await;
        let name = testing::unique("run");
        let id = run(
            &docker,
            RunSpec {
                image: testing::IMAGE.into(),
                name: Some(name.clone()),
                command: Some("sleep 30".into()),
                env: vec!["A=1".into()],
                cpus: Some(0.5),
                memory: Some(512 * 1024 * 1024),
                labels: vec!["org.example.team=web".into(), "tier=front".into()],
                ..Default::default()
            },
            None,
        )
        .await
        .unwrap();
        let listed = containers::list(&docker).await.unwrap();
        let c = listed.iter().find(|c| c.id == id).unwrap();
        assert_eq!(
            (c.name.as_str(), c.state.as_str()),
            (name.as_str(), "running")
        );
        let inspected = containers::inspect(&docker, &id).await.unwrap();
        assert_eq!(inspected["HostConfig"]["NanoCpus"], 500_000_000);
        assert_eq!(inspected["HostConfig"]["Memory"], 512 * 1024 * 1024);
        assert_eq!(inspected["Config"]["Labels"]["tier"], "front");
        containers::remove(&docker, &id, true, true).await.unwrap();
    }

    #[tokio::test]
    #[ignore = "needs a local Docker Engine"]
    async fn runs_with_dropped_capabilities_and_a_device() {
        let docker = testing::docker();
        testing::ensure_image(&docker).await;
        let id = run(
            &docker,
            RunSpec {
                image: testing::IMAGE.into(),
                name: Some(testing::unique("run-security")),
                command: Some("sleep 30".into()),
                cap_drop: vec!["ALL".into()],
                devices: vec![DeviceSpec {
                    host_path: "/dev/null".into(),
                    container_path: Some("/dev/homeport-null".into()),
                    permissions: "rw".into(),
                }],
                ..Default::default()
            },
            None,
        )
        .await
        .unwrap();
        let inspected = containers::inspect(&docker, &id).await.unwrap();
        assert_eq!(inspected["HostConfig"]["CapDrop"][0], "ALL");
        assert_eq!(
            inspected["HostConfig"]["Devices"][0]["PathInContainer"],
            "/dev/homeport-null"
        );
        containers::remove(&docker, &id, true, true).await.unwrap();
    }

    #[tokio::test]
    #[ignore = "needs a local Docker Engine"]
    async fn runs_with_a_healthcheck_and_an_extra_host() {
        let docker = testing::docker();
        testing::ensure_image(&docker).await;
        let id = run(
            &docker,
            RunSpec {
                image: testing::IMAGE.into(),
                name: Some(testing::unique("run-health")),
                command: Some("sleep 60".into()),
                healthcheck: Some(HealthcheckSpec {
                    command: "true".into(),
                    interval_s: Some(1),
                    ..Default::default()
                }),
                hostname: Some("homeport-test".into()),
                extra_hosts: vec!["nas.home:192.168.1.10".into()],
                ..Default::default()
            },
            None,
        )
        .await
        .unwrap();
        let mut status = String::new();
        for _ in 0..30 {
            let inspected = containers::inspect(&docker, &id).await.unwrap();
            status = inspected["State"]["Health"]["Status"]
                .as_str()
                .unwrap_or_default()
                .to_owned();
            if status == "healthy" {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
        assert_eq!(status, "healthy");
        let dir = tempfile::tempdir().unwrap();
        let copy = dir.path().join("hosts");
        crate::containers::files::download(&docker, &id, "/etc/hosts", &copy)
            .await
            .unwrap();
        let hosts = std::fs::read_to_string(&copy).unwrap();
        assert!(hosts.contains("192.168.1.10\tnas.home"), "{hosts}");
        let inspected = containers::inspect(&docker, &id).await.unwrap();
        assert_eq!(inspected["Config"]["Hostname"], "homeport-test");
        containers::remove(&docker, &id, true, true).await.unwrap();
    }
}
