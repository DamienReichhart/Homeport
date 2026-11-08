//! A Swarm's services: listing them with their replicas, scaling, rolling
//! back and removing them, and their logs, through a manager.

use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

use bollard::{
    container::LogOutput,
    models::{
        EndpointPortConfig, EndpointPortConfigProtocolEnum, EndpointPortConfigPublishModeEnum,
        Service, ServiceSpecMode, ServiceSpecModeReplicated,
    },
    query_parameters::{
        ListNodesOptions, ListServicesOptionsBuilder, ListTasksOptionsBuilder, LogsOptionsBuilder,
        UpdateServiceOptionsBuilder,
    },
    Docker,
};
use futures_util::{
    stream::{BoxStream, SelectAll},
    StreamExt,
};
use serde::Serialize;

use homeport_core::{
    error::{engine::from_bollard, AppError, AppResult},
    logs::{BATCH_EVERY, BATCH_LINES},
};

use crate::{
    containers::logs::{LineSplitter, LogEvent, LogLine, LogOptions},
    messages as msg,
};

/// The label `docker stack deploy` puts on everything of a swarm stack.
pub const STACK_NAMESPACE: &str = "com.docker.stack.namespace";

/// How many tasks a service runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum ServiceMode {
    /// A set number of replicas.
    Replicated,
    /// One task on every node that may run it.
    Global,
    /// Runs to completion, a set number of times.
    ReplicatedJob,
    /// Runs to completion once on every node.
    GlobalJob,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ServicePort {
    pub target: u32,
    pub published: Option<u32>,
    /// `tcp`, `udp` or `sctp`.
    pub protocol: String,
    /// `ingress` (the routing mesh) or `host`.
    pub mode: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ServiceView {
    pub id: String,
    /// The service's `Version.Index`, sent back with an update.
    pub version: u64,
    pub name: String,
    /// Its image without the digest the Swarm pinned.
    pub image: String,
    pub image_digest: Option<String>,
    pub mode: ServiceMode,
    /// The replicas asked for, for a replicated service.
    pub replicas: Option<u64>,
    /// Tasks running now.
    pub running: u64,
    /// Tasks that should be running.
    pub desired: u64,
    pub ports: Vec<ServicePort>,
    /// `updating`, `paused`, `completed`, `rollback_started`… while or after
    /// an update; `None` when the service was never updated.
    pub update_state: Option<String>,
    pub update_message: Option<String>,
    /// The swarm stack it belongs to (`com.docker.stack.namespace`).
    pub stack: Option<String>,
    pub labels: HashMap<String, String>,
    pub created: Option<String>,
    pub updated: Option<String>,
}

/// An image reference without its `@sha256:…` digest, and the digest.
pub fn split_digest(reference: &str) -> (String, Option<String>) {
    match reference.split_once('@') {
        Some((image, digest)) => (image.to_owned(), Some(digest.to_owned())),
        None => (reference.to_owned(), None),
    }
}

pub fn mode_of(mode: Option<&ServiceSpecMode>) -> (ServiceMode, Option<u64>) {
    match mode {
        Some(m) if m.global.is_some() => (ServiceMode::Global, None),
        Some(m) if m.global_job.is_some() => (ServiceMode::GlobalJob, None),
        Some(m) if m.replicated_job.is_some() => (ServiceMode::ReplicatedJob, None),
        Some(ServiceSpecMode {
            replicated: Some(r),
            ..
        }) => (
            ServiceMode::Replicated,
            Some(r.replicas.and_then(|n| u64::try_from(n).ok()).unwrap_or(1)),
        ),
        _ => (ServiceMode::Replicated, Some(1)),
    }
}

fn port(p: &EndpointPortConfig) -> ServicePort {
    let positive = |n: Option<i64>| n.and_then(|n| u32::try_from(n).ok()).filter(|n| *n > 0);
    ServicePort {
        target: positive(p.target_port).unwrap_or(0),
        published: positive(p.published_port),
        protocol: p
            .protocol
            .unwrap_or(EndpointPortConfigProtocolEnum::TCP)
            .to_string(),
        mode: p
            .publish_mode
            .unwrap_or(EndpointPortConfigPublishModeEnum::INGRESS)
            .to_string(),
    }
}

pub fn view(service: Service) -> ServiceView {
    let spec = service.spec.unwrap_or_default();
    let container = spec
        .task_template
        .as_ref()
        .and_then(|t| t.container_spec.clone())
        .unwrap_or_default();
    let (image, image_digest) = split_digest(&container.image.unwrap_or_default());
    let (mode, replicas) = mode_of(spec.mode.as_ref());
    let status = service.service_status.unwrap_or_default();
    let update = service.update_status.unwrap_or_default();
    let labels = spec.labels.unwrap_or_default();
    // The ports in effect, else those asked for (before the first task runs).
    let ports = service
        .endpoint
        .and_then(|e| e.ports)
        .filter(|p| !p.is_empty())
        .or_else(|| spec.endpoint_spec.and_then(|e| e.ports))
        .unwrap_or_default();
    ServiceView {
        id: service.id.unwrap_or_default(),
        version: service.version.and_then(|v| v.index).unwrap_or(0),
        name: spec.name.unwrap_or_default(),
        image,
        image_digest,
        mode,
        replicas,
        running: status.running_tasks.unwrap_or(0),
        desired: status.desired_tasks.unwrap_or(0),
        ports: ports.iter().map(port).collect(),
        update_state: update
            .state
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty()),
        update_message: update.message.filter(|m| !m.is_empty()),
        stack: labels.get(STACK_NAMESPACE).cloned(),
        labels,
        created: service.created_at.map(|d| d.to_string()),
        updated: service.updated_at.map(|d| d.to_string()),
    }
}

pub async fn list(docker: &Docker) -> AppResult<Vec<ServiceView>> {
    let mut services: Vec<ServiceView> = docker
        .list_services(Some(
            ListServicesOptionsBuilder::default().status(true).build(),
        ))
        .await?
        .into_iter()
        .map(view)
        .collect();
    services.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(services)
}

pub async fn inspect(docker: &Docker, id: &str) -> AppResult<serde_json::Value> {
    Ok(serde_json::to_value(
        docker.inspect_service(id, None).await?,
    )?)
}

/// The update options for a service read at `version`.
pub(crate) fn at_version(version: Option<u64>) -> UpdateServiceOptionsBuilder {
    let version = version
        .and_then(|v| i32::try_from(v).ok())
        .unwrap_or_default();
    UpdateServiceOptionsBuilder::default().version(version)
}

/// Turns an update's failure into a conflict when the service changed since
/// it was read, or a validation error for what the Engine says can't be done.
pub(crate) fn update_error(e: &bollard::errors::Error) -> AppError {
    let text = e.to_string().to_lowercase();
    if text.contains("no previous spec") {
        return AppError::validation(msg::SERVICE_NO_PREVIOUS).with_cause(e);
    }
    from_bollard(e)
}

/// Checks a number of replicas a user typed.
pub fn check_replicas(replicas: u64) -> AppResult<()> {
    if replicas > 10_000 {
        return Err(AppError::validation(msg::SERVICE_REPLICAS));
    }
    Ok(())
}

/// Sets a replicated service's replicas, from the service as read at `version`.
pub async fn scale(docker: &Docker, id: &str, version: u64, replicas: u64) -> AppResult<()> {
    check_replicas(replicas)?;
    let current = docker.inspect_service(id, None).await?;
    let mut spec = current.spec.unwrap_or_default();
    match mode_of(spec.mode.as_ref()).0 {
        ServiceMode::Replicated => {}
        _ => return Err(AppError::validation(msg::SERVICE_NOT_REPLICATED)),
    }
    spec.mode = Some(ServiceSpecMode {
        replicated: Some(ServiceSpecModeReplicated {
            replicas: Some(i64::try_from(replicas).unwrap_or(i64::MAX)),
        }),
        ..Default::default()
    });
    docker
        .update_service(id, spec, at_version(Some(version)).build(), None)
        .await
        .map(|_| ())
        .map_err(|e| update_error(&e))
}

/// Puts back the service's previous spec, as `docker service rollback` does.
pub async fn rollback(docker: &Docker, id: &str) -> AppResult<()> {
    let current = docker.inspect_service(id, None).await?;
    let version = current.version.and_then(|v| v.index);
    let options = at_version(version).rollback("previous").build();
    docker
        .update_service(id, current.spec.unwrap_or_default(), options, None)
        .await
        .map(|_| ())
        .map_err(|e| update_error(&e))
}

pub async fn remove(docker: &Docker, id: &str) -> AppResult<()> {
    Ok(docker.delete_service(id).await?)
}

/// How often a followed service's logs look for tasks started since.
const NEW_TASKS_EVERY: Duration = Duration::from_secs(3);

/// The tasks of a service that ran a container, each with the label its
/// lines carry: `service.slot@node` (`service@node` for a global one).
async fn labelled_tasks(docker: &Docker, service_id: &str) -> AppResult<Vec<(String, String)>> {
    let service = docker.inspect_service(service_id, None).await?;
    let name = service.spec.and_then(|s| s.name).unwrap_or_default();
    let nodes: HashMap<String, String> = docker
        .list_nodes(None::<ListNodesOptions>)
        .await?
        .into_iter()
        .filter_map(|n| Some((n.id?, n.description?.hostname?)))
        .collect();
    let filters: HashMap<&str, Vec<&str>> = [("service", vec![service_id])].into();
    let tasks = docker
        .list_tasks(Some(
            ListTasksOptionsBuilder::default().filters(&filters).build(),
        ))
        .await?;
    Ok(tasks
        .into_iter()
        .filter(|t| {
            t.status
                .as_ref()
                .and_then(|s| s.container_status.as_ref())
                .and_then(|c| c.container_id.as_ref())
                .is_some_and(|id| !id.is_empty())
        })
        .filter_map(|t| {
            let node = t.node_id.as_deref().unwrap_or_default();
            let host = nodes
                .get(node)
                .map_or(&node[..node.len().min(12)], String::as_str);
            let label = match t.slot {
                Some(slot) if slot > 0 => format!("{name}.{slot}@{host}"),
                _ => format!("{name}@{host}"),
            };
            Some((t.id?, label))
        })
        .collect())
}

type TaskLines = BoxStream<'static, (usize, Result<LogOutput, bollard::errors::Error>)>;

/// Adds a task's log stream to `merged`, its lines tagged with its index in `labels`.
fn follow_task(
    merged: &mut SelectAll<TaskLines>,
    labels: &mut Vec<String>,
    seen: &mut HashSet<String>,
    id: String,
    label: String,
    stream: impl futures_util::Stream<Item = Result<LogOutput, bollard::errors::Error>> + Send + 'static,
) {
    let index = labels.len();
    labels.push(label);
    seen.insert(id);
    merged.push(stream.map(move |item| (index, item)).boxed());
}

/// Streams the logs of every task of the service `service_id` to `sink`,
/// each line labelled with its task and node, until they end, the sink is
/// gone (returns `false`) or the task is cancelled. While following, tasks
/// started later join the stream.
pub async fn stream_logs(
    docker: Docker,
    service_id: String,
    options: LogOptions,
    sink: impl Fn(LogEvent) -> bool,
) {
    let tasks = match labelled_tasks(&docker, &service_id).await {
        Ok(tasks) => tasks,
        Err(error) => {
            sink(LogEvent::Error {
                error: error.logged("service logs"),
            });
            return;
        }
    };
    let logs_options = |tail: &str| {
        let mut builder = LogsOptionsBuilder::default()
            .stdout(options.stdout)
            .stderr(options.stderr)
            .timestamps(options.timestamps)
            .follow(options.follow)
            .tail(tail);
        if let Some(since) = options.since {
            builder = builder.since(since);
        }
        builder.build()
    };
    let history = options.tail.map_or("all".to_string(), |t| t.to_string());
    let mut seen: HashSet<String> = HashSet::new();
    let mut labels: Vec<String> = Vec::new();
    let mut merged: SelectAll<TaskLines> = SelectAll::new();
    for (id, label) in tasks {
        let stream = docker.task_logs(&id, Some(logs_options(&history)));
        follow_task(&mut merged, &mut labels, &mut seen, id, label, stream);
    }
    let mut splitters: Vec<(LineSplitter, String)> = Vec::new();
    let mut batch: Vec<LogLine> = Vec::new();
    let mut ticker = tokio::time::interval(BATCH_EVERY);
    let mut looker = tokio::time::interval(NEW_TASKS_EVERY);
    looker.tick().await;
    loop {
        splitters.resize_with(labels.len(), || (LineSplitter::default(), String::new()));
        tokio::select! {
            item = merged.next(), if !merged.is_empty() => match item {
                Some((index, Ok(output))) => {
                    let (stream_name, bytes) = match output {
                        LogOutput::StdOut { message } => ("stdout", message),
                        LogOutput::StdErr { message } => ("stderr", message),
                        LogOutput::Console { message } => ("console", message),
                        LogOutput::StdIn { .. } => continue,
                    };
                    let (splitter, last) = &mut splitters[index];
                    stream_name.clone_into(last);
                    batch.extend(splitter.push(&bytes).into_iter().map(|text| LogLine {
                        source: Some(labels[index].clone()),
                        stream: stream_name.to_owned(),
                        text,
                    }));
                    if batch.len() >= BATCH_LINES && !sink(LogEvent::Lines { lines: std::mem::take(&mut batch) }) {
                        return;
                    }
                }
                // A task removed meanwhile ends its own stream only.
                Some((_, Err(e))) => log::debug!("a task's log stream ended: {e}"),
                None => {}
            },
            _ = ticker.tick() => {
                if !batch.is_empty() && !sink(LogEvent::Lines { lines: std::mem::take(&mut batch) }) {
                    return;
                }
                if merged.is_empty() && !options.follow {
                    break;
                }
            }
            _ = looker.tick(), if options.follow => {
                if let Ok(tasks) = labelled_tasks(&docker, &service_id).await {
                    let new: Vec<_> = tasks.into_iter().filter(|(id, _)| !seen.contains(id)).collect();
                    for (id, label) in new {
                        let stream = docker.task_logs(&id, Some(logs_options("all")));
                        follow_task(&mut merged, &mut labels, &mut seen, id, label, stream);
                    }
                }
            }
        }
    }
    for (index, (splitter, stream_name)) in splitters.iter_mut().enumerate() {
        if let Some(text) = splitter.finish() {
            batch.push(LogLine {
                source: Some(labels[index].clone()),
                stream: stream_name.clone(),
                text,
            });
        }
    }
    if !batch.is_empty() {
        sink(LogEvent::Lines { lines: batch });
    }
    sink(LogEvent::End);
}

#[cfg(test)]
mod tests {
    use bollard::models::{
        ObjectVersion, ServiceServiceStatus, ServiceSpec, ServiceUpdateStatus,
        ServiceUpdateStatusStateEnum, TaskSpec, TaskSpecContainerSpec,
    };

    use super::*;

    fn service(mode: ServiceSpecMode) -> Service {
        Service {
            id: Some("s1".into()),
            version: Some(ObjectVersion { index: Some(9) }),
            spec: Some(ServiceSpec {
                name: Some("shop_web".into()),
                labels: Some([(STACK_NAMESPACE.to_string(), "shop".to_string())].into()),
                mode: Some(mode),
                task_template: Some(TaskSpec {
                    container_spec: Some(TaskSpecContainerSpec {
                        image: Some("nginx:1.27@sha256:abc".into()),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            endpoint: Some(bollard::models::ServiceEndpoint {
                ports: Some(vec![EndpointPortConfig {
                    target_port: Some(80),
                    published_port: Some(8080),
                    ..Default::default()
                }]),
                ..Default::default()
            }),
            service_status: Some(ServiceServiceStatus {
                running_tasks: Some(1),
                desired_tasks: Some(3),
                completed_tasks: None,
            }),
            update_status: Some(ServiceUpdateStatus {
                state: Some(ServiceUpdateStatusStateEnum::UPDATING),
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    #[test]
    fn maps_a_replicated_service_of_a_stack() {
        let v = view(service(ServiceSpecMode {
            replicated: Some(ServiceSpecModeReplicated { replicas: Some(3) }),
            ..Default::default()
        }));
        assert_eq!(v.name, "shop_web");
        assert_eq!(v.version, 9);
        assert_eq!(v.image, "nginx:1.27");
        assert_eq!(v.image_digest.as_deref(), Some("sha256:abc"));
        assert_eq!((v.mode, v.replicas), (ServiceMode::Replicated, Some(3)));
        assert_eq!((v.running, v.desired), (1, 3));
        assert_eq!(v.stack.as_deref(), Some("shop"));
        assert_eq!(v.update_state.as_deref(), Some("updating"));
        assert_eq!(
            v.ports,
            [ServicePort {
                target: 80,
                published: Some(8080),
                protocol: "tcp".into(),
                mode: "ingress".into()
            }]
        );
    }

    #[test]
    fn global_services_have_no_replica_count() {
        let v = view(service(ServiceSpecMode {
            global: Some(HashMap::new()),
            ..Default::default()
        }));
        assert_eq!((v.mode, v.replicas), (ServiceMode::Global, None));
        assert_eq!(split_digest("redis"), ("redis".into(), None));
    }

    #[test]
    fn replicas_have_a_sane_limit() {
        assert!(check_replicas(0).is_ok());
        assert!(check_replicas(10_001).is_err());
    }

    #[tokio::test]
    #[ignore = "needs a local Docker Engine"]
    async fn a_service_scales_rolls_back_logs_and_goes() {
        use std::sync::{Arc, Mutex};

        use bollard::models::ServiceSpec;

        let docker = crate::testing::docker();
        let _swarm = crate::testing::swarm(&docker).await;
        crate::testing::ensure_image(&docker).await;
        let name = crate::testing::unique("svc");
        let created = docker
            .create_service(
                ServiceSpec {
                    name: Some(name.clone()),
                    task_template: Some(TaskSpec {
                        container_spec: Some(TaskSpecContainerSpec {
                            image: Some(crate::testing::IMAGE.into()),
                            command: Some(vec![
                                "sh".into(),
                                "-c".into(),
                                "echo hello-from-task; sleep 300".into(),
                            ]),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                None,
            )
            .await
            .unwrap();
        let id = created.id.unwrap();
        let find = || async {
            list(&docker)
                .await
                .unwrap()
                .into_iter()
                .find(|s| s.id == id)
                .unwrap()
        };
        let first = find().await;
        assert_eq!(first.replicas, Some(1));

        scale(&docker, &id, first.version, 2).await.unwrap();
        let mut scaled = find().await;
        for _ in 0..100 {
            if scaled.running == 2 {
                break;
            }
            tokio::time::sleep(Duration::from_millis(200)).await;
            scaled = find().await;
        }
        assert_eq!((scaled.replicas, scaled.running), (Some(2), 2));

        rollback(&docker, &id).await.unwrap();
        assert_eq!(find().await.replicas, Some(1));

        let lines = Arc::new(Mutex::new(Vec::new()));
        let sink = {
            let lines = lines.clone();
            move |e| {
                if let LogEvent::Lines { lines: l } = e {
                    lines.lock().unwrap().extend(l);
                }
                true
            }
        };
        let options = LogOptions {
            tail: None,
            timestamps: false,
            since: None,
            stdout: true,
            stderr: true,
            follow: false,
        };
        stream_logs(docker.clone(), id.clone(), options, sink).await;
        let source = lines
            .lock()
            .unwrap()
            .iter()
            .find(|l| l.text == "hello-from-task")
            .and_then(|l| l.source.clone())
            .unwrap();
        assert!(source.starts_with(&format!("{name}.")), "{source}");

        remove(&docker, &id).await.unwrap();
        assert!(list(&docker).await.unwrap().iter().all(|s| s.id != id));
    }
}
