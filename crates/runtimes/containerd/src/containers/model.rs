//! Containers as the app shows them, the state of their task, and the actions
//! on one.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::grpc::api::containerd::v1::types::Status;

/// What a container's task is doing; `none` when it has no task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum TaskState {
    None,
    Created,
    Running,
    Paused,
    Pausing,
    Stopped,
    Unknown,
}

impl From<Status> for TaskState {
    fn from(status: Status) -> Self {
        match status {
            Status::Created => Self::Created,
            Status::Running => Self::Running,
            Status::Paused => Self::Paused,
            Status::Pausing => Self::Pausing,
            Status::Stopped => Self::Stopped,
            Status::Unknown => Self::Unknown,
        }
    }
}

/// Which tool made a container, which decides what Homeport can show of it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum CreatedBy {
    /// Kubernetes' CRI: the kubelet undoes stops and removals.
    Kubernetes,
    Nerdctl,
    /// Docker's own containers, in the `moby` namespace.
    Docker,
    /// `ctr` or another client.
    Other,
}

/// A container and its task, as the list shows them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ContainerdContainer {
    pub id: String,
    /// nerdctl's name, or the Pod's container's, or the id.
    pub name: String,
    pub image: String,
    pub labels: HashMap<String, String>,
    pub runtime: String,
    pub snapshotter: String,
    pub snapshot_key: String,
    /// RFC 3339.
    pub created_at: Option<String>,
    pub created_by: CreatedBy,
    /// The Kubernetes Pod it belongs to, as `namespace/name`.
    pub pod: Option<String>,
    /// A Pod's pause container, which holds its namespaces.
    pub sandbox: bool,
    pub state: TaskState,
    /// The task's main process.
    pub pid: Option<u32>,
    /// How a stopped task ended.
    pub exit_status: Option<u32>,
    pub exited_at: Option<String>,
}

/// A container with everything containerd holds about it.
#[derive(Debug, Clone, PartialEq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ContainerdContainerDetail {
    pub container: ContainerdContainer,
    /// Its OCI runtime spec, as containerd stores it.
    pub spec: Option<serde_json::Value>,
    /// The names of its extensions, such as CRI's metadata.
    pub extensions: Vec<String>,
    pub updated_at: Option<String>,
}

/// What can be done to a container's task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum TaskAction {
    Start,
    Stop,
    Pause,
    Resume,
}
