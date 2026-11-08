//! The use cases the app exposes, keeping the store and the live
//! connections in step.

use std::sync::Arc;

use serde::Serialize;

use homeport_core::error::{AppError, AppResult};

use crate::{
    kubeconfig::{self, ContextChoice, KubeContexts, Source},
    messages as msg,
    model::{Cluster, ClusterKind},
    registry::{ClusterRegistry, ClusterStatus},
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
}

impl ClusterService {
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
        self.registry.restart(id);
        Ok(self.view(cluster))
    }

    /// Forgets a Cluster and its credentials. Nothing changes on the Cluster.
    pub fn remove(&self, id: &str) -> AppResult<()> {
        self.store.delete(id)?;
        self.registry.remove(id);
        Ok(())
    }

    pub fn reconnect(&self, id: &str) -> AppResult<ClusterView> {
        let cluster = self.store.get(id)?;
        self.registry.restart(id);
        Ok(self.view(cluster))
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
                kind: ClusterKind::Kubernetes,
                connection,
                secrets,
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
        let Some(origin) = &cluster.connection.source else {
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
        let cluster = self.store.replace_connection(id, &connection, &secrets)?;
        self.registry.restart(id);
        Ok(self.view(cluster))
    }
}

#[cfg(test)]
mod tests {
    use homeport_core::error::ErrorCode;
    use homeport_storage::{Db, SecretBox};

    use super::*;
    use crate::registry::ClusterNotifier;

    struct Silent;
    impl ClusterNotifier for Silent {
        fn cluster_status(&self, _: &str, _: &ClusterStatus) {}
    }

    fn service() -> ClusterService {
        let db = Arc::new(Db::open_in_memory().unwrap());
        let store = Arc::new(ClusterStore::new(
            db,
            Arc::new(SecretBox::from_key(&[5; 32])),
        ));
        ClusterService {
            registry: ClusterRegistry::new(store.clone(), Arc::new(Silent)),
            store,
        }
    }

    fn kubeconfig(server: &str) -> String {
        format!(
            "clusters:\n- name: dev\n  cluster: {{ server: '{server}' }}\ncontexts:\n- name: kind-dev\n  context: {{ cluster: dev, user: me }}\n- name: other\n  context: {{ cluster: dev, user: me }}\nusers:\n- name: me\n  user: {{ token: t }}\n"
        )
    }

    fn pick(context: &str, name: &str) -> ContextChoice {
        ContextChoice {
            context: context.into(),
            name: name.into(),
        }
    }

    #[tokio::test]
    async fn importing_adds_each_context_and_lists_it_as_added() {
        let svc = service();
        let text = kubeconfig("https://127.0.0.1:1");
        let added = svc
            .import(&[pick("kind-dev", " Dev ")], Some(&text))
            .unwrap();
        assert_eq!(added[0].cluster.name, "Dev");
        let listed = svc.kubeconfig_contexts(Some(&text)).unwrap();
        let dev = listed
            .contexts
            .iter()
            .find(|c| c.name == "kind-dev")
            .unwrap();
        assert_eq!(dev.added.as_ref().unwrap().id, added[0].cluster.id);
    }

    #[tokio::test]
    async fn nothing_is_added_when_a_choice_is_wrong() {
        let svc = service();
        let text = kubeconfig("https://127.0.0.1:1");
        assert!(svc.import(&[], Some(&text)).is_err());
        let e = svc
            .import(
                &[pick("kind-dev", "dev"), pick("other", "DEV")],
                Some(&text),
            )
            .unwrap_err();
        assert!(e.is(ErrorCode::Validation));
        let e = svc
            .import(&[pick("kind-dev", "dev"), pick("gone", "x")], Some(&text))
            .unwrap_err();
        assert!(e.is(ErrorCode::NotFound));
        assert!(svc.list().unwrap().is_empty());

        svc.import(&[pick("kind-dev", "dev")], Some(&text)).unwrap();
        let e = svc
            .import(&[pick("other", "Dev")], Some(&text))
            .unwrap_err();
        assert!(e.is(ErrorCode::NameInUse));
    }

    #[tokio::test]
    async fn refreshing_rereads_the_context_and_keeps_the_name() {
        let svc = service();
        let id = svc
            .import(
                &[pick("kind-dev", "dev")],
                Some(&kubeconfig("https://127.0.0.1:1")),
            )
            .unwrap()[0]
            .cluster
            .id
            .clone();
        let e = svc.refresh(&id, None).unwrap_err();
        assert_eq!(e.key, "errors.clusters.kubeconfig.notRefreshable");
        let next = svc
            .refresh(&id, Some(&kubeconfig("https://127.0.0.1:2")))
            .unwrap();
        assert_eq!(next.cluster.name, "dev");
        assert_eq!(next.cluster.connection.server, "https://127.0.0.1:2");
    }
}
