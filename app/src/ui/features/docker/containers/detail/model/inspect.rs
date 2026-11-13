//! What the Inspect tab reads of the Engine's inspect document
//! (`ContainerInspect` in `model/types.ts`, and what `InspectTab` worked
//! out of it), read once, off the window's thread, with the document
//! itself for the raw view.

use std::collections::BTreeMap;

use serde::Deserialize;

use crate::ui::widgets::data::json::Document;

/// The parts of the document the tab shows. Whatever the Engine leaves
/// out is its empty value: Podman and older Engines say less.
#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
struct Read {
    id: String,
    created: String,
    restart_count: i64,
    state: ReadState,
    config: ReadConfig,
    host_config: ReadHostConfig,
    mounts: Option<Vec<ReadMount>>,
    network_settings: ReadNetworkSettings,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
struct ReadState {
    status: String,
    running: bool,
    started_at: String,
    exit_code: i64,
    #[serde(rename = "OOMKilled")]
    oom_killed: bool,
    health: Option<ReadHealth>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
struct ReadHealth {
    status: String,
    failing_streak: i64,
    log: Option<Vec<ReadCheck>>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
struct ReadCheck {
    start: String,
    exit_code: i64,
    output: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
struct ReadConfig {
    user: String,
    env: Option<Vec<String>>,
    cmd: Option<Vec<String>>,
    entrypoint: Option<Vec<String>>,
    image: String,
    working_dir: String,
    labels: Option<BTreeMap<String, String>>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
struct ReadHostConfig {
    restart_policy: Option<ReadRestart>,
    network_mode: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
struct ReadRestart {
    name: String,
    maximum_retry_count: i64,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
struct ReadMount {
    r#type: String,
    name: Option<String>,
    source: String,
    destination: String,
    #[serde(rename = "RW")]
    rw: bool,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
struct ReadNetworkSettings {
    ports: Option<BTreeMap<String, Option<Vec<ReadBinding>>>>,
    networks: Option<BTreeMap<String, ReadNetwork>>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
struct ReadBinding {
    host_ip: String,
    host_port: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct ReadNetwork {
    #[serde(rename = "IPAddress")]
    ip_address: String,
    #[serde(rename = "Gateway")]
    gateway: String,
    #[serde(rename = "Aliases")]
    aliases: Option<Vec<String>>,
}

/// A name and its value: a variable of the environment, a label. `at` is
/// its place in its list, which tells two rows apart.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pair {
    pub at: usize,
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mount {
    pub at: usize,
    /// `bind`, `volume`, `tmpfs`…
    pub kind: String,
    /// The volume's name, or the path on the Host.
    pub source: String,
    pub destination: String,
    pub writable: bool,
}

/// A port of the container and one address it is published on, if any.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Port {
    pub at: usize,
    /// `80/tcp`.
    pub port: String,
    /// `0.0.0.0:8080`; `*:8080` when the Engine names no address.
    pub published: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Network {
    pub at: usize,
    pub name: String,
    pub ip: Option<String>,
    pub gateway: Option<String>,
    pub aliases: Vec<String>,
}

/// One run of the healthcheck, as the document keeps it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Check {
    pub at: usize,
    /// When it started, in Unix seconds, if the Engine's time reads.
    pub started: Option<i64>,
    pub exit_code: i64,
    pub output: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HealthSummary {
    pub status: String,
    pub failing_streak: i64,
    /// The last five runs, the newest first.
    pub checks: Vec<Check>,
}

/// The document as the tab's sections show it.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Details {
    pub id: String,
    pub image: String,
    /// Unix seconds; `None` when the Engine's time doesn't read.
    pub created: Option<i64>,
    pub status: String,
    pub running: bool,
    pub exit_code: i64,
    pub out_of_memory: bool,
    /// `None` for a container that never started (`0001-01-01…`).
    pub started: Option<i64>,
    pub entrypoint: Option<String>,
    pub command: Option<String>,
    pub working_dir: Option<String>,
    pub user: Option<String>,
    /// The policy's name and its maximum of retries; `None` without one.
    pub restart: Option<(String, i64)>,
    pub restart_count: i64,
    pub network_mode: String,
    pub health: Option<HealthSummary>,
    pub env: Vec<Pair>,
    pub mounts: Vec<Mount>,
    pub ports: Vec<Port>,
    pub networks: Vec<Network>,
    /// By name.
    pub labels: Vec<Pair>,
}

/// How many runs of the healthcheck the tab shows (`slice(-5)`).
const CHECKS: usize = 5;

/// An RFC 3339 time in Unix seconds.
pub fn unix(time: &str) -> Option<i64> {
    time.parse::<jiff::Timestamp>()
        .ok()
        .map(jiff::Timestamp::as_second)
}

fn some(text: String) -> Option<String> {
    (!text.is_empty()).then_some(text)
}

impl Details {
    /// Reads the sections of an inspect document.
    pub fn of(document: &serde_json::Value) -> Self {
        // A part of another shape than the Engine's is left out with the
        // rest of the document: the raw view still shows all of it.
        let read = Read::deserialize(document).unwrap_or_default();
        let env = read.config.env.unwrap_or_default();
        let env = env.into_iter().enumerate().map(|(at, variable)| {
            let (name, value) = variable.split_once('=').unwrap_or((&variable, ""));
            Pair {
                at,
                name: name.to_owned(),
                value: value.to_owned(),
            }
        });
        let mut ports = vec![];
        for (port, bindings) in read.network_settings.ports.unwrap_or_default() {
            let bindings = bindings.unwrap_or_default();
            let mut published: Vec<Option<String>> = bindings
                .into_iter()
                .map(|binding| {
                    let ip = some(binding.host_ip).unwrap_or_else(|| "*".to_owned());
                    Some(format!("{ip}:{}", binding.host_port))
                })
                .collect();
            if published.is_empty() {
                published.push(None);
            }
            for published in published {
                ports.push(Port {
                    at: ports.len(),
                    port: port.clone(),
                    published,
                });
            }
        }
        let networks = read.network_settings.networks.unwrap_or_default();
        let networks = networks
            .into_iter()
            .enumerate()
            .map(|(at, (name, n))| Network {
                at,
                name,
                ip: some(n.ip_address),
                gateway: some(n.gateway),
                aliases: n.aliases.unwrap_or_default(),
            });
        let mounts = read.mounts.unwrap_or_default();
        let mounts = mounts.into_iter().enumerate().map(|(at, mount)| Mount {
            at,
            kind: mount.r#type,
            source: mount.name.unwrap_or(mount.source),
            destination: mount.destination,
            writable: mount.rw,
        });
        let labels = read.config.labels.unwrap_or_default();
        let labels = labels
            .into_iter()
            .enumerate()
            .map(|(at, (name, value))| Pair { at, name, value });
        let health = read.state.health.map(|health| {
            let log = health.log.unwrap_or_default();
            let last = log.into_iter().rev().take(CHECKS);
            HealthSummary {
                status: health.status,
                failing_streak: health.failing_streak,
                checks: last
                    .enumerate()
                    .map(|(at, check)| Check {
                        at,
                        started: unix(&check.start),
                        exit_code: check.exit_code,
                        output: check.output.trim().to_owned(),
                    })
                    .collect(),
            }
        });
        Self {
            id: read.id,
            image: read.config.image,
            created: unix(&read.created),
            status: read.state.status,
            running: read.state.running,
            exit_code: read.state.exit_code,
            out_of_memory: read.state.oom_killed,
            started: Some(read.state.started_at)
                .filter(|at| !at.starts_with("0001"))
                .and_then(|at| unix(&at)),
            entrypoint: read.config.entrypoint.map(|parts| parts.join(" ")),
            command: read.config.cmd.map(|parts| parts.join(" ")),
            working_dir: some(read.config.working_dir),
            user: some(read.config.user),
            restart: read
                .host_config
                .restart_policy
                .filter(|policy| !policy.name.is_empty())
                .map(|policy| (policy.name, policy.maximum_retry_count)),
            restart_count: read.restart_count,
            network_mode: read.host_config.network_mode,
            health,
            env: env.collect(),
            mounts: mounts.collect(),
            ports,
            networks: networks.collect(),
            labels: labels.collect(),
        }
    }
}

/// An inspect document, read: its sections and the document for the raw
/// view. Both are built by the read itself, off the window's thread.
#[derive(Debug)]
pub struct Inspected {
    pub details: Details,
    pub document: Document,
}

impl Inspected {
    pub fn of(value: &serde_json::Value) -> Self {
        Self {
            details: Details::of(value),
            // What `serde_json` writes of a value is JSON.
            document: Document::of(value).unwrap_or_default(),
        }
    }
}
