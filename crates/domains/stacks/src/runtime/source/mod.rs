//! What each source hands the compose CLI: which files, from which working
//! directory, what goes in on stdin, and which variables it adds.

use std::path::{Path, PathBuf};

use homeport_core::error::{AppError, AppResult, ErrorCode};

use super::StackSource;
use crate::{compose::ComposeAction, linked::files, messages as msg, model::definition};

/// The working directory of a run.
#[derive(Debug)]
pub(crate) enum WorkDir {
    /// An empty private folder, removed when the run ends, so nothing on this
    /// machine is picked up as project files.
    Temp(tempfile::TempDir),
    /// A folder on this machine.
    Folder(PathBuf),
}

impl WorkDir {
    pub(crate) fn path(&self) -> &Path {
        match self {
            Self::Temp(dir) => dir.path(),
            Self::Folder(path) => path,
        }
    }
}

/// A source ready to hand to the CLI for one run.
#[derive(Debug)]
pub(crate) struct Prepared {
    /// The arguments that name the compose files, after `--project-name`.
    pub file_args: Vec<String>,
    pub workdir: WorkDir,
    /// Written to the CLI's stdin, for a definition sent as `--file -`.
    pub stdin: Option<String>,
    /// The Stack's own variables, added to the CLI's environment.
    pub vars: Vec<(String, String)>,
    /// The compose files as this run uses them, by name, in order. Taking a
    /// Linked Stack down leaves out those it can't read.
    pub definition: Vec<(String, String)>,
}

/// A fresh folder only this user can enter, removed when dropped.
fn private_dir() -> std::io::Result<tempfile::TempDir> {
    let dir = tempfile::Builder::new().prefix("homeport-").tempdir()?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o700))?;
    }
    Ok(dir)
}

impl StackSource {
    /// Checks the source can be used for `action` and lays it out for the CLI.
    pub(crate) fn prepare(&self, action: ComposeAction) -> AppResult<Prepared> {
        self.prepare_for(action, false)
    }

    /// [`Self::prepare`], with a Swarm's rules too when `swarm` (ADR 0017).
    pub(crate) fn prepare_for(&self, action: ComposeAction, swarm: bool) -> AppResult<Prepared> {
        match self {
            Self::Managed { files, env } => {
                if action.reads_definition() {
                    let yaml: Vec<&str> = files.iter().map(|f| f.content.as_str()).collect();
                    if let Some(blocker) = definition::blocker_for(&yaml, self.kind(), swarm) {
                        return Err(blocker);
                    }
                }
                definition::validate_files(files)?;
                let dir = private_dir()?;
                let (file_args, stdin) = match files.as_slice() {
                    [only] => (
                        vec!["--file".into(), "-".into()],
                        Some(only.content.clone()),
                    ),
                    several => {
                        let mut args = Vec::new();
                        for file in several {
                            // Compose files aren't secret (ADR 0002); the environment stays out.
                            std::fs::write(dir.path().join(&file.name), &file.content)?;
                            args.extend(["--file".to_owned(), file.name.clone()]);
                        }
                        (args, None)
                    }
                };
                Ok(Prepared {
                    file_args,
                    workdir: WorkDir::Temp(dir),
                    stdin,
                    vars: definition::parse_env(env)?,
                    definition: files
                        .iter()
                        .map(|f| (f.name.clone(), f.content.clone()))
                        .collect(),
                })
            }
            Self::Linked {
                working_dir,
                config_files,
            } => prepare_folder(
                self.kind(),
                action,
                swarm,
                working_dir,
                config_files,
                vec![],
            ),
            Self::Synced {
                working_dir,
                config_files,
                env,
            } => prepare_folder(
                self.kind(),
                action,
                swarm,
                working_dir,
                config_files,
                definition::parse_env(env)?,
            ),
        }
    }
}

/// A source whose compose files are in a folder on this machine: compose
/// runs there, so `build:`, `env_file:` and relative paths work, with `vars`
/// added to its environment on top of the folder's `.env`, which compose
/// reads itself.
fn prepare_folder(
    kind: super::SourceKind,
    action: ComposeAction,
    swarm: bool,
    working_dir: &Path,
    config_files: &[String],
    vars: Vec<(String, String)>,
) -> AppResult<Prepared> {
    if !working_dir.is_dir() {
        return Err(files::folder_missing(&working_dir.to_string_lossy()));
    }
    let mut texts = Vec::new();
    for file in config_files {
        match std::fs::read_to_string(file) {
            Ok(text) => texts.push((file.clone(), text)),
            // Taking a Stack down works from its project name alone.
            Err(_) if !action.reads_definition() => {}
            Err(e) => {
                return Err(AppError::new(
                    ErrorCode::StackFolderMissing,
                    msg::FILE_MISSING.with("file", file),
                )
                .with_hint(msg::FILE_MISSING_HINT)
                .with_cause(&e))
            }
        }
    }
    if action.reads_definition() {
        let yaml: Vec<&str> = texts.iter().map(|(_, t)| t.as_str()).collect();
        if let Some(blocker) = definition::blocker_for(&yaml, kind, swarm) {
            return Err(blocker);
        }
    }
    let mut file_args = vec![
        "--project-directory".to_owned(),
        working_dir.to_string_lossy().into_owned(),
    ];
    for file in config_files {
        file_args.extend(["--file".to_owned(), file.clone()]);
    }
    Ok(Prepared {
        file_args,
        workdir: WorkDir::Folder(working_dir.to_owned()),
        stdin: None,
        vars,
        definition: texts,
    })
}

#[cfg(test)]
mod tests;
