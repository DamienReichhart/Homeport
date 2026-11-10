use std::sync::Arc;

use homeport_storage::{Db, SecretBox};

use super::*;
use crate::{
    service::{BastionKeyInput, CredentialsInput, HostInput},
    store::HostStore,
    Bastion, BastionAuth, EngineKind, HostSecrets, SshAuth,
};
use homeport_transport as transport;

/// A computer with its own database and keychain key.
fn computer(key: u8) -> HostService {
    let db = Arc::new(Db::open_in_memory().unwrap());
    let store = Arc::new(HostStore::new(
        db.clone(),
        Arc::new(SecretBox::from_key(&[key; 32])),
    ));
    HostService::for_tests(db, store)
}

const PASS: &str = "moving to the new laptop";

fn ssh_input(name: &str, jump_key: &str) -> HostInput {
    HostInput {
        name: name.into(),
        connection: HostConnection::Ssh {
            host: "192.0.2.7".into(),
            port: 22,
            user: "deploy".into(),
            socket_path: "/var/run/docker.sock".into(),
            auth: SshAuth::Generated,
            public_key: None,
            bastions: vec![Bastion {
                host: "192.0.2.1".into(),
                port: 22,
                user: "me".into(),
                auth: BastionAuth::Imported,
                public_key: None,
                host_key: None,
            }],
        },
        engine: EngineKind::Docker,
        credentials: None,
        bastion_keys: vec![BastionKeyInput {
            index: 0,
            private_key: jump_key.into(),
            passphrase: None,
        }],
    }
}

#[tokio::test]
async fn hosts_move_to_another_computer_with_their_keys_and_trust() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("hosts.homeport");
    let old = computer(1);
    let jump = transport::ssh::generate_key("jump");
    let ssh = old
        .create(ssh_input("prod", &jump.private_key))
        .unwrap()
        .host
        .id;
    let server = transport::ssh::generate_key("server").public_key;
    let bastion_server = transport::ssh::generate_key("bastion").public_key;
    old.trust_host_key(&ssh, &server, None).unwrap();
    old.trust_host_key(&ssh, &bastion_server, Some("192.0.2.1:22"))
        .unwrap();
    let lab = old
        .create(HostInput {
            name: "lab".into(),
            connection: HostConnection::Tcp {
                address: "192.0.2.9:2375".into(),
            },
            engine: EngineKind::Docker,
            credentials: None,
            bastion_keys: vec![],
        })
        .unwrap()
        .host
        .id;
    old.set_enabled(&lab, false).unwrap();

    let exported = old.export_hosts(&[ssh.clone(), lab], &file, PASS).unwrap();
    assert_eq!(exported.exported, 2);
    assert!(exported.without_secrets.is_empty());
    let bytes = std::fs::read(&file).unwrap();
    for clear in ["prod", "192.0.2.7", "deploy", "OPENSSH"] {
        assert!(
            !bytes.windows(clear.len()).any(|w| w == clear.as_bytes()),
            "{clear}"
        );
    }

    let new = computer(2);
    let preview = new.preview_bundle(&file, PASS).unwrap();
    assert_eq!(preview.len(), 2);
    assert!(preview.iter().all(|h| !h.secrets_missing && !h.name_taken));
    let outcome = new
        .import_bundle(
            &file,
            PASS,
            preview
                .iter()
                .map(|h| BundleChoice {
                    index: h.index,
                    name: h.suggested_name.clone(),
                })
                .collect(),
        )
        .unwrap();
    assert!(outcome.failed.is_empty());
    let prod = outcome
        .created
        .iter()
        .find(|v| v.host.name == "prod")
        .unwrap();
    assert_eq!(prod.host.host_key.as_deref(), Some(server.as_str()));
    let HostConnection::Ssh { bastions, .. } = &prod.host.connection else {
        panic!("expected SSH");
    };
    assert_eq!(
        bastions[0].host_key.as_deref(),
        Some(bastion_server.as_str())
    );
    // The secrets are sealed under the new computer's key and open there.
    let Some(HostSecrets::Ssh {
        private_key: Some(_),
        bastions: keys,
        ..
    }) = new.store.secrets(&prod.host.id).unwrap()
    else {
        panic!("expected the Host's and the bastion's keys");
    };
    assert_eq!(keys["192.0.2.1:22"].private_key, jump.private_key);
    let lab = outcome
        .created
        .iter()
        .find(|v| v.host.name == "lab")
        .unwrap();
    assert!(!lab.host.enabled);

    // Again: names are taken now, so the preview suggests others.
    let again = new.preview_bundle(&file, PASS).unwrap();
    assert!(again.iter().all(|h| h.name_taken));
    assert_eq!(again[0].suggested_name, format!("{} (2)", again[0].name));
}

#[tokio::test]
async fn hosts_whose_secrets_are_lost_are_exported_without_them() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("hosts.homeport");
    let db = Arc::new(Db::open_in_memory().unwrap());
    let before = HostStore::new(db.clone(), Arc::new(SecretBox::from_key(&[1; 32])));
    let id = before
        .create(HostDraft {
            name: "tls".into(),
            connection: HostConnection::Tls {
                address: "h:2376".into(),
            },
            engine: EngineKind::Docker,
            secrets: Some(HostSecrets::Tls {
                ca_pem: "CA".into(),
                cert_pem: "CERT".into(),
                key_pem: "KEY".into(),
            }),
        })
        .unwrap()
        .id;
    let store = Arc::new(HostStore::new(
        db.clone(),
        Arc::new(SecretBox::from_key(&[2; 32])),
    ));
    let svc = HostService::for_tests(db, store);
    let outcome = svc.export_hosts(&[id], &file, PASS).unwrap();
    assert_eq!(outcome.without_secrets, ["tls"]);

    let other = computer(3);
    let preview = other.preview_bundle(&file, PASS).unwrap();
    assert!(preview[0].secrets_missing);
    let imported = other
        .import_bundle(
            &file,
            PASS,
            vec![BundleChoice {
                index: 0,
                name: "tls".into(),
            }],
        )
        .unwrap();
    assert_eq!(imported.created.len(), 1);
    assert_eq!(
        other.store.secrets(&imported.created[0].host.id).unwrap(),
        None
    );
}

#[tokio::test]
async fn the_wrong_passphrase_imports_nothing() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("hosts.homeport");
    let svc = computer(1);
    let id = svc
        .create(HostInput {
            name: "tls".into(),
            connection: HostConnection::Local {
                socket_path: "/var/run/docker.sock".into(),
            },
            engine: EngineKind::Docker,
            credentials: None::<CredentialsInput>,
            bastion_keys: vec![],
        })
        .unwrap()
        .host
        .id;
    svc.export_hosts(&[id], &file, PASS).unwrap();
    let err = computer(2)
        .preview_bundle(&file, "not the passphrase")
        .unwrap_err();
    assert_eq!(err.code, ErrorCode::BundleUnreadable);
    assert!(svc.export_hosts(&[], &file, PASS).is_err());
}
