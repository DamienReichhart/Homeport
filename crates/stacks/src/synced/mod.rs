//! Synced Stacks (ADR 0014): Stacks whose compose file comes from a git
//! repository. Homeport fetches the repository into its clone on this
//! machine and deploys a commit from a worktree of it, with the compose
//! file's folder as the project directory, so `build:` and relative paths
//! work as in a Linked Stack. Nothing here edits, commits or pushes.
//!
//! - [`store`] keeps each Synced Stack's repository, ref, path and commits.
//! - [`path`] checks a compose file's path inside the repository.
//! - [`sync`] fetches a Synced Stack's ref and says what changed since its
//!   deploy, for the review before deploying it.
//! - [`poller`] fetches every Synced Stack's ref on an interval while
//!   Homeport runs, and deploys new commits for those that ask it to.

pub mod path;
pub mod poller;
pub mod store;
pub mod sync;

use std::{path::PathBuf, sync::Arc};

use homeport_core::error::{AppError, AppResult, ErrorCode};
use homeport_git::{ops, GitCli, Repositories};

use crate::{
    compose::ComposeAction, definition, messages as msg, runtime::SourceKind, runtime::StackSource,
};

pub use self::{
    store::{SyncedDraft, SyncedStack, SyncedStore},
    sync::{RepoFile, RepoFileChange, SyncReview, SyncedFiles},
};

/// Synced Stacks, their repositories' clones and their worktrees.
pub struct SyncedStacks {
    pub store: SyncedStore,
    pub repositories: Arc<Repositories>,
    /// Where each Synced Stack's worktree lives: `<root>/<id>`.
    worktrees: PathBuf,
}

impl SyncedStacks {
    pub fn new(
        store: SyncedStore,
        repositories: Arc<Repositories>,
        worktrees: impl Into<PathBuf>,
    ) -> Self {
        Self {
            store,
            repositories,
            worktrees: worktrees.into(),
        }
    }

    /// The Stack's worktree on this machine, kept in place across deploys.
    pub fn worktree(&self, id: &str) -> PathBuf {
        self.worktrees.join(id)
    }

    /// Creates a Synced Stack on a Host or a Swarm (`owner`): fetches its
    /// ref, and checks the compose file is there and can be deployed, before
    /// anything is stored.
    pub async fn create(
        &self,
        cli: &GitCli,
        owner: impl Into<crate::StackOwner>,
        draft: &SyncedDraft,
        env: &str,
    ) -> AppResult<SyncedStack> {
        definition::validate_name(draft.name.trim())?;
        definition::parse_env(env)?;
        let draft = SyncedDraft {
            compose_path: path::normalise(&draft.compose_path)?,
            ref_name: draft.ref_name.trim().to_owned(),
            ..draft.clone()
        };
        if draft.ref_name.is_empty() {
            return Err(AppError::validation(msg::SYNCED_REF_EMPTY));
        }
        let commit = self
            .fetch_commit(cli, &draft.repository_id, draft.ref_kind, &draft.ref_name)
            .await?;
        self.compose_file(cli, &draft.repository_id, &commit, &draft.compose_path)
            .await?;
        self.store.create(owner, &draft, env, &commit)
    }

    /// Fetches the ref into the repository's clone and returns its commit.
    pub async fn fetch_commit(
        &self,
        cli: &GitCli,
        repository_id: &str,
        kind: homeport_git::RefKind,
        name: &str,
    ) -> AppResult<String> {
        let repo = self
            .repositories
            .fetch(cli, repository_id, &[(kind, name)])
            .await?;
        ops::resolve(cli, &repo, kind, name)
            .await?
            .ok_or_else(|| ops::ref_missing(&kind.full(name)))
    }

    /// The compose file at `path` in `commit`, checked against the Synced
    /// rules: it must be there, be text, and be deployable.
    pub async fn compose_file(
        &self,
        cli: &GitCli,
        repository_id: &str,
        commit: &str,
        path: &str,
    ) -> AppResult<String> {
        let repo = self.repositories.clone_dir(repository_id);
        let short = &commit[..commit.len().min(7)];
        let missing = || {
            AppError::new(
                ErrorCode::SyncedComposeFileMissing,
                msg::SYNCED_FILE_MISSING
                    .with("path", path)
                    .with("commit", short),
            )
            .with_hint(msg::SYNCED_FILE_MISSING_HINT)
        };
        let bytes = ops::read_file(cli, &repo, commit, path)
            .await?
            .ok_or_else(missing)?;
        let text = String::from_utf8(bytes).map_err(|_| missing())?;
        if let Some(blocker) = definition::blocker(&[&text], SourceKind::Synced) {
            return Err(blocker);
        }
        Ok(text)
    }

    /// Where `stack` runs compose from for `action`, with the commit it uses:
    /// `commit` when a sync deploys the one it reviewed, otherwise the
    /// deployed commit, or before the first deploy the fetched one. A deploy
    /// checks its commit out first; anything else uses the worktree as it
    /// is, unless there's none yet.
    pub async fn source(
        &self,
        cli: &GitCli,
        stack: &SyncedStack,
        action: ComposeAction,
        commit: Option<&str>,
    ) -> AppResult<(StackSource, String)> {
        let worktree = self.worktree(&stack.id);
        let commit = commit
            .map(str::to_owned)
            .or_else(|| stack.deployed_commit.clone())
            .or_else(|| stack.fetched_commit.clone())
            .ok_or_else(|| AppError::validation(msg::SYNCED_NOT_FETCHED))?;
        if action.deploys() || !worktree.join(".git").exists() {
            let repo = self.repositories.clone_dir(&stack.repository_id);
            if !ops::has_commit(cli, &repo, &commit).await {
                self.fetch_commit(cli, &stack.repository_id, stack.ref_kind, &stack.ref_name)
                    .await?;
            }
            ops::checkout(cli, &repo, &worktree, &commit).await?;
        }
        let folder = path::folder_of(&stack.compose_path);
        let working_dir = if folder.is_empty() {
            worktree.clone()
        } else {
            worktree.join(folder)
        };
        let config = worktree.join(&stack.compose_path);
        Ok((
            StackSource::Synced {
                working_dir,
                config_files: vec![config.to_string_lossy().into_owned()],
                env: self.store.env(&stack.id)?,
            },
            commit,
        ))
    }

    /// Forgets the Synced Stack: its record, its history and its worktree
    /// (unless files its containers wrote there would go with it), and the
    /// repository's clone once no Synced Stack uses it. Its containers are
    /// left alone, so it shows as External.
    pub async fn forget(&self, cli: Option<&GitCli>, id: &str) -> AppResult<()> {
        let stack = self.store.get(id)?;
        self.store.delete(id)?;
        let worktree = self.worktree(id);
        let repo = self.repositories.clone_dir(&stack.repository_id);
        if let Some(cli) = cli {
            match ops::remove_worktree(cli, &repo, &worktree).await {
                Ok(true) => {}
                Ok(false) => log::warn!(
                    "kept {} after forgetting its Synced Stack: it holds files git doesn't track",
                    worktree.display()
                ),
                Err(e) => {
                    e.logged("removing a Synced Stack's worktree");
                }
            }
        }
        if self.store.using(&stack.repository_id)?.is_empty() {
            self.repositories.remove_clone(&stack.repository_id);
        }
        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod testing {
    //! A Synced Stack setup over a local repository: no network, no credentials.

    use std::{path::Path, sync::Arc};

    use homeport_git::{Repositories, RepositoryStore};
    use homeport_hosts::{
        store::{HostDraft, HostStore},
        HostConnection,
    };
    use homeport_storage::{Db, SecretBox};

    use super::{SyncedStacks, SyncedStore};

    pub(crate) fn git(dir: &Path, args: &[&str]) -> String {
        let out = std::process::Command::new("git")
            .args(args)
            .current_dir(dir)
            .env("GIT_CONFIG_GLOBAL", dir.join(".no-config"))
            .env("GIT_AUTHOR_NAME", "Ada")
            .env("GIT_AUTHOR_EMAIL", "ada@example.com")
            .env("GIT_COMMITTER_NAME", "Ada")
            .env("GIT_COMMITTER_EMAIL", "ada@example.com")
            .output()
            .unwrap();
        assert!(
            out.status.success(),
            "git {args:?}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8_lossy(&out.stdout).trim().to_owned()
    }

    pub(crate) struct Lab {
        pub(crate) dir: tempfile::TempDir,
        pub(crate) db: Arc<Db>,
        pub(crate) secrets: Arc<SecretBox>,
        pub(crate) synced: SyncedStacks,
        pub(crate) host: String,
    }

    impl Lab {
        pub(crate) fn new() -> Self {
            let dir = tempfile::tempdir().unwrap();
            git(
                dir.path(),
                &[
                    "init",
                    "--quiet",
                    "--bare",
                    "--initial-branch=main",
                    "origin.git",
                ],
            );
            git(
                dir.path(),
                &["init", "--quiet", "--initial-branch=main", "work"],
            );
            let db = Arc::new(Db::open_in_memory().unwrap());
            let secrets = Arc::new(SecretBox::from_key(&[4; 32]));
            let host = HostStore::new(db.clone(), secrets.clone())
                .create(HostDraft {
                    name: "local".into(),
                    connection: HostConnection::Local {
                        socket_path: "/var/run/docker.sock".into(),
                    },
                    secrets: None,
                })
                .unwrap()
                .id;
            // A local path isn't a URL Homeport accepts from the user, so it's
            // written directly.
            let url = dir.path().join("origin.git").to_string_lossy().into_owned();
            db.with(|c| {
                c.execute(
                    "INSERT INTO git_repositories (id, url, auth) VALUES ('r1', ?1, 'none')",
                    [&url],
                )
            })
            .unwrap();
            let repositories = Arc::new(Repositories::new(
                RepositoryStore::new(db.clone(), secrets.clone()),
                dir.path().join("data/repositories"),
            ));
            let synced = SyncedStacks::new(
                SyncedStore::new(db.clone(), secrets.clone()),
                repositories,
                dir.path().join("data/synced-stacks"),
            );
            Self {
                dir,
                db,
                secrets,
                synced,
                host,
            }
        }

        /// Writes `files`, commits and pushes them; returns the commit.
        pub(crate) fn commit(&self, message: &str, files: &[(&str, &str)]) -> String {
            let work = self.dir.path().join("work");
            for (path, content) in files {
                let path = work.join(path);
                std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                std::fs::write(path, content).unwrap();
            }
            git(&work, &["add", "--all"]);
            git(
                &work,
                &["commit", "--quiet", "--allow-empty", "-m", message],
            );
            let origin = self.dir.path().join("origin.git");
            git(
                &work,
                &[
                    "push",
                    "--quiet",
                    "--tags",
                    &origin.to_string_lossy(),
                    "HEAD:main",
                ],
            );
            git(&work, &["rev-parse", "HEAD"])
        }
    }
}

#[cfg(test)]
mod tests {
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
}
