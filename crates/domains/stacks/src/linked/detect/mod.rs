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
mod tests;
