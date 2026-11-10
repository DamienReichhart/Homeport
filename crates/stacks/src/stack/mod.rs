//! What a Stack is, and finding every Stack of a Host from its containers.

use std::collections::BTreeMap;

use serde::Serialize;

use homeport_engine::containers::Container;

use crate::linked::detect::CONFIG_FILES;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum StackKind {
    /// Homeport holds its compose definition and environment.
    Managed,
    /// Its compose files are a folder on this machine (ADR 0008).
    Linked,
    /// Its compose file comes from a git repository (ADR 0014).
    Synced,
    /// Only its containers are known.
    External,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum StackState {
    /// Every container is running.
    Running,
    /// Some containers are running.
    Partial,
    /// Containers exist but none is running.
    Stopped,
    /// A Managed, Linked or Synced Stack with no containers yet.
    NotDeployed,
}

/// Why a Stack Homeport has a record of can't be used as its kind says.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum StackProblem {
    /// A Linked Stack's compose files aren't on this machine any more, so
    /// it's shown as External until they come back.
    FilesMissing,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct StackService {
    pub name: String,
    pub container_ids: Vec<String>,
    pub running: u32,
    /// On a Swarm: the service's ID, and how many tasks should run.
    pub service_id: Option<String>,
    pub desired: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Stack {
    pub name: String,
    pub kind: StackKind,
    /// The stored record of a Managed, Linked or Synced Stack.
    pub id: Option<String>,
    pub state: StackState,
    pub services: Vec<StackService>,
    pub running: u32,
    pub total: u32,
    /// Where the compose files lived for whoever deployed it (on that machine).
    pub config_files: Option<String>,
    /// The folder of a Linked Stack, on this machine.
    pub working_dir: Option<String>,
    /// Why a recorded Stack is shown as another kind.
    pub problem: Option<StackProblem>,
    /// A Synced Stack whose fetched commit changes its compose file's folder
    /// since the deployed one.
    pub update_available: bool,
    /// On a Swarm: its deploys send the docker CLI's registry logins.
    pub registry_auth: bool,
}

/// A Stack Homeport has a record of, deployed or not.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StackRecord {
    pub id: String,
    pub name: String,
    /// Managed, Linked or Synced.
    pub kind: StackKind,
    /// The folder of a Linked Stack.
    pub working_dir: Option<String>,
    /// A Linked Stack whose files are gone: shown as External, with why.
    pub files_missing: bool,
    /// A Synced Stack with a new commit waiting to be deployed.
    pub update_available: bool,
    /// On a Swarm: its deploys send the docker CLI's registry logins.
    pub registry_auth: bool,
}

impl StackRecord {
    pub fn managed(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            kind: StackKind::Managed,
            working_dir: None,
            files_missing: false,
            update_available: false,
            registry_auth: false,
        }
    }

    pub fn linked(
        id: impl Into<String>,
        name: impl Into<String>,
        working_dir: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            kind: StackKind::Linked,
            working_dir: Some(working_dir.into()),
            files_missing: false,
            update_available: false,
            registry_auth: false,
        }
    }

    pub fn synced(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            kind: StackKind::Synced,
            working_dir: None,
            files_missing: false,
            update_available: false,
            registry_auth: false,
        }
    }

    /// Marks a Synced Stack with a new commit waiting to be deployed.
    pub fn with_update_available(mut self, available: bool) -> Self {
        self.update_available = available;
        self
    }

    /// Notes that its swarm deploys send the docker CLI's registry logins.
    pub fn with_registry_auth(mut self, on: bool) -> Self {
        self.registry_auth = on;
        self
    }

    /// Marks a Linked Stack whose files are missing on this machine.
    pub fn with_files_missing(mut self, missing: bool) -> Self {
        self.files_missing = missing;
        self
    }

    /// The kind the Stack is shown as.
    fn shown_kind(&self) -> StackKind {
        if self.files_missing {
            StackKind::External
        } else {
            self.kind
        }
    }

    fn problem(&self) -> Option<StackProblem> {
        self.files_missing.then_some(StackProblem::FilesMissing)
    }
}

/// Groups a Host's containers into Stacks, naming the kind of each from
/// Homeport's `records`, and adds the recorded Stacks that have not been
/// deployed.
pub fn derive(containers: &[Container], records: &[StackRecord]) -> Vec<Stack> {
    let mut projects: BTreeMap<&str, Vec<&Container>> = BTreeMap::new();
    for c in containers {
        if let Some(project) = c.stack.as_deref() {
            projects.entry(project).or_default().push(c);
        }
    }
    let record = |name: &str| records.iter().find(|r| r.name == name);

    let mut stacks: Vec<Stack> = projects
        .into_iter()
        .map(|(name, members)| {
            let mut services: BTreeMap<String, StackService> = BTreeMap::new();
            for c in &members {
                let service_name = c.service.clone().unwrap_or_else(|| c.name.clone());
                let s = services
                    .entry(service_name.clone())
                    .or_insert(StackService {
                        name: service_name,
                        container_ids: vec![],
                        running: 0,
                        service_id: None,
                        desired: None,
                    });
                s.container_ids.push(c.id.clone());
                s.running += u32::from(c.state == "running");
            }
            let total = members.len() as u32;
            let running = members.iter().filter(|c| c.state == "running").count() as u32;
            let record = record(name);
            Stack {
                name: name.to_owned(),
                kind: record.map_or(StackKind::External, StackRecord::shown_kind),
                id: record.map(|r| r.id.clone()),
                state: match running {
                    0 => StackState::Stopped,
                    r if r == total => StackState::Running,
                    _ => StackState::Partial,
                },
                services: services.into_values().collect(),
                running,
                total,
                config_files: members
                    .iter()
                    .find_map(|c| c.labels.get(CONFIG_FILES).cloned()),
                working_dir: record.and_then(|r| r.working_dir.clone()),
                problem: record.and_then(StackRecord::problem),
                update_available: record.is_some_and(|r| r.update_available),
                registry_auth: record.is_some_and(|r| r.registry_auth),
            }
        })
        .collect();

    for r in records {
        if !stacks.iter().any(|s| s.name == r.name) {
            stacks.push(Stack {
                name: r.name.clone(),
                kind: r.shown_kind(),
                id: Some(r.id.clone()),
                state: StackState::NotDeployed,
                services: vec![],
                running: 0,
                total: 0,
                config_files: None,
                working_dir: r.working_dir.clone(),
                problem: r.problem(),
                update_available: r.update_available,
                registry_auth: r.registry_auth,
            });
        }
    }
    stacks.sort_by(|a, b| a.name.cmp(&b.name));
    stacks
}

#[cfg(test)]
mod tests;
