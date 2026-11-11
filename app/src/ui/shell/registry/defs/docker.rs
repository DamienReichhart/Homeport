//! The features of a Host whose Engine speaks the Docker Engine API, and
//! of the Swarms such Hosts belong to.

use super::nav;
use crate::{
    t,
    ui::{
        shell::{
            registry::{Capability, Command, Feature, FeatureId, Gate, PageShortcut, Run, Scope},
            router::Route,
        },
        theme::Icon,
    },
};

pub static CONTAINERS: Feature = Feature {
    id: FeatureId::Containers,
    routes: &["containers", "containers/:id/:tab?"],
    nav: nav(
        Route::Containers,
        || t!("containers.nav"),
        Icon::Box,
        Scope::Host,
    ),
    requires: Some(Capability::Docker),
    cluster_kind: None,
    gate: Gate::Host,
    commands: &[Command::new(
        "containers:run",
        || t!("containers.palette.run"),
        Icon::Play,
        &["new", "start", "create"],
        Run::Open(Route::Containers, Some("run")),
    )
    .needs_host()],
    shortcuts: &[
        PageShortcut::search(|route| matches!(route, Route::Containers)),
        PageShortcut::action(
            |route| matches!(route, Route::Containers),
            || t!("containers.runContainer"),
            Run::Request("run"),
        ),
    ],
};

pub static IMAGES: Feature = Feature {
    id: FeatureId::Images,
    routes: &["images"],
    nav: nav(
        Route::Images,
        || t!("images.nav"),
        Icon::Layers,
        Scope::Host,
    ),
    requires: Some(Capability::Docker),
    cluster_kind: None,
    gate: Gate::Host,
    commands: &[Command::new(
        "images:pull",
        || t!("images.palette.pull"),
        Icon::Download,
        &["download", "registry"],
        Run::Open(Route::Images, Some("pull")),
    )
    .needs_host()],
    shortcuts: &[
        PageShortcut::search(|route| matches!(route, Route::Images)),
        PageShortcut::action(
            |route| matches!(route, Route::Images),
            || t!("images.pull"),
            Run::Request("pull"),
        ),
    ],
};

pub static VOLUMES: Feature = Feature {
    id: FeatureId::Volumes,
    routes: &["volumes"],
    nav: nav(
        Route::Volumes,
        || t!("volumes.nav"),
        Icon::HardDrive,
        Scope::Host,
    ),
    requires: Some(Capability::Docker),
    cluster_kind: None,
    gate: Gate::Host,
    commands: &[],
    shortcuts: &[
        PageShortcut::search(|route| matches!(route, Route::Volumes)),
        PageShortcut::action(
            |route| matches!(route, Route::Volumes),
            || t!("volumes.create"),
            Run::Request("create"),
        ),
    ],
};

pub static NETWORKS: Feature = Feature {
    id: FeatureId::Networks,
    routes: &["networks"],
    nav: nav(
        Route::Networks,
        || t!("networks.nav"),
        Icon::Network,
        Scope::Host,
    ),
    requires: Some(Capability::Docker),
    cluster_kind: None,
    gate: Gate::Host,
    commands: &[],
    shortcuts: &[
        PageShortcut::search(|route| matches!(route, Route::Networks)),
        PageShortcut::action(
            |route| matches!(route, Route::Networks),
            || t!("networks.create"),
            Run::Request("create"),
        ),
    ],
};

pub static STACKS: Feature = Feature {
    id: FeatureId::Stacks,
    routes: &["stacks", "stacks/new", "stacks/:name"],
    nav: nav(Route::Stacks, || t!("stacks.nav"), Icon::Boxes, Scope::Host),
    requires: Some(Capability::Docker),
    cluster_kind: None,
    gate: Gate::Host,
    commands: &[Command::new(
        "stacks:new",
        || t!("stacks.palette.new"),
        Icon::Plus,
        &["compose", "create", "deploy"],
        Run::Open(Route::NewStack, None),
    )
    .needs_host()],
    shortcuts: &[
        PageShortcut::search(|route| matches!(route, Route::Stacks)),
        PageShortcut::action(
            |route| matches!(route, Route::Stacks),
            || t!("stacks.new"),
            Run::Open(Route::NewStack, None),
        ),
    ],
};

/// No sidebar entry of its own: the sidebar lists each Swarm.
pub static SWARM: Feature = Feature {
    id: FeatureId::Swarm,
    routes: &[
        "swarms/:swarmId/:tab?",
        "swarms/:swarmId/services/:serviceId/:tab?",
        "swarms/:swarmId/stacks/:name",
    ],
    nav: None,
    requires: None,
    cluster_kind: None,
    gate: Gate::Swarm,
    commands: &[],
    shortcuts: &[PageShortcut::search(|route| {
        matches!(route, Route::Swarm { .. })
    })],
};

pub static EVENTS: Feature = Feature {
    id: FeatureId::Events,
    routes: &["events"],
    nav: nav(
        Route::Events,
        || t!("events.nav"),
        Icon::Activity,
        Scope::Host,
    ),
    requires: Some(Capability::Docker),
    cluster_kind: None,
    gate: Gate::Host,
    commands: &[],
    shortcuts: &[PageShortcut::search(|route| matches!(route, Route::Events))],
};
