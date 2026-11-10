//! The use cases the app exposes, keeping the store and the live
//! connections in step.

use std::sync::Arc;

use serde::Serialize;

use homeport_core::error::{AppError, AppResult};

use crate::{
    forward::PortForwards,
    kubeconfig::{self, ContextChoice, KubeContexts, Source},
    messages as msg,
    model::{Cluster, ClusterConnection, ClusterSecrets, NomadSecrets},
    nomad::NomadForm,
    proxy::ClusterAccess,
    registry::{ClusterNotifier, ClusterRegistry, ClusterStatus},
    store::{ClusterDraft, ClusterStore},
};

/// A Cluster and its connection's status, as the frontend lists them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ClusterView {
    pub cluster: Cluster,
    pub status: ClusterStatus,
}

#[derive(Clone)]
pub struct ClusterService {
    pub store: Arc<ClusterStore>,
    pub registry: Arc<ClusterRegistry>,
    /// The port forwards to every Cluster, in memory only.
    pub forwards: Arc<PortForwards>,
}

impl ClusterService {
    /// The service over `store`, reporting through `notifier`.
    pub fn new(store: Arc<ClusterStore>, notifier: Arc<dyn ClusterNotifier>) -> Self {
        Self {
            registry: ClusterRegistry::new(store.clone(), notifier.clone()),
            forwards: PortForwards::new(notifier),
            store,
        }
    }

    pub fn view(&self, cluster: Cluster) -> ClusterView {
        let status = self.registry.status(&cluster.id);
        ClusterView { cluster, status }
    }

    pub fn list(&self) -> AppResult<Vec<ClusterView>> {
        Ok(self
            .store
            .list()?
            .into_iter()
            .map(|c| self.view(c))
            .collect())
    }

    pub fn get(&self, id: &str) -> AppResult<ClusterView> {
        Ok(self.view(self.store.get(id)?))
    }

    pub fn rename(&self, id: &str, name: &str) -> AppResult<ClusterView> {
        let cluster = self.store.rename(id, name)?;
        Ok(self.view(cluster))
    }

    /// Enables or disables a Cluster; its supervisor connects or stops.
    pub fn set_enabled(&self, id: &str, enabled: bool) -> AppResult<ClusterView> {
        let cluster = self.store.set_enabled(id, enabled)?;
        if !enabled {
            self.forwards.stop_cluster(id);
        }
        self.registry.restart(id);
        Ok(self.view(cluster))
    }

    /// Forgets a Cluster and its credentials. Nothing changes on the Cluster.
    pub fn remove(&self, id: &str) -> AppResult<()> {
        self.store.delete(id)?;
        self.forwards.stop_cluster(id);
        self.registry.remove(id);
        Ok(())
    }

    /// What a CLI needs to reach the connected Cluster `id` through a
    /// private proxy (ADR 0009): the Cluster, and its supervisor's current
    /// client on each request, so renewed exec-plugin tokens are used.
    pub fn access(&self, id: &str) -> AppResult<ClusterAccess> {
        let (_, cluster) = self.registry.client(id)?;
        let registry = self.registry.clone();
        let id = id.to_owned();
        Ok(ClusterAccess {
            cluster,
            clients: Arc::new(move || registry.client(&id).map(|(client, _)| client)),
        })
    }

    pub fn reconnect(&self, id: &str) -> AppResult<ClusterView> {
        let cluster = self.store.get(id)?;
        self.registry.restart(id);
        Ok(self.view(cluster))
    }

    /// Adds the Nomad Cluster `form` describes, its credentials sealed, and
    /// starts connecting to it.
    pub fn add_nomad(&self, form: &NomadForm) -> AppResult<ClusterView> {
        let (connection, secrets) = form.draft(None)?;
        let cluster = self.store.create(ClusterDraft {
            name: form.name()?.to_owned(),
            connection: ClusterConnection::Nomad(connection),
            secrets: ClusterSecrets::Nomad(secrets),
        })?;
        self.registry.restart(&cluster.id);
        Ok(self.view(cluster))
    }

    /// Changes a Nomad Cluster's name, address and credentials; those the
    /// form leaves out are kept. Reconnects.
    pub fn update_nomad(&self, id: &str, form: &NomadForm) -> AppResult<ClusterView> {
        let cluster = self.store.get(id)?;
        cluster.nomad_or_refuse()?;
        let stored = self.nomad_secrets(id)?;
        let (connection, secrets) = form.draft(Some(&stored))?;
        if form.name()? != cluster.name {
            self.store.rename(id, form.name()?)?;
        }
        let cluster = self.store.replace_connection(
            id,
            &ClusterConnection::Nomad(connection),
            &ClusterSecrets::Nomad(secrets),
        )?;
        self.registry.restart(id);
        Ok(self.view(cluster))
    }

    /// Connects once to the Nomad Cluster `form` describes, without saving
    /// it: its status, or why it can't be reached. When editing the Cluster
    /// `id`, the credentials the form leaves out are its stored ones.
    pub async fn test_nomad(
        &self,
        form: &NomadForm,
        id: Option<&str>,
    ) -> AppResult<homeport_nomad::NomadAgent> {
        let stored = match id {
            Some(id) => Some(self.nomad_secrets(id)?),
            None => None,
        };
        let (connection, secrets) = form.draft(stored.as_ref())?;
        let name = form.name().unwrap_or(&connection.address).to_owned();
        crate::nomad::client(&name, &connection, &secrets)?
            .agent()
            .await
    }

    /// The stored credentials of the Nomad Cluster `id`.
    fn nomad_secrets(&self, id: &str) -> AppResult<NomadSecrets> {
        match self.store.secrets(id)? {
            ClusterSecrets::Nomad(secrets) => Ok(secrets),
            ClusterSecrets::Kubernetes(_) => Err(self.store.get(id)?.wrong_kind()),
        }
    }

    /// The contexts of the pasted kubeconfig `text`, or of this machine's.
    pub fn kubeconfig_contexts(&self, text: Option<&str>) -> AppResult<KubeContexts> {
        let source = Source::of(text)?;
        Ok(kubeconfig::contexts(&source, &self.store.list()?))
    }

    /// Makes a Cluster of each context picked, under the name given, and
    /// starts connecting to them. Every context is checked before any is added.
    pub fn import(
        &self,
        choices: &[ContextChoice],
        text: Option<&str>,
    ) -> AppResult<Vec<ClusterView>> {
        if choices.is_empty() {
            return Err(AppError::validation(msg::IMPORT_NOTHING));
        }
        let source = Source::of(text)?;
        let mut drafts = Vec::with_capacity(choices.len());
        for choice in choices {
            let name = choice.name.trim();
            if drafts
                .iter()
                .any(|d: &ClusterDraft| d.name.eq_ignore_ascii_case(name))
            {
                return Err(AppError::validation(
                    msg::IMPORT_SAME_NAME.with("name", name),
                ));
            }
            let (connection, secrets) = kubeconfig::resolve(&source, &choice.context)?;
            drafts.push(ClusterDraft {
                name: name.to_owned(),
                connection: ClusterConnection::Kubernetes(connection),
                secrets: ClusterSecrets::Kubernetes(secrets),
            });
        }
        let existing = self.store.list()?;
        for draft in &drafts {
            if existing
                .iter()
                .any(|c| c.name.eq_ignore_ascii_case(&draft.name))
            {
                return Err(AppError::new(
                    homeport_core::error::ErrorCode::NameInUse,
                    msg::NAME_IN_USE.with("name", &draft.name),
                )
                .with_hint(msg::NAME_IN_USE_HINT));
            }
        }
        let mut added = Vec::with_capacity(drafts.len());
        for draft in drafts {
            let cluster = self.store.create(draft)?;
            self.registry.restart(&cluster.id);
            added.push(self.view(cluster));
        }
        Ok(added)
    }

    /// Reads a Cluster's context again, from the pasted kubeconfig `text`
    /// or this machine's, as after kind or k3d recreated it with new
    /// certificates; keeps its name and reconnects.
    pub fn refresh(&self, id: &str, text: Option<&str>) -> AppResult<ClusterView> {
        let cluster = self.store.get(id)?;
        let Some(origin) = &cluster.kubernetes_or_refuse()?.source else {
            return Err(AppError::validation(
                msg::NO_SOURCE.with("name", &cluster.name),
            ));
        };
        if origin.pasted && text.is_none() {
            return Err(
                AppError::validation(msg::NOT_REFRESHABLE.with("name", &cluster.name))
                    .with_hint(msg::NOT_REFRESHABLE_HINT),
            );
        }
        let source = Source::of(text)?;
        let (connection, secrets) = kubeconfig::resolve(&source, &origin.context)?;
        let cluster = self.store.replace_connection(
            id,
            &ClusterConnection::Kubernetes(connection),
            &ClusterSecrets::Kubernetes(secrets),
        )?;
        self.registry.restart(id);
        Ok(self.view(cluster))
    }
}

#[cfg(test)]
mod tests;
