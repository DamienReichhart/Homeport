//! The git commands Synced Stacks need (ADR 0014). A repository's clone is
//! bare and only ever fetched into; each Synced Stack checks a commit out in
//! a worktree of it. Nothing here commits, pushes or edits a file.

use std::path::Path;

use serde::{Deserialize, Serialize};

use homeport_core::error::{AppError, AppResult, ErrorCode};

use crate::{access::Remote, cli::GitCli, error, messages as msg};

/// What a Synced Stack follows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum RefKind {
    Branch,
    Tag,
}

impl RefKind {
    /// The full ref, like `refs/heads/main`.
    pub fn full(self, name: &str) -> String {
        match self {
            Self::Branch => format!("refs/heads/{name}"),
            Self::Tag => format!("refs/tags/{name}"),
        }
    }

    /// As stored: `branch` or `tag`.
    pub fn text(self) -> &'static str {
        match self {
            Self::Branch => "branch",
            Self::Tag => "tag",
        }
    }

    pub fn from_text(text: &str) -> Self {
        if text == "tag" {
            Self::Tag
        } else {
            Self::Branch
        }
    }
}

/// A remote's branches and tags, as "Test access" lists them.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct RemoteRefs {
    pub branches: Vec<String>,
    pub tags: Vec<String>,
    /// The branch the remote's `HEAD` points at.
    pub default_branch: Option<String>,
}

/// One commit, as a sync shows it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Commit {
    pub hash: String,
    pub short_hash: String,
    pub subject: String,
    pub author: String,
    /// ISO 8601, as the committer's clock wrote it.
    pub date: String,
}

/// Commits listed at most between two deploys.
pub const LOG_LIMIT: usize = 50;

const LOG_FORMAT: &str = "--format=%H%x1f%h%x1f%s%x1f%an%x1f%aI%x1e";

/// A folder every command can run in without picking up a repository.
fn neutral_dir() -> std::path::PathBuf {
    std::env::temp_dir()
}

fn git_dir(repo: &Path) -> String {
    format!("--git-dir={}", repo.display())
}

/// Lists the remote's branches and tags (`git ls-remote`). Also returns the
/// server key trusted for the first time, if any, to store.
pub async fn list_remote(
    cli: &GitCli,
    remote: &Remote<'_>,
) -> AppResult<(RemoteRefs, Option<String>)> {
    let (out, learned) = cli
        .remote(
            remote,
            &neutral_dir(),
            &["ls-remote", "--symref", remote.url],
        )
        .await?;
    Ok((parse_ls_remote(&out.text()), learned))
}

fn parse_ls_remote(text: &str) -> RemoteRefs {
    let mut refs = RemoteRefs::default();
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("ref: ") {
            if let Some((target, "HEAD")) = rest.split_once('\t') {
                refs.default_branch = target.strip_prefix("refs/heads/").map(str::to_owned);
            }
            continue;
        }
        let Some((_, name)) = line.split_once('\t') else {
            continue;
        };
        if let Some(branch) = name.strip_prefix("refs/heads/") {
            refs.branches.push(branch.to_owned());
        } else if let Some(tag) = name.strip_prefix("refs/tags/") {
            let tag = tag.trim_end_matches("^{}");
            if !refs.tags.iter().any(|t| t == tag) {
                refs.tags.push(tag.to_owned());
            }
        }
    }
    refs.branches.sort();
    refs.tags.sort();
    refs
}

/// Fetches `refs` from `remote` into the bare clone at `repo`, making it
/// first if needed. Branches and tags are forced to what the remote has, so
/// a moved tag or a rewritten branch is followed. Returns the server key
/// trusted for the first time, if any.
pub async fn fetch(
    cli: &GitCli,
    repo: &Path,
    remote: &Remote<'_>,
    refs: &[(RefKind, &str)],
) -> AppResult<Option<String>> {
    if !repo.join("HEAD").is_file() {
        let parent = repo.parent().unwrap_or(repo);
        std::fs::create_dir_all(parent)?;
        let path = repo.to_string_lossy();
        cli.local(parent, &["init", "--bare", "--quiet", &path])
            .await?;
    }
    let specs: Vec<String> = refs
        .iter()
        .map(|(kind, name)| {
            let full = kind.full(name);
            format!("+{full}:{full}")
        })
        .collect();
    let dir = git_dir(repo);
    let mut args = vec![
        dir.as_str(),
        "fetch",
        "--quiet",
        "--no-tags",
        "--update-head-ok",
        remote.url,
    ];
    args.extend(specs.iter().map(String::as_str));
    let (_, learned) = cli.remote(remote, &neutral_dir(), &args).await?;
    Ok(learned)
}

/// The error for a branch or tag the repository doesn't have.
pub fn ref_missing(full: &str) -> AppError {
    let message = match full.strip_prefix("refs/tags/") {
        Some(tag) => msg::REF_MISSING_TAG.with("name", tag),
        None => msg::REF_MISSING_BRANCH.with("name", full.trim_start_matches("refs/heads/")),
    };
    AppError::new(ErrorCode::SyncedComposeFileMissing, message).with_hint(msg::REF_MISSING_HINT)
}

/// The commit `kind`/`name` points at in the clone, if it has it.
pub async fn resolve(
    cli: &GitCli,
    repo: &Path,
    kind: RefKind,
    name: &str,
) -> AppResult<Option<String>> {
    let spec = format!("{}^{{commit}}", kind.full(name));
    let dir = git_dir(repo);
    match cli
        .local_raw(
            &neutral_dir(),
            &[&dir, "rev-parse", "--verify", "--quiet", &spec],
        )
        .await
    {
        Ok(out) => Ok(Some(out.text().trim().to_owned())),
        Err(crate::cli::Failure::Exited { .. }) => Ok(None),
        Err(crate::cli::Failure::Error(e)) => Err(e),
    }
}

/// Whether the clone has `commit`.
pub async fn has_commit(cli: &GitCli, repo: &Path, commit: &str) -> bool {
    let spec = format!("{commit}^{{commit}}");
    let dir = git_dir(repo);
    cli.local_raw(&neutral_dir(), &[&dir, "cat-file", "-e", &spec])
        .await
        .is_ok()
}

/// The file at `path` in `commit`; `None` when it isn't a file there.
pub async fn read_file(
    cli: &GitCli,
    repo: &Path,
    commit: &str,
    path: &str,
) -> AppResult<Option<Vec<u8>>> {
    let object = format!("{commit}:{path}");
    let dir = git_dir(repo);
    let kind = match cli
        .local_raw(&neutral_dir(), &[&dir, "cat-file", "-t", &object])
        .await
    {
        Ok(out) => out.text(),
        Err(crate::cli::Failure::Exited { .. }) => return Ok(None),
        Err(crate::cli::Failure::Error(e)) => return Err(e),
    };
    if kind.trim() != "blob" {
        return Ok(None);
    }
    let out = cli
        .local(&neutral_dir(), &[&dir, "cat-file", "blob", &object])
        .await?;
    Ok(Some(out.stdout))
}

/// The files under `folder` (the repository's root when empty) in `commit`.
pub async fn list_files(
    cli: &GitCli,
    repo: &Path,
    commit: &str,
    folder: &str,
) -> AppResult<Vec<String>> {
    let dir = git_dir(repo);
    let mut args = vec![dir.as_str(), "ls-tree", "-r", "-z", "--name-only", commit];
    let pathspec = format!("{folder}/");
    if !folder.is_empty() {
        args.extend(["--", pathspec.as_str()]);
    }
    let out = cli.local(&neutral_dir(), &args).await?;
    Ok(split_nul(&out.text()))
}

/// The files under `folder` that differ between `from` and `to`, deleted
/// ones included.
pub async fn changed_files(
    cli: &GitCli,
    repo: &Path,
    from: &str,
    to: &str,
    folder: &str,
) -> AppResult<Vec<String>> {
    let dir = git_dir(repo);
    let pathspec = if folder.is_empty() {
        ".".to_owned()
    } else {
        format!("{folder}/")
    };
    let out = cli
        .local(
            &neutral_dir(),
            &[
                &dir,
                "diff",
                "--name-only",
                "--no-renames",
                "-z",
                from,
                to,
                "--",
                &pathspec,
            ],
        )
        .await?;
    Ok(split_nul(&out.text()))
}

/// One commit.
pub async fn commit(cli: &GitCli, repo: &Path, commit: &str) -> AppResult<Commit> {
    let dir = git_dir(repo);
    let out = cli
        .local(
            &neutral_dir(),
            &[&dir, "log", "-1", LOG_FORMAT, commit, "--"],
        )
        .await?;
    parse_log(&out.text())
        .into_iter()
        .next()
        .ok_or_else(|| error::failure("log", "no such commit", None))
}

/// The commits after `from` up to `to` that change something under
/// `folder`, newest first, at most [`LOG_LIMIT`]. With no `from`, or one the
/// clone no longer has, the history of `to`.
pub async fn log(
    cli: &GitCli,
    repo: &Path,
    from: Option<&str>,
    to: &str,
    folder: &str,
) -> AppResult<Vec<Commit>> {
    let from = match from {
        Some(f) if has_commit(cli, repo, f).await => Some(f),
        _ => None,
    };
    let range = match from {
        Some(f) => format!("{f}..{to}"),
        None => to.to_owned(),
    };
    let dir = git_dir(repo);
    let limit = format!("--max-count={LOG_LIMIT}");
    let pathspec = if folder.is_empty() {
        ".".to_owned()
    } else {
        format!("{folder}/")
    };
    let out = cli
        .local(
            &neutral_dir(),
            &[&dir, "log", &limit, LOG_FORMAT, &range, "--", &pathspec],
        )
        .await?;
    Ok(parse_log(&out.text()))
}

fn parse_log(text: &str) -> Vec<Commit> {
    text.split('\u{1e}')
        .filter_map(|record| {
            let mut f = record.trim_start_matches('\n').split('\u{1f}');
            Some(Commit {
                hash: f.next().filter(|h| !h.is_empty())?.to_owned(),
                short_hash: f.next()?.to_owned(),
                subject: f.next()?.to_owned(),
                author: f.next()?.to_owned(),
                date: f.next()?.trim().to_owned(),
            })
        })
        .collect()
}

fn split_nul(text: &str) -> Vec<String> {
    text.split('\0')
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .collect()
}

/// Checks `commit` out in the worktree at `worktree`, making it first. What
/// containers wrote through relative bind mounts is left alone: files git
/// doesn't track, and changes to tracked ones, which carry over. A commit
/// that changes a file changed here too is refused, keeping the file.
pub async fn checkout(cli: &GitCli, repo: &Path, worktree: &Path, commit: &str) -> AppResult<()> {
    let dir = git_dir(repo);
    if worktree.join(".git").exists() {
        // Never `--force`: it would put back every tracked file a container
        // wrote to, such as a committed `acme.json` holding certificates.
        cli.local(worktree, &["checkout", "--detach", "--quiet", commit])
            .await?;
        return Ok(());
    }
    if let Some(parent) = worktree.parent() {
        std::fs::create_dir_all(parent)?;
    }
    // A worktree whose folder was deleted is still registered.
    cli.local(&neutral_dir(), &[&dir, "worktree", "prune"])
        .await?;
    let path = worktree.to_string_lossy();
    cli.local(
        &neutral_dir(),
        &[
            &dir, "worktree", "add", "--detach", "--force", "--quiet", &path, commit,
        ],
    )
    .await?;
    Ok(())
}

/// Removes a Stack's worktree unless it holds files git doesn't track, which
/// would go with it; `false` when it was kept for that reason.
pub async fn remove_worktree(cli: &GitCli, repo: &Path, worktree: &Path) -> AppResult<bool> {
    let dir = git_dir(repo);
    if !worktree.exists() {
        if repo.is_dir() {
            cli.local(&neutral_dir(), &[&dir, "worktree", "prune"])
                .await?;
        }
        return Ok(true);
    }
    let status = cli
        .local_raw(
            worktree,
            &[
                "status",
                "--porcelain",
                "--ignored",
                "--untracked-files=all",
            ],
        )
        .await;
    match status {
        Ok(out) if out.text().trim().is_empty() => {
            let path = worktree.to_string_lossy();
            cli.local(
                &neutral_dir(),
                &[&dir, "worktree", "remove", "--force", &path],
            )
            .await?;
            Ok(true)
        }
        _ => Ok(false),
    }
}

#[cfg(test)]
pub(crate) mod testing {
    //! Local repositories for tests: no network, no credentials.

    use std::path::{Path, PathBuf};

    /// Runs `git` with a fixed identity, panicking on failure.
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

    /// A bare repository ("origin") and a working copy that pushes to it.
    pub(crate) struct Upstream {
        pub(crate) _dir: tempfile::TempDir,
        pub(crate) bare: PathBuf,
        pub(crate) work: PathBuf,
    }

    impl Upstream {
        pub(crate) fn new() -> Self {
            let dir = tempfile::tempdir().unwrap();
            let bare = dir.path().join("origin.git");
            let work = dir.path().join("work");
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
            Self {
                _dir: dir,
                bare,
                work,
            }
        }

        pub(crate) fn url(&self) -> String {
            self.bare.to_string_lossy().into_owned()
        }

        /// Writes `files`, commits them and pushes; returns the commit.
        pub(crate) fn commit(&self, message: &str, files: &[(&str, &str)]) -> String {
            for (path, content) in files {
                let path = self.work.join(path);
                std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                std::fs::write(path, content).unwrap();
            }
            git(&self.work, &["add", "--all"]);
            git(
                &self.work,
                &["commit", "--quiet", "--allow-empty", "-m", message],
            );
            git(
                &self.work,
                &["push", "--quiet", "--tags", &self.url(), "HEAD:main"],
            );
            git(&self.work, &["rev-parse", "HEAD"])
        }
    }
}

#[cfg(test)]
mod tests;
