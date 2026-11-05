//! What each source hands the compose CLI: which files, from which working
//! directory, what goes in on stdin, and which variables it adds.

use std::path::{Path, PathBuf};

use homeport_core::error::{AppError, AppResult, ErrorCode};

use super::StackSource;
use crate::{compose::ComposeAction, definition, linked::files, messages as msg};

/// The working directory of a run.
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
pub(crate) struct Prepared {
    /// The arguments that name the compose files, after `--project-name`.
    pub file_args: Vec<String>,
    pub workdir: WorkDir,
    /// Written to the CLI's stdin, for a definition sent as `--file -`.
    pub stdin: Option<String>,
    /// The Stack's own variables, added to the CLI's environment.
    pub vars: Vec<(String, String)>,
    /// The compose files as this run uses them, by name, in order; empty when
    /// the action doesn't read them.
    pub definition: Vec<(String, String)>,
}

impl StackSource {
    /// Checks the source can be used for `action` and lays it out for the CLI.
    pub(crate) fn prepare(&self, action: ComposeAction) -> AppResult<Prepared> {
        match self {
            Self::Managed { compose, env } => {
                if action.reads_definition() {
                    if let Some(blocker) = definition::blocker(&[compose], self.kind()) {
                        return Err(blocker);
                    }
                }
                Ok(Prepared {
                    file_args: vec!["--file".into(), "-".into()],
                    workdir: WorkDir::Temp(tempfile::tempdir()?),
                    stdin: Some(compose.clone()),
                    vars: definition::parse_env(env)?,
                    definition: vec![("compose.yaml".into(), compose.clone())],
                })
            }
            Self::Linked {
                working_dir,
                config_files,
            } => {
                if !working_dir.is_dir() {
                    return Err(files::folder_missing(&working_dir.to_string_lossy()));
                }
                let mut texts = Vec::new();
                if action.reads_definition() {
                    for file in config_files {
                        let text = std::fs::read_to_string(file).map_err(|e| {
                            AppError::new(
                                ErrorCode::StackFolderMissing,
                                msg::FILE_MISSING.with("file", file),
                            )
                            .with_hint(msg::FILE_MISSING_HINT)
                            .with_cause(&e)
                        })?;
                        texts.push((file.clone(), text));
                    }
                    let yaml: Vec<&str> = texts.iter().map(|(_, t)| t.as_str()).collect();
                    if let Some(blocker) = definition::blocker(&yaml, self.kind()) {
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
                    workdir: WorkDir::Folder(working_dir.clone()),
                    stdin: None,
                    // Compose reads the folder's .env itself.
                    vars: vec![],
                    definition: texts,
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
            StackSource::Linked { .. } => unreachable!(),
        }
    }

    fn linked(dir: &Path, files: &[(&str, &str)]) -> StackSource {
        let config_files = files
            .iter()
            .map(|(name, text)| {
                let path = dir.join(name);
                std::fs::write(&path, text).unwrap();
                path.to_string_lossy().into_owned()
            })
            .collect();
        StackSource::Linked {
            working_dir: dir.to_owned(),
            config_files,
        }
    }

    #[test]
    fn linked_stacks_run_in_their_folder_with_each_file() {
        let dir = tempfile::tempdir().unwrap();
        let source = linked(
            dir.path(),
            &[
                ("compose.yaml", "services:\n  app:\n    build: .\n"),
                (
                    "compose.override.yaml",
                    "services:\n  app:\n    env_file: .env\n",
                ),
            ],
        );
        let prepared = source.prepare(ComposeAction::Up).unwrap();
        let folder = dir.path().to_string_lossy().into_owned();
        let file = |n: &str| dir.path().join(n).to_string_lossy().into_owned();
        assert_eq!(
            prepared.file_args,
            [
                "--project-directory".to_owned(),
                folder,
                "--file".into(),
                file("compose.yaml"),
                "--file".into(),
                file("compose.override.yaml"),
            ]
        );
        assert_eq!(prepared.workdir.path(), dir.path());
        assert!(prepared.stdin.is_none());
        assert!(prepared.vars.is_empty());
        assert_eq!(prepared.definition.len(), 2);
    }

    #[test]
    fn linked_stacks_need_their_folder() {
        let dir = tempfile::tempdir().unwrap();
        let source = linked(dir.path(), &[("compose.yaml", "services: {}\n")]);
        drop(dir);
        for action in [ComposeAction::Up, ComposeAction::Down] {
            assert!(matches!(
                source.prepare(action),
                Err(ref e) if e.is(ErrorCode::StackFolderMissing)
            ));
        }
    }
}
