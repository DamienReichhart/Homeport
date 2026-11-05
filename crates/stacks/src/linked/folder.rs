//! A folder on this machine as a compose project, before it becomes a
//! Linked Stack: which compose files Compose would use, and the name it
//! would give the project.

use std::path::Path;

use serde::Serialize;
use serde_yaml_ng::Value;

use homeport_core::error::{AppError, AppResult};

use super::files::{self, LinkedFile};
use crate::messages as msg;

/// The file names Compose looks for in a folder, in the order it prefers them.
const DEFAULT_FILES: [&str; 4] = [
    "compose.yaml",
    "compose.yml",
    "docker-compose.yml",
    "docker-compose.yaml",
];

/// The override files Compose adds to the first default file it finds.
const OVERRIDE_FILES: [&str; 4] = [
    "compose.override.yml",
    "compose.override.yaml",
    "docker-compose.override.yml",
    "docker-compose.override.yaml",
];

/// What Compose would make of a folder.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct FolderProject {
    pub working_dir: String,
    /// The compose files Compose would use, in order, with their content.
    pub files: Vec<LinkedFile>,
    /// The project name: the files' top-level `name:`, or the folder's name
    /// as Compose normalises it.
    pub name: String,
}

/// Finds the compose files Compose would use in `folder` (its first default
/// file, then the first override file) and the name it would give it.
pub fn inspect(folder: &Path) -> AppResult<FolderProject> {
    if !folder.is_dir() {
        return Err(files::folder_missing(&folder.to_string_lossy()));
    }
    let first = |names: &[&str]| names.iter().map(|n| folder.join(n)).find(|p| p.is_file());
    let Some(base) = first(&DEFAULT_FILES) else {
        return Err(AppError::validation(
            msg::NO_COMPOSE_FILE.with("folder", folder.to_string_lossy()),
        ));
    };
    let paths = std::iter::once(base).chain(first(&OVERRIDE_FILES));
    let files = paths
        .map(|path| {
            let text = std::fs::read_to_string(&path)
                .map_err(|e| homeport_core::error::local_file(&path, e))?;
            Ok(LinkedFile {
                path: path.to_string_lossy().into_owned(),
                fingerprint: files::content_fingerprint(text.as_bytes()),
                content: text,
            })
        })
        .collect::<AppResult<Vec<_>>>()?;
    let declared = files
        .iter()
        .filter_map(|f| serde_yaml_ng::from_str::<Value>(&f.content).ok())
        .filter_map(|doc| doc.get("name").and_then(Value::as_str).map(str::to_owned))
        .next_back();
    let name = declared
        .map(|n| normalise(&n))
        .filter(|n| !n.is_empty())
        .unwrap_or_else(|| normalise(&folder_name(folder)));
    Ok(FolderProject {
        working_dir: folder.to_string_lossy().into_owned(),
        files,
        name,
    })
}

fn folder_name(folder: &Path) -> String {
    folder
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default()
}

/// A project name the way Compose normalises one: lowercase, only letters,
/// digits, `-` and `_`, starting with a letter or digit.
pub fn normalise(name: &str) -> String {
    let kept: String = name
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == '-' || *c == '_')
        .collect();
    kept.trim_start_matches(['-', '_']).to_owned()
}

#[cfg(test)]
mod tests {
    use homeport_core::error::ErrorCode;

    use super::*;

    fn folder(name: &str, files: &[(&str, &str)]) -> (tempfile::TempDir, std::path::PathBuf) {
        let root = tempfile::tempdir().unwrap();
        let dir = root.path().join(name);
        std::fs::create_dir(&dir).unwrap();
        for (file, text) in files {
            std::fs::write(dir.join(file), text).unwrap();
        }
        (root, dir)
    }

    const SERVICES: &str = "services:\n  web:\n    image: nginx\n";

    #[test]
    fn the_default_file_and_its_override_are_used_in_order() {
        let (_root, dir) = folder(
            "site",
            &[
                ("compose.yaml", SERVICES),
                (
                    "compose.override.yaml",
                    "services:\n  web:\n    ports: [\"80:80\"]\n",
                ),
                ("compose.prod.yaml", SERVICES),
            ],
        );
        let project = inspect(&dir).unwrap();
        let names: Vec<_> = project
            .files
            .iter()
            .map(|f| Path::new(&f.path).file_name().unwrap().to_owned())
            .collect();
        assert_eq!(names, ["compose.yaml", "compose.override.yaml"]);
        assert_eq!(project.name, "site");
        assert_eq!(project.files[0].content, SERVICES);
    }

    #[test]
    fn compose_s_preferred_name_wins() {
        let (_root, dir) = folder(
            "legacy",
            &[("docker-compose.yml", SERVICES), ("compose.yaml", SERVICES)],
        );
        let project = inspect(&dir).unwrap();
        assert!(project.files[0].path.ends_with("compose.yaml"));
        assert!(!project.files[0].path.ends_with("docker-compose.yml"));
        assert_eq!(project.files.len(), 1);
    }

    #[test]
    fn the_name_comes_from_name_or_the_normalised_folder() {
        let (_root, dir) = folder("My App.v2", &[("compose.yaml", SERVICES)]);
        assert_eq!(inspect(&dir).unwrap().name, "myappv2");
        let (_root, dir) = folder(
            "folder",
            &[("compose.yaml", "name: Shop-Front\nservices: {}\n")],
        );
        assert_eq!(inspect(&dir).unwrap().name, "shop-front");
        assert_eq!(normalise("__Odd Name!"), "oddname");
    }

    #[test]
    fn a_folder_without_compose_files_says_why() {
        let (_root, dir) = folder("empty", &[("README.md", "hi")]);
        let e = inspect(&dir).unwrap_err();
        assert!(e.is(ErrorCode::Validation));
        assert!(e.message.contains("compose.yaml"), "{}", e.message);
        let e = inspect(&dir.join("nope")).unwrap_err();
        assert!(e.is(ErrorCode::StackFolderMissing));
    }
}
