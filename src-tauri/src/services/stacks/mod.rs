//! Stacks: Compose projects on a Host (see GLOSSARY.md and ADR 0001).
//!
//! Every Stack is found through the `com.docker.compose.project` label. A
//! Managed Stack also has its compose definition stored here; an External
//! Stack only has its running containers until it is Adopted.

pub mod compose;
pub mod definition;
pub mod lifecycle;
pub mod store;

use std::collections::BTreeMap;

use serde::Serialize;

use crate::services::docker::containers::Container;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum StackKind {
    Managed,
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
    /// A Managed Stack with no containers yet.
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
    /// Present for Managed Stacks.
    pub managed_id: Option<String>,
    pub state: StackState,
    pub services: Vec<StackService>,
    pub running: u32,
    pub total: u32,
    /// Where the compose files lived for whoever deployed it (on that machine).
    pub config_files: Option<String>,
}

const CONFIG_FILES: &str = "com.docker.compose.project.config_files";

/// Groups a Host's containers into Stacks and adds Managed Stacks that have
/// not been deployed. `managed` pairs each Managed Stack's id with its name.
pub fn derive(containers: &[Container], managed: &[(String, String)]) -> Vec<Stack> {
    let mut projects: BTreeMap<&str, Vec<&Container>> = BTreeMap::new();
    for c in containers {
        if let Some(project) = c.stack.as_deref() {
            projects.entry(project).or_default().push(c);
        }
    }
    let managed_id = |name: &str| {
        managed
            .iter()
            .find(|(_, n)| n == name)
            .map(|(id, _)| id.clone())
    };

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
            let id = managed_id(name);
            Stack {
                name: name.to_owned(),
                kind: if id.is_some() {
                    StackKind::Managed
                } else {
                    StackKind::External
                },
                managed_id: id,
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
            }
        })
        .collect();

    for (id, name) in managed {
        if !stacks.iter().any(|s| &s.name == name) {
            stacks.push(Stack {
                name: name.clone(),
                kind: StackKind::Managed,
                managed_id: Some(id.clone()),
                state: StackState::NotDeployed,
                services: vec![],
                running: 0,
                total: 0,
                config_files: None,
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
        let stacks = derive(&containers, &[("m1".into(), "shop".into())]);
        assert_eq!(stacks.len(), 2);

        let blog = &stacks[0];
        assert_eq!(
            (blog.name.as_str(), blog.kind, blog.state),
            ("blog", StackKind::External, StackState::Stopped)
        );

        let shop = &stacks[1];
        assert_eq!(shop.kind, StackKind::Managed);
        assert_eq!(shop.managed_id.as_deref(), Some("m1"));
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
        let stacks = derive(&[], &[("m1".into(), "new".into())]);
        assert_eq!(stacks[0].state, StackState::NotDeployed);
        assert_eq!(stacks[0].kind, StackKind::Managed);
    }

    #[test]
    fn fully_running_stacks_are_running() {
        let stacks = derive(&[container("1", Some("a"), "x", "running")], &[]);
        assert_eq!(stacks[0].state, StackState::Running);
    }
}
