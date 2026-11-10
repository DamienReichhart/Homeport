//! Incus's Instances, system containers and virtual machines: listing
//! them with their state, and starting, stopping, restarting, freezing,
//! unfreezing and deleting them.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use homeport_core::{
    error::{AppError, AppResult, ErrorCode},
    pull::PullEvent,
};
use homeport_transport::http::Request;

use crate::{
    api::{
        client::{scoped, segment, Incus},
        operations,
    },
    messages as msg,
};

/// What an Instance is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "kebab-case")]
pub enum InstanceType {
    Container,
    VirtualMachine,
}

/// What an Instance is doing, from Incus's status code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum InstanceStatus {
    Running,
    Stopped,
    Frozen,
    Starting,
    Stopping,
    Freezing,
    Error,
    Unknown,
}

impl InstanceStatus {
    fn from_code(code: u16) -> Self {
        match code {
            101 | 103 | 111 => Self::Running,
            102 => Self::Stopped,
            106 => Self::Starting,
            107 => Self::Stopping,
            109 => Self::Freezing,
            110 => Self::Frozen,
            112 => Self::Error,
            _ => Self::Unknown,
        }
    }
}

/// One Instance, as the list shows it.
#[derive(Debug, Clone, PartialEq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct IncusInstance {
    pub name: String,
    #[serde(rename = "type")]
    pub kind: InstanceType,
    pub status: InstanceStatus,
    pub description: String,
    /// The image it was made from, as its description says: "Debian 12
    /// amd64 (20261108_05:24)"; empty when unknown.
    pub image: String,
    pub architecture: String,
    pub profiles: Vec<String>,
    /// Its global addresses, IPv4 first, without loopback.
    pub addresses: Vec<String>,
    /// CPU time used since it started, in nanoseconds.
    pub cpu_usage_ns: Option<i64>,
    pub memory_bytes: Option<i64>,
    pub processes: Option<i64>,
    /// RFC 3339.
    pub created_at: String,
    pub last_used_at: Option<String>,
    /// How many snapshots it has: deleting it deletes them too.
    pub snapshots: u32,
    pub ephemeral: bool,
    /// Its cluster member, `none` outside a cluster.
    pub location: String,
}

/// One Instance with what its page shows.
#[derive(Debug, Clone, PartialEq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct IncusInstanceDetail {
    pub instance: IncusInstance,
    /// Its own configuration, then with its profiles' applied.
    pub config: BTreeMap<String, String>,
    pub expanded_config: BTreeMap<String, String>,
    /// Its own devices, then with its profiles' applied.
    pub devices: BTreeMap<String, BTreeMap<String, String>>,
    pub expanded_devices: BTreeMap<String, BTreeMap<String, String>>,
    /// The names of its snapshots, oldest first.
    pub snapshot_names: Vec<String>,
    /// Whether it can take stateful snapshots (`migration.stateful`).
    pub stateful_snapshots: bool,
    /// Incus's own description of it, for the Raw tab.
    pub raw: serde_json::Value,
}

/// A change of state an Instance can be asked for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum InstanceAction {
    Start,
    Stop,
    Restart,
    Freeze,
    Unfreeze,
}

impl InstanceAction {
    fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Stop => "stop",
            Self::Restart => "restart",
            Self::Freeze => "freeze",
            Self::Unfreeze => "unfreeze",
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct RawInstance {
    pub(crate) name: String,
    #[serde(rename = "type", default)]
    kind: String,
    #[serde(default)]
    status_code: u16,
    #[serde(default)]
    description: String,
    #[serde(default)]
    architecture: String,
    #[serde(default)]
    profiles: Vec<String>,
    #[serde(default)]
    config: BTreeMap<String, String>,
    #[serde(default)]
    expanded_config: BTreeMap<String, String>,
    #[serde(default)]
    devices: BTreeMap<String, BTreeMap<String, String>>,
    #[serde(default)]
    expanded_devices: BTreeMap<String, BTreeMap<String, String>>,
    #[serde(default)]
    created_at: String,
    #[serde(default)]
    last_used_at: String,
    #[serde(default)]
    ephemeral: bool,
    #[serde(default)]
    location: String,
    #[serde(default)]
    snapshots: Option<Vec<RawSnapshotName>>,
    #[serde(default)]
    state: Option<RawState>,
}

#[derive(Debug, Clone, Deserialize)]
struct RawSnapshotName {
    name: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
struct RawState {
    #[serde(default)]
    cpu: Option<RawCpu>,
    #[serde(default)]
    memory: Option<RawMemory>,
    #[serde(default)]
    network: Option<BTreeMap<String, RawNic>>,
    #[serde(default)]
    processes: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
struct RawCpu {
    #[serde(default)]
    usage: i64,
}

#[derive(Debug, Clone, Deserialize)]
struct RawMemory {
    #[serde(default)]
    usage: i64,
}

#[derive(Debug, Clone, Deserialize)]
struct RawNic {
    #[serde(default)]
    addresses: Vec<RawAddress>,
    #[serde(rename = "type", default)]
    kind: String,
}

#[derive(Debug, Clone, Deserialize)]
struct RawAddress {
    #[serde(default)]
    family: String,
    #[serde(default)]
    address: String,
    #[serde(default)]
    scope: String,
}

/// Incus writes the zero time for "never".
fn time(text: &str) -> Option<String> {
    (!text.is_empty() && !text.starts_with("0001-")).then(|| text.to_owned())
}

impl RawInstance {
    fn summary(&self) -> IncusInstance {
        let state = self.state.clone().unwrap_or_default();
        let running = state.processes.is_some_and(|p| p > 0)
            || matches!(
                InstanceStatus::from_code(self.status_code),
                InstanceStatus::Running | InstanceStatus::Frozen
            );
        let mut v4 = Vec::new();
        let mut v6 = Vec::new();
        for nic in state.network.unwrap_or_default().values() {
            if nic.kind == "loopback" {
                continue;
            }
            for a in &nic.addresses {
                if a.scope != "global" {
                    continue;
                }
                match a.family.as_str() {
                    "inet" => v4.push(a.address.clone()),
                    "inet6" => v6.push(a.address.clone()),
                    _ => {}
                }
            }
        }
        v4.extend(v6);
        let config = if self.expanded_config.is_empty() {
            &self.config
        } else {
            &self.expanded_config
        };
        let image = config.get("image.description").cloned().unwrap_or_else(|| {
            [config.get("image.os"), config.get("image.release")]
                .into_iter()
                .flatten()
                .cloned()
                .collect::<Vec<_>>()
                .join(" ")
        });
        IncusInstance {
            name: self.name.clone(),
            kind: if self.kind == "virtual-machine" {
                InstanceType::VirtualMachine
            } else {
                InstanceType::Container
            },
            status: InstanceStatus::from_code(self.status_code),
            description: self.description.clone(),
            image,
            architecture: self.architecture.clone(),
            profiles: self.profiles.clone(),
            addresses: v4,
            cpu_usage_ns: state.cpu.filter(|_| running).map(|c| c.usage),
            memory_bytes: state.memory.filter(|_| running).map(|m| m.usage),
            processes: state.processes.filter(|p| *p >= 0),
            created_at: self.created_at.clone(),
            last_used_at: time(&self.last_used_at),
            snapshots: u32::try_from(self.snapshots.as_ref().map_or(0, Vec::len))
                .unwrap_or(u32::MAX),
            ephemeral: self.ephemeral,
            location: self.location.clone(),
        }
    }
}

/// The API path of the Instance `name`.
pub(crate) fn path(name: &str) -> String {
    format!("/1.0/instances/{}", segment(name))
}

/// Every Instance of `project`, with its state.
pub async fn list(incus: &Incus, project: &str) -> AppResult<Vec<IncusInstance>> {
    let raw: Vec<RawInstance> = incus
        .sync(Request::get(scoped("/1.0/instances?recursion=2", project)))
        .await?;
    let mut instances: Vec<IncusInstance> = raw.iter().map(RawInstance::summary).collect();
    instances.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(instances)
}

/// The Instance `name` of `project`, with its configuration and devices.
pub async fn get(incus: &Incus, project: &str, name: &str) -> AppResult<IncusInstanceDetail> {
    let value: serde_json::Value = incus
        .sync(Request::get(scoped(
            &format!("{}?recursion=1", path(name)),
            project,
        )))
        .await
        .map_err(|e| not_found(e, name))?;
    let mut raw: RawInstance = crate::api::client::metadata(value.clone())?;
    let state: RawState = incus
        .sync(Request::get(scoped(
            &format!("{}/state", path(name)),
            project,
        )))
        .await
        .unwrap_or_default();
    raw.state = Some(state);
    let snapshot_names = raw
        .snapshots
        .as_ref()
        .map(|s| s.iter().map(|s| s.name.clone()).collect())
        .unwrap_or_default();
    Ok(IncusInstanceDetail {
        instance: raw.summary(),
        stateful_snapshots: raw
            .expanded_config
            .get("migration.stateful")
            .is_some_and(|v| v == "true"),
        config: raw.config,
        expanded_config: raw.expanded_config,
        devices: raw.devices,
        expanded_devices: raw.expanded_devices,
        snapshot_names,
        raw: value,
    })
}

/// A missing Instance, named.
fn not_found(error: AppError, name: &str) -> AppError {
    if error.is(ErrorCode::NotFound) {
        AppError::new(
            ErrorCode::NotFound,
            msg::INSTANCE_NOT_FOUND.with("name", name),
        )
        .with_hint(msg::INSTANCE_NOT_FOUND_HINT)
        .with_detail(error.detail.unwrap_or(error.message))
    } else {
        error
    }
}

/// The body of a state change.
pub(crate) fn state_body(
    action: InstanceAction,
    force: bool,
    timeout: Option<u32>,
) -> serde_json::Value {
    let mut body = serde_json::json!({ "action": action.as_str(), "force": force });
    if let Some(timeout) = timeout {
        body["timeout"] = serde_json::json!(timeout);
    }
    body
}

/// How long a graceful stop or restart may take before Incus gives up.
pub const STOP_TIMEOUT_SECS: u32 = 30;

/// Asks the Instance `name` to change state and waits for it. A graceful
/// stop or restart that doesn't finish in time says to force it.
pub async fn act(
    incus: &Incus,
    project: &str,
    name: &str,
    action: InstanceAction,
    force: bool,
) -> AppResult<()> {
    let graceful = matches!(action, InstanceAction::Stop | InstanceAction::Restart) && !force;
    let body = state_body(action, force, graceful.then_some(STOP_TIMEOUT_SECS));
    let request = Request::put(scoped(&format!("{}/state", path(name)), project)).json(&body)?;
    match incus.run(project, request, operations::SHORT).await {
        Ok(_) => Ok(()),
        Err(e) if graceful && looks_stuck(&e) => Err(AppError::new(
            ErrorCode::Timeout,
            msg::STOP_TIMED_OUT
                .with("name", name)
                .with("seconds", STOP_TIMEOUT_SECS),
        )
        .with_hint(msg::STOP_TIMED_OUT_HINT)
        .with_detail(e.detail.unwrap_or(e.message))),
        Err(e) => Err(not_found(e, name)),
    }
}

/// Whether a graceful stop failed because the guest didn't shut down.
fn looks_stuck(error: &AppError) -> bool {
    error.is(ErrorCode::Timeout)
        || error
            .detail
            .as_deref()
            .is_some_and(|d| d.to_lowercase().contains("timed out") || d.contains("deadline"))
}

/// Deletes the Instance `name` with its snapshots; `stop_first` stops a
/// running one by force first, since Incus refuses to delete it otherwise.
pub async fn delete(incus: &Incus, project: &str, name: &str, stop_first: bool) -> AppResult<()> {
    if stop_first {
        match act(incus, project, name, InstanceAction::Stop, true).await {
            Ok(()) => {}
            // Already stopped.
            Err(e) if e.is(ErrorCode::ContainerNotRunning) || e.is(ErrorCode::AlreadyDone) => {}
            Err(e) => return Err(e),
        }
    }
    incus
        .run(
            project,
            Request::delete(scoped(&path(name), project)),
            operations::SHORT,
        )
        .await
        .map(drop)
        .map_err(|e| {
            if e.is(ErrorCode::Conflict) || e.is(ErrorCode::BadRequest) {
                let running = e
                    .detail
                    .as_deref()
                    .is_some_and(|d| d.to_lowercase().contains("running"));
                if running {
                    return AppError::new(
                        ErrorCode::ContainerRunning,
                        msg::INSTANCE_RUNNING.with("name", name),
                    )
                    .with_hint(msg::INSTANCE_RUNNING_HINT)
                    .with_detail(e.detail.unwrap_or(e.message));
                }
            }
            not_found(e, name)
        })
}

/// Where a new Instance's image comes from.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, specta::Type)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum InstanceSource {
    /// An image the Host keeps, by fingerprint.
    Local { fingerprint: String },
    /// An image of a simplestreams server, by alias: "debian/12".
    Remote { server: String, alias: String },
}

/// What the New Instance dialog asks for.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct InstanceSpec {
    pub name: String,
    #[serde(rename = "type")]
    pub kind: InstanceType,
    pub source: InstanceSource,
    /// In order: later ones override earlier ones.
    pub profiles: Vec<String>,
    /// `limits.cpu`: how many CPUs.
    pub cpu: Option<u32>,
    /// `limits.memory`: "2GiB".
    pub memory: Option<String>,
    /// The root disk's size: "20GiB".
    pub disk: Option<String>,
    /// Start it once it's made.
    pub start: bool,
}

/// Incus's rule for Instance names: a hostname label.
pub fn validate_name(name: &str) -> AppResult<()> {
    let valid = !name.is_empty()
        && name.len() <= 63
        && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
        && !name.starts_with('-')
        && !name.ends_with('-')
        && !name.starts_with(|c: char| c.is_ascii_digit());
    if valid {
        Ok(())
    } else {
        Err(AppError::validation(
            msg::INSTANCE_NAME_INVALID.with("name", name),
        ))
    }
}

/// The body of `POST /1.0/instances` for `spec`; `root_pool` is the pool
/// of the root disk its profiles give it, needed to resize that disk.
pub(crate) fn create_body(
    spec: &InstanceSpec,
    root_pool: Option<&str>,
    start: bool,
) -> serde_json::Value {
    let source = match &spec.source {
        InstanceSource::Local { fingerprint } => serde_json::json!({
            "type": "image",
            "fingerprint": fingerprint,
        }),
        InstanceSource::Remote { server, alias } => serde_json::json!({
            "type": "image",
            "mode": "pull",
            "server": server,
            "protocol": "simplestreams",
            "alias": alias,
        }),
    };
    let mut config = serde_json::Map::new();
    if let Some(cpu) = spec.cpu {
        config.insert("limits.cpu".into(), cpu.to_string().into());
    }
    if let Some(memory) = spec
        .memory
        .as_deref()
        .map(str::trim)
        .filter(|m| !m.is_empty())
    {
        config.insert("limits.memory".into(), memory.into());
    }
    let mut devices = serde_json::Map::new();
    if let (Some(size), Some(pool)) = (
        spec.disk
            .as_deref()
            .map(str::trim)
            .filter(|d| !d.is_empty()),
        root_pool,
    ) {
        devices.insert(
            "root".into(),
            serde_json::json!({ "type": "disk", "path": "/", "pool": pool, "size": size }),
        );
    }
    let mut body = serde_json::json!({
        "name": spec.name,
        "type": crate::images::instance_type(spec.kind),
        "profiles": spec.profiles,
        "config": config,
        "devices": devices,
        "source": source,
    });
    if start {
        body["start"] = true.into();
    }
    body
}

/// The pool of the root disk `profiles` give an Instance, the last one's winning.
async fn root_pool(incus: &Incus, project: &str, profiles: &[String]) -> AppResult<Option<String>> {
    let all = crate::profiles::list(incus, project).await?;
    Ok(profiles
        .iter()
        .filter_map(|name| all.iter().find(|p| &p.name == name))
        .filter_map(|p| {
            p.devices
                .values()
                .find(|d| {
                    d.get("type").map(String::as_str) == Some("disk")
                        && d.get("path").map(String::as_str) == Some("/")
                })
                .and_then(|d| d.get("pool").cloned())
        })
        .next_back())
}

/// Makes an Instance from `spec`, sending the image download's progress to
/// `sink` until it's made (and started, if asked) or failed.
pub async fn create(
    incus: &Incus,
    project: &str,
    spec: &InstanceSpec,
    mut sink: impl FnMut(PullEvent) -> bool,
) {
    let result = async {
        validate_name(&spec.name)?;
        let pool = if spec.disk.as_deref().is_some_and(|d| !d.trim().is_empty()) {
            let pool = root_pool(incus, project, &spec.profiles).await?;
            if pool.is_none() {
                return Err(AppError::validation(msg::ROOT_DISK_UNKNOWN)
                    .with_hint(msg::ROOT_DISK_UNKNOWN_HINT));
            }
            pool
        } else {
            None
        };
        // Incus 6 starts it in the same operation; older ones need a start after.
        let start_with = spec.start && incus.server().has("instance_create_start");
        let body = create_body(spec, pool.as_deref(), start_with);
        let request = Request::post(scoped("/1.0/instances", project)).json(&body)?;
        incus
            .run_with(project, request, operations::LONG, |op| {
                if let Some(event) = crate::images::progress_event(op) {
                    sink(event);
                }
            })
            .await
            .map_err(|e| {
                if e.is(ErrorCode::NameInUse) {
                    AppError::new(
                        ErrorCode::NameInUse,
                        msg::INSTANCE_EXISTS.with("name", &spec.name),
                    )
                    .with_hint(msg::ALREADY_EXISTS_HINT)
                    .with_detail(e.detail.unwrap_or(e.message))
                } else {
                    e
                }
            })?;
        if spec.start && !start_with {
            act(incus, project, &spec.name, InstanceAction::Start, false).await?;
        }
        Ok::<_, AppError>(())
    }
    .await;
    match result {
        Ok(()) => {
            sink(PullEvent::Done);
        }
        Err(error) => {
            sink(PullEvent::Error { error });
        }
    }
}

pub mod snapshots;
pub mod terminal;
#[cfg(test)]
pub(crate) mod tests;
pub mod usage;
