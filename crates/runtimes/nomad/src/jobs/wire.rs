//! Jobs as Nomad's API answers them, read into what [`super::model`] shows.

use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct GroupSummary {
    pub queued: u32,
    pub complete: u32,
    pub failed: u32,
    pub running: u32,
    pub starting: u32,
    pub lost: u32,
    pub unknown: u32,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub(super) struct JobSummary {
    pub(super) summary: BTreeMap<String, GroupSummary>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub(super) struct JobStub {
    #[serde(rename = "ID")]
    pub(super) id: String,
    #[serde(rename = "ParentID")]
    pub(super) parent_id: String,
    pub(super) name: String,
    pub(super) namespace: String,
    #[serde(rename = "Type")]
    pub(super) job_type: String,
    pub(super) priority: i32,
    pub(super) periodic: bool,
    pub(super) parameterized_job: bool,
    pub(super) stop: bool,
    pub(super) status: String,
    pub(super) status_description: String,
    pub(super) job_summary: Option<JobSummary>,
    pub(super) submit_time: i64,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub(super) struct Scaling {
    pub(super) min: Option<u64>,
    pub(super) max: Option<u64>,
    pub(super) enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub(super) struct Task {
    pub(super) name: String,
    pub(super) driver: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub(super) struct TaskGroup {
    pub(super) name: String,
    pub(super) count: Option<u32>,
    pub(super) scaling: Option<Scaling>,
    pub(super) tasks: Vec<Task>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct Job {
    #[serde(rename = "ID")]
    pub(super) id: String,
    #[serde(rename = "ParentID")]
    pub(super) parent_id: Option<String>,
    pub(super) name: String,
    pub(super) namespace: String,
    #[serde(rename = "Type")]
    pub(super) job_type: String,
    pub(super) priority: i32,
    pub(super) status: String,
    pub(super) status_description: String,
    pub(super) stop: bool,
    pub(super) version: u64,
    pub(super) submit_time: i64,
    pub(super) datacenters: Vec<String>,
    pub(super) node_pool: Option<String>,
    pub(super) periodic: Option<serde_json::Value>,
    pub(super) parameterized_job: Option<serde_json::Value>,
    pub(super) task_groups: Vec<TaskGroup>,
    pub(crate) job_modify_index: u64,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub(super) struct TaskState {
    pub(super) state: String,
    pub(super) failed: bool,
    pub(super) restarts: u32,
    pub(super) started_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct AllocationStub {
    #[serde(rename = "ID")]
    pub(super) id: String,
    pub(super) name: String,
    pub(super) namespace: String,
    #[serde(rename = "JobID")]
    pub(super) job_id: String,
    #[serde(rename = "NodeID")]
    pub(super) node_id: String,
    pub(super) node_name: String,
    pub(super) task_group: String,
    pub(super) client_status: String,
    pub(super) desired_status: String,
    pub(super) job_version: u64,
    pub(super) create_time: i64,
    pub(super) modify_time: i64,
    pub(super) task_states: Option<BTreeMap<String, TaskState>>,
}
