//! What runs on a Cluster, read through its API: lists and details mapped
//! to small serialisable views, read-only YAML, the Events of an object,
//! and watches that say when a list changed.
//!
//! A request the credentials aren't allowed to make fails with `Forbidden`,
//! naming the verb and resource, so a page can say so and the others work.

pub mod actions;
mod api_type;
mod common;
pub mod config;
pub mod dynamic;
pub mod events;
mod kind;
pub mod network;
pub mod nodes;
pub mod pods;
pub mod summary;
mod watch;
pub mod workloads;

use std::fmt::Debug;

use k8s_openapi::api::{
    apps::v1::{DaemonSet, Deployment, StatefulSet},
    batch::v1::{CronJob, Job},
    core::v1::{ConfigMap, Event, Namespace, Node, Pod, Secret, Service},
    networking::v1::Ingress,
};
use kube::{
    api::{Api, ListParams},
    Resource,
};
use serde::de::DeserializeOwned;

use homeport_core::error::{AppError, AppResult, ErrorCode};

use crate::{classify::classify, messages as msg, model::Cluster};

pub use self::{
    api_type::ApiType,
    common::{ConditionView, OwnerView},
    config::{
        ConfigMapDetail, ConfigMapView, ConfigValue, DataKey, RevealedValue, SecretDetail,
        SecretView,
    },
    dynamic::{ObjectDetail, ObjectTable, OwnerRef, TableColumn, TableRow},
    events::{EventObject, EventView},
    kind::ResourceKind,
    network::{
        ClusterServiceDetail, ClusterServicePort, ClusterServiceView, IngressDetail, IngressPath,
        IngressTls, IngressView,
    },
    nodes::{ClusterNodeDetail, ClusterNodeView, NodeTaint},
    pods::{NamespaceView, PodContainer, PodDetail, PodHealth, PodPort, PodView},
    summary::{ClusterSummary, NodeCounts, Part, PodCounts},
    watch::{watch, watch_api},
    workloads::{TemplateContainer, WorkloadDetail, WorkloadHealth, WorkloadKind, WorkloadView},
};

/// Items fetched per request of a long list.
const PAGE: u32 = 500;

/// A connected Cluster: its client, and the Cluster to name in errors.
#[derive(Clone)]
pub struct On {
    pub client: kube::Client,
    pub cluster: Cluster,
}

impl On {
    /// The API of `K` in `namespace`, or across every Namespace with `None`.
    fn api<K>(&self, namespace: Option<&str>) -> Api<K>
    where
        K: Resource<Scope = k8s_openapi::NamespaceResourceScope>,
        K::DynamicType: Default,
    {
        match namespace {
            Some(ns) => Api::namespaced(self.client.clone(), ns),
            None => Api::all(self.client.clone()),
        }
    }

    fn cluster_api<K>(&self) -> Api<K>
    where
        K: Resource<Scope = k8s_openapi::ClusterResourceScope>,
        K::DynamicType: Default,
    {
        Api::all(self.client.clone())
    }

    /// Why `verb` on `kind` failed: not allowed, gone, or the Cluster's failure.
    pub(crate) fn failed(
        &self,
        verb: &str,
        kind: ResourceKind,
        namespace: Option<&str>,
        name: Option<&str>,
        error: &kube::Error,
    ) -> AppError {
        self.failed_on(verb, &kind.api_type(), namespace, name, error)
    }

    /// Why `verb` on an object of `api` failed: not allowed, gone, changed
    /// meanwhile, or the Cluster's failure.
    pub(crate) fn failed_on(
        &self,
        verb: &str,
        api: &ApiType,
        namespace: Option<&str>,
        name: Option<&str>,
        error: &kube::Error,
    ) -> AppError {
        match (error, name) {
            (kube::Error::Api(status), _) if status.code == 403 => {
                forbidden_on(verb, api, namespace).with_detail(&status.message)
            }
            (kube::Error::Api(status), Some(name)) if status.code == 404 => AppError::new(
                ErrorCode::NotFound,
                msg::RESOURCE_GONE
                    .with("kind", &api.kind)
                    .with("name", name),
            ),
            (kube::Error::Api(status), Some(name)) if status.code == 409 => AppError::new(
                ErrorCode::Conflict,
                msg::CHANGED_MEANWHILE
                    .with("kind", &api.kind)
                    .with("name", name),
            )
            .with_hint(msg::CHANGED_MEANWHILE_HINT)
            .with_detail(&status.message),
            (e, _) => classify(&self.cluster.name, self.cluster.address(), e),
        }
    }

    /// Every item of `api`, a page at a time.
    async fn list<K>(
        &self,
        api: Api<K>,
        kind: ResourceKind,
        namespace: Option<&str>,
        params: ListParams,
    ) -> AppResult<Vec<K>>
    where
        K: Clone + DeserializeOwned + Debug,
    {
        let mut items = vec![];
        let mut params = params.limit(PAGE);
        loop {
            let page = api
                .list(&params)
                .await
                .map_err(|e| self.failed("list", kind, namespace, None, &e))?;
            items.extend(page.items);
            match page.metadata.continue_ {
                Some(token) if !token.is_empty() => params = params.continue_token(&token),
                _ => return Ok(items),
            }
        }
    }

    async fn get<K>(
        &self,
        api: Api<K>,
        kind: ResourceKind,
        namespace: Option<&str>,
        name: &str,
    ) -> AppResult<K>
    where
        K: Clone + DeserializeOwned + Debug,
    {
        api.get(name)
            .await
            .map_err(|e| self.failed("get", kind, namespace, Some(name), &e))
    }

    pub async fn namespaces(&self) -> AppResult<Vec<NamespaceView>> {
        let items: Vec<Namespace> = self
            .list(
                self.cluster_api(),
                ResourceKind::Namespace,
                None,
                ListParams::default(),
            )
            .await?;
        let mut views: Vec<_> = items.iter().map(pods::namespace).collect();
        views.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(views)
    }

    /// The workloads of `kind` in `namespace` (every one with `None`).
    pub async fn workloads(
        &self,
        kind: WorkloadKind,
        namespace: Option<&str>,
    ) -> AppResult<Vec<WorkloadView>> {
        let r = kind.resource();
        let p = ListParams::default;
        let mut views: Vec<WorkloadView> = match kind {
            WorkloadKind::Deployment => self
                .list::<Deployment>(self.api(namespace), r, namespace, p())
                .await?
                .iter()
                .map(workloads::deployment)
                .collect(),
            WorkloadKind::StatefulSet => self
                .list::<StatefulSet>(self.api(namespace), r, namespace, p())
                .await?
                .iter()
                .map(workloads::stateful_set)
                .collect(),
            WorkloadKind::DaemonSet => self
                .list::<DaemonSet>(self.api(namespace), r, namespace, p())
                .await?
                .iter()
                .map(workloads::daemon_set)
                .collect(),
            WorkloadKind::Job => self
                .list::<Job>(self.api(namespace), r, namespace, p())
                .await?
                .iter()
                .map(workloads::job)
                .collect(),
            WorkloadKind::CronJob => self
                .list::<CronJob>(self.api(namespace), r, namespace, p())
                .await?
                .iter()
                .map(workloads::cron_job)
                .collect(),
        };
        views.sort_by(|a, b| (&a.namespace, &a.name).cmp(&(&b.namespace, &b.name)));
        Ok(views)
    }

    pub async fn workload(
        &self,
        kind: WorkloadKind,
        namespace: &str,
        name: &str,
    ) -> AppResult<WorkloadDetail> {
        let r = kind.resource();
        let ns = Some(namespace);
        Ok(match kind {
            WorkloadKind::Deployment => {
                workloads::deployment_detail(&self.get(self.api(ns), r, ns, name).await?)
            }
            WorkloadKind::StatefulSet => {
                workloads::stateful_set_detail(&self.get(self.api(ns), r, ns, name).await?)
            }
            WorkloadKind::DaemonSet => {
                workloads::daemon_set_detail(&self.get(self.api(ns), r, ns, name).await?)
            }
            WorkloadKind::Job => workloads::job_detail(&self.get(self.api(ns), r, ns, name).await?),
            WorkloadKind::CronJob => {
                workloads::cron_job_detail(&self.get(self.api(ns), r, ns, name).await?)
            }
        })
    }

    /// The Jobs a CronJob started, newest first.
    pub async fn jobs_of(&self, namespace: &str, cron_job: &str) -> AppResult<Vec<WorkloadView>> {
        let mut jobs: Vec<WorkloadView> = self
            .workloads(WorkloadKind::Job, Some(namespace))
            .await?
            .into_iter()
            .filter(|j| {
                j.owner
                    .as_ref()
                    .is_some_and(|o| o.kind == "CronJob" && o.name == cron_job)
            })
            .collect();
        jobs.sort_by(|a, b| b.created.cmp(&a.created));
        Ok(jobs)
    }

    /// The Pods in `namespace` (every one with `None`), those matching a
    /// label `selector` or on a `node` when given.
    pub async fn pods(
        &self,
        namespace: Option<&str>,
        selector: Option<&str>,
        node: Option<&str>,
    ) -> AppResult<Vec<PodView>> {
        let mut params = ListParams::default();
        if let Some(selector) = selector.filter(|s| !s.is_empty()) {
            params = params.labels(selector);
        }
        if let Some(node) = node {
            params = params.fields(&format!("spec.nodeName={node}"));
        }
        let items: Vec<Pod> = self
            .list(self.api(namespace), ResourceKind::Pod, namespace, params)
            .await?;
        let mut views: Vec<_> = items.iter().map(pods::view).collect();
        views.sort_by(|a, b| (&a.namespace, &a.name).cmp(&(&b.namespace, &b.name)));
        Ok(views)
    }

    pub async fn pod(&self, namespace: &str, name: &str) -> AppResult<PodDetail> {
        let ns = Some(namespace);
        let pod: Pod = self.get(self.api(ns), ResourceKind::Pod, ns, name).await?;
        Ok(pods::detail(&pod))
    }

    pub(crate) async fn get_pod(&self, namespace: &str, name: &str) -> AppResult<Pod> {
        let ns = Some(namespace);
        self.get(self.api(ns), ResourceKind::Pod, ns, name).await
    }

    pub(crate) async fn get_service(&self, namespace: &str, name: &str) -> AppResult<Service> {
        let ns = Some(namespace);
        self.get(self.api(ns), ResourceKind::Service, ns, name)
            .await
    }

    pub async fn services(&self, namespace: Option<&str>) -> AppResult<Vec<ClusterServiceView>> {
        let items: Vec<Service> = self
            .list(
                self.api(namespace),
                ResourceKind::Service,
                namespace,
                ListParams::default(),
            )
            .await?;
        Ok(sorted(items.iter().map(network::service).collect(), |s| {
            (s.namespace.clone(), s.name.clone())
        }))
    }

    pub async fn service(&self, namespace: &str, name: &str) -> AppResult<ClusterServiceDetail> {
        let ns = Some(namespace);
        let s: Service = self
            .get(self.api(ns), ResourceKind::Service, ns, name)
            .await?;
        Ok(network::service_detail(&s))
    }

    pub async fn ingresses(&self, namespace: Option<&str>) -> AppResult<Vec<IngressView>> {
        let items: Vec<Ingress> = self
            .list(
                self.api(namespace),
                ResourceKind::Ingress,
                namespace,
                ListParams::default(),
            )
            .await?;
        Ok(sorted(items.iter().map(network::ingress).collect(), |i| {
            (i.namespace.clone(), i.name.clone())
        }))
    }

    pub async fn ingress(&self, namespace: &str, name: &str) -> AppResult<IngressDetail> {
        let ns = Some(namespace);
        let i: Ingress = self
            .get(self.api(ns), ResourceKind::Ingress, ns, name)
            .await?;
        Ok(network::ingress_detail(&i))
    }

    pub async fn config_maps(&self, namespace: Option<&str>) -> AppResult<Vec<ConfigMapView>> {
        let items: Vec<ConfigMap> = self
            .list(
                self.api(namespace),
                ResourceKind::ConfigMap,
                namespace,
                ListParams::default(),
            )
            .await?;
        Ok(sorted(
            items.iter().map(config::config_map).collect(),
            |c| (c.namespace.clone(), c.name.clone()),
        ))
    }

    pub async fn config_map(&self, namespace: &str, name: &str) -> AppResult<ConfigMapDetail> {
        let ns = Some(namespace);
        let c: ConfigMap = self
            .get(self.api(ns), ResourceKind::ConfigMap, ns, name)
            .await?;
        Ok(config::config_map_detail(&c))
    }

    /// Secrets' names, types and keys; never a value.
    pub async fn secrets(&self, namespace: Option<&str>) -> AppResult<Vec<SecretView>> {
        let items: Vec<Secret> = self
            .list(
                self.api(namespace),
                ResourceKind::Secret,
                namespace,
                ListParams::default(),
            )
            .await?;
        Ok(sorted(items.iter().map(config::secret).collect(), |s| {
            (s.namespace.clone(), s.name.clone())
        }))
    }

    pub async fn secret(&self, namespace: &str, name: &str) -> AppResult<SecretDetail> {
        let ns = Some(namespace);
        let s: Secret = self
            .get(self.api(ns), ResourceKind::Secret, ns, name)
            .await?;
        Ok(config::secret_detail(&s))
    }

    /// One value of a Secret, decoded, for the user who asked to see it.
    pub async fn reveal(&self, namespace: &str, name: &str, key: &str) -> AppResult<RevealedValue> {
        let ns = Some(namespace);
        let s: Secret = self
            .get(self.api(ns), ResourceKind::Secret, ns, name)
            .await?;
        config::value(&s, key).ok_or_else(|| {
            AppError::new(
                ErrorCode::NotFound,
                msg::SECRET_KEY_GONE.with("key", key).with("name", name),
            )
        })
    }

    pub async fn nodes(&self) -> AppResult<Vec<ClusterNodeView>> {
        let items: Vec<Node> = self
            .list(
                self.cluster_api(),
                ResourceKind::Node,
                None,
                ListParams::default(),
            )
            .await?;
        Ok(sorted(items.iter().map(nodes::view).collect(), |n| {
            n.name.clone()
        }))
    }

    pub async fn node(&self, name: &str) -> AppResult<ClusterNodeDetail> {
        let n: Node = self
            .get(self.cluster_api(), ResourceKind::Node, None, name)
            .await?;
        Ok(nodes::detail(&n))
    }

    /// The Events in `namespace` (every one with `None`), warnings first.
    pub async fn events(&self, namespace: Option<&str>) -> AppResult<Vec<EventView>> {
        let items: Vec<Event> = self
            .list(
                self.api(namespace),
                ResourceKind::Event,
                namespace,
                ListParams::default(),
            )
            .await?;
        let mut views: Vec<_> = items.iter().map(events::view).collect();
        events::sort(&mut views);
        Ok(views)
    }

    /// The Events about one object, warnings first.
    pub async fn object_events(
        &self,
        kind: ResourceKind,
        namespace: Option<&str>,
        name: &str,
    ) -> AppResult<Vec<EventView>> {
        let fields = format!(
            "involvedObject.kind={},involvedObject.name={name}",
            kind.kind()
        );
        let items: Vec<Event> = self
            .list(
                self.api(namespace),
                ResourceKind::Event,
                namespace,
                ListParams::default().fields(&fields),
            )
            .await?;
        let mut views: Vec<_> = items.iter().map(events::view).collect();
        events::sort(&mut views);
        Ok(views)
    }

    /// An object as YAML, without `managedFields`; a Secret's values are
    /// replaced, since they're only ever revealed one at a time.
    pub async fn yaml(
        &self,
        kind: ResourceKind,
        namespace: Option<&str>,
        name: &str,
    ) -> AppResult<String> {
        use serde_json::to_value as json;
        let ns = namespace;
        let mut value = match kind {
            ResourceKind::Namespace => json(
                self.get::<Namespace>(self.cluster_api(), kind, None, name)
                    .await?,
            ),
            ResourceKind::Node => json(
                self.get::<Node>(self.cluster_api(), kind, None, name)
                    .await?,
            ),
            ResourceKind::Deployment => {
                json(self.get::<Deployment>(self.api(ns), kind, ns, name).await?)
            }
            ResourceKind::StatefulSet => json(
                self.get::<StatefulSet>(self.api(ns), kind, ns, name)
                    .await?,
            ),
            ResourceKind::DaemonSet => {
                json(self.get::<DaemonSet>(self.api(ns), kind, ns, name).await?)
            }
            ResourceKind::Job => json(self.get::<Job>(self.api(ns), kind, ns, name).await?),
            ResourceKind::CronJob => json(self.get::<CronJob>(self.api(ns), kind, ns, name).await?),
            ResourceKind::Pod => json(self.get::<Pod>(self.api(ns), kind, ns, name).await?),
            ResourceKind::Service => json(self.get::<Service>(self.api(ns), kind, ns, name).await?),
            ResourceKind::Ingress => json(self.get::<Ingress>(self.api(ns), kind, ns, name).await?),
            ResourceKind::ConfigMap => {
                json(self.get::<ConfigMap>(self.api(ns), kind, ns, name).await?)
            }
            ResourceKind::Secret => json(self.get::<Secret>(self.api(ns), kind, ns, name).await?),
            ResourceKind::Event => json(self.get::<Event>(self.api(ns), kind, ns, name).await?),
        }?;
        tidy(&mut value);
        serde_yaml_ng::to_string(&value).map_err(|e| {
            AppError::new(ErrorCode::Json, homeport_core::messages::UNREADABLE_DATA).with_cause(&e)
        })
    }
}

fn sorted<T, K: Ord>(mut items: Vec<T>, key: impl Fn(&T) -> K) -> Vec<T> {
    items.sort_by_key(key);
    items
}

/// Drops `managedFields` and kubectl's last applied configuration, and
/// masks a Secret's values.
pub fn tidy(value: &mut serde_json::Value) {
    strip(value);
    if value.get("kind").and_then(|k| k.as_str()) == Some("Secret") {
        for field in ["data", "stringData"] {
            if let Some(data) = value.get_mut(field).and_then(|d| d.as_object_mut()) {
                for v in data.values_mut() {
                    *v = serde_json::Value::String(HIDDEN.into());
                }
            }
        }
    }
}

/// Drops `managedFields` and kubectl's last applied configuration, which
/// repeat what the object holds.
pub fn strip(value: &mut serde_json::Value) {
    if let Some(meta) = value.get_mut("metadata").and_then(|m| m.as_object_mut()) {
        meta.remove("managedFields");
        if let Some(annotations) = meta.get_mut("annotations").and_then(|a| a.as_object_mut()) {
            annotations.remove("kubectl.kubernetes.io/last-applied-configuration");
            if annotations.is_empty() {
                meta.remove("annotations");
            }
        }
    }
}

/// What a Secret's value shows as until it is revealed.
pub const HIDDEN: &str = "(hidden)";

/// The credentials aren't allowed `verb` on `kind`.
pub fn forbidden(verb: &str, kind: ResourceKind, namespace: Option<&str>) -> AppError {
    forbidden_on(verb, &kind.api_type(), namespace)
}

/// The credentials aren't allowed `verb` on the objects of `api`.
pub fn forbidden_on(verb: &str, api: &ApiType, namespace: Option<&str>) -> AppError {
    // kubectl knows the built-in kinds by their plural; others need their group.
    let resource = if api.group.is_empty() || BUILT_IN_GROUPS.contains(&api.group.as_str()) {
        api.plural.clone()
    } else {
        api.resource()
    };
    let (message, command) = match (api.namespaced, namespace) {
        (true, Some(ns)) => (
            msg::FORBIDDEN_IN
                .with("verb", verb)
                .with("resource", &resource)
                .with("namespace", ns),
            format!("kubectl auth can-i {verb} {resource} --namespace {ns}"),
        ),
        (true, None) => (
            msg::FORBIDDEN_EVERYWHERE
                .with("verb", verb)
                .with("resource", &resource),
            format!("kubectl auth can-i {verb} {resource} --all-namespaces"),
        ),
        (false, _) => (
            msg::FORBIDDEN
                .with("verb", verb)
                .with("resource", &resource),
            format!("kubectl auth can-i {verb} {resource}"),
        ),
    };
    let hint = if api.namespaced && namespace.is_none() {
        msg::FORBIDDEN_EVERYWHERE_HINT
    } else {
        msg::FORBIDDEN_HINT
    };
    AppError::new(ErrorCode::Forbidden, message)
        .with_hint(hint)
        .with_command(command)
}

/// The API groups of Kubernetes itself, whose resources kubectl knows by plural.
const BUILT_IN_GROUPS: &[&str] = &[
    "apps",
    "batch",
    "autoscaling",
    "networking.k8s.io",
    "policy",
    "rbac.authorization.k8s.io",
    "storage.k8s.io",
];

#[cfg(test)]
mod tests;
