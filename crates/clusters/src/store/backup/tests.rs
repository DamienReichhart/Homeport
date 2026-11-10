use super::*;
use crate::model::{AuthMethod, KubernetesConnection, KubernetesSecrets};

#[test]
fn clusters_round_trip_through_a_backup_under_another_key() {
    let db = Arc::new(Db::open_in_memory().unwrap());
    let laptop = ClusterStore::new(db, Arc::new(SecretBox::from_key(&[1; 32])));
    let id = laptop
        .create(ClusterDraft {
            name: "prod".into(),
            connection: ClusterConnection::Kubernetes(KubernetesConnection {
                server: "https://k8s.example.com".into(),
                auth: AuthMethod::Token,
                ..Default::default()
            }),
            secrets: ClusterSecrets::Kubernetes(KubernetesSecrets {
                token: Some("t0ken".into()),
                ..Default::default()
            }),
        })
        .unwrap()
        .id;
    let mut lost = vec![];
    let backed_up = laptop.backup(&mut lost).unwrap();
    assert!(lost.is_empty());
    assert!(
        !format!("{backed_up:?}").contains("t0ken"),
        "Debug hides them"
    );

    let db = Arc::new(Db::open_in_memory().unwrap());
    let key = Arc::new(SecretBox::from_key(&[2; 32]));
    db.transaction(|tx| restore(tx, &key, &backed_up)).unwrap();
    let desktop = ClusterStore::new(db, key);
    assert_eq!(desktop.get(&id).unwrap().name, "prod");
    let ClusterSecrets::Kubernetes(restored) = desktop.secrets(&id).unwrap() else {
        panic!("not Kubernetes credentials");
    };
    assert_eq!(restored.token.as_deref(), Some("t0ken"));
}

#[test]
fn nomad_clusters_round_trip_with_their_token_and_key() {
    let db = Arc::new(Db::open_in_memory().unwrap());
    let laptop = ClusterStore::new(db, Arc::new(SecretBox::from_key(&[1; 32])));
    let id = laptop
        .create(ClusterDraft {
            name: "nomad-prod".into(),
            connection: ClusterConnection::Nomad(crate::model::NomadConnection {
                address: "https://nomad.example.com:4646".into(),
                acl_token: true,
                ..Default::default()
            }),
            secrets: ClusterSecrets::Nomad(crate::model::NomadSecrets {
                token: Some("n0mad-t0ken".into()),
                client_key: Some("NOMAD KEY".into()),
                ..Default::default()
            }),
        })
        .unwrap()
        .id;
    let backed_up = laptop.backup(&mut vec![]).unwrap();
    assert!(!format!("{backed_up:?}").contains("n0mad-t0ken"));

    let db = Arc::new(Db::open_in_memory().unwrap());
    let key = Arc::new(SecretBox::from_key(&[2; 32]));
    db.transaction(|tx| restore(tx, &key, &backed_up)).unwrap();
    let desktop = ClusterStore::new(db, key);
    assert_eq!(desktop.get(&id).unwrap().kind, ClusterKind::Nomad);
    let ClusterSecrets::Nomad(restored) = desktop.secrets(&id).unwrap() else {
        panic!("not Nomad credentials");
    };
    assert_eq!(restored.token.as_deref(), Some("n0mad-t0ken"));
    assert_eq!(restored.client_key.as_deref(), Some("NOMAD KEY"));
}

/// A backup's Cluster as Homeport wrote it before Clusters had kinds.
#[test]
fn backups_made_before_kinds_restore() {
    let backed_up: Vec<BackedUpCluster> = serde_json::from_value(serde_json::json!([{
        "id": "c1",
        "name": "prod",
        "kind": "kubernetes",
        "connection": { "server": "https://k8s.example.com", "auth": { "method": "token" } },
        "enabled": true,
        "secrets": { "token": "t0ken" },
    }]))
    .unwrap();
    let db = Arc::new(Db::open_in_memory().unwrap());
    let key = Arc::new(SecretBox::from_key(&[3; 32]));
    db.transaction(|tx| restore(tx, &key, &backed_up)).unwrap();
    let store = ClusterStore::new(db, key);
    assert_eq!(
        store.get("c1").unwrap().address(),
        "https://k8s.example.com"
    );
    assert!(!store.secrets("c1").unwrap().is_empty());
}
