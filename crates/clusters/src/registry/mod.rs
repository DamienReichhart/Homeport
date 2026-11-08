//! Live connections to every Cluster, each supervised by its own task.
//!
//! A supervisor builds a kube-rs client from the Cluster's stored connection
//! and credentials, asks the API server its version, then asks again every
//! 15 s. Transient failures retry with backoff (1 s doubling to 60 s);
//! failures that need the user (refused credentials, lost credentials, an
//! unusable certificate) stop until [`ClusterRegistry::restart`] is called.
mod notifier;
mod status;
mod supervisor;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex, Weak},
    time::SystemTime,
};

use tokio::task::JoinHandle;

use homeport_core::error::{AppError, AppResult, ErrorCode};

use crate::{
    classify::classify,
    client::{self, IssuedCredentials, CONNECT_TIMEOUT},
    exec::{self, ClusterInfo, Issued},
    messages as msg,
    model::{Cluster, ClusterId, ClusterSecrets},
    store::ClusterStore,
};

pub use self::{notifier::ClusterNotifier, status::ClusterStatus};

use self::supervisor::supervise;

struct Entry {
    status: ClusterStatus,
    client: Option<kube::Client>,
    task: Option<JoinHandle<()>>,
}

pub struct ClusterRegistry {
    store: Arc<ClusterStore>,
    notifier: Arc<dyn ClusterNotifier>,
    entries: Mutex<HashMap<ClusterId, Entry>>,
    /// What each Cluster's exec plugin last issued, in memory only, until it
    /// is about to expire.
    issued: Mutex<HashMap<ClusterId, Issued>>,
    me: Weak<ClusterRegistry>,
}

impl ClusterRegistry {
    pub fn new(store: Arc<ClusterStore>, notifier: Arc<dyn ClusterNotifier>) -> Arc<Self> {
        Arc::new_cyclic(|me| Self {
            store,
            notifier,
            entries: Mutex::new(HashMap::new()),
            issued: Mutex::new(HashMap::new()),
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

    /// (Re)starts a Cluster's supervisor, e.g. after it was edited or
    /// refreshed; its exec plugin runs again.
    pub fn restart(&self, id: &str) {
        self.forget_issued(id);
        let task = tokio::spawn(supervise(self.me.clone(), id.to_owned()));
        let mut entries = self.lock();
        let entry = entries.entry(id.to_owned()).or_insert_with(|| Entry {
            status: ClusterStatus::Connecting,
            client: None,
            task: None,
        });
        if let Some(old) = entry.task.replace(task) {
            old.abort();
        }
        entry.client = None;
    }

    pub fn remove(&self, id: &str) {
        self.forget_issued(id);
        if let Some(entry) = self.lock().remove(id) {
            if let Some(task) = entry.task {
                task.abort();
            }
        }
    }

    pub fn status(&self, id: &str) -> ClusterStatus {
        self.lock()
            .get(id)
            .map_or(ClusterStatus::Connecting, |e| e.status.clone())
    }

    /// The client of a connected Cluster, and the Cluster itself for naming
    /// it in errors.
    pub fn client(&self, id: &str) -> AppResult<(kube::Client, Cluster)> {
        let status = {
            let entries = self.lock();
            match entries.get(id) {
                Some(Entry {
                    client: Some(client),
                    ..
                }) => {
                    let client = client.clone();
                    drop(entries);
                    return Ok((client, self.store.get(id)?));
                }
                Some(entry) => entry.status.clone(),
                None => ClusterStatus::Connecting,
            }
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

    /// Builds a client for `cluster` and checks that its API server answers.
    /// Credentials from an exec plugin that the API refuses are asked for
    /// once more before giving up.
    async fn connect(
        &self,
        cluster: &Cluster,
        secrets: &ClusterSecrets,
    ) -> AppResult<(kube::Client, ClusterStatus)> {
        let issued = self.credentials(cluster, secrets, false).await?;
        let client = client::client(cluster, secrets, issued.as_ref())?;
        match version(cluster, &client).await {
            Err(e) if e.is(ErrorCode::ClusterUnauthorized) && secrets.exec.is_some() => {
                let issued = self.credentials(cluster, secrets, true).await?;
                let client = client::client(cluster, secrets, issued.as_ref())?;
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
        secrets: &ClusterSecrets,
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
        let issued = self.credentials(cluster, secrets, true).await?;
        let client = client::client(cluster, secrets, issued.as_ref())?;
        if let Some(entry) = self.lock().get_mut(&cluster.id) {
            entry.client = Some(client.clone());
        }
        Ok(Some(client))
    }

    /// What `cluster`'s exec plugin issued: kept until it is about to expire,
    /// asked again when `force`. `None` for a Cluster without a plugin.
    async fn credentials(
        &self,
        cluster: &Cluster,
        secrets: &ClusterSecrets,
        force: bool,
    ) -> AppResult<Option<IssuedCredentials>> {
        let Some(plugin) = &secrets.exec else {
            return Ok(None);
        };
        if !force {
            if let Some(issued) = self.lock_issued().get(&cluster.id) {
                if !issued.is_due(SystemTime::now()) {
                    return Ok(Some(issued.credentials.clone()));
                }
            }
        }
        let info = ClusterInfo {
            server: cluster.connection.server.clone(),
            tls_server_name: cluster.connection.tls_server_name.clone(),
            insecure_skip_tls_verify: cluster.connection.insecure_skip_tls_verify,
            certificate_authority: secrets.certificate_authority.clone(),
        };
        let context = cluster
            .connection
            .source
            .as_ref()
            .map(|s| s.context.as_str());
        let issued = exec::run(&cluster.name, context, plugin, &info).await?;
        let credentials = issued.credentials.clone();
        self.lock_issued().insert(cluster.id.clone(), issued);
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
        self.entries.lock().expect("cluster registry lock poisoned")
    }

    fn set_status(&self, id: &str, status: ClusterStatus, client: Option<kube::Client>) {
        if let Some(entry) = self.lock().get_mut(id) {
            entry.status = status.clone();
            entry.client = client;
        }
        self.notifier.cluster_status(id, &status);
    }
}

/// Asks the API server its version: the Cluster's status when it answers.
async fn version(cluster: &Cluster, client: &kube::Client) -> AppResult<ClusterStatus> {
    let answer = tokio::time::timeout(CONNECT_TIMEOUT, client.apiserver_version()).await;
    let info = match answer {
        Ok(Ok(info)) => info,
        Ok(Err(e)) => return Err(classify(&cluster.name, &cluster.connection.server, &e)),
        Err(elapsed) => {
            return Err(
                AppError::new(ErrorCode::Timeout, msg::TIMEOUT.with("name", &cluster.name))
                    .with_hint(msg::TIMEOUT_HINT)
                    .with_cause(&elapsed),
            )
        }
    };
    Ok(ClusterStatus::Connected {
        server_version: info.git_version,
        platform: info.platform,
    })
}

#[cfg(test)]
mod tests;
