use homeport_git::cli;

use super::*;
use crate::{compose::ComposeAction, synced::testing::Lab, synced::SyncedDraft};

const V1: &str = "services:\n  web:\n    image: nginx:1.27\n";
const V2: &str = "services:\n  web:\n    image: nginx:1.28\n";

async fn created(lab: &Lab, git: &GitCli) -> SyncedStack {
    lab.synced
        .create(
            git,
            &lab.host,
            &SyncedDraft {
                name: "shop".into(),
                repository_id: "r1".into(),
                ref_kind: RefKind::Branch,
                ref_name: "main".into(),
                compose_path: "app/compose.yaml".into(),
            },
            "",
        )
        .await
        .unwrap()
}

#[tokio::test]
async fn a_sync_shows_the_commits_and_changed_files_since_the_deploy() {
    let git = cli::detect().await.unwrap();
    let lab = Lab::new();
    let first = lab.commit(
        "First",
        &[("app/compose.yaml", V1), ("app/.env.example", "A=1\n")],
    );
    let stack = created(&lab, &git).await;

    // Before the first deploy, every file under the folder is new.
    let review = lab.synced.sync(&git, &stack.id).await.unwrap();
    assert!(!review.up_to_date);
    assert!(!review.builds);
    assert_eq!(review.deployed, None);
    assert_eq!(review.fetched.hash, first);
    let paths: Vec<_> = review.files.iter().map(|f| f.path.as_str()).collect();
    assert_eq!(paths, ["app/compose.yaml", "app/.env.example"]);
    lab.synced.store.set_deployed(&stack.id, &first).unwrap();

    // A push that changes the image tag.
    let second = lab.commit("Bump nginx", &[("app/compose.yaml", V2)]);
    let review = lab.synced.sync(&git, &stack.id).await.unwrap();
    assert!(!review.up_to_date);
    assert_eq!(review.fetched.hash, second);
    assert_eq!(
        review.deployed.as_ref().map(|c| c.hash.as_str()),
        Some(first.as_str())
    );
    assert_eq!(review.commits.len(), 1);
    assert_eq!(review.commits[0].subject, "Bump nginx");
    assert_eq!(
        review.files,
        [RepoFileChange {
            path: "app/compose.yaml".into(),
            before: Some(V1.into()),
            after: Some(V2.into()),
            binary: false,
        }]
    );
    let stored = lab.synced.store.get(&stack.id).unwrap();
    assert_eq!(stored.fetched_commit, Some(second.clone()));
    assert!(stored.update_available);

    // Deploying it checks the new commit out in the same worktree.
    let stack = lab.synced.store.set_deployed(&stack.id, &second).unwrap();
    assert!(!stack.update_available);
    let (_, used) = lab
        .synced
        .source(&git, &stack, ComposeAction::Up, None)
        .await
        .unwrap();
    assert_eq!(used, second);
    let compose = lab.synced.worktree(&stack.id).join("app/compose.yaml");
    assert_eq!(std::fs::read_to_string(compose).unwrap(), V2);

    // A push outside the folder changes nothing to deploy.
    lab.commit("Docs", &[("README.md", "hello\n")]);
    let review = lab.synced.sync(&git, &stack.id).await.unwrap();
    assert!(review.up_to_date);
    assert!(review.files.is_empty() && review.commits.is_empty());
    assert!(!review.stack.update_available, "nothing to deploy");
}

#[tokio::test]
async fn the_definition_shows_the_folder_at_the_fetched_commit_compose_file_first() {
    let git = cli::detect().await.unwrap();
    let lab = Lab::new();
    let first = lab.commit(
        "First",
        &[
            ("app/compose.yaml", V1),
            ("app/Dockerfile", "FROM nginx\n"),
            ("app/logo.png", "\u{0}PNG"),
            ("README.md", "outside the folder\n"),
        ],
    );
    let stack = created(&lab, &git).await;
    let shown = lab.synced.files(&git, &stack).await.unwrap();
    assert_eq!(shown.commit.hash, first);
    let paths: Vec<_> = shown.files.iter().map(|f| f.path.as_str()).collect();
    assert_eq!(
        paths,
        ["app/compose.yaml", "app/Dockerfile", "app/logo.png"]
    );
    assert_eq!(shown.files[0].content.as_deref(), Some(V1));
    assert_eq!(shown.files[2].content, None);
}

#[tokio::test]
async fn moving_the_compose_file_shows_it_removed_and_binaries_by_name() {
    let git = cli::detect().await.unwrap();
    let lab = Lab::new();
    let first = lab.commit("First", &[("app/compose.yaml", V1)]);
    let stack = created(&lab, &git).await;
    lab.synced.store.set_deployed(&stack.id, &first).unwrap();
    let work = lab.dir.path().join("work");
    crate::synced::testing::git(&work, &["rm", "--quiet", "app/compose.yaml"]);
    lab.commit(
        "Move it",
        &[("app/compose.yml", V1), ("app/logo.png", "\u{0}PNG")],
    );
    let review = lab.synced.sync(&git, &stack.id).await.unwrap();
    let by_path = |p: &str| {
        review
            .files
            .iter()
            .find(|f| f.path == p)
            .unwrap_or_else(|| panic!("{p} in {:?}", review.files))
    };
    assert_eq!(by_path("app/compose.yaml").after, None);
    assert!(by_path("app/logo.png").binary);
    assert_eq!(by_path("app/compose.yml").before, None);
}

#[tokio::test]
async fn a_failed_fetch_leaves_the_stack_as_it_was() {
    let git = cli::detect().await.unwrap();
    let lab = Lab::new();
    let first = lab.commit("First", &[("app/compose.yaml", V1)]);
    let stack = created(&lab, &git).await;
    std::fs::remove_dir_all(lab.dir.path().join("origin.git")).unwrap();
    assert!(lab.synced.sync(&git, &stack.id).await.is_err());
    let after = lab.synced.store.get(&stack.id).unwrap();
    assert_eq!(after.fetched_commit, Some(first));
    assert_eq!(after.deployed_commit, None);
}

/// Deploys commit 1 of a repository whose service is built, then syncs
/// to commit 2 and deploys it from the same worktree.
#[tokio::test]
#[ignore = "needs a local Docker Engine and the compose CLI"]
async fn a_synced_stack_deploys_then_syncs_to_a_new_commit_with_a_build() {
    use homeport_core::host::HostConnection;
    use homeport_engine::{auth::Logins, testing};

    use crate::{
        compose::{detect, ComposeEvent},
        runtime::{lifecycle, ComposeJob, StackTarget},
    };

    let git = cli::detect().await.unwrap();
    let docker = testing::docker();
    testing::ensure_image(&docker).await;
    let project = testing::unique("synced");
    let lab = Lab::new();
    let compose = |greeting: &str| {
        format!(
                "services:\n  app:\n    build: .\n    environment:\n      GREETING: {greeting}\n      SECRET: ${{SECRET}}\n"
            )
    };
    let dockerfile = format!("FROM {}\nCMD [\"sleep\", \"60\"]\n", testing::IMAGE);
    lab.commit(
        "First",
        &[
            ("app/Dockerfile", &dockerfile),
            ("app/compose.yaml", &compose("one")),
        ],
    );
    let stack = lab
        .synced
        .create(
            &git,
            &lab.host,
            &SyncedDraft {
                name: project.clone(),
                repository_id: "r1".into(),
                ref_kind: RefKind::Branch,
                ref_name: "main".into(),
                compose_path: "app/compose.yaml".into(),
            },
            "SECRET=hunter2\n",
        )
        .await
        .unwrap();
    let deploy = |stack: SyncedStack, commit: Option<String>| {
        let synced = &lab.synced;
        let git = &git;
        let project = project.clone();
        async move {
            let (source, used) = synced
                .source(git, &stack, ComposeAction::Up, commit.as_deref())
                .await
                .unwrap();
            let job = ComposeJob {
                cli: detect().await.expect("docker compose is installed"),
                project,
                source,
                target: StackTarget::Host(crate::runtime::Reach {
                    name: "local".into(),
                    connection: HostConnection::Local {
                        socket_path: "/var/run/docker.sock".into(),
                    },
                    dialer: None,
                    docker: testing::docker(),
                }),
                logins: Logins::none(),
                action: ComposeAction::Up,
                profiles: vec![],
                record: None,
                registry_auth: false,
            };
            let events = std::sync::Mutex::new(Vec::new());
            crate::runtime::run(job, |e| {
                events.lock().unwrap().push(e);
                true
            })
            .await;
            let events = events.into_inner().unwrap();
            assert!(
                matches!(
                    events.last(),
                    Some(ComposeEvent::Exit { success: true, .. })
                ),
                "{events:?}"
            );
            synced.store.set_deployed(&stack.id, &used).unwrap()
        }
    };
    let greeting = || async {
        let running = lifecycle::containers_of(&docker, &project).await.unwrap();
        let inspect = docker
            .inspect_container(&running[0].id, None)
            .await
            .unwrap();
        inspect.config.unwrap().env.unwrap()
    };

    let stack = deploy(stack, None).await;
    let env = greeting().await;
    assert!(env.contains(&"GREETING=one".to_owned()), "{env:?}");
    assert!(env.contains(&"SECRET=hunter2".to_owned()), "{env:?}");

    let second = lab.commit("Say two", &[("app/compose.yaml", &compose("two"))]);
    let review = lab.synced.sync(&git, &stack.id).await.unwrap();
    assert_eq!(review.fetched.hash, second);
    assert!(review.builds);
    let stack = deploy(review.stack, Some(review.fetched.hash)).await;
    assert_eq!(stack.deployed_commit, Some(second));
    let env = greeting().await;
    lifecycle::remove(&docker, &project).await.unwrap();
    let _ = docker
        .remove_image(
            &format!("{project}-app"),
            None::<bollard::query_parameters::RemoveImageOptions>,
            None,
        )
        .await;
    assert!(env.contains(&"GREETING=two".to_owned()), "{env:?}");
}
