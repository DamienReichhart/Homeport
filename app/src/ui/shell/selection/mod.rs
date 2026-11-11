//! The selection: the one Host or Cluster the resource pages show
//! (`src/core/selection`). It is remembered across launches in the
//! Settings (`lastSelection`, and `lastHostId` so an older Homeport opens
//! on the same Host), exactly where the React app kept it.

pub mod remember;
#[cfg(test)]
mod tests;

use homeport_settings::Settings;

use crate::{
    data::{keys, Key},
    ui::shell::registry::Scope,
};

pub use homeport_settings::Selection;

/// A Host or a Cluster the selection may fall back to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidate {
    pub id: String,
    /// Connected, so its pages show something at once.
    pub usable: bool,
}

impl Candidate {
    pub fn new(id: impl Into<String>, usable: bool) -> Self {
        Self {
            id: id.into(),
            usable,
        }
    }
}

/// The id of what is selected, of either kind.
pub fn id(selection: &Selection) -> &str {
    match selection {
        Selection::Host { id } | Selection::Cluster { id } => id,
    }
}

/// What kind of pages a selection shows.
pub fn kind(selection: &Selection) -> Scope {
    match selection {
        Selection::Host { .. } => Scope::Host,
        Selection::Cluster { .. } => Scope::Cluster,
    }
}

/// The remembered selection, from the settings of this or an older version.
pub fn remembered(settings: &Settings) -> Option<Selection> {
    settings.last_selection.clone().or_else(|| {
        settings
            .last_host_id
            .clone()
            .filter(|id| !id.is_empty())
            .map(|id| Selection::Host { id })
    })
}

/// The selection the pages use: `chosen` while it still exists, else the
/// first usable Host, the first Host, the first usable Cluster or the first
/// Cluster. A list still loading (`None`) keeps `chosen` as it is.
pub fn resolve(
    chosen: Option<&Selection>,
    hosts: Option<&[Candidate]>,
    clusters: Option<&[Candidate]>,
) -> Option<Selection> {
    match chosen {
        Some(chosen) => {
            let list = match chosen {
                Selection::Host { .. } => hosts,
                Selection::Cluster { .. } => clusters,
            };
            if list.is_none_or(|list| list.iter().any(|c| c.id == id(chosen))) {
                return Some(chosen.clone());
            }
        }
        None if hosts.is_none() => return None,
        None => {}
    }
    fn first(list: Option<&[Candidate]>) -> Option<&Candidate> {
        let list = list?;
        list.iter().find(|c| c.usable).or_else(|| list.first())
    }
    if let Some(host) = first(hosts) {
        return Some(Selection::Host {
            id: host.id.clone(),
        });
    }
    first(clusters).map(|cluster| Selection::Cluster {
        id: cluster.id.clone(),
    })
}

/// Whether a page of `scope` must give way to the Overview: a Host page
/// shows only while a Host is selected, and a Cluster page only while a
/// Cluster is (`ScopeGuard`). With nothing selected every page shows, each
/// explaining what to add.
pub fn sends_to_overview(scope: Scope, selection: Option<&Selection>) -> bool {
    scope != Scope::App && selection.is_some_and(|selection| kind(selection) != scope)
}

/// Everything the data layer holds about what is selected: what
/// `Data::release` lets go of when the selection moves.
pub fn scopes(selection: &Selection) -> Vec<Key> {
    match selection {
        Selection::Host { id } => vec![keys::host::scope(id)],
        Selection::Cluster { id } => vec![keys::cluster::scope(id), keys::nomad::scope(id)],
    }
}
