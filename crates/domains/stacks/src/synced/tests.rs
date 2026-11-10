use homeport_git::{cli, RefKind};

use super::{testing::Lab, *};
use crate::{store::StackStore, StackKind};

const COMPOSE: &str = "services:\n  web:\n    build: .\n    image: shop:dev\n";

fn draft(name: &str, path: &str, ref_name: &str) -> SyncedDraft {
    SyncedDraft {
        name: name.into(),
        repository_id: "r1".into(),
        ref_kind: RefKind::Branch,
        ref_name: ref_name.into(),
        compose_path: path.into(),
    }
}

#[tokio::test]
async fn a_synced_stack_is_created_from_a_local_repository() {
    let git = cli::detect().await.unwrap();
    let lab = Lab::new();
    let commit = lab.commit("First", &[("deploy/compose.yaml", COMPOSE)]);

    let stack = lab
        .synced
        .create(
            &git,
            &lab.host,
            &draft("shop", "./deploy/compose.yaml", "main"),
            "TOKEN=hunter2\n",
        )
        .await
        .unwrap();
    assert_eq!(stack.compose_path, "deploy/compose.yaml");
    assert_eq!(stack.fetched_commit.as_deref(), Some(commit.as_str()));
    assert_eq!(stack.deployed_commit, None);
    assert!(stack.has_env);
    assert_eq!(lab.synced.store.env(&stack.id).unwrap(), "TOKEN=hunter2\n");
    assert!(!serde_json::to_string(&stack).unwrap().contains("hunter2"));
    assert_eq!(
        lab.synced.store.list(&lab.host).unwrap(),
        std::slice::from_ref(&stack)
    );

    // Deploying lays the commit out in the Stack's worktree, and compose
    // runs from the compose file's folder.
    let (source, used) = lab
        .synced
        .source(&git, &stack, ComposeAction::Up, None)
        .await
        .unwrap();
    assert_eq!(used, commit);
    let StackSource::Synced {
        working_dir,
        config_files,
        env,
    } = source
    else {
        panic!("not synced");
    };
    assert_eq!(working_dir, lab.synced.worktree(&stack.id).join("deploy"));
    assert_eq!(std::fs::read_to_string(&config_files[0]).unwrap(), COMPOSE);
    assert_eq!(env, "TOKEN=hunter2\n");

    // Names are unique across kinds.
    let managed = StackStore::new(lab.db.clone(), lab.secrets.clone());
    assert!(matches!(
        managed.create(&lab.host, "shop", &[crate::store::ComposeFile::new("compose.yaml", "services: {}\n")], ""),
        Err(ref e) if e.is(ErrorCode::NameInUse)
    ));

    // Forgetting removes the record, the worktree and the unused clone.
    lab.synced.forget(Some(&git), &stack.id).await.unwrap();
    assert!(lab.synced.store.get(&stack.id).is_err());
    assert!(!lab.synced.worktree(&stack.id).exists());
    assert!(!lab.synced.repositories.clone_dir("r1").exists());
    let derived = crate::derive(&[], &[]);
    assert!(derived.iter().all(|s| s.kind != StackKind::Synced));
}

#[tokio::test]
async fn a_wrong_path_or_ref_is_reported_before_anything_is_stored() {
    let git = cli::detect().await.unwrap();
    let lab = Lab::new();
    lab.commit(
        "First",
        &[("compose.yaml", COMPOSE), ("notes.yaml", "just: notes\n")],
    );
    let refused = |e: &AppError| e.is(ErrorCode::SyncedComposeFileMissing);
    let e = lab
        .synced
        .create(
            &git,
            &lab.host,
            &draft("a", "nope/compose.yaml", "main"),
            "",
        )
        .await
        .unwrap_err();
    assert!(refused(&e), "{e:?}");
    let e = lab
        .synced
        .create(&git, &lab.host, &draft("a", "compose.yaml", "release"), "")
        .await
        .unwrap_err();
    assert!(refused(&e), "{e:?}");
    for path in ["../compose.yaml", "/etc/compose.yaml"] {
        let e = lab
            .synced
            .create(&git, &lab.host, &draft("a", path, "main"), "")
            .await
            .unwrap_err();
        assert!(e.is(ErrorCode::Validation), "{path}: {e:?}");
    }
    let e = lab
        .synced
        .create(&git, &lab.host, &draft("a", "notes.yaml", "main"), "")
        .await
        .unwrap_err();
    assert!(
        e.is(ErrorCode::ComposeInvalidDefinition) || e.is(ErrorCode::Validation),
        "{e:?}"
    );
    assert!(lab.synced.store.list(&lab.host).unwrap().is_empty());
}

#[tokio::test]
async fn a_tag_can_be_followed() {
    let git = cli::detect().await.unwrap();
    let lab = Lab::new();
    let commit = lab.commit("First", &[("compose.yaml", COMPOSE)]);
    git_tag(&lab, "v1");
    let stack = lab
        .synced
        .create(
            &git,
            &lab.host,
            &SyncedDraft {
                ref_kind: RefKind::Tag,
                ..draft("shop", "compose.yaml", "v1")
            },
            "",
        )
        .await
        .unwrap();
    assert_eq!(stack.fetched_commit, Some(commit));
    assert_eq!(stack.ref_kind, RefKind::Tag);
}

fn git_tag(lab: &Lab, tag: &str) {
    let work = lab.dir.path().join("work");
    testing::git(&work, &["tag", tag]);
    let origin = lab.dir.path().join("origin.git");
    testing::git(&work, &["push", "--quiet", &origin.to_string_lossy(), tag]);
}

#[test]
fn deleting_the_host_forgets_its_synced_stacks() {
    let lab = Lab::new();
    let stack = lab
        .synced
        .store
        .create(&lab.host, &draft("shop", "compose.yaml", "main"), "", "abc")
        .unwrap();
    let hosts = homeport_hosts::store::HostStore::new(lab.db.clone(), lab.secrets.clone());
    hosts.delete(&lab.host).unwrap();
    assert!(lab.synced.store.get(&stack.id).is_err());
}
