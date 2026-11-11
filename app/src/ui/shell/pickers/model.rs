//! Which namespace or project a Host's or a Cluster's pages show: the one
//! remembered for it in the Settings, else the one that makes sense
//! (`src/core/host/model/{containerd,incus}.ts`,
//! `src/core/cluster/model/namespace.ts`, `src/core/nomad/model/namespace.ts`).

use std::collections::BTreeMap;

use homeport_clusters::service::ClusterView;
use homeport_containerd::namespaces::ContainerdNamespace;
use homeport_incus::projects::IncusProject;
use homeport_nomad::namespaces::NomadNamespaces;

use crate::ui::shell::selection::remember::ALL_NAMESPACES;

/// The namespace nerdctl and `ctr` use when none is named.
pub const DEFAULT_NAMESPACE: &str = "default";
/// The project every Incus has.
pub const DEFAULT_PROJECT: &str = "default";
/// What Nomad's API calls every namespace the token can read.
pub const ALL_NOMAD_NAMESPACES: &str = "*";

/// What was picked for each Host or Cluster, by id.
pub type Remembered = BTreeMap<String, String>;

/// The namespace a containerd Host's pages show until one is picked:
/// `default`, unless it holds nothing and exactly one other namespace does.
pub fn preferred_namespace(namespaces: &[ContainerdNamespace]) -> &str {
    let holding: Vec<&ContainerdNamespace> = namespaces
        .iter()
        .filter(|n| n.containers > 0 || n.images > 0)
        .collect();
    let default_holds = holding.iter().any(|n| n.name == DEFAULT_NAMESPACE);
    match holding.as_slice() {
        [only] if !default_holds => &only.name,
        _ => DEFAULT_NAMESPACE,
    }
}

/// The namespace remembered for `host_id`, else the preferred one of
/// `namespaces`; `None` while neither is known.
pub fn containerd_namespace(
    host_id: &str,
    remembered: &Remembered,
    namespaces: Option<&[ContainerdNamespace]>,
) -> Option<String> {
    if let Some(value) = remembered.get(host_id).filter(|value| !value.is_empty()) {
        return Some(value.clone());
    }
    namespaces.map(|list| preferred_namespace(list).to_owned())
}

/// The project an Incus Host's pages show until one is picked: `default`
/// when the connection may use it, else the first it lists (the restricted
/// socket's own project).
pub fn preferred_project(projects: &[IncusProject]) -> &str {
    match projects.first() {
        Some(first) if !projects.iter().any(|p| p.name == DEFAULT_PROJECT) => &first.name,
        _ => DEFAULT_PROJECT,
    }
}

/// The project remembered for `host_id` if the Host still has it, else the
/// preferred one of `projects`; `None` while neither is known.
pub fn incus_project(
    host_id: &str,
    remembered: &Remembered,
    projects: Option<&[IncusProject]>,
) -> Option<String> {
    let still_there = |value: &String| {
        projects.is_none_or(|list| list.is_empty() || list.iter().any(|p| &p.name == value))
    };
    if let Some(value) = remembered
        .get(host_id)
        .filter(|value| !value.is_empty() && still_there(value))
    {
        return Some(value.clone());
    }
    projects.map(|list| preferred_project(list).to_owned())
}

/// The Namespace a Cluster's pages show: the one remembered for it, else
/// its context's, else `default`. `None` means every Namespace.
pub fn cluster_namespace(cluster: &ClusterView, remembered: &Remembered) -> Option<String> {
    match remembered.get(&cluster.cluster.id) {
        Some(value) if value == ALL_NAMESPACES => None,
        Some(value) => Some(value.clone()),
        None => Some(
            cluster
                .cluster
                .kubernetes()
                .and_then(|connection| connection.namespace.clone())
                .unwrap_or_else(|| DEFAULT_NAMESPACE.to_owned()),
        ),
    }
}

/// The namespace the Jobs page asks Nomad for: `None` (the token's
/// default) while the namespaces are unknown or can't be listed, `*` for
/// all, the one chosen, or `default` when the chosen one is gone.
pub fn nomad_namespace(listing: Option<&NomadNamespaces>, choice: Option<&str>) -> Option<String> {
    let listing = listing.filter(|listing| listing.listable)?;
    Some(match choice {
        None => ALL_NOMAD_NAMESPACES.to_owned(),
        Some(choice) if listing.namespaces.iter().any(|n| n.name == choice) => choice.to_owned(),
        Some(_) => DEFAULT_NAMESPACE.to_owned(),
    })
}

/// The names a picker lists: those known, with the chosen one first when
/// it isn't among them (it may be gone, or still loading).
pub fn listed(names: impl IntoIterator<Item = String>, chosen: Option<&str>) -> Vec<String> {
    let mut names: Vec<String> = names.into_iter().collect();
    if let Some(chosen) = chosen.filter(|chosen| !names.iter().any(|name| name == chosen)) {
        names.insert(0, chosen.to_owned());
    }
    names
}
