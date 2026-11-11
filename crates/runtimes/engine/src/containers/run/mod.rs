//! The Run dialog: create and start a container from an image, or from
//! another container's configuration (Duplicate and Edit).
//!
//! [`RunSpec`] is what the dialog shows; [`create_body`] turns it into the
//! Engine's request and [`spec_from_inspect`] reads one back from a container.

mod body;
mod checks;
mod from_inspect;

use bollard::{
    models::{ContainerInspectResponse, EndpointSettings, ImageInspect, NetworkConnectRequest},
    query_parameters::CreateContainerOptionsBuilder,
    Docker,
};
use serde::{Deserialize, Serialize};

use homeport_core::error::AppResult;

use crate::{auth::RegistryAuth, containers, images};

pub use body::{create_body, Base};
pub(crate) use checks::label;
pub use checks::CAPABILITIES;
pub(crate) use from_inspect::anonymous_volumes;
pub use from_inspect::spec_from_inspect;

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
    /// Start from this container's full configuration and apply the spec on
    /// top, so settings the dialog doesn't show (ulimits, sysctls, security
    /// options, tmpfs, other networks…) are kept.
    pub based_on: Option<String>,
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
    let original = match &spec.based_on {
        Some(id) => Some(Original::inspect(docker, id).await?),
        None => None,
    };
    let name = spec
        .name
        .as_deref()
        .map(str::trim)
        .filter(|n| !n.is_empty());
    let base = original.as_ref().map(|o| o.base(false));
    let id = create(docker, &spec, base.as_ref(), name, auth).await?;
    if let Err(e) = containers::act(docker, &id, containers::ContainerAction::Start).await {
        let _ = containers::remove(docker, &id, true, true).await;
        return Err(e);
    }
    Ok(id)
}

/// A container's inspect document and its image's, read once.
pub(crate) struct Original {
    pub container: ContainerInspectResponse,
    pub image: Option<ImageInspect>,
}

impl Original {
    pub(crate) async fn inspect(docker: &Docker, id: &str) -> AppResult<Self> {
        let container = docker.inspect_container(id, None).await?;
        let image = match container.image.as_deref() {
            Some(image) => docker.inspect_image(image).await.ok(),
            None => None,
        };
        Ok(Self { container, image })
    }

    pub(crate) fn base(&self, same_identity: bool) -> Base<'_> {
        Base {
            inspect: &self.container,
            image: self.image.as_ref(),
            same_identity,
        }
    }
}

/// Creates the container without starting it, pulling the image if needed,
/// and connects it to the base container's other networks.
pub(crate) async fn create(
    docker: &Docker,
    spec: &RunSpec,
    base: Option<&Base<'_>>,
    name: Option<&str>,
    auth: Option<RegistryAuth>,
) -> AppResult<String> {
    let body = create_body(spec, base)?;
    let image = spec.image.trim();
    if docker.inspect_image(image).await.is_err() {
        pull(docker, image, auth).await?;
    }
    let mut options = CreateContainerOptionsBuilder::default();
    if let Some(name) = name {
        options = options.name(name);
    }
    let id = docker
        .create_container(Some(options.build()), body)
        .await?
        .id;
    for (network, aliases) in base.map(|b| b.other_networks(spec)).unwrap_or_default() {
        let connected = docker
            .connect_network(
                &network,
                NetworkConnectRequest {
                    container: id.clone(),
                    endpoint_config: Some(EndpointSettings {
                        aliases: (!aliases.is_empty()).then_some(aliases),
                        ..Default::default()
                    }),
                },
            )
            .await;
        if let Err(e) = connected {
            let _ = containers::remove(docker, &id, true, true).await;
            return Err(e.into());
        }
    }
    Ok(id)
}

/// The Run dialog's form for an existing container.
pub async fn spec_for(docker: &Docker, id: &str) -> AppResult<RunSpec> {
    let original = Original::inspect(docker, id).await?;
    Ok(spec_from_inspect(
        &original.container,
        original.image.as_ref(),
    ))
}

pub(crate) async fn pull(
    docker: &Docker,
    image: &str,
    auth: Option<RegistryAuth>,
) -> AppResult<()> {
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
mod tests;
