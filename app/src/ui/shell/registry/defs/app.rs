//! The features about the app as a whole: shown whatever is selected.

use homeport_settings::Theme;

use super::nav;
use crate::{
    t,
    ui::{
        shell::{
            registry::{Command, Feature, FeatureId, Gate, Heading, PageShortcut, Run, Scope},
            router::Route,
        },
        theme::Icon,
    },
};

pub static OVERVIEW: Feature = Feature {
    id: FeatureId::Overview,
    routes: &[""],
    nav: nav(
        Route::Overview,
        || t!("overview.nav"),
        Icon::LayoutDashboard,
        Scope::App,
    ),
    requires: None,
    cluster_kind: None,
    gate: Gate::Selection,
    commands: &[Command::new(
        "overview:prune",
        || t!("overview.palette.prune"),
        Icon::BrushCleaning,
        &["clean", "disk", "space", "remove unused"],
        Run::Open(Route::Overview, Some("prune")),
    )
    .needs_host()],
    shortcuts: &[],
};

/// Not behind a gate: it shows every Host at once.
pub static BOARD: Feature = Feature {
    id: FeatureId::Board,
    routes: &["board"],
    nav: nav(Route::Board, || t!("board.nav"), Icon::Table, Scope::App),
    requires: None,
    cluster_kind: None,
    gate: Gate::Open,
    commands: &[],
    shortcuts: &[PageShortcut::search(|route| matches!(route, Route::Board))],
};

const fn theme(id: &'static str, value: Theme, label: fn() -> String, icon: Icon) -> Command {
    let keywords: &[&str] = match value {
        Theme::Light => &["theme", "appearance", "light"],
        Theme::Dark => &["theme", "appearance", "dark"],
        Theme::System => &["theme", "appearance", "system"],
    };
    Command::new(id, label, icon, keywords, Run::Theme(value)).under(Heading::Settings)
}

pub static SETTINGS: Feature = Feature {
    id: FeatureId::Settings,
    routes: &["settings"],
    nav: nav(
        Route::Settings,
        || t!("settings.nav"),
        Icon::Settings,
        Scope::App,
    ),
    requires: None,
    cluster_kind: None,
    gate: Gate::Open,
    commands: &[
        theme(
            "settings:theme:light",
            Theme::Light,
            || t!("settings.palette.themeLight"),
            Icon::Sun,
        ),
        theme(
            "settings:theme:dark",
            Theme::Dark,
            || t!("settings.palette.themeDark"),
            Icon::Moon,
        ),
        theme(
            "settings:theme:system",
            Theme::System,
            || t!("settings.palette.themeSystem"),
            Icon::Monitor,
        ),
    ],
    shortcuts: &[],
};

/// No sidebar entry: the Host switcher leads here.
pub static HOSTS: Feature = Feature {
    id: FeatureId::Hosts,
    routes: &["hosts", "hosts/new", "hosts/:hostId"],
    nav: None,
    requires: None,
    cluster_kind: None,
    gate: Gate::Open,
    commands: &[
        Command::new(
            "hosts:add",
            || t!("hosts.palette.add"),
            Icon::Plus,
            &["new", "connect", "ssh", "engine"],
            Run::Open(Route::AddHost, None),
        ),
        Command::new(
            "hosts:manage",
            || t!("hosts.palette.manage"),
            Icon::ServerCog,
            &["hosts", "export", "import"],
            Run::Open(Route::Hosts, None),
        ),
    ],
    shortcuts: &[PageShortcut::action(
        |route| matches!(route, Route::Hosts),
        || t!("hosts.add"),
        Run::Open(Route::AddHost, None),
    )],
};

/// No sidebar entry: the switcher leads here, as for Hosts.
pub static CLUSTERS: Feature = Feature {
    id: FeatureId::Clusters,
    routes: &["clusters", "clusters/new", "clusters/:clusterId"],
    nav: None,
    requires: None,
    cluster_kind: None,
    gate: Gate::Open,
    commands: &[
        Command::new(
            "clusters:add",
            || t!("clusters.palette.add"),
            Icon::Plus,
            &["kubernetes", "k8s", "kubeconfig", "context", "new"],
            Run::Open(Route::AddCluster, None),
        ),
        Command::new(
            "clusters:manage",
            || t!("clusters.palette.manage"),
            Icon::ShipWheel,
            &["kubernetes", "k8s", "clusters"],
            Run::Open(Route::Clusters, None),
        ),
    ],
    shortcuts: &[],
};

/// No pages: the updates live in Settings and the bar at the top.
pub static UPDATES: Feature = Feature {
    id: FeatureId::Updates,
    routes: &[],
    nav: None,
    requires: None,
    cluster_kind: None,
    gate: Gate::Open,
    commands: &[Command::new(
        "updates:check",
        || t!("updates.palette.check"),
        Icon::RefreshCw,
        &["upgrade", "version", "new version"],
        Run::CheckForUpdates,
    )],
    shortcuts: &[],
};
