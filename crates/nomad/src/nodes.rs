//! A Nomad Cluster's client nodes: listing them with how many allocations
//! each runs, a node's detail, draining it (and cancelling a drain), and
//! marking it eligible or ineligible for new allocations.

use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use homeport_core::error::{AppError, AppResult, ErrorCode};

use crate::{
    allocations::{full_view, Allocation},
    client::{segment, Nomad},
    jobs::NomadAllocationView,
    messages as msg,
};

/// The rule a token needs to drain nodes, for the hint.
const NODE_WRITE_RULE: &str = "node {\n  policy = \"write\"\n}";

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
struct NodeStub {
    #[serde(rename = "ID")]
    id: String,
    name: String,
    datacenter: String,
    node_pool: String,
    node_class: String,
    address: String,
    version: String,
    drain: bool,
    scheduling_eligibility: String,
    status: String,
    status_description: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
struct DrainStrategy {
    /// Nanoseconds; negative to force the drain at once, zero for none.
    deadline: i64,
    force_deadline: Option<String>,
    ignore_system_jobs: bool,
    started_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
struct LastDrain {
    status: String,
    updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
struct Driver {
    detected: bool,
    healthy: bool,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
struct Node {
    #[serde(rename = "ID")]
    id: String,
    name: String,
    datacenter: String,
    node_pool: String,
    node_class: String,
    #[serde(rename = "HTTPAddr")]
    http_addr: String,
    status: String,
    status_description: String,
    scheduling_eligibility: String,
    drain_strategy: Option<DrainStrategy>,
    last_drain: Option<LastDrain>,
    attributes: BTreeMap<String, String>,
    drivers: BTreeMap<String, Driver>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
struct AllocationPlacement {
    #[serde(rename = "NodeID")]
    node_id: String,
    client_status: String,
}

/// A client node as the Nodes page lists it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct NomadNodeView {
    pub id: String,
    pub name: String,
    pub datacenter: String,
    pub node_pool: Option<String>,
    pub node_class: Option<String>,
    pub address: Option<String>,
    /// The Nomad version its agent runs.
    pub version: Option<String>,
    /// `initializing`, `ready`, `down` or `disconnected`.
    pub status: String,
    pub status_description: Option<String>,
    /// Takes new allocations.
    pub eligible: bool,
    pub draining: bool,
    /// Allocations running on it now; `None` when the token can't list them.
    pub running: Option<u32>,
}

/// A drain in progress.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct NomadDrain {
    /// When remaining allocations are stopped regardless; `None` for no deadline.
    pub deadline: Option<String>,
    pub forced: bool,
    pub ignore_system_jobs: bool,
    pub started: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct NomadDriverView {
    pub name: String,
    pub detected: bool,
    pub healthy: bool,
}

/// A node's detail: what the list shows, its drain, drivers, a few
/// attributes, and its allocations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct NomadNodeDetail {
    pub node: NomadNodeView,
    pub drain: Option<NomadDrain>,
    /// How the last drain ended, such as `complete` or `canceled`.
    pub last_drain: Option<String>,
    pub drivers: Vec<NomadDriverView>,
    /// The operating system, kernel, CPU architecture and the like.
    pub attributes: Vec<(String, String)>,
    pub allocations: Vec<NomadAllocationView>,
}

/// How to drain a node, as the Drain dialog asks.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct NomadDrainSpec {
    /// Seconds before remaining allocations are stopped; `None` for no deadline.
    pub deadline_secs: Option<u32>,
    /// Leave system jobs' allocations running.
    pub ignore_system_jobs: bool,
}

/// The attributes a node's page shows, in this order.
const SHOWN_ATTRIBUTES: &[&str] = &[
    "unique.hostname",
    "os.name",
    "os.version",
    "kernel.name",
    "kernel.version",
    "cpu.arch",
    "cpu.numcores",
    "memory.totalbytes",
    "nomad.version",
];

fn some(text: String) -> Option<String> {
    (!text.is_empty()).then_some(text)
}

/// The body that starts draining with `spec`, or cancels with `None`.
pub fn drain_body(spec: Option<&NomadDrainSpec>, mark_eligible: bool) -> Value {
    match spec {
        Some(spec) => json!({
            "DrainSpec": {
                "Deadline": spec
                    .deadline_secs
                    .map_or(0, |secs| i64::from(secs) * 1_000_000_000),
                "IgnoreSystemJobs": spec.ignore_system_jobs,
            },
            "Meta": { "message": "Drained from Homeport" },
        }),
        None => json!({ "DrainSpec": null, "MarkEligible": mark_eligible }),
    }
}

fn node_view(stub: NodeStub, running: Option<u32>) -> NomadNodeView {
    NomadNodeView {
        id: stub.id,
        name: stub.name,
        datacenter: stub.datacenter,
        node_pool: some(stub.node_pool),
        node_class: some(stub.node_class),
        address: some(stub.address),
        version: some(stub.version),
        status: stub.status,
        status_description: some(stub.status_description),
        eligible: stub.scheduling_eligibility == "eligible",
        draining: stub.drain,
        running,
    }
}

impl Nomad {
    fn node_error(&self, error: AppError, node: Option<&str>, write: bool) -> AppError {
        if error.is(ErrorCode::NomadPermissionDenied) {
            let (message, hint) = if write {
                (msg::NODE_WRITE_DENIED, msg::NODE_WRITE_DENIED_HINT)
            } else {
                (msg::NODE_READ_DENIED, msg::NODE_READ_DENIED_HINT)
            };
            let denied = AppError {
                detail: error.detail,
                ..AppError::new(
                    ErrorCode::NomadPermissionDenied,
                    message.with("name", self.name()),
                )
                .with_hint(hint)
            };
            return if write {
                denied.with_command(NODE_WRITE_RULE)
            } else {
                denied
            };
        }
        match node {
            Some(node) if error.is(ErrorCode::NotFound) => AppError {
                detail: error.detail,
                ..AppError::new(
                    ErrorCode::NotFound,
                    msg::NODE_NOT_FOUND
                        .with("name", self.name())
                        .with("node", node.get(..8).unwrap_or(node)),
                )
                .with_hint(msg::NODE_NOT_FOUND_HINT)
            },
            _ => error,
        }
    }

    /// The Cluster's client nodes, by name, with how many allocations
    /// each runs when the token can list them.
    pub async fn nodes(&self) -> AppResult<Vec<NomadNodeView>> {
        let stubs: Vec<NodeStub> = self
            .get("/v1/nodes", &[])
            .await
            .map_err(|e| self.node_error(e, None, false))?;
        let running = match self
            .get::<Vec<AllocationPlacement>>(
                "/v1/allocations",
                &[("namespace", "*".into()), ("task_states", "false".into())],
            )
            .await
        {
            Ok(allocations) => {
                let mut counts: HashMap<String, u32> = HashMap::new();
                for a in allocations
                    .into_iter()
                    .filter(|a| a.client_status == "running")
                {
                    *counts.entry(a.node_id).or_default() += 1;
                }
                Some(counts)
            }
            // A token that reads nodes but not every namespace's jobs.
            Err(_) => None,
        };
        let mut nodes: Vec<_> = stubs
            .into_iter()
            .map(|stub| {
                let count = running
                    .as_ref()
                    .map(|c| c.get(&stub.id).copied().unwrap_or(0));
                node_view(stub, count)
            })
            .collect();
        nodes.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(nodes)
    }

    /// The node `id`, with its drain, drivers and allocations, newest first.
    pub async fn node(&self, id: &str) -> AppResult<NomadNodeDetail> {
        let path = format!("/v1/node/{}", segment(id));
        let node: Node = self
            .get(&path, &[])
            .await
            .map_err(|e| self.node_error(e, Some(id), false))?;
        let allocations: Vec<Allocation> = self
            .get(&format!("{path}/allocations"), &[])
            .await
            .map_err(|e| self.node_error(e, Some(id), false))?;
        let mut allocations: Vec<_> = allocations.into_iter().map(full_view).collect();
        allocations.sort_by(|a, b| b.created.cmp(&a.created));
        let running = allocations
            .iter()
            .filter(|a| a.client_status == "running")
            .count();
        let version = node.attributes.get("nomad.version").cloned();
        let drain = node.drain_strategy.map(|d| NomadDrain {
            deadline: (d.deadline > 0)
                .then_some(d.force_deadline)
                .flatten()
                .filter(|t| !t.starts_with("0001-")),
            forced: d.deadline < 0,
            ignore_system_jobs: d.ignore_system_jobs,
            started: d.started_at.filter(|t| !t.starts_with("0001-")),
        });
        Ok(NomadNodeDetail {
            node: NomadNodeView {
                id: node.id,
                name: node.name,
                datacenter: node.datacenter,
                node_pool: some(node.node_pool),
                node_class: some(node.node_class),
                address: some(node.http_addr),
                version,
                status: node.status,
                status_description: some(node.status_description),
                eligible: node.scheduling_eligibility == "eligible",
                draining: drain.is_some(),
                running: Some(u32::try_from(running).unwrap_or(u32::MAX)),
            },
            drain,
            last_drain: node.last_drain.map(|l| l.status).filter(|s| !s.is_empty()),
            drivers: node
                .drivers
                .into_iter()
                .map(|(name, d)| NomadDriverView {
                    name,
                    detected: d.detected,
                    healthy: d.healthy,
                })
                .collect(),
            attributes: SHOWN_ATTRIBUTES
                .iter()
                .filter_map(|key| Some(((*key).to_owned(), node.attributes.get(*key)?.clone())))
                .collect(),
            allocations,
        })
    }

    /// Starts draining node `id` with `spec`, which also marks it
    /// ineligible, as Nomad does; with `None`, cancels its drain and, with
    /// `mark_eligible`, makes it eligible again.
    pub async fn drain_node(
        &self,
        id: &str,
        spec: Option<&NomadDrainSpec>,
        mark_eligible: bool,
    ) -> AppResult<()> {
        self.post::<Value>(
            &format!("/v1/node/{}/drain", segment(id)),
            &[],
            &drain_body(spec, mark_eligible),
        )
        .await
        .map(drop)
        .map_err(|e| self.node_error(e, Some(id), true))
    }

    /// Marks node `id` eligible or ineligible for new allocations; those
    /// already on it stay.
    pub async fn set_eligibility(&self, id: &str, eligible: bool) -> AppResult<()> {
        let eligibility = if eligible { "eligible" } else { "ineligible" };
        self.post::<Value>(
            &format!("/v1/node/{}/eligibility", segment(id)),
            &[],
            &json!({ "NodeID": id, "Eligibility": eligibility }),
        )
        .await
        .map(drop)
        .map_err(|e| self.node_error(e, Some(id), true))
    }
}

#[cfg(test)]
mod tests;
