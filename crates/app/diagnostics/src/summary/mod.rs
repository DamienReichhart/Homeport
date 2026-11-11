//! `summary.json`: what Homeport is running as and against, without anything
//! that identifies a machine unless the user asks for names.

use serde::Serialize;
use serde_json::Value;

use homeport_core::error::ErrorCode;

use crate::redact::Aliases;

/// The settings that say which Host or Cluster the user was last on.
const SELECTION_SETTINGS: &[&str] = &["lastHostId", "lastSelection"];
/// The settings keyed by Host or Cluster id.
const KEYED_SETTINGS: &[&str] = &["clusterNamespaces", "containerdNamespaces", "incusProjects"];

/// Homeport itself and the computer it runs on.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppFacts {
    pub version: String,
    /// How this copy was installed: `appimage`, `deb`, `nsis`… or `unknown`.
    pub bundle: String,
    pub os: String,
    pub arch: String,
    /// The database's schema version (its number of migrations).
    pub schema_version: u32,
    /// Whether the OS keychain is reachable, so secrets can be stored.
    pub keychain: bool,
}

/// One Host, as the registry sees it now.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostFacts {
    pub name: String,
    /// Its address, when it has one (`host:port`); a socket path isn't one.
    pub address: Option<String>,
    /// `local`, `tcp`, `tls` or `ssh`.
    pub connection: String,
    /// The number of bastions an SSH Host goes through.
    pub bastions: usize,
    /// `docker`, `podman`, `containerd` or `incus`.
    pub engine: String,
    pub engine_version: Option<String>,
    pub api_version: Option<String>,
    /// `connected`, `unreachable`, `disabled`…
    pub status: String,
    /// Why it isn't connected, when it isn't.
    pub error: Option<ErrorCode>,
}

/// One Cluster, as the registry sees it now.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClusterFacts {
    pub name: String,
    pub address: String,
    /// `kubernetes` or `nomad`.
    pub kind: String,
    pub version: Option<String>,
    pub status: String,
    pub error: Option<ErrorCode>,
}

/// How many Stacks Homeport keeps, by kind.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StackCounts {
    pub managed: usize,
    pub linked: usize,
    pub synced: usize,
}

/// Everything the summary is built from.
#[derive(Debug, Clone)]
pub struct Inputs {
    pub app: AppFacts,
    /// The settings as stored (a JSON object).
    pub settings: Value,
    pub hosts: Vec<HostFacts>,
    pub clusters: Vec<ClusterFacts>,
    pub stacks: StackCounts,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HostSummary {
    /// Its name, or `host-1`, `host-2`… when names are left out.
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    pub connection: String,
    pub bastions: usize,
    pub engine: String,
    pub engine_version: Option<String>,
    pub api_version: Option<String>,
    pub status: String,
    pub error: Option<ErrorCode>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterSummary {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    pub kind: String,
    pub version: Option<String>,
    pub status: String,
    pub error: Option<ErrorCode>,
}

/// The contents of `summary.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
    pub app: AppFacts,
    /// Whether Hosts and Clusters appear under their own names and addresses.
    pub names_included: bool,
    pub settings: Value,
    pub hosts: Vec<HostSummary>,
    pub clusters: Vec<ClusterSummary>,
    pub stacks: StackCounts,
}

/// Builds the summary and the aliases its Hosts and Clusters go by.
///
/// Without `include_names`, each Host is `host-1`, `host-2`… and each Cluster
/// `cluster-1`…, addresses are left out, and the returned [`Aliases`] replace
/// every name and address in the logs with the same labels. With it, the
/// aliases are empty.
pub fn summary(inputs: Inputs, include_names: bool) -> (Summary, Aliases) {
    let mut aliases = Aliases::default();
    let hosts = inputs
        .hosts
        .into_iter()
        .enumerate()
        .map(|(index, host)| {
            let label = format!("host-{}", index + 1);
            if !include_names {
                aliases.add(&host.name, &label);
                if let Some(address) = &host.address {
                    aliases.add_address(address, &label);
                }
            }
            HostSummary {
                name: if include_names { host.name } else { label },
                address: host.address.filter(|_| include_names),
                connection: host.connection,
                bastions: host.bastions,
                engine: host.engine,
                engine_version: host.engine_version,
                api_version: host.api_version,
                status: host.status,
                error: host.error,
            }
        })
        .collect();
    let clusters = inputs
        .clusters
        .into_iter()
        .enumerate()
        .map(|(index, cluster)| {
            let label = format!("cluster-{}", index + 1);
            if !include_names {
                aliases.add(&cluster.name, &label);
                aliases.add_address(&cluster.address, &label);
            }
            ClusterSummary {
                name: if include_names { cluster.name } else { label },
                address: include_names.then_some(cluster.address),
                kind: cluster.kind,
                version: cluster.version,
                status: cluster.status,
                error: cluster.error,
            }
        })
        .collect();
    let summary = Summary {
        app: inputs.app,
        names_included: include_names,
        settings: settings(inputs.settings),
        hosts,
        clusters,
        stacks: inputs.stacks,
    };
    (summary, aliases)
}

/// The settings without what points at a Host or a Cluster: the last
/// selection goes, and the maps keyed by id keep only their size.
fn settings(mut settings: Value) -> Value {
    if let Some(map) = settings.as_object_mut() {
        for key in SELECTION_SETTINGS {
            map.remove(*key);
        }
        for key in KEYED_SETTINGS {
            if let Some(value) = map.get_mut(*key) {
                let entries = value.as_object().map_or(0, serde_json::Map::len);
                *value = Value::from(entries);
            }
        }
    }
    settings
}

#[cfg(test)]
mod tests;
