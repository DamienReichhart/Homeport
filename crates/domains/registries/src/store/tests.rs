use super::*;

fn store() -> RegistryStore {
    RegistryStore::new(
        Arc::new(Db::open_in_memory().unwrap()),
        Arc::new(SecretBox::from_key(&[3; 32])),
    )
}

fn draft(address: &str, username: &str, secret: Option<&str>) -> RegistryDraft {
    RegistryDraft {
        address: address.into(),
        username: username.into(),
        secret: secret.map(Into::into),
    }
}

#[test]
fn logins_are_stored_with_their_secret_sealed() {
    let store = store();
    let saved = store
        .save(None, draft("https://GHCR.io/", "alice", Some("ghp_token")))
        .unwrap();
    assert_eq!(saved.address, "ghcr.io");
    assert_eq!(store.list().unwrap(), [saved]);

    let raw: Vec<u8> = store
        .db
        .with(|c| c.query_row("SELECT secret FROM registries", [], |r| r.get(0)))
        .unwrap();
    assert!(!raw.windows(9).any(|w| w == b"ghp_token"));

    let creds = store.credentials_for("ghcr.io/org/app:1").unwrap().unwrap();
    assert_eq!(creds.username, "alice");
    assert_eq!(creds.secret, "ghp_token");
    assert!(!format!("{creds:?}").contains("ghp_token"));
}

#[test]
fn references_without_a_registry_use_the_docker_hub_login() {
    let store = store();
    store
        .save(None, draft("index.docker.io", "bob", Some("pw")))
        .unwrap();
    assert_eq!(
        store
            .credentials_for("nginx:1.27")
            .unwrap()
            .unwrap()
            .address,
        DOCKER_HUB
    );
    assert_eq!(
        store.login_for("bitnami/redis").unwrap().unwrap().username,
        "bob"
    );
    assert!(store.credentials_for("ghcr.io/x/y").unwrap().is_none());
}

#[test]
fn editing_keeps_the_secret_unless_a_new_one_is_given() {
    let store = store();
    let saved = store
        .save(None, draft("ghcr.io", "alice", Some("one")))
        .unwrap();
    store
        .save(Some(&saved.id), draft("ghcr.io", "alice2", None))
        .unwrap();
    let creds = store.credentials("ghcr.io").unwrap().unwrap();
    assert_eq!(
        (creds.username.as_str(), creds.secret.as_str()),
        ("alice2", "one")
    );

    store
        .save(Some(&saved.id), draft("ghcr.io", "alice2", Some("two")))
        .unwrap();
    assert_eq!(store.credentials("ghcr.io").unwrap().unwrap().secret, "two");
}

#[test]
fn drafts_are_checked_with_the_stored_secret_when_none_is_typed() {
    let store = store();
    let saved = store
        .save(None, draft("ghcr.io", "alice", Some("stored")))
        .unwrap();
    let creds = store
        .resolve(Some(&saved.id), draft("https://ghcr.io", "alice", None))
        .unwrap();
    assert_eq!(
        (creds.address.as_str(), creds.secret.as_str()),
        ("ghcr.io", "stored")
    );
    let typed = store
        .resolve(Some(&saved.id), draft("ghcr.io", "alice", Some("new")))
        .unwrap();
    assert_eq!(typed.secret, "new");
    assert!(store
        .resolve(None, draft("ghcr.io", "alice", None))
        .unwrap_err()
        .is(ErrorCode::Validation));
}

#[test]
fn one_login_per_address() {
    let store = store();
    store
        .save(None, draft("docker.io", "a", Some("x")))
        .unwrap();
    let err = store
        .save(None, draft("https://index.docker.io/v1/", "b", Some("y")))
        .unwrap_err();
    assert!(err.is(ErrorCode::NameInUse));
    assert!(err.message.contains("docker.io"));
}

#[test]
fn new_logins_need_every_field() {
    let store = store();
    for (d, field) in [
        (draft("", "a", Some("x")), "address"),
        (draft("ghcr.io", " ", Some("x")), "user name"),
        (draft("ghcr.io", "a", None), "password"),
        (draft("ghcr.io", "a", Some("")), "password"),
    ] {
        let err = store.save(None, d).unwrap_err();
        assert!(err.is(ErrorCode::Validation));
        assert!(err.message.contains(field), "{}", err.message);
    }
}

#[test]
fn removed_logins_are_gone() {
    let store = store();
    let saved = store.save(None, draft("ghcr.io", "a", Some("x"))).unwrap();
    store.remove(&saved.id).unwrap();
    assert!(store.list().unwrap().is_empty());
    assert!(store.remove(&saved.id).unwrap_err().is(ErrorCode::NotFound));
    assert!(store
        .save(Some(&saved.id), draft("ghcr.io", "a", None))
        .unwrap_err()
        .is(ErrorCode::NotFound));
}

#[test]
fn every_login_can_be_opened_for_builds() {
    let store = store();
    store.save(None, draft("ghcr.io", "a", Some("x"))).unwrap();
    store
        .save(None, draft("docker.io", "b", Some("y")))
        .unwrap();
    let all = store.all_credentials().unwrap();
    assert_eq!(
        all.iter().map(|c| c.address.as_str()).collect::<Vec<_>>(),
        ["docker.io", "ghcr.io"]
    );
}

#[test]
fn without_a_keychain_logins_cant_be_stored() {
    struct NoKeychain;
    impl homeport_storage::KeyStore for NoKeychain {
        fn load(&self) -> Result<Option<homeport_storage::crypto::Key>, String> {
            Err("no secret service".into())
        }
        fn create(&self) -> Result<homeport_storage::crypto::Key, String> {
            Err("no secret service".into())
        }
    }
    let store = RegistryStore::new(
        Arc::new(Db::open_in_memory().unwrap()),
        Arc::new(SecretBox::from_store(&NoKeychain, false)),
    );
    let err = store
        .save(None, draft("ghcr.io", "a", Some("x")))
        .unwrap_err();
    assert!(err.is(ErrorCode::KeychainUnavailable));
}

use crate::DOCKER_HUB;
