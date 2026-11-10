//! Every store of Stacks in one place, as the app holds them, and the use
//! cases that need more than one of them or the Host a Stack runs on.

use std::sync::Arc;

use bollard::Docker;
use homeport_core::error::AppResult;
use homeport_engine::auth::Logins;
use homeport_engine::{containers, swarm::services};
use homeport_git::cli as git;

use crate::{
    compose::{self, ComposeAction},
    diff::{self, Candidate, DeployDiff},
    external,
    history::DeployHistory,
    linked::{self, files::LinkedFiles, folder, FileCache, LinkedStack, LinkedStore},
    owner::StackOwner,
    runtime::{ComposeJob, Recorder, SourceKind, StackSource, StackTarget},
    store::{ComposeFile, StackStore},
    synced::SyncedStacks,
};

/// The Stacks Homeport knows, of every kind, and what it remembers about
/// them.
pub struct Stacks {
    pub managed: StackStore,
    pub linked: LinkedStore,
    /// The compose files of Linked Stacks, as last read.
    pub linked_files: FileCache,
    /// What each deploy used, for the history and rolling back.
    pub history: Arc<DeployHistory>,
    /// Synced Stacks and their worktrees on this machine.
    pub synced: Arc<SyncedStacks>,
}

/// A Managed, Linked or Synced Stack, ready to run.
struct Resolved {
    owner: StackOwner,
    registry_auth: bool,
    name: String,
    source: StackSource,
    profiles: Vec<String>,
    /// The commit a Synced Stack runs.
    commit: Option<String>,
}

impl Stacks {
    /// Refuses a new Stack named like one that runs on `owner` outside
    /// Homeport: deploying it would remove or replace that Stack's containers
    /// (`external`). A Linked Stack for `folder` may take the project compose
    /// started from that folder. The owner's Engine must answer: a Host that
    /// isn't connected can't say what runs there. `engine` is the Host's, or
    /// the one of a manager of the Swarm.
    pub async fn refuse_taking_over(
        &self,
        engine: &Docker,
        owner: &StackOwner,
        name: &str,
        folder: Option<&str>,
    ) -> AppResult<()> {
        let running = match owner {
            StackOwner::Host(_) => external::on_host(&containers::list(engine).await?),
            StackOwner::Swarm(_) => external::on_swarm(&services::list(engine).await?),
        };
        let mut known: Vec<String> = self
            .managed
            .list_of(owner)?
            .into_iter()
            .map(|s| s.name)
            .collect();
        known.extend(self.linked.list_of(owner)?.into_iter().map(|s| s.name));
        known.extend(
            self.synced
                .store
                .list_of(owner)?
                .into_iter()
                .map(|s| s.name),
        );
        external::refuse_taking_over(name, &running, &known, folder)
    }

    /// Records the Linked Stack `name` of `owner` for the compose files
    /// `config_files` of the folder `working_dir`. The files are checked
    /// again here: the caller only names them. `engine` is the owner's, as
    /// for [`Self::refuse_taking_over`].
    pub async fn link(
        &self,
        engine: &Docker,
        owner: &StackOwner,
        name: &str,
        working_dir: &str,
        config_files: &[String],
    ) -> AppResult<LinkedStack> {
        let found = folder::inspect(std::path::Path::new(working_dir))?;
        if config_files.is_empty()
            || config_files
                .iter()
                .any(|f| !found.files.iter().any(|g| &g.path == f))
        {
            return Err(linked::files::not_a_stack_file(
                config_files.first().map_or("", String::as_str),
            ));
        }
        self.refuse_taking_over(engine, owner, name, Some(&found.working_dir))
            .await?;
        let fingerprint = linked::files::files_fingerprint(config_files);
        self.linked
            .create(owner, name, working_dir, config_files, &fingerprint)
    }

    /// A Linked Stack's compose files as they are on disk now. With
    /// `acknowledge`, a change made outside Homeport becomes the known state.
    pub fn linked_files(&self, id: &str, acknowledge: bool) -> AppResult<LinkedFiles> {
        let stack = self.linked.get(id)?;
        let mut files = linked::files::read(&stack)?;
        if acknowledge && files.changed_outside {
            self.linked.set_fingerprint(id, &files.fingerprint)?;
            files.changed_outside = false;
        }
        Ok(files)
    }

    /// Saves one of a Linked Stack's compose files; `expected` is the
    /// fingerprint the editor loaded, so a change made outside is refused.
    pub fn write_linked_file(
        &self,
        id: &str,
        path: &str,
        content: &str,
        expected: Option<&str>,
    ) -> AppResult<LinkedFiles> {
        let stack = self.linked.get(id)?;
        linked::files::write(&stack, path, content, expected)?;
        let stack = self
            .linked
            .set_fingerprint(id, &linked::files::files_fingerprint(&stack.config_files))?;
        linked::files::read(&stack)
    }

    /// Changes which compose files of its folder a Linked Stack uses.
    pub fn set_linked_files(&self, id: &str, config_files: &[String]) -> AppResult<LinkedFiles> {
        let stack = self.linked.get(id)?;
        linked::files::validate_choice(&stack.working_dir, config_files)?;
        let fingerprint = linked::files::files_fingerprint(config_files);
        let stack = self.linked.set_files(id, config_files, &fingerprint)?;
        linked::files::read(&stack)
    }

    /// Turns sending registry logins with a Stack's deploys on or off.
    pub fn set_registry_auth(&self, kind: SourceKind, id: &str, on: bool) -> AppResult<bool> {
        Ok(match kind {
            SourceKind::Managed => self.managed.set_registry_auth(id, on)?.registry_auth,
            SourceKind::Linked => self.linked.set_registry_auth(id, on)?.registry_auth,
            SourceKind::Synced => self.synced.store.set_registry_auth(id, on)?.registry_auth,
        })
    }

    /// Sets the Compose profiles a Stack is deployed with.
    pub fn set_profiles(
        &self,
        kind: SourceKind,
        id: &str,
        profiles: &[String],
    ) -> AppResult<Vec<String>> {
        Ok(match kind {
            SourceKind::Managed => self.managed.set_profiles(id, profiles)?.profiles,
            SourceKind::Linked => self.linked.set_profiles(id, profiles)?.profiles,
            SourceKind::Synced => self.synced.store.set_profiles(id, profiles)?.profiles,
        })
    }

    /// The Host, name, source and profiles of the Managed, Linked or Synced
    /// Stack `id`; a Synced Stack's commit is checked out for `action` first.
    async fn resolve(
        &self,
        kind: SourceKind,
        id: &str,
        action: ComposeAction,
        commit: Option<&str>,
    ) -> AppResult<Resolved> {
        Ok(match kind {
            SourceKind::Managed => {
                let stack = self.managed.get(id)?;
                Resolved {
                    source: StackSource::Managed {
                        env: self.managed.env(id)?,
                        files: stack.files,
                    },
                    owner: stack.owner,
                    registry_auth: stack.registry_auth,
                    name: stack.name,
                    profiles: stack.profiles,
                    commit: None,
                }
            }
            SourceKind::Linked => {
                let stack = self.linked.get(id)?;
                Resolved {
                    source: StackSource::Linked {
                        working_dir: stack.working_dir.into(),
                        config_files: stack.config_files,
                    },
                    owner: stack.owner,
                    registry_auth: stack.registry_auth,
                    name: stack.name,
                    profiles: stack.profiles,
                    commit: None,
                }
            }
            SourceKind::Synced => {
                let stack = self.synced.store.get(id)?;
                let cli = git::detect().await?;
                let (source, commit) = self.synced.source(&cli, &stack, action, commit).await?;
                Resolved {
                    source,
                    owner: stack.owner,
                    registry_auth: stack.registry_auth,
                    name: stack.name,
                    profiles: stack.profiles,
                    commit: Some(commit),
                }
            }
        })
    }

    /// The compose job for `action` on the Managed, Linked or Synced Stack
    /// `id`, its deploys recorded in its history (as started by auto-deploy
    /// when `automatic`), with the commit a Synced Stack uses. `target` says
    /// where the Stacks of its owner run.
    #[allow(clippy::too_many_arguments)]
    pub async fn job(
        &self,
        target: impl FnOnce(&StackOwner) -> AppResult<StackTarget>,
        logins: Logins,
        kind: SourceKind,
        id: &str,
        action: ComposeAction,
        commit: Option<&str>,
        automatic: bool,
    ) -> AppResult<(ComposeJob, Option<String>)> {
        let stack = self.resolve(kind, id, action, commit).await?;
        let cli = if stack.owner.is_swarm() {
            compose::stack::detect().await?
        } else {
            compose::detect().await?
        };
        let job = ComposeJob {
            cli,
            source: stack.source,
            target: target(&stack.owner)?,
            logins,
            project: stack.name,
            action,
            profiles: stack.profiles,
            record: Some(Recorder {
                history: self.history.clone(),
                owner: stack.owner,
                stack_id: id.to_owned(),
                commit: stack.commit.clone(),
                automatic,
            }),
            registry_auth: stack.registry_auth,
        };
        Ok((job, stack.commit))
    }

    /// What redeploying the Managed, Linked or Synced Stack `id` would
    /// change: its latest successful deploy against `candidate`, or against
    /// its stored definition (a Linked Stack's files as they are now, a
    /// Synced Stack's compose file at its fetched commit) when there's none.
    /// Environment values are compared here and never sent back.
    pub async fn diff(
        &self,
        kind: SourceKind,
        id: &str,
        candidate: Option<Candidate>,
    ) -> AppResult<DeployDiff> {
        let (stored_files, stored_env, profiles) = match kind {
            SourceKind::Managed => {
                let stack = self.managed.get(id)?;
                let env = match &candidate {
                    Some(Candidate { env: Some(_), .. }) => String::new(),
                    _ => self.managed.env(id)?,
                };
                (stack.files, env, stack.profiles)
            }
            SourceKind::Linked => {
                let stack = self.linked.get(id)?;
                let files = match &candidate {
                    Some(_) => vec![],
                    None => linked::files::read(&stack)?
                        .files
                        .into_iter()
                        .map(|f| ComposeFile::new(f.path, f.content))
                        .collect(),
                };
                (files, String::new(), stack.profiles)
            }
            SourceKind::Synced => {
                let stack = self.synced.store.get(id)?;
                // A redeploy runs the deployed commit again; a first deploy, the fetched one.
                let commit = stack
                    .deployed_commit
                    .as_ref()
                    .or(stack.fetched_commit.as_ref());
                let files = match (&candidate, commit) {
                    (None, Some(commit)) => {
                        let cli = git::detect().await?;
                        let text = self
                            .synced
                            .compose_file(&cli, &stack.repository_id, commit, &stack.compose_path)
                            .await?;
                        let path = self.synced.worktree(&stack.id).join(&stack.compose_path);
                        vec![ComposeFile::new(path.to_string_lossy(), text)]
                    }
                    _ => vec![],
                };
                (files, self.synced.store.env(id)?, stack.profiles)
            }
        };
        let (files, env) = match candidate {
            Some(c) => (c.files, c.env.unwrap_or(stored_env)),
            None => (stored_files, stored_env),
        };
        let latest = self.history.latest_successful(id)?;
        let deployed_env = match &latest {
            // An environment the keychain can't open compares as empty.
            Some(d) => self.history.env(&d.summary.id).unwrap_or_default(),
            None => String::new(),
        };
        let deployed = latest.as_ref().map(|d| {
            (
                d.summary.started_at.as_str(),
                diff::Definition {
                    files: &d.files,
                    env: &deployed_env,
                    profiles: &d.profiles,
                },
            )
        });
        Ok(diff::diff(
            deployed,
            diff::Definition {
                files: &files,
                env: &env,
                profiles: &profiles,
            },
        ))
    }
}
