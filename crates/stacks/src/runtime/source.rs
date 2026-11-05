//! What each source hands the compose CLI: which files, from which working
//! directory, what goes in on stdin, and which variables it adds.

use std::path::{Path, PathBuf};

use homeport_core::error::AppResult;

use super::StackSource;
use crate::{compose::ComposeAction, definition};

/// The working directory of a run.
pub(crate) enum WorkDir {
    /// An empty private folder, removed when the run ends, so nothing on this
    /// machine is picked up as project files.
    Temp(tempfile::TempDir),
    /// A folder on this machine.
    #[allow(dead_code)]
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
pub(crate) struct Prepared {
    /// The arguments that name the compose files, after `--project-name`.
    pub file_args: Vec<String>,
    pub workdir: WorkDir,
    /// Written to the CLI's stdin, for a definition sent as `--file -`.
    pub stdin: Option<String>,
    /// The Stack's own variables, added to the CLI's environment.
    pub vars: Vec<(String, String)>,
}

impl StackSource {
    /// Checks the source can be used for `action` and lays it out for the CLI.
    pub(crate) fn prepare(&self, action: ComposeAction) -> AppResult<Prepared> {
        match self {
            Self::Managed { compose, env } => {
                if action.reads_definition() {
                    if let Some(blocker) = definition::blocker(compose, self.kind()) {
                        return Err(blocker);
                    }
                }
                Ok(Prepared {
                    file_args: vec!["--file".into(), "-".into()],
                    workdir: WorkDir::Temp(tempfile::tempdir()?),
                    stdin: Some(compose.clone()),
                    vars: definition::parse_env(env)?,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn managed_definitions_go_in_on_stdin_from_an_empty_folder() {
        let source = StackSource::Managed {
            compose: "services:\n  web:\n    image: nginx\n".into(),
            env: "A=1".into(),
        };
        let prepared = source.prepare(ComposeAction::Up).unwrap();
        assert_eq!(prepared.file_args, ["--file", "-"]);
        assert_eq!(prepared.stdin.as_deref(), Some(source_compose(&source)));
        assert_eq!(prepared.vars, [("A".to_owned(), "1".to_owned())]);
        assert!(matches!(prepared.workdir, WorkDir::Temp(_)));
        assert_eq!(
            std::fs::read_dir(prepared.workdir.path()).unwrap().count(),
            0
        );
    }

    #[test]
    fn a_blocked_definition_can_still_be_taken_down() {
        let source = StackSource::Managed {
            compose: "services:\n  app:\n    build: .\n".into(),
            env: String::new(),
        };
        assert!(source.prepare(ComposeAction::Up).is_err());
        assert!(source.prepare(ComposeAction::Down).is_ok());
    }

    fn source_compose(source: &StackSource) -> &str {
        match source {
            StackSource::Managed { compose, .. } => compose,
        }
    }
}
