use super::*;
use homeport_kubernetes::{AuthMethod, ExecConfig, KubernetesConnection, KubernetesSecrets};

fn store() -> (Arc<Db>, ClusterStore) {
    let db = Arc::new(Db::open_in_memory().unwrap());
    let store = ClusterStore::new(db.clone(), Arc::new(SecretBox::from_key(&[7; 32])));
    (db, store)
}

fn draft(name: &str) -> ClusterDraft {
    ClusterDraft {
        name: name.into(),
        connection: ClusterConnection::Kubernetes(KubernetesConnection {
            server: "https://127.0.0.1:6443".into(),
            namespace: Some("shop".into()),
            auth: AuthMethod::Token,
            ..Default::default()
        }),
        secrets: ClusterSecrets::Kubernetes(KubernetesSecrets {
            token: Some("very-secret-token".into()),
            client_key: Some("-----BEGIN EC PRIVATE KEY-----".into()),
            exec: Some(ExecConfig {
                command: "aws".into(),
                env: vec![("AWS_PROFILE".into(), "prod-profile".into())],
                ..Default::default()
            }),
            ..Default::default()
        }),
    }
}

#[test]
fn clusters_round_trip_with_their_secrets_sealed() {
    let (db, store) = store();
    let created = store.create(draft("kind-dev")).unwrap();
    assert_eq!(store.get(&created.id).unwrap(), created);
    assert_eq!(store.secrets(&created.id).unwrap(), draft("x").secrets);
    assert_eq!(created.kind, ClusterKind::Kubernetes);

    let (connection, sealed): (String, Vec<u8>) = db
        .with(|c| {
            c.query_row("SELECT connection, secrets FROM clusters", [], |r| {
                Ok((r.get(0)?, r.get(1)?))
            })
        })
        .unwrap();
    for secret in ["very-secret-token", "PRIVATE KEY", "prod-profile"] {
        assert!(!connection.contains(secret), "{secret} in the connection");
        let raw = String::from_utf8_lossy(&sealed);
        assert!(!raw.contains(secret), "{secret} in clear in the secrets");
    }
}

#[test]
fn a_cluster_without_credentials_stores_none() {
    let (_, store) = store();
    let mut d = draft("anonymous");
    d.secrets = ClusterSecrets::none(ClusterKind::Kubernetes);
    let c = store.create(d).unwrap();
    assert_eq!(
        store.secrets(&c.id).unwrap(),
        ClusterSecrets::none(ClusterKind::Kubernetes)
    );
}

#[test]
fn secrets_sealed_with_another_key_are_lost() {
    let db = Arc::new(Db::open_in_memory().unwrap());
    let before = ClusterStore::new(db.clone(), Arc::new(SecretBox::from_key(&[1; 32])));
    let id = before.create(draft("prod")).unwrap().id;
    let after = ClusterStore::new(db, Arc::new(SecretBox::from_key(&[2; 32])));
    assert!(after
        .secrets(&id)
        .unwrap_err()
        .is(ErrorCode::CredentialsLost));
}

#[test]
fn names_are_unique_and_required() {
    let (_, store) = store();
    let c = store.create(draft("prod")).unwrap();
    let e = store.create(draft("PROD")).unwrap_err();
    assert!(e.is(ErrorCode::NameInUse));
    assert!(e.message.contains("PROD"));
    assert!(store
        .rename(&c.id, "  ")
        .unwrap_err()
        .is(ErrorCode::Validation));
    assert_eq!(store.rename(&c.id, " staging ").unwrap().name, "staging");
}

#[test]
fn a_server_must_be_a_url() {
    let (_, store) = store();
    let mut d = draft("bad");
    if let ClusterConnection::Kubernetes(c) = &mut d.connection {
        c.server = "127.0.0.1:6443".into();
    }
    assert!(store.create(d).unwrap_err().is(ErrorCode::Validation));
}

#[test]
fn refreshing_replaces_the_connection_and_keeps_the_name() {
    let (_, store) = store();
    let c = store.create(draft("kind-dev")).unwrap();
    let connection = ClusterConnection::Kubernetes(KubernetesConnection {
        server: "https://127.0.0.1:7443".into(),
        ..c.kubernetes().unwrap().clone()
    });
    let secrets = ClusterSecrets::Kubernetes(KubernetesSecrets {
        token: Some("new".into()),
        ..Default::default()
    });
    let next = store
        .replace_connection(&c.id, &connection, &secrets)
        .unwrap();
    assert_eq!(next.name, "kind-dev");
    assert_eq!(next.address(), "https://127.0.0.1:7443");
    assert_eq!(store.secrets(&c.id).unwrap(), secrets);
}

#[test]
fn a_cluster_keeps_its_kind() {
    let (_, store) = store();
    let c = store.create(draft("kind-dev")).unwrap();
    assert_eq!(c.kind, ClusterKind::Kubernetes);
    let nomad = ClusterConnection::Nomad(crate::model::NomadConnection {
        address: "http://127.0.0.1:4646".into(),
        ..Default::default()
    });
    let secrets = ClusterSecrets::none(ClusterKind::Nomad);
    assert!(store
        .replace_connection(&c.id, &nomad, &secrets)
        .unwrap_err()
        .is(ErrorCode::Validation));
    let mut mixed = draft("mixed");
    mixed.secrets = secrets;
    assert!(store.create(mixed).unwrap_err().is(ErrorCode::Validation));
}

/// Rows written before Clusters had kinds other than Kubernetes, in the
/// shape migration 0016 left them: the connection's JSON untagged and
/// the credentials sealed as they were.
#[test]
fn clusters_stored_before_kinds_keep_every_field_and_secret() {
    let (db, store) = store();
    let key = SecretBox::from_key(&[7; 32]);
    let old_secrets = serde_json::json!({ "token": "old-token", "clientKey": "KEY" });
    let sealed = key.seal_json(&old_secrets).unwrap();
    db.with(|c| {
            c.execute(
                "INSERT INTO clusters (id, name, connection, secrets) VALUES ('old', 'old', ?1, ?2)",
                (
                    r#"{"server":"https://10.0.0.1:6443","namespace":"shop","auth":{"method":"token"},"source":{"context":"prod","pasted":false}}"#,
                    &sealed,
                ),
            )
        })
        .unwrap();
    let old = store.get("old").unwrap();
    assert_eq!(old.kind, ClusterKind::Kubernetes);
    let k = old.kubernetes().unwrap();
    assert_eq!(k.server, "https://10.0.0.1:6443");
    assert_eq!(k.namespace.as_deref(), Some("shop"));
    assert_eq!(k.source.as_ref().unwrap().context, "prod");
    let ClusterSecrets::Kubernetes(secrets) = store.secrets("old").unwrap() else {
        panic!("not Kubernetes credentials");
    };
    assert_eq!(secrets.token.as_deref(), Some("old-token"));
    assert_eq!(secrets.client_key.as_deref(), Some("KEY"));
}

#[test]
fn enabling_and_deleting() {
    let (_, store) = store();
    let c = store.create(draft("kind-dev")).unwrap();
    assert!(!store.set_enabled(&c.id, false).unwrap().enabled);
    store.delete(&c.id).unwrap();
    assert!(store.get(&c.id).unwrap_err().is(ErrorCode::NotFound));
    assert!(store.delete(&c.id).unwrap_err().is(ErrorCode::NotFound));
}
