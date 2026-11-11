//! A Swarm's tasks: each slot of each service on its node, with why it's in
//! the state it's in, read through a manager.

use std::collections::HashMap;

use bollard::{
    models::{Node, Task},
    query_parameters::{ListNodesOptions, ListServicesOptions, ListTasksOptionsBuilder},
    Docker,
};
use serde::Serialize;

use homeport_core::error::AppResult;

use super::services::split_digest;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct TaskView {
    pub id: String,
    pub service_id: String,
    pub service_name: String,
    /// The replica's number, for a replicated service; `None` for a global
    /// one, whose tasks are told apart by node.
    pub slot: Option<u64>,
    pub node_id: Option<String>,
    pub node_hostname: Option<String>,
    /// What the Swarm wants: `running`, `shutdown`…
    pub desired_state: String,
    /// What the task is doing: `running`, `preparing`, `failed`, `rejected`…
    pub state: String,
    /// When it got into `state`.
    pub since: Option<String>,
    /// Why it failed or was rejected, in the Engine's words.
    pub error: Option<String>,
    pub message: Option<String>,
    pub image: String,
    pub container_id: Option<String>,
    pub exit_code: Option<i64>,
}

pub fn view(
    task: Task,
    services: &HashMap<String, String>,
    nodes: &HashMap<String, String>,
) -> TaskView {
    let status = task.status.unwrap_or_default();
    let container = status.container_status.unwrap_or_default();
    let service_id = task.service_id.unwrap_or_default();
    let non_empty = |s: Option<String>| s.filter(|s| !s.is_empty());
    let failed = status.err.as_ref().is_some_and(|e| !e.is_empty());
    let image = task
        .spec
        .and_then(|s| s.container_spec)
        .and_then(|c| c.image)
        .unwrap_or_default();
    TaskView {
        id: task.id.unwrap_or_default(),
        service_name: services.get(&service_id).cloned().unwrap_or_default(),
        service_id,
        slot: task
            .slot
            .and_then(|s| u64::try_from(s).ok())
            .filter(|s| *s > 0),
        node_hostname: task.node_id.as_ref().and_then(|n| nodes.get(n).cloned()),
        node_id: non_empty(task.node_id),
        desired_state: task
            .desired_state
            .map(|s| s.to_string())
            .unwrap_or_default(),
        state: status.state.map(|s| s.to_string()).unwrap_or_default(),
        since: status.timestamp.map(|d| d.to_string()),
        error: non_empty(status.err),
        message: non_empty(status.message),
        image: split_digest(&image).0,
        container_id: non_empty(container.container_id),
        exit_code: container.exit_code.filter(|_| failed),
    }
}

/// Which tasks to list: all of them, or those of a service or of a node.
#[derive(Debug, Clone, Default, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct TaskFilter {
    pub service_id: Option<String>,
    pub node_id: Option<String>,
}

pub async fn list(docker: &Docker, filter: &TaskFilter) -> AppResult<Vec<TaskView>> {
    let mut filters: HashMap<&str, Vec<&str>> = HashMap::new();
    if let Some(service) = &filter.service_id {
        filters.insert("service", vec![service]);
    }
    if let Some(node) = &filter.node_id {
        filters.insert("node", vec![node]);
    }
    let tasks = docker
        .list_tasks(Some(
            ListTasksOptionsBuilder::default().filters(&filters).build(),
        ))
        .await?;
    let services: HashMap<String, String> = docker
        .list_services(None::<ListServicesOptions>)
        .await?
        .into_iter()
        .filter_map(|s| Some((s.id?, s.spec?.name?)))
        .collect();
    let nodes: HashMap<String, String> = docker
        .list_nodes(None::<ListNodesOptions>)
        .await?
        .into_iter()
        .filter_map(|n: Node| Some((n.id?, n.description?.hostname?)))
        .collect();
    let mut views: Vec<TaskView> = tasks
        .into_iter()
        .map(|t| view(t, &services, &nodes))
        .collect();
    // Newest first; the frontend groups them.
    views.sort_by(|a, b| b.since.cmp(&a.since));
    Ok(views)
}

pub async fn inspect(docker: &Docker, id: &str) -> AppResult<serde_json::Value> {
    Ok(serde_json::to_value(docker.inspect_task(id).await?)?)
}

#[cfg(test)]
mod tests {
    use bollard::models::{
        ContainerStatus, TaskSpec, TaskSpecContainerSpec, TaskState, TaskStatus,
    };

    use super::*;

    #[test]
    fn maps_a_rejected_task_with_its_error() {
        let task = Task {
            id: Some("t1".into()),
            service_id: Some("s1".into()),
            slot: Some(2),
            node_id: Some("n1".into()),
            desired_state: Some(TaskState::RUNNING),
            status: Some(TaskStatus {
                state: Some(TaskState::REJECTED),
                err: Some("No such image: nope:1".into()),
                timestamp: Some("2026-11-08T10:00:00Z".into()),
                container_status: Some(ContainerStatus::default()),
                ..Default::default()
            }),
            spec: Some(TaskSpec {
                container_spec: Some(TaskSpecContainerSpec {
                    image: Some("nope:1@sha256:abc".into()),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        };
        let services = [("s1".to_string(), "web".to_string())].into();
        let nodes = [("n1".to_string(), "alpha".to_string())].into();
        let v = view(task, &services, &nodes);
        assert_eq!(v.service_name, "web");
        assert_eq!(v.slot, Some(2));
        assert_eq!(v.node_hostname.as_deref(), Some("alpha"));
        assert_eq!(
            (v.desired_state.as_str(), v.state.as_str()),
            ("running", "rejected")
        );
        assert_eq!(v.error.as_deref(), Some("No such image: nope:1"));
        assert_eq!(v.image, "nope:1");
        assert_eq!(v.container_id, None);
    }

    #[tokio::test]
    #[ignore = "needs a local Docker Engine"]
    async fn an_unpullable_image_shows_in_the_tasks() {
        use bollard::models::ServiceSpec;

        let docker = crate::testing::docker();
        let _swarm = crate::testing::swarm(&docker).await;
        let name = crate::testing::unique("tasks");
        let id = docker
            .create_service(
                ServiceSpec {
                    name: Some(name),
                    task_template: Some(TaskSpec {
                        container_spec: Some(TaskSpecContainerSpec {
                            image: Some("homeport-test/does-not-exist:nope".into()),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                None,
            )
            .await
            .unwrap()
            .id
            .unwrap();
        let filter = TaskFilter {
            service_id: Some(id.clone()),
            node_id: None,
        };
        let mut failed = None;
        for _ in 0..150 {
            let tasks = list(&docker, &filter).await.unwrap();
            if let Some(t) = tasks.into_iter().find(|t| t.error.is_some()) {
                failed = Some(t);
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        }
        docker.delete_service(&id).await.unwrap();
        let failed = failed.expect("a task with an error");
        assert_eq!(failed.slot, Some(1));
        assert!(failed.node_hostname.is_some());
    }
}
