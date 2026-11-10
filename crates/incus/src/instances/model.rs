//! Instances as the app shows them, the actions on one, and what creating one
//! takes.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

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
    pub(super) fn from_code(code: u16) -> Self {
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
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Stop => "stop",
            Self::Restart => "restart",
            Self::Freeze => "freeze",
            Self::Unfreeze => "unfreeze",
        }
    }
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
