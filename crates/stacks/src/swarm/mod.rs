//! Swarm stacks: Stacks deployed to a Swarm, found through the
//! `com.docker.stack.namespace` label `docker stack deploy` puts on their
//! services, and removed the way `docker stack rm` does, through the
//! Engine API of a manager (ADR 0016).

use std::{collections::BTreeMap, collections::HashMap, time::Duration};

use bollard::{
    query_parameters::{
        ListConfigsOptionsBuilder, ListNetworksOptionsBuilder, ListSecretsOptionsBuilder,
        ListServicesOptionsBuilder, ListTasksOptionsBuilder,
    },
    Docker,
};
use serde::Serialize;

use homeport_core::error::AppResult;
use homeport_engine::swarm::services::{ServiceView, STACK_NAMESPACE};

use crate::{
    linked::{detect, FileCache, LinkedStore},
    owner::StackOwner,
    stack::{Stack, StackKind, StackRecord, StackService, StackState},
    store::StackStore,
    synced::SyncedStore,
};

/// How long removing a swarm stack waits for its tasks to stop before
/// removing its networks.
const TASKS_GONE_WITHIN: Duration = Duration::from_secs(30);

/// A service's name within its stack: `docker stack deploy` names it
/// `<stack>_<service>`.
pub fn short_name<'a>(stack: &str, service: &'a str) -> &'a str {
    service
        .strip_prefix(stack)
        .and_then(|rest| rest.strip_prefix('_'))
        .unwrap_or(service)
}

/// Groups a Swarm's services into Stacks by namespace, naming the kind of
/// each from Homeport's `records`, and adds the recorded Stacks that have
/// not been deployed. A Stack runs when every task of every service does,
/// and is stopped when every service is scaled to 0.
pub fn derive_swarm(services: &[ServiceView], records: &[StackRecord]) -> Vec<Stack> {
    let mut namespaces: BTreeMap<&str, Vec<&ServiceView>> = BTreeMap::new();
    for s in services {
        if let Some(namespace) = s.stack.as_deref() {
            namespaces.entry(namespace).or_default().push(s);
        }
    }
    let record = |name: &str| records.iter().find(|r| r.name == name);
    let count = |n: u64| u32::try_from(n).unwrap_or(u32::MAX);
    let mut stacks: Vec<Stack> = namespaces
        .into_iter()
        .map(|(name, members)| {
            let running: u64 = members.iter().map(|s| s.running).sum();
            let desired: u64 = members.iter().map(|s| s.desired).sum();
            let record = record(name);
            Stack {
                name: name.to_owned(),
                kind: record.map_or(StackKind::External, |r| r.kind),
                id: record.map(|r| r.id.clone()),
                state: if desired == 0 {
                    StackState::Stopped
                } else if running >= desired {
                    StackState::Running
                } else {
                    StackState::Partial
                },
                services: members
                    .iter()
                    .map(|s| StackService {
                        name: short_name(name, &s.name).to_owned(),
                        container_ids: vec![],
                        running: count(s.running),
                        service_id: Some(s.id.clone()),
                        desired: Some(count(s.desired)),
                    })
                    .collect(),
                running: count(running),
                total: count(desired),
                config_files: None,
                working_dir: record.and_then(|r| r.working_dir.clone()),
                problem: None,
                update_available: record.is_some_and(|r| r.update_available),
                registry_auth: record.is_some_and(|r| r.registry_auth),
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
                problem: None,
                update_available: r.update_available,
                registry_auth: r.registry_auth,
            });
        }
    }
    stacks.sort_by(|a, b| a.name.cmp(&b.name));
    stacks
}

/// Every Stack of the Swarm `swarm_id`, given its `services`: those deployed
/// (External unless Homeport has a record of them) and the Managed, Linked
/// and Synced ones not deployed yet. A swarm stack is never recorded as
/// Linked on its own: its services don't say where their files are.
pub fn list(
    swarm_id: &str,
    services: &[ServiceView],
    managed: &StackStore,
    linked: &LinkedStore,
    synced: &SyncedStore,
    cache: &FileCache,
) -> AppResult<Vec<Stack>> {
    let owner = StackOwner::Swarm(swarm_id.to_owned());
    let mut records: Vec<StackRecord> = managed
        .list_of(&owner)?
        .into_iter()
        .map(|s| StackRecord::managed(s.id, s.name).with_registry_auth(s.registry_auth))
        .collect();
    records.extend(synced.list_of(&owner)?.into_iter().map(|s| {
        StackRecord::synced(s.id, s.name)
            .with_update_available(s.update_available)
            .with_registry_auth(s.registry_auth)
    }));
    records.extend(linked.list_of(&owner)?.into_iter().map(|l| {
        let missing = !detect::files_present(cache, &l.config_files);
        StackRecord::linked(l.id, l.name, l.working_dir)
            .with_files_missing(missing)
            .with_registry_auth(l.registry_auth)
    }));
    let mut stacks = derive_swarm(services, &records);
    for stack in &mut stacks {
        if let Some(record) = records
            .iter()
            .find(|r| r.name == stack.name && r.files_missing)
        {
            stack.kind = StackKind::External;
            stack.problem = Some(crate::stack::StackProblem::FilesMissing);
            stack.id = Some(record.id.clone());
        }
    }
    Ok(stacks)
}

/// Checks that the compose files `texts` describe the swarm stack `name` as
/// it runs: the same services, by name. A swarm stack's services don't say
/// where their files are, so linking one to a folder is the user's call,
/// and this keeps it from being linked to another project's.
pub fn check_link(name: &str, services: &[ServiceView], texts: &[&str]) -> AppResult<()> {
    let mut running: Vec<String> = services
        .iter()
        .filter(|s| s.stack.as_deref() == Some(name))
        .map(|s| short_name(name, &s.name).to_owned())
        .collect();
    running.sort();
    let declared = crate::definition::service_names(texts);
    if running == declared {
        return Ok(());
    }
    Err(homeport_core::error::AppError::validation(
        crate::messages::SWARM_LINK_MISMATCH
            .with("running", running.join(", "))
            .with("declared", declared.join(", ")),
    ))
}

/// What a swarm stack is made of, by name.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SwarmStackParts {
    pub networks: Vec<String>,
    pub secrets: Vec<String>,
    pub configs: Vec<String>,
}

fn namespace_filter(name: &str) -> HashMap<&'static str, Vec<String>> {
    HashMap::from([("label", vec![format!("{STACK_NAMESPACE}={name}")])])
}

/// The networks, secrets and configs of the swarm stack `name`.
pub async fn parts(docker: &Docker, name: &str) -> AppResult<SwarmStackParts> {
    let filter = namespace_filter(name);
    let networks = docker
        .list_networks(Some(
            ListNetworksOptionsBuilder::default()
                .filters(&filter)
                .build(),
        ))
        .await?;
    let secrets = docker
        .list_secrets(Some(
            ListSecretsOptionsBuilder::default()
                .filters(&filter)
                .build(),
        ))
        .await?;
    let configs = docker
        .list_configs(Some(
            ListConfigsOptionsBuilder::default()
                .filters(&filter)
                .build(),
        ))
        .await?;
    let mut parts = SwarmStackParts {
        networks: networks.into_iter().filter_map(|n| n.name).collect(),
        secrets: secrets.into_iter().filter_map(|s| s.spec?.name).collect(),
        configs: configs.into_iter().filter_map(|c| c.spec?.name).collect(),
    };
    parts.networks.sort();
    parts.secrets.sort();
    parts.configs.sort();
    Ok(parts)
}

/// What removing a swarm stack left behind, by name, with why.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct RemoveReport {
    pub kept: Vec<String>,
}

/// What `docker stack rm` removes, in its order: the services, then (once
/// their tasks are gone) the networks, secrets and configs of the namespace.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Part {
    Service,
    Network,
    Secret,
    Config,
}

pub(crate) const REMOVAL_ORDER: [Part; 4] =
    [Part::Service, Part::Network, Part::Secret, Part::Config];

/// Removes the swarm stack `name` through the Engine API, as `docker stack
/// rm` does; what can't be removed is reported, the rest still goes.
pub async fn remove(docker: &Docker, name: &str) -> AppResult<RemoveReport> {
    let filter = namespace_filter(name);
    let mut report = RemoveReport::default();
    let mut removed_services = Vec::new();
    for part in REMOVAL_ORDER {
        match part {
            Part::Service => {
                let services = docker
                    .list_services(Some(
                        ListServicesOptionsBuilder::default()
                            .filters(&filter)
                            .build(),
                    ))
                    .await?;
                for s in services {
                    let (Some(id), label) = (s.id, s.spec.and_then(|s| s.name)) else {
                        continue;
                    };
                    match docker.delete_service(&id).await {
                        Ok(()) => removed_services.push(id),
                        Err(e) => report.kept.push(format!("{} ({e})", label.unwrap_or(id))),
                    }
                }
                wait_for_tasks(docker, &removed_services).await;
            }
            Part::Network => {
                let networks = docker
                    .list_networks(Some(
                        ListNetworksOptionsBuilder::default()
                            .filters(&filter)
                            .build(),
                    ))
                    .await?;
                for n in networks {
                    let Some(id) = n.id else { continue };
                    if let Err(e) = docker.remove_network(&id).await {
                        report.kept.push(format!("{} ({e})", n.name.unwrap_or(id)));
                    }
                }
            }
            Part::Secret => {
                let secrets = docker
                    .list_secrets(Some(
                        ListSecretsOptionsBuilder::default()
                            .filters(&filter)
                            .build(),
                    ))
                    .await?;
                for s in secrets {
                    let Some(id) = s.id else { continue };
                    if let Err(e) = docker.delete_secret(&id).await {
                        let label = s.spec.and_then(|s| s.name).unwrap_or(id);
                        report.kept.push(format!("{label} ({e})"));
                    }
                }
            }
            Part::Config => {
                let configs = docker
                    .list_configs(Some(
                        ListConfigsOptionsBuilder::default()
                            .filters(&filter)
                            .build(),
                    ))
                    .await?;
                for c in configs {
                    let Some(id) = c.id else { continue };
                    if let Err(e) = docker.delete_config(&id).await {
                        let label = c.spec.and_then(|s| s.name).unwrap_or(id);
                        report.kept.push(format!("{label} ({e})"));
                    }
                }
            }
        }
    }
    Ok(report)
}

/// Waits, up to a timeout, until the removed services have no task left,
/// since a network can't be removed while a task is attached to it.
async fn wait_for_tasks(docker: &Docker, services: &[String]) {
    if services.is_empty() {
        return;
    }
    let ids: Vec<&str> = services.iter().map(String::as_str).collect();
    let filters: HashMap<&str, Vec<&str>> = [("service", ids)].into();
    let deadline = tokio::time::Instant::now() + TASKS_GONE_WITHIN;
    while tokio::time::Instant::now() < deadline {
        match docker
            .list_tasks(Some(
                ListTasksOptionsBuilder::default().filters(&filters).build(),
            ))
            .await
        {
            Ok(tasks) if tasks.is_empty() => return,
            Ok(_) => tokio::time::sleep(Duration::from_millis(500)).await,
            Err(_) => return,
        }
    }
}

#[cfg(test)]
mod tests;
