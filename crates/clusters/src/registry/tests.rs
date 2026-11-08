use std::{sync::Mutex, time::Duration};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

use homeport_storage::{Db, SecretBox};

use super::*;
use crate::{
    model::{ClusterConnection, ClusterKind},
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
            kind: ClusterKind::Kubernetes,
            connection: ClusterConnection {
                server: server.into(),
                ..Default::default()
            },
            secrets: ClusterSecrets {
                token: Some("t".into()),
                ..Default::default()
            },
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
            platform: "linux/arm64".into(),
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
