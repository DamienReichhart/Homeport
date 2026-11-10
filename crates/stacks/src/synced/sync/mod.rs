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
mod tests;
