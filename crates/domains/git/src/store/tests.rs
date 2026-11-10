use homeport_storage::KeyStore;

use super::*;

fn store(secrets: SecretBox) -> (RepositoryStore, Arc<Db>) {
    let db = Arc::new(Db::open_in_memory().unwrap());
    (RepositoryStore::new(db.clone(), Arc::new(secrets)), db)
}

fn draft(url: &str, auth: RepoAuth) -> RepositoryDraft {
    RepositoryDraft {
        url: url.into(),
        auth,
        username: Some("deploy".into()),
        token: Some("glpat-hunter2".into()),
    }
}

#[test]
fn deploy_keys_are_generated_and_sealed() {
    let (store, db) = store(SecretBox::from_key(&[4; 32]));
    let repo = store
        .create(&draft("git@github.com:owner/shop.git", RepoAuth::SshKey))
        .unwrap();
    let public = repo.public_key.clone().unwrap();
    assert!(public.starts_with("ssh-ed25519 ") && public.ends_with(" homeport-shop"));
    assert!(repo
        .key_fingerprint
        .as_deref()
        .unwrap()
        .starts_with("SHA256:"));
    assert_eq!((repo.username.as_deref(), repo.has_token), (None, false));

    let Secret::SshKey { private_key } = store.access(&repo.id).unwrap().secret else {
        panic!("no key");
    };
    assert!(private_key.contains("OPENSSH PRIVATE KEY"));
    // Nothing in the row reads as the key.
    let sealed: Vec<u8> = db
        .with(|c| c.query_row("SELECT private_key FROM git_repositories", [], |r| r.get(0)))
        .unwrap();
    assert!(!String::from_utf8_lossy(&sealed).contains("PRIVATE KEY"));

    let rotated = store.rotate_key(&repo.id).unwrap();
    assert_ne!(rotated.public_key, repo.public_key);
    assert!(store.set_token(&repo.id, "a", "b").is_err());
}

#[test]
fn tokens_are_sealed_and_replaced() {
    let (store, db) = store(SecretBox::from_key(&[4; 32]));
    let repo = store
        .create(&draft(
            "https://gitlab.com/owner/shop.git",
            RepoAuth::HttpsToken,
        ))
        .unwrap();
    assert!(repo.has_token);
    assert_eq!(repo.username.as_deref(), Some("deploy"));
    assert!(!serde_json::to_string(&repo).unwrap().contains("hunter2"));
    assert!(!format!("{:?}", store.access(&repo.id).unwrap()).contains("hunter2"));
    let dump: String = db
        .with(|c| {
            c.query_row(
                "SELECT quote(token) || username FROM git_repositories",
                [],
                |r| r.get(0),
            )
        })
        .unwrap();
    assert!(!dump.contains("hunter2"));

    store.set_token(&repo.id, "bot", "new-token").unwrap();
    assert_eq!(
        store.access(&repo.id).unwrap().secret,
        Secret::Token {
            username: "bot".into(),
            token: "new-token".into()
        }
    );
    assert!(store.set_token(&repo.id, "bot", " ").is_err());
    assert!(store.rotate_key(&repo.id).is_err());
}

#[test]
fn the_url_and_the_kind_of_access_must_agree() {
    let (store, _) = store(SecretBox::from_key(&[4; 32]));
    for (url, auth) in [
        ("https://github.com/o/r.git", RepoAuth::SshKey),
        ("git@github.com:o/r.git", RepoAuth::HttpsToken),
        ("git@github.com:o/r.git", RepoAuth::None),
        ("http://github.com/o/r.git", RepoAuth::None),
    ] {
        assert!(
            matches!(store.create(&draft(url, auth)), Err(ref e) if e.is(ErrorCode::Validation)),
            "{url} {auth:?}"
        );
    }
    let public = store
        .create(&draft("https://github.com/o/r.git", RepoAuth::None))
        .unwrap();
    assert_eq!(store.access(&public.id).unwrap().secret, Secret::None);
    assert!(matches!(
        store.create(&draft(" https://github.com/o/r.git ", RepoAuth::None)),
        Err(ref e) if e.is(ErrorCode::NameInUse) && e.hint.is_some()
    ));
}

#[test]
fn host_keys_are_trusted_then_reset() {
    let (store, _) = store(SecretBox::from_key(&[4; 32]));
    let repo = store
        .create(&draft("git@github.com:owner/shop.git", RepoAuth::SshKey))
        .unwrap();
    let server = ssh::generate_key("server").public_key;
    let line = format!(
        "github.com {}",
        server.split(' ').take(2).collect::<Vec<_>>().join(" ")
    );
    store.set_host_key(&repo.id, &line).unwrap();
    assert_eq!(
        store.access(&repo.id).unwrap().host_key.as_deref(),
        Some(line.as_str())
    );
    assert_eq!(
        store.get(&repo.id).unwrap().host_key_fingerprint,
        ssh::fingerprint(&server)
    );
    let reset = store.reset_host_key(&repo.id).unwrap();
    assert_eq!(reset.host_key_fingerprint, None);
    assert_eq!(store.access(&repo.id).unwrap().host_key, None);
}

#[test]
fn repositories_in_use_are_not_removed() {
    let (store, db) = store(SecretBox::from_key(&[4; 32]));
    let repo = store
        .create(&draft("https://github.com/o/r.git", RepoAuth::None))
        .unwrap();
    db.with(|c| {
            c.execute(
                "INSERT INTO hosts (id, name, connection) VALUES ('h', 'local', '{\"kind\":\"local\",\"socketPath\":\"/var/run/docker.sock\"}')",
                [],
            )?;
            c.execute(
                "INSERT INTO synced_stacks (id, host_id, name, repository_id, ref_kind, ref_name, compose_path)
                 VALUES ('s', 'h', 'shop', ?1, 'branch', 'main', 'compose.yaml')",
                [&repo.id],
            )
        })
        .unwrap();
    assert!(matches!(store.delete(&repo.id), Err(ref e) if e.is(ErrorCode::InUse)));
    db.with(|c| c.execute("DELETE FROM synced_stacks", []))
        .unwrap();
    store.delete(&repo.id).unwrap();
    assert!(matches!(store.get(&repo.id), Err(ref e) if e.is(ErrorCode::NotFound)));
}

#[test]
fn without_a_keychain_only_public_repositories_can_be_added() {
    struct NoKeychain;
    impl KeyStore for NoKeychain {
        fn load(&self) -> Result<Option<homeport_storage::crypto::Key>, String> {
            Err("none".into())
        }
        fn create(&self) -> Result<homeport_storage::crypto::Key, String> {
            Err("none".into())
        }
    }
    let (store, _) = store(SecretBox::from_store(&NoKeychain, false));
    for (url, auth) in [
        ("git@github.com:o/r.git", RepoAuth::SshKey),
        ("https://gitlab.com/o/r.git", RepoAuth::HttpsToken),
    ] {
        assert!(matches!(
            store.create(&draft(url, auth)),
            Err(ref e) if e.is(ErrorCode::KeychainUnavailable)
        ));
    }
    assert!(store
        .create(&draft("https://github.com/o/r.git", RepoAuth::None))
        .is_ok());
}
