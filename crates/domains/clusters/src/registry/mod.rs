//! Live connections to every Cluster, each supervised by its own task.
//!
//! A supervisor builds a client of the Cluster's kind from its stored
//! connection and credentials (kube-rs for Kubernetes), asks the API its
//! version, then asks again every 15 s. Transient failures retry with backoff (1 s doubling to 60 s);
//! failures that need the user (refused credentials, lost credentials, an
//! unusable certificate) stop until [`ClusterRegistry::restart`] is called.
mod notifier;
mod status;
mod supervisor;

use std::{
    cell::Cell,
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex, Weak,
    },
    time::SystemTime,
};

use tokio::task::JoinHandle;

use homeport_core::error::{AppError, AppResult, ErrorCode};

use homeport_kubernetes::{
    api::{
        classify::classify,
        client::{self, IssuedCredentials, CONNECT_TIMEOUT},
        credentials::{self, ClusterInfo, Issued},
    },
    messages as kube_msg, KubeCluster, KubernetesSecrets,
};

use crate::{
    messages as msg,
    model::{Cluster, ClusterId, ClusterKind, ClusterSecrets},
    store::ClusterStore,
};

pub use self::{notifier::ClusterNotifier, status::ClusterStatus};

use self::supervisor::supervise;

/// The live client of a connected Cluster, of its kind.
#[derive(Clone)]
pub enum ClusterClient {
    Kubernetes(kube::Client),
    Nomad(homeport_nomad::Nomad),
}

struct Entry {
    status: ClusterStatus,
    client: Option<ClusterClient>,
    task: Option<JoinHandle<()>>,
    /// Which supervisor the entry is: what one started before it says is
    /// let go of.
    generation: u64,
}

thread_local! {
    /// This thread is telling a notifier a status, the entries held.
    static TELLING: Cell<bool> = const { Cell::new(false) };
}

pub struct ClusterRegistry {
    store: Arc<ClusterStore>,
    notifier: Arc<dyn ClusterNotifier>,
    entries: Mutex<HashMap<ClusterId, Entry>>,
    /// What each Cluster's exec plugin last issued, in memory only, until it
    /// is about to expire.
    issued: Mutex<HashMap<ClusterId, Issued>>,
    /// The last supervisor's number, across every Cluster.
    generations: AtomicU64,
    me: Weak<ClusterRegistry>,
}

impl ClusterRegistry {
    pub fn new(store: Arc<ClusterStore>, notifier: Arc<dyn ClusterNotifier>) -> Arc<Self> {
        Arc::new_cyclic(|me| Self {
            store,
            notifier,
            entries: Mutex::new(HashMap::new()),
            issued: Mutex::new(HashMap::new()),
            generations: AtomicU64::new(0),
            me: me.clone(),
        })
    }

    /// Starts supervising every stored Cluster. Must run inside a Tokio runtime.
    pub fn start_all(&self) -> AppResult<()> {
        for cluster in self.store.list()? {
            self.restart(&cluster.id);
        }
        Ok(())
    }

    /// Drops every connection and connects to the stored Clusters afresh, as
    /// after a restore replaced them all. Must run inside a Tokio runtime.
    pub fn reload(&self) -> AppResult<()> {
        let old: Vec<Entry> = self.lock().drain().map(|(_, entry)| entry).collect();
        for task in old.into_iter().filter_map(|entry| entry.task) {
            task.abort();
        }
        self.lock_issued().clear();
        self.start_all()
    }

    /// (Re)starts a Cluster's supervisor, e.g. after it was edited or
    /// refreshed; its exec plugin runs again.
    pub fn restart(&self, id: &str) {
        // The entry is reset before the task that fills it starts, and the
        // task is numbered: one that was replaced may still be running when
        // it is aborted, until its next await, and what it says then is
        // let go of instead of overwriting the new one's (as for Hosts).
        let generation = self.generations.fetch_add(1, Ordering::Relaxed) + 1;
        let mut entries = self.lock();
        let entry = entries.entry(id.to_owned()).or_insert_with(|| Entry {
            status: ClusterStatus::Connecting,
            client: None,
            task: None,
            generation,
        });
        if let Some(old) = entry.task.take() {
            old.abort();
        }
        // Not the old status, which the edit that restarted it may have fixed.
        entry.status = ClusterStatus::Connecting;
        entry.client = None;
        entry.generation = generation;
        // What the plugin issued is asked again; forgotten with the
        // entries held, so a replaced supervisor's can't come back after.
        self.forget_issued(id);
        // The task waits for the lock before it tells its first status.
        entry.task = Some(tokio::spawn(supervise(
            self.me.clone(),
            id.to_owned(),
            generation,
        )));
    }

    pub fn remove(&self, id: &str) {
        let entry = {
            let mut entries = self.lock();
            self.forget_issued(id);
            entries.remove(id)
        };
        if let Some(task) = entry.and_then(|entry| entry.task) {
            task.abort();
        }
    }

    pub fn status(&self, id: &str) -> ClusterStatus {
        self.lock()
            .get(id)
            .map_or(ClusterStatus::Connecting, |e| e.status.clone())
    }

    /// The kube-rs client of a connected Kubernetes Cluster, and the Cluster
    /// itself for naming it in errors. Refused for a Cluster of another kind.
    pub fn client(&self, id: &str) -> AppResult<(kube::Client, KubeCluster)> {
        match self.live(id, ClusterKind::Kubernetes)? {
            (ClusterClient::Kubernetes(client), cluster) => Ok((client, cluster.kube()?)),
            (ClusterClient::Nomad(_), cluster) => Err(cluster.wrong_kind()),
        }
    }

    /// The client of a connected Nomad Cluster, and the Cluster itself.
    /// Refused for a Cluster of another kind.
    pub fn nomad(&self, id: &str) -> AppResult<(homeport_nomad::Nomad, Cluster)> {
        match self.live(id, ClusterKind::Nomad)? {
            (ClusterClient::Nomad(client), cluster) => Ok((client, cluster)),
            (ClusterClient::Kubernetes(_), cluster) => Err(cluster.wrong_kind()),
        }
    }

    /// The live client of a connected Cluster of `kind`.
    fn live(&self, id: &str, kind: ClusterKind) -> AppResult<(ClusterClient, Cluster)> {
        let cluster = self.store.get(id)?;
        if cluster.kind != kind {
            return Err(cluster.wrong_kind());
        }
        let status = match self.lock().get(id) {
            Some(Entry {
                client: Some(client),
                ..
            }) => return Ok((client.clone(), cluster)),
            Some(entry) => entry.status.clone(),
            None => ClusterStatus::Connecting,
        };
        Err(self.not_connected(id, &status))
    }

    /// Why a command can't reach the Cluster right now, carrying the cause from its status.
    fn not_connected(&self, id: &str, status: &ClusterStatus) -> AppError {
        let message = match self.store.get(id) {
            Ok(cluster) => msg::NOT_CONNECTED.with("name", cluster.name),
            Err(_) => msg::NOT_CONNECTED_UNKNOWN.into(),
        };
        let error = AppError::new(ErrorCode::ClusterNotConnected, message);
        if let Some(cause) = status.error() {
            return AppError {
                hint: cause.hint.clone(),
                detail: Some(match &cause.detail {
                    Some(detail) => format!("{}\n{detail}", cause.message),
                    None => cause.message.clone(),
                }),
                ..error
            };
        }
        match status {
            ClusterStatus::Disabled => error.with_hint(msg::NOT_CONNECTED_DISABLED_HINT),
            ClusterStatus::Connecting => error.with_hint(msg::NOT_CONNECTED_CONNECTING_HINT),
            _ => error.with_hint(msg::NOT_CONNECTED_HINT),
        }
    }

    /// Builds a client of `cluster`'s kind and checks that its API answers.
    async fn connect(
        &self,
        cluster: &Cluster,
        secrets: &ClusterSecrets,
        generation: u64,
    ) -> AppResult<(ClusterClient, ClusterStatus)> {
        match secrets {
            ClusterSecrets::Kubernetes(secrets) => {
                let (client, status) = self
                    .connect_kubernetes(cluster, secrets, generation)
                    .await?;
                Ok((ClusterClient::Kubernetes(client), status))
            }
            ClusterSecrets::Nomad(secrets) => {
                let connection = cluster.nomad_or_refuse()?;
                let client = crate::nomad::client(&cluster.name, connection, secrets)?;
                let status = nomad_status(cluster, &client).await?;
                Ok((ClusterClient::Nomad(client), status))
            }
        }
    }

    /// Checks that a connected Cluster still answers: its status, and a
    /// renewed client when its credentials had to be renewed on the way.
    async fn check(
        &self,
        cluster: &Cluster,
        secrets: &ClusterSecrets,
        client: &ClusterClient,
        generation: u64,
    ) -> AppResult<Option<ClusterClient>> {
        match (secrets, client) {
            (ClusterSecrets::Kubernetes(secrets), ClusterClient::Kubernetes(client)) => {
                let renewed = self.renewed(cluster, secrets, generation).await?;
                version(cluster, renewed.as_ref().unwrap_or(client)).await?;
                Ok(renewed.map(ClusterClient::Kubernetes))
            }
            (_, ClusterClient::Nomad(client)) => {
                nomad_status(cluster, client).await?;
                Ok(None)
            }
            (ClusterSecrets::Nomad(_), ClusterClient::Kubernetes(_)) => Err(cluster.wrong_kind()),
        }
    }

    /// Builds a kube-rs client for `cluster` and checks that its API server
    /// answers. Credentials from an exec plugin that the API refuses are
    /// asked for once more before giving up.
    async fn connect_kubernetes(
        &self,
        cluster: &Cluster,
        secrets: &KubernetesSecrets,
        generation: u64,
    ) -> AppResult<(kube::Client, ClusterStatus)> {
        let issued = self
            .credentials(cluster, secrets, false, generation)
            .await?;
        let client = client::client(&cluster.kube()?, secrets, issued.as_ref())?;
        match version(cluster, &client).await {
            Err(e) if e.is(ErrorCode::ClusterUnauthorized) && secrets.exec.is_some() => {
                let issued = self.credentials(cluster, secrets, true, generation).await?;
                let client = client::client(&cluster.kube()?, secrets, issued.as_ref())?;
                let status = version(cluster, &client).await?;
                Ok((client, status))
            }
            answer => answer.map(|status| (client, status)),
        }
    }

    /// A new client when the exec plugin's credentials are about to expire,
    /// `None` while they're good or the Cluster has no plugin.
    async fn renewed(
        &self,
        cluster: &Cluster,
        secrets: &KubernetesSecrets,
        generation: u64,
    ) -> AppResult<Option<kube::Client>> {
        if secrets.exec.is_none() {
            return Ok(None);
        }
        let due = self
            .lock_issued()
            .get(&cluster.id)
            .is_none_or(|i| i.is_due(SystemTime::now()));
        if !due {
            return Ok(None);
        }
        let issued = self.credentials(cluster, secrets, true, generation).await?;
        let client = client::client(&cluster.kube()?, secrets, issued.as_ref())?;
        if let Some(entry) = self
            .lock()
            .get_mut(&cluster.id)
            .filter(|entry| entry.generation == generation)
        {
            entry.client = Some(ClusterClient::Kubernetes(client.clone()));
        }
        Ok(Some(client))
    }

    /// What `cluster`'s exec plugin issued: kept until it is about to expire
    /// (when the supervisor `generation` asking is still the live one),
    /// asked again when `force`. `None` for a Cluster without a plugin.
    async fn credentials(
        &self,
        cluster: &Cluster,
        secrets: &KubernetesSecrets,
        force: bool,
        generation: u64,
    ) -> AppResult<Option<IssuedCredentials>> {
        let Some(plugin) = &secrets.exec else {
            return Ok(None);
        };
        let connection = cluster.kubernetes_or_refuse()?;
        if !force {
            if let Some(issued) = self.lock_issued().get(&cluster.id) {
                if !issued.is_due(SystemTime::now()) {
                    return Ok(Some(issued.credentials.clone()));
                }
            }
        }
        let info = ClusterInfo {
            server: connection.server.clone(),
            tls_server_name: connection.tls_server_name.clone(),
            insecure_skip_tls_verify: connection.insecure_skip_tls_verify,
            certificate_authority: secrets.certificate_authority.clone(),
        };
        let context = connection.source.as_ref().map(|s| s.context.as_str());
        let issued = credentials::run(&cluster.name, context, plugin, &info).await?;
        let credentials = issued.credentials.clone();
        // Kept with the entries held, as a restart forgets them: what a
        // replaced supervisor's plugin issued (from the secrets before an
        // edit) is not kept for the one that replaced it.
        let entries = self.lock();
        if entries
            .get(&cluster.id)
            .is_some_and(|entry| entry.generation == generation)
        {
            self.lock_issued().insert(cluster.id.clone(), issued);
        }
        Ok(Some(credentials))
    }

    fn forget_issued(&self, id: &str) {
        self.lock_issued().remove(id);
    }

    fn lock_issued(&self) -> std::sync::MutexGuard<'_, HashMap<ClusterId, Issued>> {
        self.issued
            .lock()
            .expect("cluster credentials lock poisoned")
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, HashMap<ClusterId, Entry>> {
        debug_assert!(
            !TELLING.with(Cell::get),
            "a notifier called the Clusters registry back while it is told a status: \
             it runs with the registry's entries held, and would wait on them for ever"
        );
        self.entries.lock().expect("cluster registry lock poisoned")
    }

    /// Tells the notifier `id`'s status, with the entries held (see
    /// [`ClusterNotifier`]). A notifier that calls the registry back would
    /// wait on itself for ever: a build with debug assertions panics
    /// instead.
    fn tell(&self, id: &str, status: &ClusterStatus) {
        struct Telling;
        impl Drop for Telling {
            fn drop(&mut self) {
                TELLING.with(|telling| telling.set(false));
            }
        }
        TELLING.with(|telling| telling.set(true));
        let _telling = Telling;
        self.notifier.cluster_status(id, status);
    }

    /// What the supervisor `generation` found: let go of when another has
    /// replaced it since. Told while the entry is held, so what is told
    /// last is what it holds.
    fn set_status(
        &self,
        id: &str,
        generation: u64,
        status: ClusterStatus,
        client: Option<ClusterClient>,
    ) {
        let mut entries = self.lock();
        let Some(entry) = entries.get_mut(id) else {
            return;
        };
        if entry.generation != generation {
            return;
        }
        entry.status = status.clone();
        entry.client = client;
        self.tell(id, &status);
    }

    /// The supervisor `generation` found its Cluster gone from the store:
    /// compared and removed under one lock.
    fn removed(&self, id: &str, generation: u64) {
        let entry = {
            let mut entries = self.lock();
            match entries.get(id) {
                Some(entry) if entry.generation == generation => entries.remove(id),
                _ => None,
            }
        };
        if let Some(task) = entry.and_then(|entry| entry.task) {
            task.abort();
        }
    }
}

/// Asks the API server its version: the Cluster's status when it answers.
async fn version(cluster: &Cluster, client: &kube::Client) -> AppResult<ClusterStatus> {
    let answer = tokio::time::timeout(CONNECT_TIMEOUT, client.apiserver_version()).await;
    let info = match answer {
        Ok(Ok(info)) => info,
        Ok(Err(e)) => return Err(classify(&cluster.name, cluster.address(), &e)),
        Err(elapsed) => {
            return Err(AppError::new(
                ErrorCode::Timeout,
                kube_msg::TIMEOUT.with("name", &cluster.name),
            )
            .with_hint(kube_msg::TIMEOUT_HINT)
            .with_cause(&elapsed))
        }
    };
    Ok(ClusterStatus::Connected {
        server_version: info.git_version,
        platform: Some(info.platform),
        region: None,
    })
}

/// Asks a Nomad Cluster its leader, version and region.
async fn nomad_status(
    cluster: &Cluster,
    client: &homeport_nomad::Nomad,
) -> AppResult<ClusterStatus> {
    let agent = tokio::time::timeout(CONNECT_TIMEOUT + CONNECT_TIMEOUT, client.agent())
        .await
        .map_err(|elapsed| {
            AppError::new(
                ErrorCode::Timeout,
                kube_msg::TIMEOUT.with("name", &cluster.name),
            )
            .with_hint(kube_msg::TIMEOUT_HINT)
            .with_cause(&elapsed)
        })??;
    Ok(ClusterStatus::Connected {
        server_version: agent.version.unwrap_or_default(),
        platform: None,
        region: agent.region,
    })
}

#[cfg(test)]
mod tests;
