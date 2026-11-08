//! Port forwards from this computer to a Pod, or to a Service through one
//! of its ready Pods, like `kubectl port-forward`. Each listens on loopback
//! only and opens a forward through the Kubernetes API per connection. They
//! live in memory: nothing outlives Homeport, nothing is restored.

use std::{
    collections::HashMap,
    net::{Ipv4Addr, Ipv6Addr, SocketAddr},
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc, Mutex,
    },
};

use k8s_openapi::{
    api::core::v1::{Pod, Service},
    apimachinery::pkg::util::intstr::IntOrString,
};
use kube::api::{Api, ListParams};
use serde::{Deserialize, Serialize};
use tokio::{net::TcpListener, task::AbortHandle};

use homeport_core::error::{AppError, AppResult, ErrorCode};

use crate::{
    messages as msg,
    registry::ClusterNotifier,
    resources::{forbidden_on, ApiType, On, ResourceKind},
};

pub type ForwardId = u32;

/// What a forward reaches.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum ForwardTarget {
    Pod {
        namespace: String,
        name: String,
    },
    /// Through one of its ready Pods, picked again when that one goes.
    Service {
        namespace: String,
        name: String,
    },
}

impl ForwardTarget {
    fn namespace(&self) -> &str {
        match self {
            Self::Pod { namespace, .. } | Self::Service { namespace, .. } => namespace,
        }
    }
}

/// An active forward, as the list shows it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct PortForward {
    pub id: ForwardId,
    pub cluster_id: String,
    pub cluster_name: String,
    pub target: ForwardTarget,
    /// The port of the Pod, or of the Service.
    pub remote_port: u16,
    pub local_port: u16,
    /// Where it listens: `127.0.0.1:8080`, and `[::1]:8080` when that was free.
    pub addresses: Vec<String>,
    /// Connections open through it now.
    pub connections: u32,
    /// The Pod connections go to, once one went through.
    pub pod: Option<String>,
    pub started: String,
}

/// A forward failed, or ended because its target is gone.
#[derive(Debug, Clone, PartialEq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ForwardEvent {
    pub id: ForwardId,
    pub cluster_id: String,
    pub error: AppError,
    /// The forward was stopped.
    pub ended: bool,
}

struct Running {
    view: PortForward,
    connections: Arc<AtomicU32>,
    pod: Arc<Mutex<Option<(String, u16)>>>,
    tasks: Vec<AbortHandle>,
}

impl Drop for Running {
    fn drop(&mut self) {
        for task in &self.tasks {
            task.abort();
        }
    }
}

type Open = Arc<Mutex<HashMap<ForwardId, Running>>>;

/// Every active forward, by id.
pub struct PortForwards {
    next: AtomicU32,
    open: Open,
    notifier: Arc<dyn ClusterNotifier>,
}

impl PortForwards {
    pub fn new(notifier: Arc<dyn ClusterNotifier>) -> Arc<Self> {
        Arc::new(Self {
            next: AtomicU32::new(0),
            open: Arc::default(),
            notifier,
        })
    }

    /// Starts forwarding `local_port` (a free one without) to `remote_port`
    /// of `target`, after checking the target can be reached.
    pub async fn start(
        &self,
        on: On,
        target: ForwardTarget,
        remote_port: u16,
        local_port: Option<u16>,
    ) -> AppResult<PortForward> {
        let first = resolve(&on, &target, remote_port).await?;
        let (listeners, port) = bind(local_port).await?;
        let id = self.next.fetch_add(1, Ordering::Relaxed) + 1;
        let view = PortForward {
            id,
            cluster_id: on.cluster.id.clone(),
            cluster_name: on.cluster.name.clone(),
            target: target.clone(),
            remote_port,
            local_port: port,
            addresses: listeners
                .iter()
                .filter_map(|l| l.local_addr().ok())
                .map(|a| a.to_string())
                .collect(),
            connections: 0,
            pod: None,
            started: jiff::Timestamp::now().to_string(),
        };
        let connections = Arc::new(AtomicU32::new(0));
        let pod = Arc::new(Mutex::new(Some(first)));
        let shared = Shared {
            id,
            on,
            target,
            remote_port,
            connections: connections.clone(),
            pod: pod.clone(),
            open: self.open.clone(),
            notifier: self.notifier.clone(),
        };
        let mut open = self.open.lock().expect("port forwards lock poisoned");
        let tasks = listeners
            .into_iter()
            .map(|listener| tokio::spawn(accept(listener, shared.clone())).abort_handle())
            .collect();
        open.insert(
            id,
            Running {
                view: view.clone(),
                connections,
                pod,
                tasks,
            },
        );
        Ok(view)
    }

    /// Every active forward, oldest first.
    pub fn list(&self) -> Vec<PortForward> {
        let open = self.open.lock().expect("port forwards lock poisoned");
        let mut list: Vec<PortForward> = open
            .values()
            .map(|r| PortForward {
                connections: r.connections.load(Ordering::Relaxed),
                pod: r
                    .pod
                    .lock()
                    .expect("port forward lock poisoned")
                    .as_ref()
                    .map(|(name, _)| name.clone()),
                ..r.view.clone()
            })
            .collect();
        list.sort_by_key(|f| f.id);
        list
    }

    /// Stops a forward: it stops listening and its connections close.
    pub fn stop(&self, id: ForwardId) {
        self.open
            .lock()
            .expect("port forwards lock poisoned")
            .remove(&id);
    }

    /// Stops the forwards to a Cluster, as when it's removed or disabled.
    pub fn stop_cluster(&self, cluster_id: &str) {
        self.open
            .lock()
            .expect("port forwards lock poisoned")
            .retain(|_, r| r.view.cluster_id != cluster_id);
    }

    pub fn stop_all(&self) {
        self.open
            .lock()
            .expect("port forwards lock poisoned")
            .clear();
    }
}

/// What every connection of a forward needs.
#[derive(Clone)]
struct Shared {
    id: ForwardId,
    on: On,
    target: ForwardTarget,
    remote_port: u16,
    connections: Arc<AtomicU32>,
    /// The Pod and port connections go to, until it goes away.
    pod: Arc<Mutex<Option<(String, u16)>>>,
    open: Open,
    notifier: Arc<dyn ClusterNotifier>,
}

impl Shared {
    fn report(&self, error: AppError, ended: bool) {
        if ended {
            self.open
                .lock()
                .expect("port forwards lock poisoned")
                .remove(&self.id);
        }
        self.notifier.port_forward(&ForwardEvent {
            id: self.id,
            cluster_id: self.on.cluster.id.clone(),
            error,
            ended,
        });
    }

    /// The Pod and port to send a connection to: the last one, or found again.
    async fn destination(&self) -> AppResult<(String, u16)> {
        if let Some(known) = self.pod.lock().expect("port forward lock poisoned").clone() {
            return Ok(known);
        }
        let found = resolve(&self.on, &self.target, self.remote_port).await?;
        *self.pod.lock().expect("port forward lock poisoned") = Some(found.clone());
        Ok(found)
    }
}

/// Accepts connections until stopped; stopping it (dropping its task) ends
/// the connections it opened too.
async fn accept(listener: TcpListener, shared: Shared) {
    let mut connections = tokio::task::JoinSet::new();
    loop {
        tokio::select! {
            accepted = listener.accept() => match accepted {
                Ok((socket, _)) => {
                    connections.spawn(connect(socket, shared.clone()));
                }
                Err(e) => {
                    log::debug!("port forward {} stopped accepting: {e}", shared.id);
                    return;
                }
            },
            // Forget the connections that ended.
            Some(_) = connections.join_next(), if !connections.is_empty() => {}
        }
    }
}

/// Carries one connection to the target and back.
async fn connect(mut socket: tokio::net::TcpStream, shared: Shared) {
    let namespace = shared.target.namespace().to_owned();
    let api: Api<Pod> = Api::namespaced(shared.on.client.clone(), &namespace);
    let mut retried = false;
    loop {
        let (pod, port) = match shared.destination().await {
            Ok(found) => found,
            Err(error) => {
                let gone = matches!(shared.target, ForwardTarget::Pod { .. })
                    && error.is(ErrorCode::NotFound);
                shared.report(error, gone);
                return;
            }
        };
        match api.portforward(&pod, &[port]).await {
            Ok(mut forwarder) => {
                let Some(mut upstream) = forwarder.take_stream(port) else {
                    return;
                };
                shared.connections.fetch_add(1, Ordering::Relaxed);
                if let Err(e) = tokio::io::copy_bidirectional(&mut socket, &mut upstream).await {
                    log::debug!("port forward {} connection ended: {e}", shared.id);
                }
                shared.connections.fetch_sub(1, Ordering::Relaxed);
                drop(upstream);
                if let Err(e) = forwarder.join().await {
                    log::debug!("port forward {} closed: {e}", shared.id);
                }
                return;
            }
            // The Pod went away: a Service's next ready Pod takes over.
            Err(kube::Error::Api(status))
                if status.code == 404
                    && !retried
                    && matches!(shared.target, ForwardTarget::Service { .. }) =>
            {
                retried = true;
                *shared.pod.lock().expect("port forward lock poisoned") = None;
            }
            Err(e) => {
                let error = forward_failed(&shared.on, &namespace, &pod, &e);
                let gone = error.is(ErrorCode::NotFound)
                    && matches!(shared.target, ForwardTarget::Pod { .. });
                *shared.pod.lock().expect("port forward lock poisoned") = None;
                shared.report(error, gone);
                return;
            }
        }
    }
}

/// The `pods/portforward` subresource, as RBAC names it.
fn pod_portforward() -> ApiType {
    ApiType {
        group: String::new(),
        version: "v1".into(),
        kind: "Pod".into(),
        plural: "pods/portforward".into(),
        namespaced: true,
    }
}

fn forward_failed(on: &On, namespace: &str, pod: &str, error: &kube::Error) -> AppError {
    match error {
        kube::Error::Api(status) if status.code == 403 => {
            forbidden_on("create", &pod_portforward(), Some(namespace)).with_detail(&status.message)
        }
        e => on.failed_on("create", &pod_portforward(), Some(namespace), Some(pod), e),
    }
}

/// Listens on loopback at `port`, or a free port without one; on IPv6's
/// loopback too when the same port is free there.
pub async fn bind(port: Option<u16>) -> AppResult<(Vec<TcpListener>, u16)> {
    let v4 = SocketAddr::from((Ipv4Addr::LOCALHOST, port.unwrap_or(0)));
    let first = TcpListener::bind(v4)
        .await
        .map_err(|e| bind_failed(port, &e))?;
    let port = first.local_addr()?.port();
    let mut listeners = vec![first];
    if let Ok(v6) = TcpListener::bind(SocketAddr::from((Ipv6Addr::LOCALHOST, port))).await {
        listeners.push(v6);
    }
    Ok((listeners, port))
}

/// Whether `port` is free on this computer's loopback, for proposing it.
pub async fn is_free(port: u16) -> bool {
    TcpListener::bind(SocketAddr::from((Ipv4Addr::LOCALHOST, port)))
        .await
        .is_ok()
}

fn bind_failed(port: Option<u16>, error: &std::io::Error) -> AppError {
    let port = port.unwrap_or_default();
    match error.kind() {
        std::io::ErrorKind::AddrInUse => AppError::new(
            ErrorCode::PortInUse,
            msg::LOCAL_PORT_IN_USE.with("port", port),
        )
        .with_hint(msg::LOCAL_PORT_IN_USE_HINT),
        std::io::ErrorKind::PermissionDenied => AppError::new(
            ErrorCode::PermissionDenied,
            msg::LOCAL_PORT_PRIVILEGED.with("port", port),
        )
        .with_hint(msg::LOCAL_PORT_IN_USE_HINT),
        _ => AppError::from(std::io::Error::new(error.kind(), error.to_string())),
    }
}

/// The Pod and its port that a connection to `target` goes to.
pub async fn resolve(on: &On, target: &ForwardTarget, port: u16) -> AppResult<(String, u16)> {
    match target {
        ForwardTarget::Pod { namespace, name } => {
            let pod = on.get_pod(namespace, name).await?;
            if !running(&pod) {
                return Err(AppError::new(
                    ErrorCode::ContainerNotRunning,
                    msg::FORWARD_POD_NOT_RUNNING.with("pod", name),
                )
                .with_hint(msg::FORWARD_POD_NOT_RUNNING_HINT));
            }
            Ok((name.clone(), port))
        }
        ForwardTarget::Service { namespace, name } => {
            let service: Service = on.get_service(namespace, name).await?;
            let (selector, target_port) =
                service_target(&service, port).map_err(|why| why.explained(name))?;
            let api: Api<Pod> = Api::namespaced(on.client.clone(), namespace);
            let pods = api
                .list(&ListParams::default().labels(&selector))
                .await
                .map_err(|e| on.failed("list", ResourceKind::Pod, Some(namespace), None, &e))?;
            let no_pod = || {
                AppError::new(
                    ErrorCode::NotFound,
                    msg::FORWARD_NO_READY_POD.with("service", name),
                )
                .with_hint(msg::FORWARD_NO_READY_POD_HINT)
            };
            let pod = ready_pod(&pods.items).ok_or_else(no_pod)?;
            let pod_port = pod_port(pod, &target_port).ok_or_else(|| {
                AppError::new(
                    ErrorCode::NotFound,
                    msg::FORWARD_NAMED_PORT_MISSING
                        .with("port", target_port.label())
                        .with("pod", pod.metadata.name.clone().unwrap_or_default()),
                )
            })?;
            Ok((pod.metadata.name.clone().unwrap_or_default(), pod_port))
        }
    }
}

/// Where a Service sends its port: a number, or a container port's name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetPort {
    Number(u16),
    Name(String),
}

impl TargetPort {
    fn label(&self) -> String {
        match self {
            Self::Number(n) => n.to_string(),
            Self::Name(name) => name.clone(),
        }
    }
}

/// Why a Service's port can't be forwarded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceProblem {
    NoSelector,
    NoSuchPort(u16),
}

impl ServiceProblem {
    fn explained(self, service: &str) -> AppError {
        match self {
            Self::NoSelector => AppError::new(
                ErrorCode::Validation,
                msg::FORWARD_NO_SELECTOR.with("service", service),
            )
            .with_hint(msg::FORWARD_NO_SELECTOR_HINT),
            Self::NoSuchPort(port) => AppError::new(
                ErrorCode::NotFound,
                msg::FORWARD_NO_SUCH_PORT
                    .with("service", service)
                    .with("port", port),
            ),
        }
    }
}

/// The label selector of a Service's Pods and where its `port` goes in them.
pub fn service_target(
    service: &Service,
    port: u16,
) -> Result<(String, TargetPort), ServiceProblem> {
    let spec = service.spec.clone().unwrap_or_default();
    let selector = spec
        .selector
        .filter(|s| !s.is_empty())
        .ok_or(ServiceProblem::NoSelector)?
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join(",");
    let entry = spec
        .ports
        .unwrap_or_default()
        .into_iter()
        .find(|p| u16::try_from(p.port).ok() == Some(port))
        .ok_or(ServiceProblem::NoSuchPort(port))?;
    let target = match entry.target_port {
        None => TargetPort::Number(port),
        Some(IntOrString::Int(n)) => TargetPort::Number(u16::try_from(n).unwrap_or(port)),
        Some(IntOrString::String(name)) => match name.parse() {
            Ok(n) => TargetPort::Number(n),
            Err(_) => TargetPort::Name(name),
        },
    };
    Ok((selector, target))
}

/// The number of `target` in `pod`: itself, or the container port of that name.
pub fn pod_port(pod: &Pod, target: &TargetPort) -> Option<u16> {
    match target {
        TargetPort::Number(n) => Some(*n),
        TargetPort::Name(name) => pod
            .spec
            .as_ref()?
            .containers
            .iter()
            .flat_map(|c| c.ports.iter().flatten())
            .find(|p| p.name.as_deref() == Some(name))
            .and_then(|p| u16::try_from(p.container_port).ok()),
    }
}

fn running(pod: &Pod) -> bool {
    pod.metadata.deletion_timestamp.is_none()
        && pod.status.as_ref().and_then(|s| s.phase.as_deref()) == Some("Running")
}

/// The first running Pod (by name) whose `Ready` condition holds.
pub fn ready_pod(pods: &[Pod]) -> Option<&Pod> {
    let ready = |p: &&Pod| {
        running(p)
            && p.status
                .as_ref()
                .and_then(|s| s.conditions.as_ref())
                .is_some_and(|c| c.iter().any(|c| c.type_ == "Ready" && c.status == "True"))
    };
    let mut candidates: Vec<&Pod> = pods.iter().filter(ready).collect();
    candidates.sort_by_key(|p| p.metadata.name.clone());
    candidates.into_iter().next()
}

#[cfg(test)]
mod tests;
