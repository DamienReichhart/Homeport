//! The feature registry: what `src/app/router/features.ts` and each
//! feature's `manifest.ts` were. [`FEATURES`] lists every feature in
//! sidebar order; each says, as data, which routes are its own, its sidebar
//! entry and what it is about, what it needs of the selected Host or
//! Cluster, its palette commands and its page shortcuts.
//!
//! The shell reads nothing else to build the sidebar, the palette, the
//! shortcuts and the gates, so a feature plugs in by its line here and its
//! page in `ui/features/<group>/<name>/`.

mod defs;
mod list;
pub mod nav;
#[cfg(test)]
mod tests;

use homeport_clusters::model::ClusterKind;

use crate::ui::{shell::router::Route, theme::Icon};

pub use self::list::FEATURES;

/// A feature of the app, named as its folder under `src/features`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FeatureId {
    Overview,
    Board,
    Containers,
    Images,
    Volumes,
    Networks,
    Stacks,
    PodmanPods,
    Quadlets,
    ContainerdContainers,
    ContainerdImages,
    ContainerdSnapshots,
    ContainerdNamespaces,
    IncusInstances,
    IncusImages,
    IncusProfiles,
    IncusStorage,
    IncusNetworks,
    Swarm,
    Workloads,
    Pods,
    Network,
    Config,
    Nodes,
    ClusterEvents,
    ApiResources,
    Rbac,
    Apply,
    Helm,
    PortForwards,
    NomadJobs,
    NomadNodes,
    Events,
    Settings,
    Hosts,
    Clusters,
    Updates,
}

impl FeatureId {
    /// Its folder under `src/features`: `podman-pods`.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Overview => "overview",
            Self::Board => "board",
            Self::Containers => "containers",
            Self::Images => "images",
            Self::Volumes => "volumes",
            Self::Networks => "networks",
            Self::Stacks => "stacks",
            Self::PodmanPods => "podman-pods",
            Self::Quadlets => "quadlets",
            Self::ContainerdContainers => "containerd-containers",
            Self::ContainerdImages => "containerd-images",
            Self::ContainerdSnapshots => "containerd-snapshots",
            Self::ContainerdNamespaces => "containerd-namespaces",
            Self::IncusInstances => "incus-instances",
            Self::IncusImages => "incus-images",
            Self::IncusProfiles => "incus-profiles",
            Self::IncusStorage => "incus-storage",
            Self::IncusNetworks => "incus-networks",
            Self::Swarm => "swarm",
            Self::Workloads => "workloads",
            Self::Pods => "pods",
            Self::Network => "network",
            Self::Config => "config",
            Self::Nodes => "nodes",
            Self::ClusterEvents => "cluster-events",
            Self::ApiResources => "api-resources",
            Self::Rbac => "rbac",
            Self::Apply => "apply",
            Self::Helm => "helm",
            Self::PortForwards => "port-forwards",
            Self::NomadJobs => "nomad-jobs",
            Self::NomadNodes => "nomad-nodes",
            Self::Events => "events",
            Self::Settings => "settings",
            Self::Hosts => "hosts",
            Self::Clusters => "clusters",
            Self::Updates => "updates",
        }
    }
}

/// What a feature's pages are about: the selected Host, the selected
/// Cluster, or the app as a whole (shown whatever is selected).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    Host,
    Cluster,
    App,
}

/// What a Host page needs from the selected Host's Engine: the Docker
/// Engine API (Docker, or Podman through its compatible API), Podman's
/// own, containerd's or Incus's.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Capability {
    Docker,
    Podman,
    Containerd,
    Incus,
}

/// A message of the catalogue, read when it is drawn so it is in the
/// language of the moment. Always a `|| t!("…")`: the catalogue check
/// then holds every label to a message that exists.
pub type Label = fn() -> String;

/// A sidebar entry.
#[derive(Debug, Clone)]
pub struct Nav {
    /// The page it opens.
    pub route: Route,
    pub label: Label,
    pub icon: Icon,
    pub scope: Scope,
}

/// What stands before a feature's page until the selection can show it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gate {
    /// The page shows whatever is selected.
    Open,
    /// The Overview: the Host's gate, or the Cluster's while one is selected.
    Selection,
    /// `HostGate`: the selected Host, connected.
    Host,
    /// `ContainerdGate`: that, and the namespace its pages show.
    Containerd,
    /// `IncusGate`: that, and the project its pages show.
    Incus,
    /// `ClusterGate`: the selected Cluster, connected.
    Cluster,
    /// `SwarmGate`: the Swarm of the route, with a manager that answers.
    Swarm,
}

/// What running a command or a shortcut does.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Run {
    /// Opens a page; with an action, the page starts it as if its button
    /// was pressed (`ctx.open(path, action)`).
    Open(Route, Option<&'static str>),
    /// Asks the page that shows to start one of its actions.
    Request(&'static str),
    /// Sets the theme in the Settings.
    Theme(homeport_settings::Theme),
    /// Opens the Settings and looks for a new version now.
    CheckForUpdates,
    /// Opens the keyboard shortcuts' sheet.
    ShortcutsHelp,
}

/// The palette's heading a command is listed under.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Heading {
    Actions,
    Settings,
}

/// An action a feature offers in the command palette.
#[derive(Debug, Clone)]
pub struct Command {
    /// Unique across features: `containers:run`.
    pub id: &'static str,
    pub label: Label,
    pub heading: Heading,
    pub icon: Icon,
    /// More words it is found by, in English.
    pub keywords: &'static [&'static str],
    /// Its keyboard shortcut, shown next to it.
    pub shortcut: Option<&'static str>,
    /// Hidden unless the selected Host is connected.
    pub needs_host: bool,
    pub run: Run,
}

impl Command {
    pub const fn new(
        id: &'static str,
        label: Label,
        icon: Icon,
        keywords: &'static [&'static str],
        run: Run,
    ) -> Self {
        Self {
            id,
            label,
            heading: Heading::Actions,
            icon,
            keywords,
            shortcut: None,
            needs_host: false,
            run,
        }
    }

    #[must_use]
    pub const fn needs_host(mut self) -> Self {
        self.needs_host = true;
        self
    }

    #[must_use]
    pub const fn under(mut self, heading: Heading) -> Self {
        self.heading = heading;
        self
    }

    #[must_use]
    pub const fn shortcut(mut self, keys: &'static str) -> Self {
        self.shortcut = Some(keys);
        self
    }
}

/// The key that starts a page's main action (`PAGE_ACTION_KEYS`).
pub const PAGE_ACTION: &str = "n";
/// The key that goes to a page's search field.
pub const PAGE_SEARCH: &str = "/";
/// What a page is asked for when its search key is pressed.
pub const SEARCH: &str = "search";

/// A shortcut of one page, listed under "This page" while it shows.
#[derive(Debug, Clone)]
pub struct PageShortcut {
    /// The pages it works on.
    pub on: fn(&Route) -> bool,
    pub keys: &'static str,
    pub label: Label,
    pub run: Run,
}

impl PageShortcut {
    /// `usePageAction`: `n` starts the page's main action.
    pub const fn action(on: fn(&Route) -> bool, label: Label, run: Run) -> Self {
        Self {
            on,
            keys: PAGE_ACTION,
            label,
            run,
        }
    }

    /// `SearchInput`: `/` goes to the page's search field.
    pub const fn search(on: fn(&Route) -> bool) -> Self {
        Self {
            on,
            keys: PAGE_SEARCH,
            label: || crate::t!("ui.shortcuts.focusSearch"),
            run: Run::Request(SEARCH),
        }
    }
}

/// What a feature plugs into the shell (`FeatureManifest`).
#[derive(Debug)]
pub struct Feature {
    pub id: FeatureId,
    /// Its routes, as its manifest wrote them: `containers/:id/:tab?`.
    /// [`Route`] has a variant for each.
    pub routes: &'static [&'static str],
    /// Its sidebar entry, if it has a top-level one.
    pub nav: Option<Nav>,
    /// What the selected Host's Engine must offer for its pages: the
    /// sidebar hides their entry otherwise, and opening one explains why.
    pub requires: Option<Capability>,
    /// The kind of Cluster its pages are for.
    pub cluster_kind: Option<ClusterKind>,
    pub gate: Gate,
    pub commands: &'static [Command],
    pub shortcuts: &'static [PageShortcut],
}

impl Feature {
    /// The scope of its pages: its sidebar entry's, else the app's.
    pub fn scope(&self) -> Scope {
        self.nav.as_ref().map_or(Scope::App, |nav| nav.scope)
    }
}

/// The feature named `id`.
pub fn feature(id: FeatureId) -> &'static Feature {
    FEATURES
        .iter()
        .copied()
        .find(|feature| feature.id == id)
        .unwrap_or(&defs::app::OVERVIEW)
}
