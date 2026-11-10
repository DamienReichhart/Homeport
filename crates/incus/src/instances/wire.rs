//! Instances as Incus's API answers them, read into what [`super::model`] shows.

use std::collections::BTreeMap;

use serde::Deserialize;

use super::model::{IncusInstance, InstanceStatus, InstanceType};

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct RawInstance {
    pub(crate) name: String,
    #[serde(rename = "type", default)]
    pub(super) kind: String,
    #[serde(default)]
    pub(super) status_code: u16,
    #[serde(default)]
    pub(super) description: String,
    #[serde(default)]
    pub(super) architecture: String,
    #[serde(default)]
    pub(super) profiles: Vec<String>,
    #[serde(default)]
    pub(super) config: BTreeMap<String, String>,
    #[serde(default)]
    pub(super) expanded_config: BTreeMap<String, String>,
    #[serde(default)]
    pub(super) devices: BTreeMap<String, BTreeMap<String, String>>,
    #[serde(default)]
    pub(super) expanded_devices: BTreeMap<String, BTreeMap<String, String>>,
    #[serde(default)]
    pub(super) created_at: String,
    #[serde(default)]
    pub(super) last_used_at: String,
    #[serde(default)]
    pub(super) ephemeral: bool,
    #[serde(default)]
    pub(super) location: String,
    #[serde(default)]
    pub(super) snapshots: Option<Vec<RawSnapshotName>>,
    #[serde(default)]
    pub(super) state: Option<RawState>,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct RawSnapshotName {
    pub(super) name: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub(super) struct RawState {
    #[serde(default)]
    pub(super) cpu: Option<RawCpu>,
    #[serde(default)]
    pub(super) memory: Option<RawMemory>,
    #[serde(default)]
    pub(super) network: Option<BTreeMap<String, RawNic>>,
    #[serde(default)]
    pub(super) processes: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct RawCpu {
    #[serde(default)]
    pub(super) usage: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct RawMemory {
    #[serde(default)]
    pub(super) usage: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct RawNic {
    #[serde(default)]
    pub(super) addresses: Vec<RawAddress>,
    #[serde(rename = "type", default)]
    pub(super) kind: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct RawAddress {
    #[serde(default)]
    pub(super) family: String,
    #[serde(default)]
    pub(super) address: String,
    #[serde(default)]
    pub(super) scope: String,
}

/// Incus writes the zero time for "never".
pub(super) fn time(text: &str) -> Option<String> {
    (!text.is_empty() && !text.starts_with("0001-")).then(|| text.to_owned())
}

impl RawInstance {
    pub(super) fn summary(&self) -> IncusInstance {
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
