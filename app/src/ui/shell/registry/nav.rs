//! The sidebar's entries for a selection, and the keys that open them
//! (`visibleNavEntries`, `navEntriesOf` and `navShortcuts` in React).

use homeport_clusters::model::ClusterKind;

use super::{Capability, Feature, FeatureId, Label, Scope};
use crate::ui::{shell::router::Route, theme::Icon};

/// A sidebar entry, carrying what its feature requires.
#[derive(Debug, Clone)]
pub struct Entry {
    pub feature: FeatureId,
    pub route: Route,
    pub label: Label,
    pub icon: Icon,
    pub scope: Scope,
    /// What the selected Host must offer for the entry to show.
    pub requires: Option<Capability>,
    /// The kind the selected Cluster must be for the entry to show.
    pub cluster_kind: Option<ClusterKind>,
}

/// The sidebar entries of `features`, in their order.
pub fn entries_of(features: &[&Feature]) -> Vec<Entry> {
    features
        .iter()
        .filter_map(|feature| {
            let nav = feature.nav.as_ref()?;
            Some(Entry {
                feature: feature.id,
                route: nav.route.clone(),
                label: nav.label,
                icon: nav.icon,
                scope: nav.scope,
                requires: feature.requires,
                cluster_kind: feature.cluster_kind,
            })
        })
        .collect()
}

/// The entries the sidebar shows for a selection of `kind`: the app's
/// always, and those of the selected kind. With nothing selected, the Host
/// pages show, each explaining how to add one.
pub fn visible(
    entries: &[Entry],
    kind: Option<Scope>,
    supported: impl Fn(Capability) -> bool,
    cluster_kind: Option<ClusterKind>,
) -> Vec<Entry> {
    let selected = kind.unwrap_or(Scope::Host);
    entries
        .iter()
        .filter(|entry| entry.scope == Scope::App || entry.scope == selected)
        .filter(|entry| entry.scope != Scope::Host || entry.requires.is_none_or(&supported))
        .filter(|entry| {
            let other_kind = matches!(
                (entry.cluster_kind, cluster_kind),
                (Some(needed), Some(selected)) if needed != selected
            );
            entry.scope != Scope::Cluster || !other_kind
        })
        .cloned()
        .collect()
}

/// How many entries have a key of their own.
pub const NAV_SHORTCUTS: usize = 9;

/// Mod+1…9 open the sidebar's entries in order: each of the first nine
/// with its keys.
pub fn shortcuts(entries: &[Entry]) -> Vec<(&Entry, String)> {
    entries
        .iter()
        .take(NAV_SHORTCUTS)
        .enumerate()
        .map(|(index, entry)| (entry, format!("Mod+{}", index + 1)))
        .collect()
}
