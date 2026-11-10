use super::*;
use crate::{Bastion, BastionAuth, SshAuth};

fn store() -> HostStore {
    HostStore::new(
        Arc::new(Db::open_in_memory().unwrap()),
        Arc::new(SecretBox::from_key(&[9; 32])),
    )
}

fn local(name: &str) -> HostDraft {
    HostDraft {
        name: name.into(),
        connection: HostConnection::Local {
            socket_path: "/var/run/docker.sock".into(),
        },
        engine: EngineKind::Docker,
        secrets: None,
    }
}

fn ssh(name: &str, host: &str) -> HostDraft {
    HostDraft {
        name: name.into(),
        connection: HostConnection::Ssh {
            host: host.into(),
            port: 22,
            user: "deploy".into(),
            socket_path: "/var/run/docker.sock".into(),
            auth: SshAuth::Generated,
            public_key: Some("ssh-ed25519 AAAA homeport".into()),
            bastions: vec![],
        },
        engine: EngineKind::Docker,
        secrets: Some(HostSecrets::ssh_key("PRIVATE".into(), None)),
    }
}

fn bastion(host: &str) -> Bastion {
    Bastion {
        host: host.into(),
        port: 22,
        user: "me".into(),
        auth: BastionAuth::SameAsHost,
        public_key: None,
        host_key: None,
    }
}

fn behind(name: &str, bastions: Vec<Bastion>) -> HostDraft {
    let mut draft = ssh(name, "10.0.0.5");
    if let HostConnection::Ssh { bastions: b, .. } = &mut draft.connection {
        *b = bastions;
    }
    draft.secrets = None;
    draft
}

fn bastion_keys(host: &Host) -> Vec<(String, Option<String>)> {
    match &host.connection {
        HostConnection::Ssh { bastions, .. } => bastions
            .iter()
            .map(|b| (b.host.clone(), b.host_key.clone()))
            .collect(),
        _ => vec![],
    }
}

#[test]
fn created_hosts_are_listed_by_name() {
    let s = store();
    s.create(local("zeta")).unwrap();
    let alpha = s.create(local("  Alpha ")).unwrap();
    assert_eq!(alpha.name, "Alpha");
    assert!(alpha.enabled);
    let names: Vec<_> = s.list().unwrap().into_iter().map(|h| h.name).collect();
    assert_eq!(names, ["Alpha", "zeta"]);
}

#[test]
fn names_are_unique_ignoring_case() {
    let s = store();
    s.create(local("prod")).unwrap();
    let err = s.create(local("PROD")).unwrap_err();
    assert!(err.message.contains("already exists"));
}

#[test]
fn secrets_are_kept_and_readable_only_through_the_store() {
    let s = store();
    let host = s.create(ssh("box", "10.0.0.5")).unwrap();
    assert_eq!(
        s.secrets(&host.id).unwrap(),
        Some(HostSecrets::ssh_key("PRIVATE".into(), None))
    );
    assert!(!serde_json::to_string(&host).unwrap().contains("PRIVATE"));
}

#[test]
fn connections_that_need_credentials_require_them() {
    let mut draft = ssh("box", "10.0.0.5");
    draft.secrets = None;
    assert!(matches!(
        store().create(draft),
        Err(ref e) if e.is(ErrorCode::Validation)
    ));
}

#[test]
fn secrets_from_another_key_are_reported_lost() {
    let db = Arc::new(Db::open_in_memory().unwrap());
    let before = HostStore::new(db.clone(), Arc::new(SecretBox::from_key(&[1; 32])));
    let host = before.create(ssh("box", "10.0.0.5")).unwrap();
    let after = HostStore::new(db, Arc::new(SecretBox::from_key(&[2; 32])));
    assert!(matches!(
        after.secrets(&host.id),
        Err(ref e) if e.is(ErrorCode::CredentialsLost)
    ));
}

#[test]
fn update_keeps_secrets_and_host_key_for_the_same_server() {
    let s = store();
    let host = s.create(ssh("box", "10.0.0.5")).unwrap();
    s.set_host_key(&host.id, Some("ssh-ed25519 SERVER"))
        .unwrap();

    let mut renamed = ssh("renamed", "10.0.0.5");
    renamed.secrets = None;
    let updated = s.update(&host.id, renamed).unwrap();
    assert_eq!(updated.name, "renamed");
    assert_eq!(updated.host_key.as_deref(), Some("ssh-ed25519 SERVER"));
    assert!(s.secrets(&host.id).unwrap().is_some());

    let mut moved = ssh("renamed", "10.0.0.6");
    moved.secrets = None;
    assert_eq!(s.update(&host.id, moved).unwrap().host_key, None);
}

#[test]
fn bastions_keep_their_trusted_key_by_address() {
    let s = store();
    let mut draft = behind("box", vec![bastion("jump"), bastion("inner")]);
    draft.secrets = Some(HostSecrets::ssh_key("PRIVATE".into(), None));
    let host = s.create(draft).unwrap();
    s.set_bastion_key(&host.id, "jump:22", "ssh-ed25519 JUMP")
        .unwrap();
    s.set_bastion_key(&host.id, "inner:22", "ssh-ed25519 INNER")
        .unwrap();
    assert!(s.set_bastion_key(&host.id, "other:22", "k").is_err());

    // Reordered, one moved elsewhere, and a key the form tried to set.
    let mut forged = bastion("new");
    forged.host_key = Some("ssh-ed25519 FORGED".into());
    let updated = s
        .update(&host.id, behind("box", vec![bastion("inner"), forged]))
        .unwrap();
    assert_eq!(
        bastion_keys(&updated),
        [
            ("inner".into(), Some("ssh-ed25519 INNER".into())),
            ("new".into(), None)
        ]
    );
}

#[test]
fn hosts_keep_their_kind_of_engine() {
    let s = store();
    let host = s.create(local("local")).unwrap();
    assert_eq!(host.engine, EngineKind::Docker);

    let mut draft = local("k3s");
    draft.engine = EngineKind::Containerd;
    assert_eq!(s.create(draft).unwrap().engine, EngineKind::Containerd);

    let mut draft = local("lab");
    draft.engine = EngineKind::Incus;
    let lab = s.create(draft).unwrap();
    assert_eq!(s.get(&lab.id).unwrap().engine, EngineKind::Incus);

    // Incus doesn't answer on plain TCP.
    let mut draft = local("local");
    draft.engine = EngineKind::Incus;
    draft.connection = HostConnection::Tcp {
        address: "10.0.0.2:8443".into(),
    };
    assert!(matches!(s.update(&host.id, draft), Err(ref e) if e.is(ErrorCode::Validation)));
    assert_eq!(s.get(&host.id).unwrap().engine, EngineKind::Docker);
}

#[test]
fn hosts_can_be_disabled_and_deleted() {
    let s = store();
    let host = s.create(local("local")).unwrap();
    assert!(!s.set_enabled(&host.id, false).unwrap().enabled);
    s.delete(&host.id).unwrap();
    assert!(matches!(s.get(&host.id), Err(ref e) if e.is(ErrorCode::NotFound)));
    assert!(matches!(s.delete(&host.id), Err(ref e) if e.is(ErrorCode::NotFound)));
}
