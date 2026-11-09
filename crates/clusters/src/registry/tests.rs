use std::{sync::Mutex, time::Duration};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

use homeport_storage::{Db, SecretBox};

use super::*;
use crate::{
    model::{ClusterConnection, KubernetesConnection, KubernetesSecrets},
    store::ClusterDraft,
};

#[derive(Default)]
struct Recorder(Mutex<Vec<(String, ClusterStatus)>>);

impl ClusterNotifier for Recorder {
    fn cluster_status(&self, id: &str, status: &ClusterStatus) {
        self.0.lock().unwrap().push((id.to_owned(), status.clone()));
    }
}

/// An API server answering every request with `status` and `body`.
async fn fake_api(status: &'static str, body: &'static str) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        loop {
            let Ok((mut socket, _)) = listener.accept().await else {
                return;
            };
            tokio::spawn(async move {
                let mut buf = [0; 4096];
                while let Ok(n) = socket.read(&mut buf).await {
                    if n == 0 {
                        return;
                    }
                    let reply = format!(
                        "HTTP/1.1 {status}\r\ncontent-type: application/json\r\ncontent-length: {}\r\n\r\n{body}",
                        body.len()
                    );
                    if socket.write_all(reply.as_bytes()).await.is_err() {
                        return;
                    }
                }
            });
        }
    });
    format!("http://{addr}")
}

fn registry(server: &str) -> (Arc<ClusterRegistry>, Arc<Recorder>, String) {
    let db = Arc::new(Db::open_in_memory().unwrap());
    let store = Arc::new(ClusterStore::new(
        db,
        Arc::new(SecretBox::from_key(&[3; 32])),
    ));
    let id = store
        .create(ClusterDraft {
            name: "kind-dev".into(),
            connection: ClusterConnection::Kubernetes(KubernetesConnection {
                server: server.into(),
                ..Default::default()
            }),
            secrets: ClusterSecrets::Kubernetes(KubernetesSecrets {
                token: Some("t".into()),
                ..Default::default()
            }),
        })
        .unwrap()
        .id;
    let recorder = Arc::new(Recorder::default());
    (ClusterRegistry::new(store, recorder.clone()), recorder, id)
}

async fn settled(registry: &ClusterRegistry, id: &str) -> ClusterStatus {
    for _ in 0..100 {
        let status = registry.status(id);
        if !matches!(status, ClusterStatus::Connecting) {
            return status;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    registry.status(id)
}

const VERSION: &str = r#"{"major":"1","minor":"33","gitVersion":"v1.33.1","gitCommit":"","gitTreeState":"","buildDate":"","goVersion":"","compiler":"","platform":"linux/arm64"}"#;

#[tokio::test]
async fn a_cluster_that_answers_is_connected() {
    let server = fake_api("200 OK", VERSION).await;
    let (registry, recorder, id) = registry(&server);
    registry.start_all().unwrap();
    assert_eq!(
        settled(&registry, &id).await,
        ClusterStatus::Connected {
            server_version: "v1.33.1".into(),
            platform: Some("linux/arm64".into()),
            region: None,
        }
    );
    assert!(registry.client(&id).is_ok());
    assert!(recorder
        .0
        .lock()
        .unwrap()
        .iter()
        .any(|(c, s)| c == &id && s.is_connected()));
}

#[tokio::test]
async fn kubernetes_commands_refuse_a_cluster_of_another_kind() {
    let server = fake_api("200 OK", VERSION).await;
    let (registry, _, kubernetes) = registry(&server);
    let nomad = registry
        .store
        .create(ClusterDraft {
            name: "nomad-dev".into(),
            connection: ClusterConnection::Nomad(crate::model::NomadConnection {
                address: "http://127.0.0.1:4646".into(),
                ..Default::default()
            }),
            secrets: ClusterSecrets::none(ClusterKind::Nomad),
        })
        .unwrap()
        .id;
    registry.start_all().unwrap();
    assert!(settled(&registry, &kubernetes).await.is_connected());
    let Err(e) = registry.client(&nomad) else {
        panic!("a Nomad Cluster gave a kube-rs client");
    };
    assert!(e.is(ErrorCode::Validation), "{e:?}");
    assert!(e.message.contains("nomad-dev") && e.message.contains("Nomad"));
}

#[tokio::test]
async fn refused_credentials_wait_for_the_user() {
    let server = fake_api(
        "401 Unauthorized",
        r#"{"kind":"Status","apiVersion":"v1","status":"Failure","message":"Unauthorized","reason":"Unauthorized","code":401}"#,
    )
    .await;
    let (registry, _, id) = registry(&server);
    registry.start_all().unwrap();
    let status = settled(&registry, &id).await;
    let ClusterStatus::Unauthorized { error } = status else {
        panic!("{status:?}");
    };
    assert!(error.is(ErrorCode::ClusterUnauthorized));
    let Err(e) = registry.client(&id) else {
        panic!("connected");
    };
    assert!(e.is(ErrorCode::ClusterNotConnected));
    assert!(e.hint.is_some(), "carries the cause's hint");
}

#[tokio::test]
async fn an_unreachable_cluster_is_retried() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let server = format!("http://{}", listener.local_addr().unwrap());
    drop(listener);
    let (registry, _, id) = registry(&server);
    registry.start_all().unwrap();
    let status = settled(&registry, &id).await;
    let ClusterStatus::Unreachable { error, .. } = status else {
        panic!("{status:?}");
    };
    assert!(error.is(ErrorCode::ConnectionRefused), "{error:?}");
}

#[tokio::test]
async fn a_disabled_cluster_isnt_connected() {
    let (registry, _, id) = registry("https://127.0.0.1:1");
    registry.store.set_enabled(&id, false).unwrap();
    registry.start_all().unwrap();
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert_eq!(registry.status(&id), ClusterStatus::Disabled);
    let Err(e) = registry.client(&id) else {
        panic!("connected");
    };
    assert_eq!(
        e.hint.unwrap().key,
        "errors.clusters.notConnected.hintDisabled"
    );
}

#[cfg(unix)]
mod exec_plugins {
    use std::os::unix::fs::PermissionsExt;

    use super::*;
    use crate::model::ExecConfig;

    /// A plugin that counts its runs in `runs` and issues a token expiring at `expiry`.
    fn counting_plugin(dir: &std::path::Path, expiry: &str) -> ExecConfig {
        let script = dir.join("plugin");
        let runs = dir.join("runs");
        std::fs::write(
            &script,
            format!(
                "#!/bin/sh\necho run >> '{}'\necho '{{\"kind\":\"ExecCredential\",\"status\":{{\"token\":\"t\",\"expirationTimestamp\":\"{expiry}\"}}}}'\n",
                runs.display()
            ),
        )
        .unwrap();
        std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755)).unwrap();
        ExecConfig {
            api_version: "client.authentication.k8s.io/v1".into(),
            command: script.display().to_string(),
            ..Default::default()
        }
    }

    fn runs(dir: &std::path::Path) -> usize {
        std::fs::read_to_string(dir.join("runs"))
            .map(|s| s.lines().count())
            .unwrap_or(0)
    }

    fn with_plugin(server: &str, exec: ExecConfig) -> (Arc<ClusterRegistry>, String) {
        let (registry, _, id) = registry(server);
        let cluster = registry.store.get(&id).unwrap();
        let mut connection = cluster.kubernetes().unwrap().clone();
        connection.auth = crate::AuthMethod::Exec {
            command: exec.command.clone(),
        };
        let secrets = ClusterSecrets::Kubernetes(KubernetesSecrets {
            exec: Some(exec),
            ..Default::default()
        });
        registry
            .store
            .replace_connection(&id, &ClusterConnection::Kubernetes(connection), &secrets)
            .unwrap();
        (registry, id)
    }

    #[tokio::test]
    async fn the_plugin_runs_once_per_token_lifetime() {
        let dir = tempfile::tempdir().unwrap();
        let server = fake_api("200 OK", VERSION).await;
        let (registry, id) =
            with_plugin(&server, counting_plugin(dir.path(), "2099-01-01T00:00:00Z"));
        registry.start_all().unwrap();
        assert!(settled(&registry, &id).await.is_connected());
        assert_eq!(runs(dir.path()), 1);

        let cluster = registry.store.get(&id).unwrap();
        let secrets = kubernetes_secrets(&registry, &id);
        registry
            .credentials(&cluster, &secrets, false)
            .await
            .unwrap();
        assert!(registry
            .renewed(&cluster, &secrets)
            .await
            .unwrap()
            .is_none());
        assert_eq!(runs(dir.path()), 1, "reused before it expires");
    }

    #[tokio::test]
    async fn a_token_about_to_expire_is_renewed() {
        let dir = tempfile::tempdir().unwrap();
        let server = fake_api("200 OK", VERSION).await;
        let (registry, id) =
            with_plugin(&server, counting_plugin(dir.path(), "2000-01-01T00:00:00Z"));
        let cluster = registry.store.get(&id).unwrap();
        let secrets = kubernetes_secrets(&registry, &id);
        registry
            .credentials(&cluster, &secrets, false)
            .await
            .unwrap();
        registry
            .credentials(&cluster, &secrets, false)
            .await
            .unwrap();
        assert_eq!(runs(dir.path()), 2, "an expired token is asked for again");
        assert!(registry
            .renewed(&cluster, &secrets)
            .await
            .unwrap()
            .is_some());
        assert_eq!(runs(dir.path()), 3);
    }

    #[tokio::test]
    async fn a_refused_token_is_asked_for_once_more() {
        let dir = tempfile::tempdir().unwrap();
        let server = fake_api(
            "401 Unauthorized",
            r#"{"kind":"Status","apiVersion":"v1","status":"Failure","message":"Unauthorized","reason":"Unauthorized","code":401}"#,
        )
        .await;
        let (registry, id) =
            with_plugin(&server, counting_plugin(dir.path(), "2099-01-01T00:00:00Z"));
        registry.start_all().unwrap();
        let status = settled(&registry, &id).await;
        assert!(
            matches!(status, ClusterStatus::Unauthorized { .. }),
            "{status:?}"
        );
        assert_eq!(runs(dir.path()), 2);
    }

    #[tokio::test]
    async fn a_missing_plugin_waits_for_the_user() {
        let (registry, id) = with_plugin(
            "https://127.0.0.1:1",
            ExecConfig {
                command: "homeport-no-such-plugin".into(),
                ..Default::default()
            },
        );
        registry.start_all().unwrap();
        let status = settled(&registry, &id).await;
        let ClusterStatus::Invalid { error } = status else {
            panic!("{status:?}");
        };
        assert!(error.is(ErrorCode::ExecPluginMissing));
    }
}

/// The Kubernetes credentials stored for `id`, for the exec plugins' tests.
#[cfg(unix)]
fn kubernetes_secrets(registry: &ClusterRegistry, id: &str) -> KubernetesSecrets {
    match registry.store.secrets(id).unwrap() {
        ClusterSecrets::Kubernetes(s) => s,
        ClusterSecrets::Nomad(_) => panic!("not a Kubernetes Cluster"),
    }
}
