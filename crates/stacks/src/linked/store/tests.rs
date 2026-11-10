use homeport_hosts::{
    store::{HostDraft, HostStore},
    HostConnection,
};
use homeport_storage::SecretBox;

use super::*;
use crate::store::StackStore;

fn setup() -> (LinkedStore, StackStore, HostStore, String) {
    let db = Arc::new(Db::open_in_memory().unwrap());
    let secrets = Arc::new(SecretBox::from_key(&[4; 32]));
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
    (
        LinkedStore::new(db.clone()),
        StackStore::new(db, secrets),
        hosts,
        host.id,
    )
}

fn files() -> Vec<String> {
    vec![
        "/srv/shop/compose.yaml".into(),
        "/srv/shop/compose.override.yaml".into(),
    ]
}

#[test]
fn linked_stacks_keep_their_folder_and_files_in_order() {
    let (store, _, _, host) = setup();
    let s = store
        .create(&host, "shop", "/srv/shop", &files(), "fp1")
        .unwrap();
    assert_eq!(s.config_files, files());
    assert_eq!(s.working_dir, "/srv/shop");
    let s = store.set_fingerprint(&s.id, "fp2").unwrap();
    assert_eq!(s.fingerprint, "fp2");
    let reordered: Vec<String> = files().into_iter().rev().collect();
    let s = store.set_files(&s.id, &reordered, "fp3").unwrap();
    assert_eq!(
        (s.config_files.clone(), s.fingerprint.as_str()),
        (reordered, "fp3")
    );
    assert!(s.profiles.is_empty());
    let s = store.set_profiles(&s.id, &["debug".into()]).unwrap();
    assert_eq!(s.profiles, ["debug"]);
    assert_eq!(store.list(&host).unwrap(), std::slice::from_ref(&s));
    store.delete(&s.id).unwrap();
    assert!(matches!(store.get(&s.id), Err(ref e) if e.is(ErrorCode::NotFound)));
}

#[test]
fn names_are_unique_per_host_across_kinds() {
    let (linked, managed, _, host) = setup();
    managed
        .create(
            &host,
            "shop",
            &[crate::store::ComposeFile::new(
                "compose.yaml",
                "services: {}\n",
            )],
            "",
        )
        .unwrap();
    assert!(matches!(
        linked.create(&host, "shop", "/srv/shop", &files(), "fp"),
        Err(ref e) if e.is(ErrorCode::NameInUse) && e.message.contains("\"shop\"")
    ));
    linked
        .create(&host, "blog", "/srv/blog", &files(), "fp")
        .unwrap();
    assert!(matches!(
        managed.create(&host, "blog", &[crate::store::ComposeFile::new("compose.yaml", "services: {}\n")], ""),
        Err(ref e) if e.is(ErrorCode::NameInUse)
    ));
    assert!(matches!(
        linked.create(&host, "blog", "/srv/blog", &files(), "fp"),
        Err(ref e) if e.is(ErrorCode::NameInUse)
    ));
}

#[test]
fn deleting_a_host_forgets_its_linked_stacks() {
    let (store, _, hosts, host) = setup();
    let s = store
        .create(&host, "shop", "/srv/shop", &files(), "fp")
        .unwrap();
    hosts.delete(&host).unwrap();
    assert!(matches!(store.get(&s.id), Err(ref e) if e.is(ErrorCode::NotFound)));
}
