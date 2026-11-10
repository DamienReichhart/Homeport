use super::{testing::*, *};
use crate::{cli, store::Secret};

fn remote(url: &str) -> Remote<'_> {
    Remote {
        url,
        secret: &Secret::None,
        host_key: None,
    }
}

#[test]
fn ls_remote_lists_branches_tags_and_the_default_branch() {
    let refs = parse_ls_remote(
        "ref: refs/heads/main\tHEAD\n\
             aaa\tHEAD\n\
             aaa\trefs/heads/main\n\
             bbb\trefs/heads/dev\n\
             ccc\trefs/tags/v1\n\
             aaa\trefs/tags/v1^{}\n\
             ddd\trefs/pull/1/head\n",
    );
    assert_eq!(refs.branches, ["dev", "main"]);
    assert_eq!(refs.tags, ["v1"]);
    assert_eq!(refs.default_branch.as_deref(), Some("main"));
}

#[tokio::test]
async fn a_local_repository_is_cloned_fetched_read_and_checked_out() {
    let cli = cli::detect().await.unwrap();
    let up = Upstream::new();
    let first = up.commit(
        "First",
        &[
            (
                "app/compose.yaml",
                "services:\n  web:\n    image: nginx:1\n",
            ),
            ("app/Dockerfile", "FROM nginx\n"),
            ("README.md", "hi\n"),
        ],
    );
    git(&up.work, &["tag", "v1"]);
    git(&up.work, &["push", "--quiet", &up.url(), "v1"]);
    let url = up.url();

    let (refs, learned) = list_remote(&cli, &remote(&url)).await.unwrap();
    assert_eq!(
        (refs.branches.as_slice(), refs.tags.as_slice()),
        (&["main".to_owned()][..], &["v1".to_owned()][..])
    );
    assert_eq!(learned, None, "no SSH server, no key");

    let data = tempfile::tempdir().unwrap();
    let repo = data.path().join("repositories/r.git");
    fetch(
        &cli,
        &repo,
        &remote(&url),
        &[(RefKind::Branch, "main"), (RefKind::Tag, "v1")],
    )
    .await
    .unwrap();
    assert_eq!(
        resolve(&cli, &repo, RefKind::Branch, "main")
            .await
            .unwrap()
            .as_deref(),
        Some(first.as_str())
    );
    assert_eq!(
        resolve(&cli, &repo, RefKind::Tag, "v1")
            .await
            .unwrap()
            .as_deref(),
        Some(first.as_str())
    );
    assert_eq!(
        resolve(&cli, &repo, RefKind::Branch, "nope").await.unwrap(),
        None
    );
    assert_eq!(
        read_file(&cli, &repo, &first, "app/compose.yaml")
            .await
            .unwrap()
            .unwrap(),
        b"services:\n  web:\n    image: nginx:1\n"
    );
    assert_eq!(
        read_file(&cli, &repo, &first, "app").await.unwrap(),
        None,
        "a folder"
    );
    assert_eq!(
        read_file(&cli, &repo, &first, "nope.yaml").await.unwrap(),
        None
    );
    assert_eq!(
        list_files(&cli, &repo, &first, "app").await.unwrap(),
        ["app/Dockerfile", "app/compose.yaml"]
    );

    let second = up.commit(
        "Bump nginx",
        &[(
            "app/compose.yaml",
            "services:\n  web:\n    image: nginx:2\n",
        )],
    );
    let third = up.commit("Docs only", &[("README.md", "hello\n")]);
    fetch(&cli, &repo, &remote(&url), &[(RefKind::Branch, "main")])
        .await
        .unwrap();
    assert_eq!(
        resolve(&cli, &repo, RefKind::Branch, "main")
            .await
            .unwrap()
            .as_deref(),
        Some(third.as_str())
    );
    let commits = log(&cli, &repo, Some(&first), &third, "app").await.unwrap();
    assert_eq!(commits.len(), 1, "only commits under the folder");
    assert_eq!(commits[0].hash, second);
    assert_eq!(
        (commits[0].subject.as_str(), commits[0].author.as_str()),
        ("Bump nginx", "Ada")
    );
    assert_eq!(
        changed_files(&cli, &repo, &first, &third, "app")
            .await
            .unwrap(),
        ["app/compose.yaml"]
    );
    assert!(changed_files(&cli, &repo, &second, &third, "app")
        .await
        .unwrap()
        .is_empty());
    assert_eq!(log(&cli, &repo, None, &third, "").await.unwrap().len(), 3);
    assert_eq!(
        commit(&cli, &repo, &second).await.unwrap().subject,
        "Bump nginx"
    );

    // A worktree per Stack, kept in place across deploys.
    let worktree = data.path().join("synced-stacks/s1");
    checkout(&cli, &repo, &worktree, &first).await.unwrap();
    let compose = worktree.join("app/compose.yaml");
    assert!(std::fs::read_to_string(&compose)
        .unwrap()
        .contains("nginx:1"));
    std::fs::write(worktree.join("app/data.db"), "written by a container").unwrap();
    checkout(&cli, &repo, &worktree, &second).await.unwrap();
    assert!(std::fs::read_to_string(&compose)
        .unwrap()
        .contains("nginx:2"));
    assert!(
        worktree.join("app/data.db").exists(),
        "untracked files stay"
    );
    assert!(
        !remove_worktree(&cli, &repo, &worktree).await.unwrap(),
        "kept with data"
    );
    std::fs::remove_file(worktree.join("app/data.db")).unwrap();
    assert!(remove_worktree(&cli, &repo, &worktree).await.unwrap());
    assert!(!worktree.exists());
}

#[tokio::test]
async fn checking_out_never_discards_what_a_container_wrote_to_a_tracked_file() {
    let cli = cli::detect().await.unwrap();
    let up = Upstream::new();
    // Traefik's `acme.json` is often committed empty, then filled with
    // certificates by the container through a relative bind mount.
    let first = up.commit(
        "First",
        &[
            ("compose.yaml", "services:\n  web:\n    image: nginx:1\n"),
            ("acme.json", ""),
        ],
    );
    let data = tempfile::tempdir().unwrap();
    let repo = data.path().join("r.git");
    let url = up.url();
    fetch(&cli, &repo, &remote(&url), &[(RefKind::Branch, "main")])
        .await
        .unwrap();
    let worktree = data.path().join("synced-stacks/s1");
    checkout(&cli, &repo, &worktree, &first).await.unwrap();
    let acme = worktree.join("acme.json");
    std::fs::write(&acme, "{\"certificates\": [\"written by a container\"]}").unwrap();

    // Redeploying the same commit, then a commit that leaves the file alone.
    checkout(&cli, &repo, &worktree, &first).await.unwrap();
    assert!(std::fs::read_to_string(&acme)
        .unwrap()
        .contains("written by a container"));
    let second = up.commit(
        "Bump nginx",
        &[("compose.yaml", "services:\n  web:\n    image: nginx:2\n")],
    );
    fetch(&cli, &repo, &remote(&url), &[(RefKind::Branch, "main")])
        .await
        .unwrap();
    checkout(&cli, &repo, &worktree, &second).await.unwrap();
    assert!(std::fs::read_to_string(worktree.join("compose.yaml"))
        .unwrap()
        .contains("nginx:2"));
    assert!(std::fs::read_to_string(&acme)
        .unwrap()
        .contains("written by a container"));

    // A commit that changes the file too is refused, and the file kept.
    let third = up.commit("Reset acme", &[("acme.json", "{}")]);
    fetch(&cli, &repo, &remote(&url), &[(RefKind::Branch, "main")])
        .await
        .unwrap();
    assert!(checkout(&cli, &repo, &worktree, &third).await.is_err());
    assert!(std::fs::read_to_string(&acme)
        .unwrap()
        .contains("written by a container"));
}

#[tokio::test]
async fn a_missing_branch_is_named() {
    let cli = cli::detect().await.unwrap();
    let up = Upstream::new();
    up.commit("First", &[("compose.yaml", "services: {}\n")]);
    let data = tempfile::tempdir().unwrap();
    let url = up.url();
    let e = fetch(
        &cli,
        &data.path().join("r.git"),
        &remote(&url),
        &[(RefKind::Branch, "release")],
    )
    .await
    .unwrap_err();
    assert!(e.is(ErrorCode::SyncedComposeFileMissing));
    assert_eq!(e.message, "The repository has no branch named release.");
}

#[tokio::test]
async fn a_moved_tag_is_followed() {
    let cli = cli::detect().await.unwrap();
    let up = Upstream::new();
    up.commit("First", &[("compose.yaml", "services: {}\n")]);
    git(&up.work, &["tag", "stable"]);
    git(&up.work, &["push", "--quiet", &up.url(), "stable"]);
    let data = tempfile::tempdir().unwrap();
    let repo = data.path().join("r.git");
    let url = up.url();
    fetch(&cli, &repo, &remote(&url), &[(RefKind::Tag, "stable")])
        .await
        .unwrap();
    let second = up.commit(
        "Second",
        &[("compose.yaml", "services:\n  a:\n    image: x\n")],
    );
    git(&up.work, &["tag", "--force", "stable"]);
    git(
        &up.work,
        &["push", "--quiet", "--force", &up.url(), "stable"],
    );
    fetch(&cli, &repo, &remote(&url), &[(RefKind::Tag, "stable")])
        .await
        .unwrap();
    assert_eq!(
        resolve(&cli, &repo, RefKind::Tag, "stable").await.unwrap(),
        Some(second)
    );
}

/// Set `HOMEPORT_TEST_GIT_SSH_URL` to a repository on an SSH server and
/// `HOMEPORT_TEST_GIT_SSH_KEY` to the path of a private key it accepts.
#[tokio::test]
#[ignore = "needs a git SSH server, named by HOMEPORT_TEST_GIT_SSH_URL and HOMEPORT_TEST_GIT_SSH_KEY"]
async fn a_deploy_key_reaches_an_ssh_server_whose_key_is_trusted_on_first_use() {
    let (Ok(url), Ok(key)) = (
        std::env::var("HOMEPORT_TEST_GIT_SSH_URL"),
        std::env::var("HOMEPORT_TEST_GIT_SSH_KEY"),
    ) else {
        eprintln!("skipped: no SSH server named");
        return;
    };
    let cli = cli::detect().await.unwrap();
    let secret = Secret::SshKey {
        private_key: std::fs::read_to_string(key).unwrap(),
    };
    let first = Remote {
        url: &url,
        secret: &secret,
        host_key: None,
    };
    let (_, learned) = list_remote(&cli, &first).await.unwrap();
    let learned = learned.expect("the server's key, trusted on first use");

    let trusted = Remote {
        host_key: Some(&learned),
        ..first
    };
    let (_, again) = list_remote(&cli, &trusted).await.unwrap();
    assert_eq!(again, None);
    let data = tempfile::tempdir().unwrap();
    let repo = data.path().join("r.git");
    fetch(&cli, &repo, &trusted, &[(RefKind::Branch, "main")])
        .await
        .unwrap();

    let (host, _) = learned.split_once(' ').unwrap();
    let other = homeport_transport::ssh::generate_key("other").public_key;
    let wrong = format!(
        "{host} {}",
        other.split(' ').take(2).collect::<Vec<_>>().join(" ")
    );
    let e = list_remote(
        &cli,
        &Remote {
            host_key: Some(&wrong),
            ..first
        },
    )
    .await
    .unwrap_err();
    assert!(e.is(ErrorCode::GitHostKeyChanged), "{e:?}");

    let stranger = Secret::SshKey {
        private_key: homeport_transport::ssh::generate_key("stranger").private_key,
    };
    let e = list_remote(
        &cli,
        &Remote {
            secret: &stranger,
            ..trusted
        },
    )
    .await
    .unwrap_err();
    assert!(e.is(ErrorCode::GitAuthFailed), "{e:?}");
}
