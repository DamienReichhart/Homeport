//! Syncing a Synced Stack on demand: fetching its ref, and what changed in
//! its compose file's folder between the deployed commit and the fetched one.

use serde::Serialize;

use homeport_core::error::AppResult;
use homeport_git::{ops, Commit, GitCli, RefKind};

use super::{path, SyncedStack, SyncedStacks};

/// Files larger than this, or with a NUL byte, are named but not compared.
const MAX_TEXT: usize = 512 * 1024;

/// One file under the compose file's folder that differs between two commits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct RepoFileChange {
    /// Relative to the repository's root.
    pub path: String,
    /// `None` when the file is new, or isn't text.
    pub before: Option<String>,
    /// `None` when the file was deleted, or isn't text.
    pub after: Option<String>,
    /// The file isn't text Homeport shows, or is too large to compare.
    pub binary: bool,
}

/// One file under a Synced Stack's compose file's folder, read-only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct RepoFile {
    /// Relative to the repository's root.
    pub path: String,
    /// `None` when it isn't text Homeport shows, or is too large.
    pub content: Option<String>,
}

/// A Synced Stack's files at its fetched commit, for its Definition tab.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SyncedFiles {
    pub commit: Commit,
    /// The files under the compose file's folder, the compose file first.
    pub files: Vec<RepoFile>,
}

/// What syncing would deploy, for the review before it does.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SyncReview {
    pub stack: SyncedStack,
    /// What is deployed now, `None` before the first deploy.
    pub deployed: Option<Commit>,
    /// What would be deployed: the ref as just fetched.
    pub fetched: Commit,
    /// The commits after the deployed one that change the compose file's
    /// folder, newest first, at most [`ops::LOG_LIMIT`].
    pub commits: Vec<Commit>,
    /// The files that changed under the compose file's folder, the compose
    /// file first.
    pub files: Vec<RepoFileChange>,
    /// Nothing under the compose file's folder changed since the deploy.
    pub up_to_date: bool,
    /// A tag now points at another commit than when it was last fetched.
    pub tag_moved: bool,
    /// A service is built from the repository's files, so the deploy
    /// rebuilds it, and a changed Dockerfile applies.
    pub builds: bool,
}

impl SyncedStacks {
    /// Fetches the Stack's ref and notes its commit. Returns the Stack as
    /// now stored, and whether a tag moved since it was last fetched.
    pub async fn fetch(&self, cli: &GitCli, id: &str) -> AppResult<(SyncedStack, bool)> {
        let stack = self.store.get(id)?;
        let commit = self
            .fetch_commit(cli, &stack.repository_id, stack.ref_kind, &stack.ref_name)
            .await?;
        if stack.fetched_commit.as_deref() == Some(commit.as_str()) {
            return Ok((stack, false));
        }
        let moved = stack.ref_kind == RefKind::Tag && stack.fetched_commit.is_some();
        Ok((self.note_fetched(cli, &stack, &commit).await?, moved))
    }

    /// Notes `commit` as the Stack's fetched one, with whether it changes
    /// the compose file's folder since the deployed commit: an update that's
    /// waiting. Before the first deploy, nothing is waiting.
    pub async fn note_fetched(
        &self,
        cli: &GitCli,
        stack: &SyncedStack,
        commit: &str,
    ) -> AppResult<SyncedStack> {
        let repo = self.repositories.clone_dir(&stack.repository_id);
        let folder = path::folder_of(&stack.compose_path);
        let available = match &stack.deployed_commit {
            None => false,
            Some(d) if d == commit => false,
            Some(d) if ops::has_commit(cli, &repo, d).await => {
                !ops::changed_files(cli, &repo, d, commit, folder)
                    .await?
                    .is_empty()
            }
            // The deployed commit is gone from the history: treat it as changed.
            Some(_) => true,
        };
        self.store.set_fetched(&stack.id, commit, available)
    }

    /// Fetches the Stack's ref and says what deploying it would change.
    pub async fn sync(&self, cli: &GitCli, id: &str) -> AppResult<SyncReview> {
        let (stack, tag_moved) = self.fetch(cli, id).await?;
        let mut review = self.review(cli, &stack).await?;
        review.tag_moved = tag_moved;
        Ok(review)
    }

    /// The Stack's files at its fetched commit, as they'd be deployed. Only
    /// reading: no command writes a Synced Stack's files, which change
    /// through git.
    pub async fn files(&self, cli: &GitCli, stack: &SyncedStack) -> AppResult<SyncedFiles> {
        let repo = self.repositories.clone_dir(&stack.repository_id);
        let commit = match &stack.fetched_commit {
            Some(c) if ops::has_commit(cli, &repo, c).await => c.clone(),
            _ => {
                self.fetch_commit(cli, &stack.repository_id, stack.ref_kind, &stack.ref_name)
                    .await?
            }
        };
        let folder = path::folder_of(&stack.compose_path);
        let paths = ops::list_files(cli, &repo, &commit, folder).await?;
        let mut files = Vec::with_capacity(paths.len());
        for p in ordered(paths, &stack.compose_path) {
            let (content, _) = text(ops::read_file(cli, &repo, &commit, &p).await?);
            files.push(RepoFile { path: p, content });
        }
        Ok(SyncedFiles {
            commit: ops::commit(cli, &repo, &commit).await?,
            files,
        })
    }

    /// What deploying the fetched commit would change, from what is stored.
    pub async fn review(&self, cli: &GitCli, stack: &SyncedStack) -> AppResult<SyncReview> {
        let repo = self.repositories.clone_dir(&stack.repository_id);
        let fetched = match &stack.fetched_commit {
            Some(c) if ops::has_commit(cli, &repo, c).await => c.clone(),
            _ => {
                self.fetch_commit(cli, &stack.repository_id, stack.ref_kind, &stack.ref_name)
                    .await?
            }
        };
        let folder = path::folder_of(&stack.compose_path);
        // A deployed commit the clone lost (a force-push, then a fresh clone)
        // compares as nothing deployed.
        let deployed = match &stack.deployed_commit {
            Some(c) if ops::has_commit(cli, &repo, c).await => Some(c.clone()),
            _ => None,
        };
        let paths = match &deployed {
            Some(d) => ops::changed_files(cli, &repo, d, &fetched, folder).await?,
            None => ops::list_files(cli, &repo, &fetched, folder).await?,
        };
        let mut files = Vec::with_capacity(paths.len());
        for p in ordered(paths, &stack.compose_path) {
            let before = match &deployed {
                Some(d) => ops::read_file(cli, &repo, d, &p).await?,
                None => None,
            };
            let after = ops::read_file(cli, &repo, &fetched, &p).await?;
            files.push(change(p, before, after));
        }
        let commits = ops::log(cli, &repo, deployed.as_deref(), &fetched, folder).await?;
        let compose = ops::read_file(cli, &repo, &fetched, &stack.compose_path)
            .await?
            .map(|b| String::from_utf8_lossy(&b).into_owned())
            .unwrap_or_default();
        let deployed_info = match &deployed {
            Some(d) => Some(ops::commit(cli, &repo, d).await?),
            None => None,
        };
        Ok(SyncReview {
            stack: stack.clone(),
            up_to_date: deployed.is_some() && files.is_empty(),
            deployed: deployed_info,
            fetched: ops::commit(cli, &repo, &fetched).await?,
            commits: if deployed.as_deref() == Some(fetched.as_str()) {
                vec![]
            } else {
                commits
            },
            files,
            tag_moved: false,
            builds: crate::definition::builds(&[&compose]),
        })
    }
}

/// The compose file first, then the rest in path order.
fn ordered(mut paths: Vec<String>, compose: &str) -> Vec<String> {
    paths.sort();
    if let Some(i) = paths.iter().position(|p| p == compose) {
        let first = paths.remove(i);
        paths.insert(0, first);
    }
    paths
}

fn text(bytes: Option<Vec<u8>>) -> (Option<String>, bool) {
    match bytes {
        None => (None, false),
        Some(b) if b.len() > MAX_TEXT || b.contains(&0) => (None, true),
        Some(b) => match String::from_utf8(b) {
            Ok(t) => (Some(t), false),
            Err(_) => (None, true),
        },
    }
}

fn change(path: String, before: Option<Vec<u8>>, after: Option<Vec<u8>>) -> RepoFileChange {
    let (before, binary_before) = text(before);
    let (after, binary_after) = text(after);
    RepoFileChange {
        path,
        before,
        after,
        binary: binary_before || binary_after,
    }
}

#[cfg(test)]
mod tests {
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
            lifecycle,
            runtime::{ComposeJob, StackTarget},
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
                    target: StackTarget::Host {
                        name: "local".into(),
                        connection: HostConnection::Local {
                            socket_path: "/var/run/docker.sock".into(),
                        },
                        dialer: None,
                        docker: testing::docker(),
                    },
                    logins: Logins::none(),
                    action: ComposeAction::Up,
                    profiles: vec![],
                    record: None,
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
}
