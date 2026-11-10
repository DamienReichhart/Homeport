//! Podman pods: containers that share a network namespace and one
//! lifecycle, around an infra container that holds the namespaces and the
//! published ports. Through `/libpod/pods/…`, which Podman 4.0 and later serve.

use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

use homeport_core::error::{AppError, AppResult, ErrorCode};
use homeport_transport::http::{self, Request};

use super::Podman;
use crate::{containers::run::PortSpec, messages as msg};

/// A pod as the Pods page lists it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "PodmanPod")]
pub struct Pod {
    pub id: String,
    pub name: String,
    /// `created`, `running`, `paused`, `stopped`, `exited`, `degraded`
    /// (some containers running), `dead` or `error`.
    pub status: String,
    /// When it was created, RFC 3339.
    pub created: String,
    /// The container that holds its namespaces and published ports, if it has one.
    pub infra_id: Option<String>,
    pub labels: BTreeMap<String, String>,
    pub networks: Vec<String>,
    /// Its containers, the infra container first.
    pub containers: Vec<PodContainer>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "PodmanPodContainer")]
pub struct PodContainer {
    pub id: String,
    pub name: String,
    /// The container's state, as the Containers page shows it (`running`, `exited`…).
    pub state: String,
    /// Whether it's the pod's infra container.
    pub infra: bool,
}

/// A pod with what only its inspect document says.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "PodmanPodDetail")]
pub struct PodDetail {
    #[serde(flatten)]
    pub pod: Pod,
    /// The ports published on the pod, which every container in it shares.
    pub ports: Vec<PodPort>,
    /// The namespaces its containers share, such as `net`, `ipc` and `uts`.
    pub shared_namespaces: Vec<String>,
    pub hostname: String,
    /// The `podman pod create` command it was made with, when Podman kept it.
    pub create_command: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "PodmanPodPort")]
pub struct PodPort {
    pub container_port: u16,
    pub protocol: String,
    pub host_ip: Option<String>,
    pub host_port: Option<u16>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "PodmanPodAction")]
pub enum PodAction {
    Start,
    Stop,
    Restart,
    Pause,
    Unpause,
}

impl PodAction {
    fn verb(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Stop => "stop",
            Self::Restart => "restart",
            Self::Pause => "pause",
            Self::Unpause => "unpause",
        }
    }

    /// What to say when some of the pod's containers refuse this action.
    fn failed(self) -> homeport_core::error::Message {
        match self {
            Self::Start => msg::POD_START_FAILED,
            Self::Stop => msg::POD_STOP_FAILED,
            Self::Restart => msg::POD_RESTART_FAILED,
            Self::Pause => msg::POD_PAUSE_FAILED,
            Self::Unpause => msg::POD_UNPAUSE_FAILED,
        }
    }
}

/// A new pod, as the **New pod** dialog fills it.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "PodmanPodSpec")]
pub struct PodSpec {
    pub name: String,
    /// Published on the pod's infra container, for every container in it.
    pub ports: Vec<PortSpec>,
    /// `key=value` entries.
    pub labels: Vec<String>,
}

// What libpod answers, as Podman's Go types name the fields.

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Listed {
    id: String,
    name: String,
    #[serde(default)]
    status: String,
    #[serde(default)]
    created: String,
    #[serde(default)]
    infra_id: String,
    #[serde(default)]
    labels: Option<BTreeMap<String, String>>,
    #[serde(default)]
    networks: Option<Vec<String>>,
    #[serde(default)]
    containers: Option<Vec<ListedContainer>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ListedContainer {
    id: String,
    #[serde(default)]
    names: String,
    #[serde(default)]
    status: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Inspected {
    id: String,
    name: String,
    #[serde(default)]
    created: String,
    #[serde(default)]
    state: String,
    #[serde(default)]
    hostname: String,
    #[serde(default)]
    labels: Option<BTreeMap<String, String>>,
    #[serde(default)]
    create_command: Option<Vec<String>>,
    #[serde(default, rename = "InfraContainerID")]
    infra_container_id: String,
    #[serde(default)]
    infra_config: Option<InfraConfig>,
    #[serde(default)]
    shared_namespaces: Option<Vec<String>>,
    #[serde(default)]
    containers: Option<Vec<InspectedContainer>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct InfraConfig {
    #[serde(default)]
    port_bindings: Option<HashMap<String, Option<Vec<HostPort>>>>,
    #[serde(default)]
    networks: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct HostPort {
    #[serde(rename = "HostIp", default)]
    host_ip: String,
    #[serde(rename = "HostPort", default)]
    host_port: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct InspectedContainer {
    id: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    state: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Removed {
    #[serde(default)]
    removed_ctrs: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Deserialize)]
struct Created {
    #[serde(rename = "Id")]
    id: String,
}

/// Puts the infra container first, then the others by name.
fn sort_containers(containers: &mut [PodContainer]) {
    containers.sort_by(|a, b| b.infra.cmp(&a.infra).then_with(|| a.name.cmp(&b.name)));
}

impl From<Listed> for Pod {
    fn from(p: Listed) -> Self {
        let infra = (!p.infra_id.is_empty()).then_some(p.infra_id);
        let mut containers: Vec<PodContainer> = p
            .containers
            .unwrap_or_default()
            .into_iter()
            .map(|c| PodContainer {
                infra: infra.as_deref() == Some(c.id.as_str()),
                id: c.id,
                name: c.names,
                state: c.status.to_lowercase(),
            })
            .collect();
        sort_containers(&mut containers);
        let mut networks = p.networks.unwrap_or_default();
        networks.sort();
        Self {
            id: p.id,
            name: p.name,
            status: p.status.to_lowercase(),
            created: p.created,
            infra_id: infra,
            labels: p.labels.unwrap_or_default(),
            networks,
            containers,
        }
    }
}

impl From<Inspected> for PodDetail {
    fn from(p: Inspected) -> Self {
        let infra = (!p.infra_container_id.is_empty()).then_some(p.infra_container_id);
        let mut containers: Vec<PodContainer> = p
            .containers
            .unwrap_or_default()
            .into_iter()
            .map(|c| PodContainer {
                infra: infra.as_deref() == Some(c.id.as_str()),
                id: c.id,
                name: c.name,
                state: c.state.to_lowercase(),
            })
            .collect();
        sort_containers(&mut containers);
        let config = p.infra_config;
        let mut ports: Vec<PodPort> = config
            .as_ref()
            .and_then(|c| c.port_bindings.as_ref())
            .into_iter()
            .flatten()
            .flat_map(|(port, bindings)| {
                let (number, protocol) = port.split_once('/').unwrap_or((port, "tcp"));
                let number: u16 = number.parse().unwrap_or_default();
                let protocol = protocol.to_owned();
                let bindings = bindings.as_deref().unwrap_or_default();
                let unbound = bindings.is_empty().then(|| PodPort {
                    container_port: number,
                    protocol: protocol.clone(),
                    host_ip: None,
                    host_port: None,
                });
                bindings
                    .iter()
                    .map(move |b| PodPort {
                        container_port: number,
                        protocol: protocol.clone(),
                        host_ip: Some(b.host_ip.clone()).filter(|ip| !ip.is_empty()),
                        host_port: b.host_port.parse().ok(),
                    })
                    .chain(unbound)
                    .collect::<Vec<_>>()
            })
            .collect();
        ports.sort_by_key(|p| (p.container_port, p.host_port, p.host_ip.clone()));
        let mut networks = config.and_then(|c| c.networks).unwrap_or_default();
        networks.sort();
        Self {
            pod: Pod {
                id: p.id,
                name: p.name,
                status: p.state.to_lowercase(),
                created: p.created,
                infra_id: infra,
                labels: p.labels.unwrap_or_default(),
                networks,
                containers,
            },
            ports,
            shared_namespaces: p.shared_namespaces.unwrap_or_default(),
            hostname: p.hostname,
            create_command: p.create_command.unwrap_or_default(),
        }
    }
}

/// Every pod on the Host, by name.
pub async fn list(podman: &Podman) -> AppResult<Vec<Pod>> {
    let listed: Option<Vec<Listed>> = podman.json(Request::get(podman.path("/pods/json"))).await?;
    let mut pods: Vec<Pod> = listed
        .unwrap_or_default()
        .into_iter()
        .map(Pod::from)
        .collect();
    pods.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(pods)
}

/// One pod, with its ports and shared namespaces.
pub async fn inspect(podman: &Podman, id: &str) -> AppResult<PodDetail> {
    let path = podman.path(&format!("/pods/{}/json", http::encode(id)));
    let inspected: Inspected = podman.json(Request::get(path)).await.map_err(gone)?;
    Ok(inspected.into())
}

/// Starts, stops, restarts, pauses or resumes every container of a pod.
pub async fn act(podman: &Podman, id: &str, action: PodAction) -> AppResult<()> {
    let path = podman.path(&format!("/pods/{}/{}", http::encode(id), action.verb()));
    let answer = podman.send(Request::post(path)).await?;
    match answer.status.as_u16() {
        200..=299 => Ok(()),
        304 => Err(AppError::new(ErrorCode::AlreadyDone, msg::POD_ALREADY)),
        404 => Err(gone_error()),
        // Some of the pod's containers failed; Podman names them in the body.
        409 => Err(AppError::new(ErrorCode::Conflict, action.failed())
            .with_hint(msg::POD_ACTION_FAILED_HINT)
            .with_detail(String::from_utf8_lossy(&answer.body))),
        _ => answer.ok().map(drop),
    }
}

/// Removes a pod and its containers; `force` stops running ones first.
/// Returns how many containers went with it.
pub async fn remove(podman: &Podman, id: &str, force: bool) -> AppResult<u32> {
    let path = podman.path(&format!("/pods/{}?force={force}", http::encode(id)));
    let answer = podman.send(Request::delete(path)).await?;
    if !answer.status.is_success() {
        let error = http::status_error(answer.status, &answer.body);
        let text = String::from_utf8_lossy(&answer.body).to_lowercase();
        if !force && (text.contains("without force") || text.contains("is running")) {
            return Err(AppError::new(ErrorCode::PodRunning, msg::POD_RUNNING)
                .with_hint(msg::POD_RUNNING_HINT)
                .with_detail(error.detail.unwrap_or_default()));
        }
        return Err(if answer.status.as_u16() == 404 {
            gone_error()
        } else {
            error
        });
    }
    let removed: Removed = answer.json()?;
    Ok(removed.removed_ctrs.map_or(0, |c| c.len() as u32))
}

/// Creates a pod with its infra container, which holds the published ports;
/// returns its id. The pod starts with its first container.
pub async fn create(podman: &Podman, spec: &PodSpec) -> AppResult<String> {
    let body = create_body(spec)?;
    let answer = podman
        .send(Request::post(podman.path("/pods/create")).json(&body)?)
        .await?;
    if answer.status.as_u16() == 409
        || String::from_utf8_lossy(&answer.body).contains("pod already exists")
    {
        return Err(AppError::new(
            ErrorCode::NameInUse,
            msg::POD_NAME_IN_USE.with("name", spec.name.trim()),
        )
        .with_hint(msg::POD_NAME_IN_USE_HINT));
    }
    let created: Created = answer.ok()?.json()?;
    Ok(created.id)
}

/// The body of `POST /libpod/pods/create` (Podman's `PodSpecGenerator`).
pub fn create_body(spec: &PodSpec) -> AppResult<serde_json::Value> {
    let name = spec.name.trim();
    if !valid_name(name) {
        return Err(AppError::validation(msg::POD_NAME.with("name", name)));
    }
    let mut labels = BTreeMap::new();
    for entry in &spec.labels {
        let (k, v) = crate::containers::run::label(entry)?;
        labels.insert(k.to_owned(), v.to_owned());
    }
    let mut portmappings = Vec::new();
    for p in &spec.ports {
        let protocol = p.protocol.trim().to_lowercase();
        if !matches!(protocol.as_str(), "tcp" | "udp" | "sctp") {
            return Err(AppError::validation(msg::RUN_PORT_PROTOCOL));
        }
        let mut mapping = serde_json::json!({
            "container_port": p.container_port,
            "protocol": protocol,
        });
        if let Some(host_port) = p.host_port {
            mapping["host_port"] = host_port.into();
        }
        if let Some(ip) = p
            .host_ip
            .as_deref()
            .map(str::trim)
            .filter(|ip| !ip.is_empty())
        {
            mapping["host_ip"] = ip.into();
        }
        portmappings.push(mapping);
    }
    Ok(serde_json::json!({
        "name": name,
        "labels": labels,
        "portmappings": portmappings,
    }))
}

/// Podman's rule for pod and container names.
fn valid_name(name: &str) -> bool {
    let mut chars = name.chars();
    chars.next().is_some_and(|c| c.is_ascii_alphanumeric())
        && chars.all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '-'))
}

fn gone_error() -> AppError {
    AppError::not_found(msg::POD_GONE).with_hint(msg::POD_GONE_HINT)
}

/// A pod that's no longer there says so in its own words.
fn gone(e: AppError) -> AppError {
    if e.is(ErrorCode::NotFound) {
        gone_error().with_detail(e.detail.unwrap_or_default())
    } else {
        e
    }
}

#[cfg(all(test, unix))]
mod tests;
