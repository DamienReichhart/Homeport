use homeport_core::error::ErrorCode;
use homeport_hosts::{
    store::{HostDraft, HostStore},
    HostConnection,
};
use homeport_storage::KeyStore;

use super::*;

fn setup(secrets: SecretBox) -> (DeployHistory, HostStore, StackOwner) {
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
    (
        DeployHistory::new(db, secrets),
        hosts,
        StackOwner::Host(host.id),
    )
}

fn deploy<'a>(host: &'a StackOwner, files: &'a [(String, String)], env: &'a str) -> NewDeploy<'a> {
    NewDeploy {
        owner: host,
        stack_id: "s1",
        kind: SourceKind::Managed,
        name: "shop",
        action: ComposeAction::Up,
        files,
        env,
        profiles: &[],
        commit: None,
        automatic: false,
    }
}

fn files(tag: &str) -> Vec<(String, String)> {
    vec![(
        "compose.yaml".into(),
        format!("services:\n  web:\n    image: nginx:{tag}\n"),
    )]
}

#[test]
fn deploys_keep_their_definition_and_a_sealed_env() {
    let (history, _, host) = setup(SecretBox::from_key(&[4; 32]));
    let f = files("1.27");
    let id = history
        .start(&deploy(&host, &f, "TOKEN=hunter2\n"))
        .unwrap();
    let running = history.get(&id).unwrap();
    assert_eq!(running.summary.finished_at, None);
    assert!(running.summary.has_env);
    assert_eq!(running.files[0].content, f[0].1);
    assert!(!serde_json::to_string(&running).unwrap().contains("hunter2"));
    assert_eq!(history.env(&id).unwrap(), "TOKEN=hunter2\n");
    assert_eq!(history.env_keys(&id).unwrap(), Some(vec!["TOKEN".into()]));

    history.finish(&id, true, Some(0)).unwrap();
    let done = history.get(&id).unwrap();
    assert_eq!(
        (done.summary.success, done.summary.exit_code),
        (Some(true), Some(0))
    );
    assert!(done.summary.finished_at.is_some());
}

#[test]
fn the_latest_success_is_what_is_deployed() {
    let (history, _, host) = setup(SecretBox::from_key(&[4; 32]));
    let (a, b, c) = (files("1"), files("2"), files("3"));
    let first = history.start(&deploy(&host, &a, "")).unwrap();
    history.finish(&first, true, Some(0)).unwrap();
    let second = history.start(&deploy(&host, &b, "")).unwrap();
    history.finish(&second, true, Some(0)).unwrap();
    let failed = history.start(&deploy(&host, &c, "")).unwrap();
    history.finish(&failed, false, Some(1)).unwrap();
    let _cancelled = history.start(&deploy(&host, &c, "")).unwrap();

    let latest = history.latest_successful("s1").unwrap().unwrap();
    assert_eq!(latest.summary.id, second);
    let listed: Vec<_> = history.list("s1").unwrap();
    assert_eq!(listed.len(), 4);
    assert_eq!(listed[1].id, failed, "newest first");
    assert_eq!(listed[0].finished_at, None);
    assert!(history.latest_successful("other").unwrap().is_none());
}

#[test]
fn only_the_last_twenty_are_kept() {
    let (history, _, host) = setup(SecretBox::from_key(&[4; 32]));
    let f = files("1");
    let ids: Vec<String> = (0..KEPT + 3)
        .map(|_| history.start(&deploy(&host, &f, "")).unwrap())
        .collect();
    let listed = history.list("s1").unwrap();
    assert_eq!(listed.len(), KEPT);
    assert_eq!(listed[0].id, ids[ids.len() - 1]);
    assert!(matches!(history.get(&ids[0]), Err(ref e) if e.is(ErrorCode::NotFound)));
}

#[test]
fn rolling_back_restores_the_definition_env_and_profiles() {
    let secrets = SecretBox::from_key(&[4; 32]);
    let (history, _, host) = setup(secrets);
    let stacks = StackStore::new(history.db.clone(), history.secrets.clone());
    let v1 = vec![ComposeFile::new(
        "compose.yaml",
        "services:\n  web:\n    image: nginx:1\n",
    )];
    let stack = stacks.create(&host, "shop", &v1, "TAG=one\n").unwrap();
    let files: Vec<(String, String)> = v1
        .iter()
        .map(|f| (f.name.clone(), f.content.clone()))
        .collect();
    let first = history
        .start(&NewDeploy {
            stack_id: &stack.id,
            profiles: &["debug".into()],
            ..deploy(&host, &files, "TAG=one\n")
        })
        .unwrap();
    history.finish(&first, true, Some(0)).unwrap();
    let v2 = vec![ComposeFile::new(
        "compose.yaml",
        "services:\n  web:\n    image: nginx:2\n",
    )];
    stacks.update(&stack.id, &v2, "TAG=two\n").unwrap();

    let restored = restore(&history, &stacks, &first).unwrap();
    assert_eq!(restored.files, v1);
    assert_eq!(restored.profiles, ["debug"]);
    assert_eq!(stacks.env(&stack.id).unwrap(), "TAG=one\n");

    // Forgetting the Stack forgets its history.
    stacks.delete(&stack.id).unwrap();
    assert!(history.list(&stack.id).unwrap().is_empty());
}

#[test]
fn linked_stacks_are_not_rolled_back() {
    let (history, _, host) = setup(SecretBox::from_key(&[4; 32]));
    let stacks = StackStore::new(history.db.clone(), history.secrets.clone());
    let f = files("1");
    let id = history
        .start(&NewDeploy {
            kind: SourceKind::Linked,
            ..deploy(&host, &f, "")
        })
        .unwrap();
    assert!(matches!(
        restore(&history, &stacks, &id),
        Err(ref e) if e.is(ErrorCode::Validation) && e.hint.is_some()
    ));
}

#[test]
fn synced_deploys_keep_their_commit_and_are_not_rolled_back() {
    let (history, _, host) = setup(SecretBox::from_key(&[4; 32]));
    let stacks = StackStore::new(history.db.clone(), history.secrets.clone());
    let f = files("1");
    let id = history
        .start(&NewDeploy {
            kind: SourceKind::Synced,
            commit: Some("0123456789abcdef"),
            ..deploy(&host, &f, "")
        })
        .unwrap();
    let summary = history.get(&id).unwrap().summary;
    assert_eq!(
        (summary.kind, summary.commit.as_deref()),
        (SourceKind::Synced, Some("0123456789abcdef"))
    );
    assert!(matches!(
        restore(&history, &stacks, &id),
        Err(ref e) if e.key == "errors.stacks.rollbackSynced"
    ));
}

#[test]
fn deleting_the_host_deletes_its_history() {
    let (history, hosts, host) = setup(SecretBox::from_key(&[4; 32]));
    let f = files("1");
    let id = history.start(&deploy(&host, &f, "")).unwrap();
    hosts.delete(host.id()).unwrap();
    assert!(history.get(&id).is_err());
}

#[test]
fn without_a_keychain_the_list_still_works() {
    struct NoKeychain;
    impl KeyStore for NoKeychain {
        fn load(&self) -> Result<Option<homeport_storage::crypto::Key>, String> {
            Err("none".into())
        }
        fn create(&self) -> Result<homeport_storage::crypto::Key, String> {
            Err("none".into())
        }
    }
    // Recorded with a key, then read on a machine without one.
    let db = Arc::new(Db::open_in_memory().unwrap());
    let with_key = Arc::new(SecretBox::from_key(&[4; 32]));
    let host = HostStore::new(db.clone(), with_key.clone())
        .create(HostDraft {
            name: "local".into(),
            connection: HostConnection::Local {
                socket_path: "/var/run/docker.sock".into(),
            },
            engine: homeport_hosts::EngineKind::Docker,
            secrets: None,
        })
        .unwrap()
        .id;
    let f = files("1");
    let id = DeployHistory::new(db.clone(), with_key)
        .start(&deploy(&StackOwner::Host(host), &f, "A=1"))
        .unwrap();
    let without = DeployHistory::new(db, Arc::new(SecretBox::from_store(&NoKeychain, false)));
    assert_eq!(without.list("s1").unwrap().len(), 1);
    assert!(without.get(&id).unwrap().summary.has_env);
    assert_eq!(without.env_keys(&id).unwrap(), None);
    assert!(without.env(&id).is_err());
}
