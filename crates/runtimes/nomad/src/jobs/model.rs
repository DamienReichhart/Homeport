//! Jobs as the app shows them: the list's rows, a job's detail with its groups,
//! allocations and deployments, and what scaling or stopping one answers.

use serde::Serialize;

/// A task group's counts of allocations by state, and how many it wants.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct NomadGroupCounts {
    pub name: String,
    /// The count in the job's definition; `None` when it couldn't be read.
    pub desired: Option<u32>,
    pub running: u32,
    pub starting: u32,
    pub queued: u32,
    pub failed: u32,
    pub lost: u32,
    pub complete: u32,
}

/// A job as the Jobs page lists it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct NomadJobView {
    pub id: String,
    pub name: String,
    pub namespace: String,
    /// `service`, `batch`, `system` or `sysbatch`.
    pub job_type: String,
    /// `pending`, `running` or `dead`.
    pub status: String,
    pub status_description: Option<String>,
    /// Stopped by a user, and not yet garbage-collected.
    pub stopped: bool,
    pub priority: i32,
    pub periodic: bool,
    pub parameterized: bool,
    /// The periodic or parameterized job that launched this one.
    pub parent_id: Option<String>,
    pub submitted: Option<String>,
    pub groups: Vec<NomadGroupCounts>,
}

/// A task group's scaling bounds, from its `scaling` block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct NomadScaling {
    pub min: Option<u32>,
    pub max: Option<u32>,
    /// Whether an autoscaler acts on the policy.
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct NomadTaskView {
    pub name: String,
    pub driver: String,
}

/// A task group of a job's detail.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct NomadTaskGroupView {
    pub counts: NomadGroupCounts,
    pub scaling: Option<NomadScaling>,
    pub tasks: Vec<NomadTaskView>,
}

/// A task's state in an allocation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct NomadTaskState {
    pub name: String,
    /// `pending`, `running` or `dead`.
    pub state: String,
    pub failed: bool,
    pub restarts: u32,
    pub started: Option<String>,
}

/// An allocation: one placement of a task group on a client node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct NomadAllocationView {
    pub id: String,
    /// Such as `web.api[0]`.
    pub name: String,
    pub namespace: String,
    pub job_id: String,
    pub task_group: String,
    pub node_id: String,
    pub node_name: String,
    /// `pending`, `running`, `complete`, `failed`, `lost` or `unknown`.
    pub client_status: String,
    /// `run`, `stop` or `evict`.
    pub desired_status: String,
    pub job_version: u32,
    pub created: Option<String>,
    pub modified: Option<String>,
    pub tasks: Vec<NomadTaskState>,
}

/// A job's detail: its definition's main settings, its task groups and its allocations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct NomadJobDetail {
    pub job: NomadJobView,
    pub version: u32,
    pub datacenters: Vec<String>,
    pub node_pool: Option<String>,
    pub groups: Vec<NomadTaskGroupView>,
    pub allocations: Vec<NomadAllocationView>,
}
