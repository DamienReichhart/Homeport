//! The features of a Host whose Engine is Podman, containerd or Incus.

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

pub static PODMAN_PODS: Feature = Feature {
    id: FeatureId::PodmanPods,
    routes: &["podman-pods", "podman-pods/:id"],
    nav: nav(
        Route::PodmanPods,
        || t!("podmanpods.nav"),
        Icon::Group,
        Scope::Host,
    ),
    requires: Some(Capability::Podman),
    cluster_kind: None,
    gate: Gate::Host,
    commands: &[Command::new(
        "podman-pods:new",
        || t!("podmanpods.new"),
        Icon::Plus,
        &["podman", "pod", "create"],
        Run::Open(Route::PodmanPods, Some("new")),
    )
    .needs_host()],
    shortcuts: &[
        PageShortcut::search(|route| matches!(route, Route::PodmanPods)),
        PageShortcut::action(
            |route| matches!(route, Route::PodmanPods),
            || t!("podmanpods.new"),
            Run::Request("new"),
        ),
    ],
};

pub static QUADLETS: Feature = Feature {
    id: FeatureId::Quadlets,
    routes: &["quadlets", "quadlets/:name"],
    nav: nav(
        Route::Quadlets,
        || t!("quadlets.nav"),
        Icon::ScrollText,
        Scope::Host,
    ),
    requires: Some(Capability::Podman),
    cluster_kind: None,
    gate: Gate::Host,
    commands: &[Command::new(
        "quadlets:new",
        || t!("quadlets.new"),
        Icon::Plus,
        &["podman", "quadlet", "systemd", "unit", "install"],
        Run::Open(Route::Quadlets, Some("new")),
    )
    .needs_host()],
    shortcuts: &[
        PageShortcut::search(|route| matches!(route, Route::Quadlets)),
        PageShortcut::action(
            |route| matches!(route, Route::Quadlets),
            || t!("quadlets.new"),
            Run::Request("new"),
        ),
    ],
};

pub static CONTAINERD_CONTAINERS: Feature = Feature {
    id: FeatureId::ContainerdContainers,
    routes: &["containerd/containers", "containerd/containers/:id/:tab?"],
    nav: nav(
        Route::ContainerdContainers,
        || t!("containerd.containers.nav"),
        Icon::Box,
        Scope::Host,
    ),
    requires: Some(Capability::Containerd),
    cluster_kind: None,
    gate: Gate::Containerd,
    commands: &[],
    shortcuts: &[PageShortcut::search(|route| {
        matches!(route, Route::ContainerdContainers)
    })],
};

pub static CONTAINERD_IMAGES: Feature = Feature {
    id: FeatureId::ContainerdImages,
    routes: &["containerd/images"],
    nav: nav(
        Route::ContainerdImages,
        || t!("containerd.images.nav"),
        Icon::Layers,
        Scope::Host,
    ),
    requires: Some(Capability::Containerd),
    cluster_kind: None,
    gate: Gate::Containerd,
    commands: &[Command::new(
        "containerd-images:pull",
        || t!("containerd.images.pull"),
        Icon::Download,
        &["containerd", "image", "pull"],
        Run::Open(Route::ContainerdImages, Some("pull")),
    )
    .needs_host()],
    shortcuts: &[
        PageShortcut::search(|route| matches!(route, Route::ContainerdImages)),
        PageShortcut::action(
            |route| matches!(route, Route::ContainerdImages),
            || t!("containerd.images.pull"),
            Run::Request("pull"),
        ),
    ],
};

pub static CONTAINERD_SNAPSHOTS: Feature = Feature {
    id: FeatureId::ContainerdSnapshots,
    routes: &["containerd/snapshots"],
    nav: nav(
        Route::ContainerdSnapshots,
        || t!("containerd.snapshots.nav"),
        Icon::HardDrive,
        Scope::Host,
    ),
    requires: Some(Capability::Containerd),
    cluster_kind: None,
    gate: Gate::Containerd,
    commands: &[],
    shortcuts: &[],
};

/// Behind the Host's gate, not containerd's: it lists every namespace.
pub static CONTAINERD_NAMESPACES: Feature = Feature {
    id: FeatureId::ContainerdNamespaces,
    routes: &["containerd/namespaces"],
    nav: nav(
        Route::ContainerdNamespaces,
        || t!("containerd.namespaces.nav"),
        Icon::FolderTree,
        Scope::Host,
    ),
    requires: Some(Capability::Containerd),
    cluster_kind: None,
    gate: Gate::Host,
    commands: &[Command::new(
        "containerd-namespaces:new",
        || t!("containerd.namespaces.new"),
        Icon::Plus,
        &["containerd", "namespace", "create"],
        Run::Open(Route::ContainerdNamespaces, Some("new")),
    )
    .needs_host()],
    shortcuts: &[PageShortcut::action(
        |route| matches!(route, Route::ContainerdNamespaces),
        || t!("containerd.namespaces.new"),
        Run::Request("new"),
    )],
};

pub static INCUS_INSTANCES: Feature = Feature {
    id: FeatureId::IncusInstances,
    routes: &["incus/instances", "incus/instances/:name/:tab?"],
    nav: nav(
        Route::IncusInstances,
        || t!("incus.instances.nav"),
        Icon::Server,
        Scope::Host,
    ),
    requires: Some(Capability::Incus),
    cluster_kind: None,
    gate: Gate::Incus,
    commands: &[Command::new(
        "incus-instances:new",
        || t!("incus.new.action"),
        Icon::Plus,
        &[
            "incus",
            "instance",
            "container",
            "virtual machine",
            "launch",
            "create",
        ],
        Run::Open(Route::IncusInstances, Some("new")),
    )
    .needs_host()],
    shortcuts: &[
        PageShortcut::search(|route| matches!(route, Route::IncusInstances)),
        PageShortcut::action(
            |route| matches!(route, Route::IncusInstances),
            || t!("incus.new.action"),
            Run::Request("new"),
        ),
    ],
};

pub static INCUS_IMAGES: Feature = Feature {
    id: FeatureId::IncusImages,
    routes: &["incus/images"],
    nav: nav(
        Route::IncusImages,
        || t!("incus.images.nav"),
        Icon::Disc,
        Scope::Host,
    ),
    requires: Some(Capability::Incus),
    cluster_kind: None,
    gate: Gate::Incus,
    commands: &[Command::new(
        "incus-images:get",
        || t!("incus.images.get"),
        Icon::Download,
        &["incus", "image", "download", "copy"],
        Run::Open(Route::IncusImages, Some("get")),
    )
    .needs_host()],
    shortcuts: &[
        PageShortcut::search(|route| matches!(route, Route::IncusImages)),
        PageShortcut::action(
            |route| matches!(route, Route::IncusImages),
            || t!("incus.images.get"),
            Run::Request("get"),
        ),
    ],
};

pub static INCUS_PROFILES: Feature = Feature {
    id: FeatureId::IncusProfiles,
    routes: &["incus/profiles", "incus/profiles/:name"],
    nav: nav(
        Route::IncusProfiles,
        || t!("incus.profiles.nav"),
        Icon::SlidersHorizontal,
        Scope::Host,
    ),
    requires: Some(Capability::Incus),
    cluster_kind: None,
    gate: Gate::Incus,
    commands: &[],
    shortcuts: &[
        PageShortcut::search(|route| matches!(route, Route::IncusProfiles)),
        PageShortcut::action(
            |route| matches!(route, Route::IncusProfiles),
            || t!("incus.profiles.new"),
            Run::Request("new"),
        ),
    ],
};

pub static INCUS_STORAGE: Feature = Feature {
    id: FeatureId::IncusStorage,
    routes: &["incus/storage/:pool?"],
    nav: nav(
        Route::IncusStorage { pool: None },
        || t!("incus.storage.nav"),
        Icon::HardDrive,
        Scope::Host,
    ),
    requires: Some(Capability::Incus),
    cluster_kind: None,
    gate: Gate::Incus,
    commands: &[],
    shortcuts: &[],
};

pub static INCUS_NETWORKS: Feature = Feature {
    id: FeatureId::IncusNetworks,
    routes: &["incus/networks", "incus/networks/:name"],
    nav: nav(
        Route::IncusNetworks,
        || t!("incus.networks.nav"),
        Icon::Network,
        Scope::Host,
    ),
    requires: Some(Capability::Incus),
    cluster_kind: None,
    gate: Gate::Incus,
    commands: &[],
    shortcuts: &[
        PageShortcut::search(|route| matches!(route, Route::IncusNetworks)),
        PageShortcut::action(
            |route| matches!(route, Route::IncusNetworks),
            || t!("incus.networks.new"),
            Run::Request("new"),
        ),
    ],
};
