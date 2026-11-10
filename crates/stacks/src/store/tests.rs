use homeport_storage::KeyStore;

use super::*;
use homeport_hosts::{
    store::{HostDraft, HostStore},
    HostConnection,
};

const COMPOSE: &str = "services:\n  web:\n    image: nginx\n";

fn one(content: &str) -> Vec<ComposeFile> {
    vec![ComposeFile::new("compose.yaml", content)]
}

fn setup(secrets: SecretBox) -> (StackStore, HostStore, String) {
    let db = Arc::new(Db::open_in_memory().unwrap());
    let secrets = Arc::new(secrets);
    let hosts = HostStore::new(db.clone(), secrets.clone());
    let host = hosts
        .create(HostDraft {
            name: "local".into(),
            connection: HostConnection::Local {
                socket_path: "/var/run/docker.sock".into(),
            },
            engine: homeport_hosts::EngineKind::Docker,
            secrets: None,
        })
        .unwrap();
    (StackStore::new(db, secrets), hosts, host.id)
}

#[test]
fn stacks_keep_their_definition_and_sealed_env() {
    let (store, _, host) = setup(SecretBox::from_key(&[4; 32]));
    // The secret must not be hex, or it could turn up inside the random id.
    let s = store
        .create(&host, "shop", &one(COMPOSE), "TOKEN=hunter2\n")
        .unwrap();
    assert!(s.has_env);
    assert_eq!(store.env(&s.id).unwrap(), "TOKEN=hunter2\n");
    assert!(!serde_json::to_string(&s).unwrap().contains("hunter2"));

    let updated = store.update(&s.id, &one("services: {}\n"), "").unwrap();
    assert!(!updated.has_env);
    assert_eq!(store.env(&s.id).unwrap(), "");
    assert_eq!(store.list(&host).unwrap(), [updated]);
}

#[test]
fn names_are_unique_per_host_and_valid() {
    let (store, _, host) = setup(SecretBox::from_key(&[4; 32]));
    store.create(&host, "shop", &one(COMPOSE), "").unwrap();
    assert!(matches!(
        store.create(&host, "shop", &one(COMPOSE), ""),
        Err(ref e) if e.is(ErrorCode::NameInUse) && e.message.contains("\"shop\"")
    ));
    assert!(matches!(
        store.create(&host, "Shop!", &one(COMPOSE), ""),
        Err(ref e) if e.is(ErrorCode::Validation)
    ));
    assert!(matches!(
        store.create("nope", "x", &one(COMPOSE), ""),
        Err(ref e) if e.is(ErrorCode::NotFound)
    ));
}

#[test]
fn stacks_keep_several_files_in_order() {
    let (store, _, host) = setup(SecretBox::from_key(&[4; 32]));
    let files = vec![
        ComposeFile::new("compose.yaml", COMPOSE),
        ComposeFile::new(
            "compose.prod.yaml",
            "services:\n  web:\n    ports: [\"80:80\"]\n",
        ),
    ];
    let s = store.create(&host, "shop", &files, "").unwrap();
    assert_eq!(s.files, files);
    let reversed: Vec<_> = files.iter().rev().cloned().collect();
    assert_eq!(store.update(&s.id, &reversed, "").unwrap().files, reversed);
    assert!(matches!(
        store.update(&s.id, &[], ""),
        Err(ref e) if e.is(ErrorCode::Validation)
    ));
    assert_eq!(
        store.get(&s.id).unwrap().files,
        reversed,
        "a refused update changes nothing"
    );
}

#[test]
fn stacks_keep_their_profiles() {
    let (store, _, host) = setup(SecretBox::from_key(&[4; 32]));
    let s = store.create(&host, "shop", &one(COMPOSE), "").unwrap();
    assert!(s.profiles.is_empty());
    let s = store.set_profiles(&s.id, &["debug".into()]).unwrap();
    assert_eq!(s.profiles, ["debug"]);
    assert!(store.set_profiles(&s.id, &["no good".into()]).is_err());
}

#[test]
fn deleting_a_host_deletes_its_stacks() {
    let (store, hosts, host) = setup(SecretBox::from_key(&[4; 32]));
    let s = store.create(&host, "shop", &one(COMPOSE), "").unwrap();
    hosts.delete(&host).unwrap();
    assert!(matches!(store.get(&s.id), Err(ref e) if e.is(ErrorCode::NotFound)));
}

#[test]
fn swarm_stacks_have_names_of_their_own_and_outlive_any_host() {
    let (store, hosts, host) = setup(SecretBox::from_key(&[4; 32]));
    let swarm = StackOwner::Swarm("swarm1".into());
    store.create(&host, "shop", &one(COMPOSE), "").unwrap();
    let s = store.create(&swarm, "shop", &one(COMPOSE), "").unwrap();
    assert_eq!(s.owner, swarm);
    assert!(!s.registry_auth);
    assert!(store.create(&swarm, "shop", &one(COMPOSE), "").is_err());
    assert_eq!(store.list_of(&swarm).unwrap().len(), 1);
    assert!(store.set_registry_auth(&s.id, true).unwrap().registry_auth);
    hosts.delete(&host).unwrap();
    assert_eq!(store.get(&s.id).unwrap().name, "shop");
}

#[test]
fn env_needs_the_keychain_but_empty_env_does_not() {
    struct NoKeychain;
    impl KeyStore for NoKeychain {
        fn load(&self) -> Result<Option<homeport_storage::crypto::Key>, String> {
            Err("none".into())
        }
        fn create(&self) -> Result<homeport_storage::crypto::Key, String> {
            Err("none".into())
        }
    }
    let (store, _, host) = setup(SecretBox::from_store(&NoKeychain, false));
    assert!(store.create(&host, "plain", &one(COMPOSE), "  \n").is_ok());
    assert!(matches!(
        store.create(&host, "secret", &one(COMPOSE), "A=1"),
        Err(ref e) if e.is(ErrorCode::KeychainUnavailable)
    ));
}
