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
        ClusterService::new(store, Arc::new(Silent))
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

    /// A Nomad agent answering its status endpoints, and 403 to an ACL
    /// token other than `token`.
    async fn fake_nomad(token: &'static str) -> String {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = format!("http://{}", listener.local_addr().unwrap());
        tokio::spawn(async move {
            while let Ok((mut socket, _)) = listener.accept().await {
                tokio::spawn(async move {
                    let mut buf = [0; 8192];
                    while let Ok(n) = socket.read(&mut buf).await {
                        if n == 0 {
                            return;
                        }
                        let head = String::from_utf8_lossy(&buf[..n]).to_lowercase();
                        let path = head.split(' ').nth(1).unwrap_or_default().to_owned();
                        let authorised = head.contains(&format!("x-nomad-token: {token}"));
                        let (status, body) = if path.starts_with("/v1/status/leader") {
                            ("200 OK", r#""10.0.0.5:4647""#)
                        } else if !authorised {
                            ("403 Forbidden", "ACL token not found")
                        } else if path.starts_with("/v1/agent/self") {
                            (
                                "200 OK",
                                r#"{"config":{"Region":"global","Version":{"Version":"1.10.3"}}}"#,
                            )
                        } else {
                            ("200 OK", "{}")
                        };
                        let reply = format!(
                            "HTTP/1.1 {status}\r\ncontent-length: {}\r\n\r\n{body}",
                            body.len()
                        );
                        if socket.write_all(reply.as_bytes()).await.is_err() {
                            return;
                        }
                    }
                });
            }
        });
        address
    }

    fn nomad_form(address: &str, token: Option<&str>) -> NomadForm {
        NomadForm {
            name: "nomad-dev".into(),
            address: address.into(),
            token: token.map(str::to_owned),
            ..Default::default()
        }
    }

    async fn settled(svc: &ClusterService, id: &str) -> ClusterStatus {
        for _ in 0..100 {
            let status = svc.registry.status(id);
            if !matches!(status, ClusterStatus::Connecting) {
                return status;
            }
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
        svc.registry.status(id)
    }

    #[tokio::test]
    async fn a_nomad_cluster_connects_with_its_version_and_region() {
        let svc = service();
        let address = fake_nomad("good").await;
        let view = svc.add_nomad(&nomad_form(&address, Some("good"))).unwrap();
        assert_eq!(view.cluster.kind, crate::ClusterKind::Nomad);
        assert_eq!(
            settled(&svc, &view.cluster.id).await,
            ClusterStatus::Connected {
                server_version: "1.10.3".into(),
                platform: None,
                region: Some("global".into()),
            }
        );
        assert!(svc.registry.nomad(&view.cluster.id).is_ok());
        let Err(e) = svc.registry.client(&view.cluster.id) else {
            panic!("a Nomad Cluster gave a kube-rs client");
        };
        assert!(e.is(ErrorCode::Validation));
        assert!(svc
            .refresh(&view.cluster.id, None)
            .unwrap_err()
            .is(ErrorCode::Validation));
    }

    #[tokio::test]
    async fn a_wrong_token_waits_for_the_user() {
        let svc = service();
        let address = fake_nomad("good").await;
        let view = svc.add_nomad(&nomad_form(&address, Some("bad"))).unwrap();
        let status = settled(&svc, &view.cluster.id).await;
        let ClusterStatus::Unauthorized { error } = status else {
            panic!("{status:?}");
        };
        assert!(error.is(ErrorCode::NomadTokenInvalid));

        // Testing the fix before saving it, then saving it, connects.
        let fixed = nomad_form(&address, Some("good"));
        assert!(svc
            .test_nomad(&nomad_form(&address, None), Some(&view.cluster.id))
            .await
            .unwrap_err()
            .is(ErrorCode::NomadTokenInvalid));
        let agent = svc
            .test_nomad(&fixed, Some(&view.cluster.id))
            .await
            .unwrap();
        assert_eq!(agent.version.as_deref(), Some("1.10.3"));
        svc.update_nomad(&view.cluster.id, &fixed).unwrap();
        assert!(settled(&svc, &view.cluster.id).await.is_connected());
    }

    #[tokio::test]
    async fn editing_a_nomad_cluster_keeps_its_token_and_renames_it() {
        let svc = service();
        let view = svc
            .add_nomad(&nomad_form("http://127.0.0.1:1", Some("t0ken")))
            .unwrap();
        let edited = svc
            .update_nomad(
                &view.cluster.id,
                &NomadForm {
                    name: "nomad-prod".into(),
                    ..nomad_form("127.0.0.1:2", None)
                },
            )
            .unwrap();
        assert_eq!(edited.cluster.name, "nomad-prod");
        assert_eq!(edited.cluster.address(), "http://127.0.0.1:2");
        assert!(edited.cluster.nomad().unwrap().acl_token);
        assert_eq!(
            svc.nomad_secrets(&view.cluster.id)
                .unwrap()
                .token
                .as_deref(),
            Some("t0ken")
        );
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
        assert_eq!(next.cluster.address(), "https://127.0.0.1:2");
    }
}
