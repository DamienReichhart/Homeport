use homeport_storage::{Db, SecretBox};

use super::*;
use homeport_core::error::ErrorCode;

fn service() -> HostService {
    let db = Arc::new(Db::open_in_memory().unwrap());
    let store = Arc::new(HostStore::new(
        db.clone(),
        Arc::new(SecretBox::from_key(&[5; 32])),
    ));
    HostService::for_tests(db, store)
}

fn ssh(auth: SshAuth, credentials: Option<CredentialsInput>) -> HostInput {
    HostInput {
        name: "prod".into(),
        connection: HostConnection::Ssh {
            // An unroutable address keeps the supervisor from reaching anything.
            host: "192.0.2.1".into(),
            port: 22,
            user: "deploy".into(),
            socket_path: "/var/run/docker.sock".into(),
            auth,
            public_key: None,
            bastions: vec![],
        },
        engine: EngineKind::Docker,
        credentials,
        bastion_keys: vec![],
    }
}

fn public_key(view: &HostView) -> Option<String> {
    match &view.host.connection {
        HostConnection::Ssh { public_key, .. } => public_key.clone(),
        _ => None,
    }
}

#[tokio::test]
async fn ssh_hosts_get_a_generated_key_and_setup_instructions() {
    let svc = service();
    let view = svc.create(ssh(SshAuth::Generated, None)).unwrap();
    let key = public_key(&view).unwrap();
    assert!(key.starts_with("ssh-ed25519 "));
    assert!(key.ends_with("-prod"));

    let setup = svc.ssh_setup(&view.host.id).unwrap();
    assert!(setup.install_command.contains(&key));
    assert_eq!(setup.docker_group_command, "sudo usermod -aG docker deploy");
    assert!(setup.fingerprint.starts_with("SHA256:"));

    let Some(HostSecrets::Ssh {
        private_key: Some(private_key),
        ..
    }) = svc.store.secrets(&view.host.id).unwrap()
    else {
        panic!("expected SSH secrets");
    };
    assert_eq!(
        transport::ssh::public_key_of(&private_key, None).unwrap(),
        key
    );
}

#[tokio::test]
async fn editing_an_ssh_host_keeps_its_key_until_regenerated() {
    let svc = service();
    let id = svc.create(ssh(SshAuth::Generated, None)).unwrap().host.id;
    let original = public_key(&svc.get(&id).unwrap());

    let mut renamed = ssh(SshAuth::Generated, None);
    renamed.name = "production".into();
    assert_eq!(public_key(&svc.update(&id, renamed).unwrap()), original);

    let regenerated = public_key(&svc.regenerate_ssh_key(&id).unwrap());
    assert!(regenerated.is_some());
    assert_ne!(regenerated, original);
}

#[tokio::test]
async fn imported_keys_show_their_public_half() {
    let generated = transport::ssh::generate_key("mine");
    let view = service()
        .create(ssh(
            SshAuth::Imported,
            Some(CredentialsInput::SshKey {
                private_key: generated.private_key,
                passphrase: Some(String::new()),
            }),
        ))
        .unwrap();
    assert_eq!(public_key(&view), Some(generated.public_key));
}

#[tokio::test]
async fn agent_hosts_store_no_secrets() {
    let svc = service();
    let id = svc.create(ssh(SshAuth::Agent, None)).unwrap().host.id;
    assert_eq!(svc.store.secrets(&id).unwrap(), None);
    assert!(svc.ssh_setup(&id).is_err());
}

#[tokio::test]
async fn unusable_credentials_are_rejected_up_front() {
    let svc = service();
    let bad_key = ssh(
        SshAuth::Imported,
        Some(CredentialsInput::SshKey {
            private_key: "nope".into(),
            passphrase: None,
        }),
    );
    assert!(matches!(svc.create(bad_key), Err(ref e) if e.is(ErrorCode::Validation)));

    let bad_tls = HostInput {
        name: "tls".into(),
        connection: HostConnection::Tls {
            address: "h:2376".into(),
        },
        engine: EngineKind::Docker,
        credentials: Some(CredentialsInput::Tls {
            ca_pem: "x".into(),
            cert_pem: "x".into(),
            key_pem: "x".into(),
        }),
        bastion_keys: vec![],
    };
    assert!(matches!(svc.create(bad_tls), Err(ref e) if e.is(ErrorCode::Validation)));
}

#[tokio::test]
async fn trusting_requires_a_real_key() {
    let svc = service();
    let id = svc.create(ssh(SshAuth::Agent, None)).unwrap().host.id;
    assert!(svc.trust_host_key(&id, "garbage", None).is_err());
    let key = transport::ssh::generate_key("server").public_key;
    assert_eq!(
        svc.trust_host_key(&id, &key, None).unwrap().host.host_key,
        Some(key)
    );
}

fn bastion(host: &str, auth: BastionAuth) -> crate::Bastion {
    crate::Bastion {
        host: host.into(),
        port: 22,
        user: "me".into(),
        auth,
        public_key: None,
        host_key: None,
    }
}

fn behind(auth: SshAuth, bastions: Vec<crate::Bastion>, keys: Vec<BastionKeyInput>) -> HostInput {
    let mut input = ssh(auth, None);
    if let HostConnection::Ssh { bastions: b, .. } = &mut input.connection {
        *b = bastions;
    }
    input.bastion_keys = keys;
    input
}

fn bastions_of(view: &HostView) -> Vec<crate::Bastion> {
    match &view.host.connection {
        HostConnection::Ssh { bastions, .. } => bastions.clone(),
        _ => vec![],
    }
}

fn key_input(index: u32, private_key: &str) -> BastionKeyInput {
    BastionKeyInput {
        index,
        private_key: private_key.into(),
        passphrase: None,
    }
}

#[tokio::test]
async fn bastions_with_their_own_key_keep_it_sealed_by_address() {
    let svc = service();
    let jump_key = transport::ssh::generate_key("jump");
    let view = svc
        .create(behind(
            SshAuth::Agent,
            vec![
                bastion("jump", BastionAuth::Imported),
                bastion("inner", BastionAuth::SameAsHost),
            ],
            vec![key_input(0, &jump_key.private_key)],
        ))
        .unwrap();
    let bastions = bastions_of(&view);
    assert_eq!(
        bastions[0].public_key.as_deref(),
        Some(jump_key.public_key.as_str())
    );
    assert_eq!(bastions[1].public_key, None);
    let Some(HostSecrets::Ssh {
        private_key,
        bastions: keys,
        ..
    }) = svc.store.secrets(&view.host.id).unwrap()
    else {
        panic!("expected SSH secrets");
    };
    assert_eq!(private_key, None);
    assert_eq!(keys["jump:22"].private_key, jump_key.private_key);

    // Moved after another bastion and renamed, its key isn't retyped.
    let updated = svc
        .update(
            &view.host.id,
            behind(
                SshAuth::Agent,
                vec![
                    bastion("first", BastionAuth::Agent),
                    bastion("jump", BastionAuth::Imported),
                ],
                vec![],
            ),
        )
        .unwrap();
    assert_eq!(
        bastions_of(&updated)[1].public_key.as_deref(),
        Some(jump_key.public_key.as_str())
    );
}

#[tokio::test]
async fn a_bastion_using_its_own_key_needs_one() {
    let svc = service();
    let err = svc
        .create(behind(
            SshAuth::Agent,
            vec![
                bastion("first", BastionAuth::Agent),
                bastion("jump", BastionAuth::Imported),
            ],
            vec![],
        ))
        .unwrap_err();
    assert!(err.is(ErrorCode::Validation));
    assert_eq!(err.message, "Bastion 2 needs its private key.");
}

#[tokio::test]
async fn adding_a_bastion_key_keeps_the_hosts_own_key() {
    let svc = service();
    let id = svc.create(ssh(SshAuth::Generated, None)).unwrap().host.id;
    let original = public_key(&svc.get(&id).unwrap());
    let jump_key = transport::ssh::generate_key("jump");
    let view = svc
        .update(
            &id,
            behind(
                SshAuth::Generated,
                vec![bastion("jump", BastionAuth::Imported)],
                vec![key_input(0, &jump_key.private_key)],
            ),
        )
        .unwrap();
    assert_eq!(public_key(&view), original);
    let Some(HostSecrets::Ssh {
        private_key: Some(_),
        bastions,
        ..
    }) = svc.store.secrets(&id).unwrap()
    else {
        panic!("expected the Host's key and the bastion's");
    };
    assert!(bastions.contains_key("jump:22"));

    // Regenerating the Host's key leaves the bastion's alone.
    svc.regenerate_ssh_key(&id).unwrap();
    let Some(HostSecrets::Ssh { bastions, .. }) = svc.store.secrets(&id).unwrap() else {
        panic!("expected SSH secrets");
    };
    assert!(bastions.contains_key("jump:22"));
}

#[tokio::test]
async fn bastion_keys_are_trusted_on_the_bastion_and_the_form_cannot_set_them() {
    let svc = service();
    let mut forged = bastion("jump", BastionAuth::SameAsHost);
    forged.host_key = Some("ssh-ed25519 FORGED".into());
    let id = svc
        .create(behind(SshAuth::Agent, vec![forged], vec![]))
        .unwrap()
        .host
        .id;
    assert_eq!(bastions_of(&svc.get(&id).unwrap())[0].host_key, None);

    let key = transport::ssh::generate_key("jump").public_key;
    let view = svc.trust_host_key(&id, &key, Some("jump:22")).unwrap();
    assert_eq!(bastions_of(&view)[0].host_key, Some(key));
    assert_eq!(view.host.host_key, None);
    assert!(svc
        .trust_host_key(&id, &key_of("x"), Some("gone:22"))
        .is_err());
}

fn key_of(comment: &str) -> String {
    transport::ssh::generate_key(comment).public_key
}

#[tokio::test]
async fn the_setup_lists_every_server_the_generated_key_logs_in_to() {
    let svc = service();
    let id = svc
        .create(behind(
            SshAuth::Generated,
            vec![
                bastion("jump", BastionAuth::SameAsHost),
                bastion("inner", BastionAuth::Agent),
            ],
            vec![],
        ))
        .unwrap()
        .host
        .id;
    assert_eq!(
        svc.ssh_setup(&id).unwrap().servers,
        ["me@jump", "deploy@192.0.2.1"]
    );
}
