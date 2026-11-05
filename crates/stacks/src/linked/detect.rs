//! Telling whether a Stack found running was started from compose files on
//! this machine (ADR 0008): its labels name them, they exist here, and they
//! define the services that run.

use std::{
    collections::{BTreeMap, HashMap},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::SystemTime,
};

use serde_yaml_ng::Value;

use homeport_engine::containers::Container;

/// Where compose recorded the project's files, comma-separated, in order.
pub const CONFIG_FILES: &str = "com.docker.compose.project.config_files";
/// The project folder compose ran in.
pub const WORKING_DIR: &str = "com.docker.compose.project.working_dir";

/// What one compose file says about services, for the match.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Parsed {
    /// Each service it mentions, and whether it sits behind `profiles:`.
    services: BTreeMap<String, bool>,
    /// It uses `include:` or `extends:`, which the match doesn't follow.
    unsupported: bool,
}

/// A file as last read: its modification time and size, and what it said.
type Entry = (SystemTime, u64, Option<Arc<Parsed>>);

/// Parsed compose files by path, kept while their modification time and
/// size stay the same, so refreshing the Stack list doesn't re-read them.
#[derive(Default)]
pub struct FileCache {
    entries: Mutex<HashMap<PathBuf, Entry>>,
}

impl FileCache {
    pub fn new() -> Self {
        Self::default()
    }

    /// Whether `path` is a file this machine has.
    pub fn exists(&self, path: &Path) -> bool {
        path.is_file()
    }

    /// The file at `path`, parsed; `None` when it's missing, unreadable or
    /// not a compose file.
    fn parsed(&self, path: &Path) -> Option<Arc<Parsed>> {
        let meta = std::fs::metadata(path).ok().filter(|m| m.is_file())?;
        let modified = meta.modified().ok()?;
        let mut entries = self.entries.lock().expect("file cache poisoned");
        if let Some((m, len, parsed)) = entries.get(path) {
            if *m == modified && *len == meta.len() {
                return parsed.clone();
            }
        }
        let parsed = std::fs::read_to_string(path)
            .ok()
            .and_then(|text| parse(&text))
            .map(Arc::new);
        entries.insert(path.to_owned(), (modified, meta.len(), parsed.clone()));
        parsed
    }
}

fn parse(text: &str) -> Option<Parsed> {
    let doc: Value = serde_yaml_ng::from_str(text).ok()?;
    let mut parsed = Parsed {
        unsupported: doc.get("include").is_some(),
        ..Parsed::default()
    };
    for (name, service) in doc
        .get("services")
        .and_then(Value::as_mapping)
        .into_iter()
        .flatten()
    {
        let name = name.as_str()?.to_owned();
        parsed.unsupported |= service.get("extends").is_some();
        let profiles = service
            .get("profiles")
            .and_then(Value::as_sequence)
            .is_some_and(|p| !p.is_empty());
        parsed.services.insert(name, profiles);
    }
    Some(parsed)
}

/// Where a Stack found running says its compose files are.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Origin {
    pub working_dir: String,
    pub config_files: Vec<String>,
}

/// The folder and files a Stack's containers were started from, when they
/// carry both labels.
pub fn origin(members: &[&Container]) -> Option<Origin> {
    let label = |key: &str| {
        members
            .iter()
            .find_map(|c| c.labels.get(key).filter(|v| !v.trim().is_empty()))
    };
    let working_dir = label(WORKING_DIR)?.clone();
    let config_files: Vec<String> = label(CONFIG_FILES)?
        .split(',')
        .map(str::trim)
        .filter(|f| !f.is_empty())
        .map(str::to_owned)
        .collect();
    (!config_files.is_empty()).then_some(Origin {
        working_dir,
        config_files,
    })
}

/// Whether `origin`'s files exist here and define the services `members`
/// run: every running service is defined in them, and every service they
/// define outside a profile has a container. A path that exists here by
/// coincidence defines other services and doesn't match.
pub fn matches(cache: &FileCache, origin: &Origin, members: &[&Container]) -> bool {
    if !Path::new(&origin.working_dir).is_dir() {
        return false;
    }
    let mut defined: BTreeMap<String, bool> = BTreeMap::new();
    for file in &origin.config_files {
        let Some(parsed) = cache.parsed(Path::new(file)) else {
            return false;
        };
        if parsed.unsupported {
            return false;
        }
        for (name, profiles) in &parsed.services {
            *defined.entry(name.clone()).or_default() |= *profiles;
        }
    }
    if defined.is_empty() {
        return false;
    }
    let service = |c: &Container| c.service.clone().unwrap_or_else(|| c.name.clone());
    let running_defined = members
        .iter()
        .filter(|c| c.state == "running")
        .all(|c| defined.contains_key(&service(c)));
    let with_container: Vec<String> = members.iter().map(|c| service(c)).collect();
    let defined_present = defined
        .iter()
        .filter(|(_, profiles)| !**profiles)
        .all(|(name, _)| with_container.contains(name));
    running_defined && defined_present
}

/// Whether every one of a recorded Linked Stack's files is still here.
pub fn files_present(cache: &FileCache, config_files: &[String]) -> bool {
    !config_files.is_empty() && config_files.iter().all(|f| cache.exists(Path::new(f)))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    fn container(service: &str, state: &str, origin: Option<&Origin>) -> Container {
        let mut labels = HashMap::new();
        if let Some(o) = origin {
            labels.insert(WORKING_DIR.to_owned(), o.working_dir.clone());
            labels.insert(CONFIG_FILES.to_owned(), o.config_files.join(","));
        }
        Container {
            id: format!("{service}-id"),
            name: format!("app-{service}-1"),
            image: "img".into(),
            image_id: "sha".into(),
            command: String::new(),
            created: 0,
            state: state.into(),
            status: String::new(),
            health: None,
            ports: vec![],
            labels,
            stack: Some("app".into()),
            service: Some(service.into()),
            networks: vec![],
        }
    }

    /// A folder holding `files` (name, content), and its origin.
    fn folder(files: &[(&str, &str)]) -> (tempfile::TempDir, Origin) {
        let dir = tempfile::tempdir().unwrap();
        let mut config_files = vec![];
        for (name, content) in files {
            let path = dir.path().join(name);
            std::fs::write(&path, content).unwrap();
            config_files.push(path.to_string_lossy().into_owned());
        }
        let origin = Origin {
            working_dir: dir.path().to_string_lossy().into_owned(),
            config_files,
        };
        (dir, origin)
    }

    const BASE: &str = "services:\n  web:\n    image: nginx\n  db:\n    image: postgres\n  debug:\n    image: busybox\n    profiles: [debug]\n";

    #[test]
    fn labels_give_the_folder_and_files_in_order() {
        let o = Origin {
            working_dir: "/srv/app".into(),
            config_files: vec!["/srv/app/a.yaml".into(), "/srv/app/b.yaml".into()],
        };
        let c = container("web", "running", Some(&o));
        assert_eq!(origin(&[&c]), Some(o));
        assert_eq!(origin(&[&container("web", "running", None)]), None);
    }

    #[test]
    fn files_that_define_the_running_services_match() {
        let (_dir, o) = folder(&[
            ("compose.yaml", BASE),
            (
                "compose.override.yaml",
                "services:\n  web:\n    ports: [\"80:80\"]\n",
            ),
        ]);
        let cache = FileCache::new();
        let (web, db) = (
            container("web", "running", Some(&o)),
            container("db", "exited", Some(&o)),
        );
        assert!(matches(&cache, &o, &[&web, &db]));
        // A service behind a profile may run or not.
        let debug = container("debug", "running", Some(&o));
        assert!(matches(&cache, &o, &[&web, &db, &debug]));
    }

    #[test]
    fn a_path_that_exists_here_by_coincidence_does_not_match() {
        let (_dir, o) = folder(&[("compose.yaml", "services:\n  other:\n    image: x\n")]);
        let cache = FileCache::new();
        let web = container("web", "running", Some(&o));
        assert!(!matches(&cache, &o, &[&web]));
        // Nor does a folder missing a service it defines.
        let (_dir, o) = folder(&[("compose.yaml", BASE)]);
        let web = container("web", "running", Some(&o));
        assert!(!matches(&cache, &o, &[&web]));
    }

    #[test]
    fn missing_folders_and_unsupported_files_do_not_match() {
        let (dir, o) = folder(&[("compose.yaml", BASE)]);
        let cache = FileCache::new();
        let members = [
            container("web", "running", Some(&o)),
            container("db", "running", Some(&o)),
        ];
        let members: Vec<&Container> = members.iter().collect();
        assert!(matches(&cache, &o, &members));
        assert!(files_present(&cache, &o.config_files));
        drop(dir);
        assert!(!matches(&cache, &o, &members));
        assert!(!files_present(&cache, &o.config_files));

        let (_dir, o) = folder(&[(
            "compose.yaml",
            "include:\n  - other.yaml\nservices:\n  web:\n    image: nginx\n",
        )]);
        let web = container("web", "running", Some(&o));
        assert!(!matches(&cache, &o, &[&web]));
    }

    #[test]
    fn unchanged_files_are_read_once() {
        let (_dir, o) = folder(&[("compose.yaml", BASE)]);
        let cache = FileCache::new();
        let path = Path::new(&o.config_files[0]);
        let first = cache.parsed(path).unwrap();
        assert!(Arc::ptr_eq(&first, &cache.parsed(path).unwrap()));
        std::fs::write(path, "services:\n  web:\n    image: nginx:1.27\n").unwrap();
        let edited = cache.parsed(path).unwrap();
        assert_eq!(edited.services.len(), 1);
    }
}
