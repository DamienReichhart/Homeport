//! The namespace and project pickers: the containerd namespace and the
//! Incus project under the Host switcher, and the Kubernetes and Nomad
//! namespaces a Cluster page puts in its header. What is picked is
//! remembered per Host or Cluster in the Settings.

pub mod model;
#[cfg(test)]
mod tests;
pub mod view;

use homeport_clusters::model::ClusterKind;

use super::{page::World, router::Route, selection::remember::Remember, status};
use crate::{t, ui::widgets::form::select};

/// The pickers' lists, open or closed, and the Namespace being typed where
/// the credentials can't list them.
#[derive(Debug, Clone, Default)]
pub struct State {
    containerd: select::State,
    incus: select::State,
    namespace: select::State,
    typed: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    Containerd(select::Message),
    Incus(select::Message),
    Namespace(select::Message),
    /// A Namespace is being typed.
    Typed(String),
    /// Enter in that field.
    TypedSubmit,
}

/// What a pick leads to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Picked {
    Remember(Remember),
    /// "Manage namespaces", the last line of the containerd picker.
    Go(Route),
}

/// A picker's lines: what each stands for, what each says, and the chosen.
pub struct Options {
    /// The value of each line; `None` for "All namespaces" and for
    /// "Manage namespaces".
    pub values: Vec<Option<String>>,
    pub labels: Vec<String>,
    pub selected: Option<usize>,
}

impl Options {
    fn of(names: Vec<String>, says: impl Fn(&str) -> String, chosen: Option<&str>) -> Self {
        Self {
            selected: chosen.and_then(|chosen| names.iter().position(|name| name == chosen)),
            labels: names.iter().map(|name| says(name)).collect(),
            values: names.into_iter().map(Some).collect(),
        }
    }

    /// The same with a line that stands for no name, first or last.
    fn with(mut self, label: String, first: bool, selected: bool) -> Self {
        let at = if first { 0 } else { self.values.len() };
        self.values.insert(at, None);
        self.labels.insert(at, label);
        self.selected = if selected {
            Some(at)
        } else {
            self.selected.map(|index| index + usize::from(first))
        };
        self
    }
}

/// The connected containerd Host that is selected, if one is.
fn engine_host(world: &World, capability: super::registry::Capability) -> Option<&str> {
    let host = world.host()?;
    (status::engine::supports(Some(host), capability) && status::host::is_connected(&host.status))
        .then_some(host.host.id.as_str())
}

/// The containerd picker's lines: each namespace with what it holds, then
/// "Manage namespaces". `None` unless a connected containerd Host is selected.
pub fn containerd(world: &World) -> Option<(&str, Options)> {
    let host_id = engine_host(world, super::registry::Capability::Containerd)?;
    let known = world.containerd.data().map(|list| list.as_slice());
    let failed = world.containerd.error().map(|_| [].as_slice());
    let chosen = model::containerd_namespace(
        host_id,
        &world.settings.containerd_namespaces,
        known.or(failed),
    );
    let list = known.unwrap_or_default();
    let names = model::listed(list.iter().map(|n| n.name.clone()), chosen.as_deref());
    let says = |name: &str| match list.iter().find(|n| n.name == name) {
        Some(n) => format!(
            "{name}  {}",
            t!(
                "core.containerd.holds",
                containers = n.containers,
                images = n.images
            )
        ),
        None => name.to_owned(),
    };
    let options = Options::of(names, says, chosen.as_deref()).with(
        t!("core.containerd.manage"),
        false,
        false,
    );
    Some((host_id, options))
}

/// The Incus picker's lines: each project with what it holds.
pub fn incus(world: &World) -> Option<(&str, Options)> {
    let host_id = engine_host(world, super::registry::Capability::Incus)?;
    let known = world.incus.data().map(|list| list.as_slice());
    let failed = world.incus.error().map(|_| [].as_slice());
    let chosen = model::incus_project(host_id, &world.settings.incus_projects, known.or(failed));
    let list = known.unwrap_or_default();
    let names = model::listed(list.iter().map(|p| p.name.clone()), chosen.as_deref());
    let says = |name: &str| match list.iter().find(|p| p.name == name) {
        Some(p) => format!(
            "{name}  {}",
            t!(
                "core.incus.holds",
                instances = p.instances,
                images = p.images
            )
        ),
        None => name.to_owned(),
    };
    Some((host_id, Options::of(names, says, chosen.as_deref())))
}

/// The Namespace picker's lines for the selected Cluster: "All namespaces"
/// first. `None` when there is nothing to pick from: no Cluster, a
/// Kubernetes one whose Namespaces can't be listed (they are typed), a
/// Nomad one whose token can't list them.
pub fn namespace(world: &World) -> Option<(&str, Options)> {
    let cluster = world.cluster()?;
    let chosen = model::cluster_namespace(cluster, &world.settings.cluster_namespaces);
    let (names, all) = match cluster.cluster.kind {
        ClusterKind::Kubernetes => {
            if world.namespaces.error().is_some() {
                return None;
            }
            let known = world.namespaces.data().map(|list| list.as_slice());
            let names = known.unwrap_or_default().iter().map(|n| n.name.clone());
            (
                model::listed(names, chosen.as_deref()),
                t!("core.namespace.all"),
            )
        }
        ClusterKind::Nomad => {
            let listing = world.nomad.data().filter(|listing| listing.listable)?;
            let names = listing.namespaces.iter().map(|n| n.name.clone()).collect();
            (names, t!("nomad.namespace.all"))
        }
    };
    let options =
        Options::of(names, str::to_owned, chosen.as_deref()).with(all, true, chosen.is_none());
    Some((cluster.cluster.id.as_str(), options))
}

impl State {
    /// What is typed in the Namespace field, starting from the chosen one.
    pub fn typed(&self, world: &World) -> String {
        self.typed.clone().unwrap_or_else(|| {
            world
                .cluster()
                .and_then(|c| model::cluster_namespace(c, &world.settings.cluster_namespaces))
                .unwrap_or_default()
        })
    }

    /// Applies `message` and returns what it picked, if it picked.
    pub fn update(&mut self, message: Message, world: &World) -> Option<Picked> {
        match message {
            Message::Containerd(message) => {
                let (host_id, options) = containerd(world)?;
                let index = self.containerd.update(message, &options.labels)?;
                Some(match options.values.get(index)?.clone() {
                    Some(namespace) => Picked::Remember(Remember::ContainerdNamespace {
                        host_id: host_id.to_owned(),
                        namespace,
                    }),
                    None => Picked::Go(Route::ContainerdNamespaces),
                })
            }
            Message::Incus(message) => {
                let (host_id, options) = incus(world)?;
                let index = self.incus.update(message, &options.labels)?;
                Some(Picked::Remember(Remember::IncusProject {
                    host_id: host_id.to_owned(),
                    project: options.values.get(index)?.clone()?,
                }))
            }
            Message::Namespace(message) => {
                let (cluster_id, options) = namespace(world)?;
                let index = self.namespace.update(message, &options.labels)?;
                Some(Picked::Remember(Remember::ClusterNamespace {
                    cluster_id: cluster_id.to_owned(),
                    namespace: options.values.get(index)?.clone(),
                }))
            }
            Message::Typed(typed) => {
                self.typed = Some(typed);
                None
            }
            Message::TypedSubmit => {
                let typed = self.typed.take()?;
                let namespace = Some(typed.trim().to_owned()).filter(|name| !name.is_empty());
                Some(Picked::Remember(Remember::ClusterNamespace {
                    cluster_id: world.cluster_id()?.to_owned(),
                    namespace,
                }))
            }
        }
    }
}
