//! What a Stack is, and finding every Stack of a Host from its containers.

use std::collections::BTreeMap;

use serde::Serialize;

use homeport_engine::containers::Container;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum StackKind {
    /// Homeport holds its compose definition and environment.
    Managed,
    /// Its compose files are a folder on this machine (ADR 0008).
    Linked,
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
    /// A Managed or Linked Stack with no containers yet.
    NotDeployed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct StackService {
    pub name: String,
    pub container_ids: Vec<String>,
    pub running: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Stack {
    pub name: String,
    pub kind: StackKind,
    /// The stored record of a Managed or Linked Stack.
    pub id: Option<String>,
    pub state: StackState,
    pub services: Vec<StackService>,
    pub running: u32,
    pub total: u32,
    /// Where the compose files lived for whoever deployed it (on that machine).
    pub config_files: Option<String>,
    /// The folder of a Linked Stack, on this machine.
    pub working_dir: Option<String>,
}

/// A Stack Homeport has a record of, deployed or not.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StackRecord {
    pub id: String,
    pub name: String,
    /// Managed or Linked.
    pub kind: StackKind,
    /// The folder of a Linked Stack.
    pub working_dir: Option<String>,
}

impl StackRecord {
    pub fn managed(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            kind: StackKind::Managed,
            working_dir: None,
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
        }
    }
}

const CONFIG_FILES: &str = "com.docker.compose.project.config_files";

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
                    });
                s.container_ids.push(c.id.clone());
                s.running += u32::from(c.state == "running");
            }
            let total = members.len() as u32;
            let running = members.iter().filter(|c| c.state == "running").count() as u32;
            let record = record(name);
            Stack {
                name: name.to_owned(),
                kind: record.map_or(StackKind::External, |r| r.kind),
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
            }
        })
        .collect();

    for r in records {
        if !stacks.iter().any(|s| s.name == r.name) {
            stacks.push(Stack {
                name: r.name.clone(),
                kind: r.kind,
                id: Some(r.id.clone()),
                state: StackState::NotDeployed,
                services: vec![],
                running: 0,
                total: 0,
                config_files: None,
                working_dir: r.working_dir.clone(),
            });
        }
    }
    stacks.sort_by(|a, b| a.name.cmp(&b.name));
    stacks
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    fn container(id: &str, stack: Option<&str>, service: &str, state: &str) -> Container {
        Container {
            id: id.into(),
            name: format!("{}-{service}-1", stack.unwrap_or("solo")),
            image: "img".into(),
            image_id: "sha".into(),
            command: String::new(),
            created: 0,
            state: state.into(),
            status: String::new(),
            health: None,
            ports: vec![],
            labels: HashMap::new(),
            stack: stack.map(str::to_owned),
            service: Some(service.into()),
            networks: vec![],
        }
    }

    #[test]
    fn containers_group_into_stacks_by_project() {
        let containers = [
            container("1", Some("shop"), "web", "running"),
            container("2", Some("shop"), "web", "running"),
            container("3", Some("shop"), "db", "exited"),
            container("4", Some("blog"), "app", "exited"),
            container("5", None, "lonely", "running"),
        ];
        let stacks = derive(&containers, &[StackRecord::managed("m1", "shop")]);
        assert_eq!(stacks.len(), 2);

        let blog = &stacks[0];
        assert_eq!(
            (blog.name.as_str(), blog.kind, blog.state),
            ("blog", StackKind::External, StackState::Stopped)
        );

        let shop = &stacks[1];
        assert_eq!(shop.kind, StackKind::Managed);
        assert_eq!(shop.id.as_deref(), Some("m1"));
        assert_eq!(
            (shop.running, shop.total, shop.state),
            (2, 3, StackState::Partial)
        );
        let services: Vec<_> = shop
            .services
            .iter()
            .map(|s| (s.name.as_str(), s.container_ids.len(), s.running))
            .collect();
        assert_eq!(services, [("db", 1, 0), ("web", 2, 2)]);
    }

    #[test]
    fn managed_stacks_without_containers_are_not_deployed() {
        let stacks = derive(&[], &[StackRecord::managed("m1", "new")]);
        assert_eq!(stacks[0].state, StackState::NotDeployed);
        assert_eq!(stacks[0].kind, StackKind::Managed);
    }

    #[test]
    fn all_three_kinds_come_from_containers_and_records() {
        let containers = [
            container("1", Some("shop"), "web", "running"),
            container("2", Some("blog"), "app", "running"),
            container("3", Some("misc"), "x", "running"),
        ];
        let records = [
            StackRecord::managed("m1", "shop"),
            StackRecord::linked("l1", "blog", "/srv/blog"),
            StackRecord::linked("l2", "draft", "/srv/draft"),
        ];
        let stacks = derive(&containers, &records);
        let kinds: Vec<_> = stacks
            .iter()
            .map(|s| (s.name.as_str(), s.kind, s.id.as_deref(), s.state))
            .collect();
        assert_eq!(
            kinds,
            [
                ("blog", StackKind::Linked, Some("l1"), StackState::Running),
                (
                    "draft",
                    StackKind::Linked,
                    Some("l2"),
                    StackState::NotDeployed
                ),
                ("misc", StackKind::External, None, StackState::Running),
                ("shop", StackKind::Managed, Some("m1"), StackState::Running),
            ]
        );
        assert_eq!(stacks[0].working_dir.as_deref(), Some("/srv/blog"));
        assert_eq!(stacks[1].working_dir.as_deref(), Some("/srv/draft"));
    }

    #[test]
    fn fully_running_stacks_are_running() {
        let stacks = derive(&[container("1", Some("a"), "x", "running")], &[]);
        assert_eq!(stacks[0].state, StackState::Running);
    }
}
