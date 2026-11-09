//! One allocation of a Nomad job: its tasks, their states and recent
//! events, and the errors its logs and terminals share.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use homeport_core::error::{AppError, AppResult, ErrorCode, Message, Text};

use crate::{
    client::{segment, Nomad},
    error::mentions,
    jobs::{NomadAllocationView, NomadTaskState},
    messages as msg,
    time::iso,
};

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
struct TaskEvent {
    #[serde(rename = "Type")]
    kind: String,
    display_message: String,
    time: i64,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
struct TaskState {
    state: String,
    failed: bool,
    restarts: u32,
    started_at: Option<String>,
    events: Option<Vec<TaskEvent>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
struct JobVersion {
    version: u64,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
struct Allocation {
    #[serde(rename = "ID")]
    id: String,
    name: String,
    namespace: String,
    #[serde(rename = "JobID")]
    job_id: String,
    #[serde(rename = "NodeID")]
    node_id: String,
    node_name: String,
    task_group: String,
    client_status: String,
    client_description: String,
    desired_status: String,
    job: Option<JobVersion>,
    create_time: i64,
    modify_time: i64,
    task_states: Option<BTreeMap<String, TaskState>>,
}

/// Something that happened to a task, as Nomad records it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct NomadTaskEvent {
    pub task: String,
    /// Such as `Started`, `Terminated` or `Driver Failure`.
    pub kind: String,
    pub message: String,
    pub time: Option<String>,
}

/// An allocation's detail: what the job pages list, its tasks' events,
/// newest first, and why the client gave it its status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct NomadAllocationDetail {
    pub allocation: NomadAllocationView,
    pub client_description: Option<String>,
    pub events: Vec<NomadTaskEvent>,
}

fn detail(a: Allocation) -> NomadAllocationDetail {
    let states = a.task_states.unwrap_or_default();
    let mut events: Vec<NomadTaskEvent> = states
        .iter()
        .flat_map(|(task, s)| {
            s.events.iter().flatten().map(move |e| NomadTaskEvent {
                task: task.clone(),
                kind: e.kind.clone(),
                message: e.display_message.clone(),
                time: iso(e.time),
            })
        })
        .collect();
    events.sort_by(|a, b| b.time.cmp(&a.time));
    NomadAllocationDetail {
        allocation: NomadAllocationView {
            tasks: states
                .into_iter()
                .map(|(name, s)| NomadTaskState {
                    name,
                    state: s.state,
                    failed: s.failed,
                    restarts: s.restarts,
                    started: s.started_at.filter(|t| !t.starts_with("0001-")),
                })
                .collect(),
            id: a.id,
            name: a.name,
            namespace: a.namespace,
            job_id: a.job_id,
            task_group: a.task_group,
            node_id: a.node_id,
            node_name: a.node_name,
            client_status: a.client_status,
            desired_status: a.desired_status,
            job_version: a
                .job
                .map_or(0, |j| u32::try_from(j.version).unwrap_or(u32::MAX)),
            created: iso(a.create_time),
            modified: iso(a.modify_time),
        },
        client_description: Some(a.client_description).filter(|d| !d.is_empty()),
        events,
    }
}

impl Nomad {
    /// The allocation `id`, with its tasks' events.
    pub async fn allocation(&self, id: &str) -> AppResult<NomadAllocationDetail> {
        let allocation: Allocation = self
            .get(&format!("/v1/allocation/{}", segment(id)), &[])
            .await
            .map_err(|e| {
                self.allocation_error(e, id, None, msg::LOGS_DENIED, msg::LOGS_DENIED_HINT)
            })?;
        Ok(detail(allocation))
    }

    /// An error reading logs of, or opening a terminal in, task `task` of
    /// allocation `alloc`, in words for what the user tried.
    pub(crate) fn allocation_error(
        &self,
        error: AppError,
        alloc: &str,
        task: Option<&str>,
        denied: Message,
        denied_hint: Message,
    ) -> AppError {
        let text = error
            .detail
            .clone()
            .unwrap_or_else(|| error.message.clone());
        let short = alloc.get(..8).unwrap_or(alloc);
        let named = |code: ErrorCode, message: Text, hint: Message| AppError {
            detail: error.detail.clone(),
            ..AppError::new(code, message).with_hint(hint)
        };
        if error.is(ErrorCode::NomadPermissionDenied) {
            return named(
                ErrorCode::NomadPermissionDenied,
                denied.with("name", self.name()),
                denied_hint,
            );
        }
        if mentions(
            &text,
            &[
                "no path to node",
                "unknown node",
                "node is down",
                "node down",
                "failed to find client connection",
            ],
        ) {
            return named(
                ErrorCode::NomadNodeUnreachable,
                msg::NODE_UNREACHABLE
                    .with("name", self.name())
                    .with("alloc", short),
                msg::NODE_UNREACHABLE_HINT,
            )
            .with_command("nomad node status");
        }
        if mentions(&text, &["unknown allocation", "allocation not found"])
            || (error.is(ErrorCode::NotFound) && task.is_none())
        {
            return named(
                ErrorCode::NotFound,
                msg::ALLOC_NOT_FOUND
                    .with("name", self.name())
                    .with("alloc", short),
                msg::ALLOC_NOT_FOUND_HINT,
            );
        }
        if let Some(task) = task {
            if mentions(&text, &["not running", "task not found", "is not running"]) {
                return named(
                    ErrorCode::NomadAllocationNotRunning,
                    msg::ALLOC_NOT_RUNNING
                        .with("task", task)
                        .with("alloc", short),
                    msg::ALLOC_NOT_RUNNING_HINT,
                );
            }
        }
        error
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::testing::{agent, ok};

    const ALLOCATION: &str = r#"{
      "ID": "a1b2c3d4-0000-0000-0000-000000000001", "Name": "web.api[0]", "Namespace": "default",
      "JobID": "web", "NodeID": "n1", "NodeName": "client-1", "TaskGroup": "api",
      "ClientStatus": "running", "ClientDescription": "Tasks are running", "DesiredStatus": "run",
      "Job": { "Version": 3 }, "CreateTime": 1762000000000000000, "ModifyTime": 1762000060000000000,
      "TaskStates": { "server": { "State": "running", "Restarts": 1, "StartedAt": "2025-11-01T12:26:41Z",
        "Events": [
          { "Type": "Received", "DisplayMessage": "Task received by client", "Time": 1762000000000000000 },
          { "Type": "Started", "DisplayMessage": "Task started by client", "Time": 1762000001000000000 }
        ] } }
    }"#;

    #[tokio::test]
    async fn an_allocation_has_its_tasks_and_their_events_newest_first() {
        let fake = agent(vec![ok(
            "GET /v1/allocation/a1b2c3d4-0000-0000-0000-000000000001",
            ALLOCATION,
        )])
        .await;
        let detail = fake
            .client(None)
            .allocation("a1b2c3d4-0000-0000-0000-000000000001")
            .await
            .unwrap();
        assert_eq!(detail.allocation.job_version, 3);
        assert_eq!(detail.allocation.tasks[0].name, "server");
        assert_eq!(detail.events[0].kind, "Started");
        assert_eq!(
            detail.client_description.as_deref(),
            Some("Tasks are running")
        );
    }

    #[tokio::test]
    async fn a_missing_allocation_is_named_by_its_short_id() {
        let fake = agent(vec![]).await;
        let e = fake
            .client(None)
            .allocation("deadbeef-0000-0000-0000-000000000000")
            .await
            .unwrap_err();
        assert!(e.is(ErrorCode::NotFound));
        assert!(e.message.contains("deadbeef") && !e.message.contains("deadbeef-"));
    }

    #[test]
    fn errors_of_logs_and_terminals_are_told_apart() {
        let nomad = crate::Nomad::new(&crate::NomadConfig {
            name: "prod".into(),
            address: "http://127.0.0.1:1".into(),
            ..Default::default()
        })
        .unwrap();
        let classify = |status: u16, text: &str| {
            nomad.allocation_error(
                crate::error::answer("prod", status, text),
                "a1b2c3d4-0000",
                Some("server"),
                msg::EXEC_DENIED,
                msg::EXEC_DENIED_HINT,
            )
        };
        let denied = classify(403, "Permission denied");
        assert_eq!(denied.key, "errors.nomad.execDenied");
        assert!(classify(500, "rpc error: No path to node").is(ErrorCode::NomadNodeUnreachable));
        assert!(classify(500, "task \"server\" is not running")
            .is(ErrorCode::NomadAllocationNotRunning));
        assert!(classify(404, "Unknown allocation \"a1b2\"").is(ErrorCode::NotFound));
    }
}
