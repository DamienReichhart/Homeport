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
