//! The registry against the React app: the entries the sidebar shows for a
//! selection (`manifest.test.ts`), the whole list pinned, and every
//! `manifest.ts` still in the tree read and compared.

use std::{fs, path::Path};

use homeport_clusters::model::ClusterKind;

use super::{
    nav::{entries_of, shortcuts, visible, Entry},
    Capability, Feature, FeatureId, Gate, Run, Scope, FEATURES,
};
use crate::{
    i18n,
    test_support::snapshot::in_language,
    ui::{shell::router::Route, theme::Icon},
};

fn entry(path: &str, scope: Scope) -> Entry {
    Entry {
        feature: FeatureId::Overview,
        route: Route::parse(path).unwrap(),
        label: || crate::t!("overview.nav"),
        icon: Icon::Box,
        scope,
        requires: None,
        cluster_kind: None,
    }
}

fn entries() -> Vec<Entry> {
    vec![
        entry("/", Scope::App),
        entry("/containers", Scope::Host),
        entry("/pods", Scope::Cluster),
    ]
}

fn paths(entries: &[Entry]) -> Vec<String> {
    entries.iter().map(|entry| entry.route.path()).collect()
}

#[test]
fn the_sidebar_shows_the_app_s_entries_and_those_of_the_selected_kind() {
    let all = entries();
    let shown = |kind| paths(&visible(&all, kind, |_| true, None));
    assert_eq!(shown(Some(Scope::Host)), ["/", "/containers"]);
    assert_eq!(shown(Some(Scope::Cluster)), ["/", "/pods"]);
    // With nothing selected the Host entries show, as before Clusters.
    assert_eq!(shown(None), ["/", "/containers"]);
}

#[test]
fn an_entry_hides_when_the_selected_host_does_not_offer_what_it_requires() {
    let podman = Entry {
        requires: Some(Capability::Podman),
        ..entry("/podman-pods", Scope::Host)
    };
    let all = [entries(), vec![podman]].concat();
    let docker = |capability| capability == Capability::Docker;
    assert_eq!(
        paths(&visible(&all, Some(Scope::Host), docker, None)),
        ["/", "/containers"]
    );
    assert!(paths(&visible(&all, Some(Scope::Host), |_| true, None))
        .contains(&"/podman-pods".to_owned()));
    // The other scopes are left alone.
    assert_eq!(
        paths(&visible(&all, Some(Scope::Cluster), |_| false, None)),
        ["/", "/pods"]
    );
}

#[test]
fn only_the_pages_of_the_selected_cluster_s_kind_show() {
    let of = |path, kind| Entry {
        cluster_kind: Some(kind),
        ..entry(path, Scope::Cluster)
    };
    let all = [
        entry("/", Scope::App),
        of("/pods", ClusterKind::Kubernetes),
        of("/nomad/jobs", ClusterKind::Nomad),
    ];
    let shown = |kind| paths(&visible(&all, Some(Scope::Cluster), |_| true, Some(kind)));
    assert_eq!(shown(ClusterKind::Kubernetes), ["/", "/pods"]);
    assert_eq!(shown(ClusterKind::Nomad), ["/", "/nomad/jobs"]);
}

#[test]
fn an_entry_carries_what_its_feature_requires_and_its_scope_is_the_feature_s() {
    let entries = entries_of(FEATURES);
    let of = |id| entries.iter().find(|entry| entry.feature == id).unwrap();
    assert_eq!(of(FeatureId::PodmanPods).requires, Some(Capability::Podman));
    assert_eq!(of(FeatureId::Overview).requires, None);
    assert_eq!(
        of(FeatureId::NomadJobs).cluster_kind,
        Some(ClusterKind::Nomad)
    );
    assert_eq!(super::feature(FeatureId::Containers).scope(), Scope::Host);
    // Without a sidebar entry, a feature is about the app.
    assert_eq!(super::feature(FeatureId::Hosts).scope(), Scope::App);
}

#[test]
fn the_first_nine_entries_have_mod_and_their_number() {
    let all = entries_of(FEATURES);
    let host = visible(&all, Some(Scope::Host), |c| c == Capability::Docker, None);
    let keys = shortcuts(&host);
    assert_eq!(keys.len(), 9);
    assert_eq!(keys[0].1, "Mod+1");
    assert_eq!(keys[0].0.feature, FeatureId::Overview);
    assert_eq!(keys[3].0.feature, FeatureId::Images, "Mod+4 opens Images");
    assert_eq!(keys[8].1, "Mod+9");
}

/// One line per feature: its sidebar path, label, icon and scope, what it
/// requires, its gate and its commands.
fn line(feature: &Feature) -> String {
    let nav = feature.nav.as_ref().map_or_else(
        || "-".to_owned(),
        |nav| {
            format!(
                "{} \"{}\" {} {:?}",
                nav.route.path(),
                (nav.label)(),
                nav.icon.name(),
                nav.scope
            )
        },
    );
    let requires = match (feature.requires, feature.cluster_kind) {
        (Some(capability), _) => format!("{capability:?}"),
        (None, Some(kind)) => format!("{kind:?}"),
        (None, None) => "-".to_owned(),
    };
    let commands: Vec<&str> = feature.commands.iter().map(|c| c.id).collect();
    format!(
        "{} | {nav} | {requires} | {:?} | {}",
        feature.id.name(),
        feature.gate,
        commands.join(",")
    )
}

const PINNED: &str = r#"
overview | / "Overview" layout-dashboard App | - | Selection | overview:prune
board | /board "Board" table App | - | Open |
containers | /containers "Containers" box Host | Docker | Host | containers:run
images | /images "Images" layers Host | Docker | Host | images:pull
volumes | /volumes "Volumes" hard-drive Host | Docker | Host |
networks | /networks "Networks" network Host | Docker | Host |
stacks | /stacks "Stacks" boxes Host | Docker | Host | stacks:new
podman-pods | /podman-pods "Pods" group Host | Podman | Host | podman-pods:new
quadlets | /quadlets "Quadlet" scroll-text Host | Podman | Host | quadlets:new
containerd-containers | /containerd/containers "Containers" box Host | Containerd | Containerd |
containerd-images | /containerd/images "Images" layers Host | Containerd | Containerd | containerd-images:pull
containerd-snapshots | /containerd/snapshots "Snapshots" hard-drive Host | Containerd | Containerd |
containerd-namespaces | /containerd/namespaces "Namespaces" folder-tree Host | Containerd | Host | containerd-namespaces:new
incus-instances | /incus/instances "Instances" server Host | Incus | Incus | incus-instances:new
incus-images | /incus/images "Images" disc Host | Incus | Incus | incus-images:get
incus-profiles | /incus/profiles "Profiles" sliders-horizontal Host | Incus | Incus |
incus-storage | /incus/storage "Storage" hard-drive Host | Incus | Incus |
incus-networks | /incus/networks "Networks" network Host | Incus | Incus |
swarm | - | - | Swarm |
workloads | /workloads "Workloads" layers Cluster | Kubernetes | Cluster |
pods | /pods "Pods" container Cluster | Kubernetes | Cluster |
network | /services "Network" waypoints Cluster | Kubernetes | Cluster |
config | /configmaps "Configuration" file-cog Cluster | Kubernetes | Cluster |
nodes | /nodes "Nodes" server Cluster | Kubernetes | Cluster |
cluster-events | /cluster-events "Events" activity Cluster | Kubernetes | Cluster |
api-resources | /api-resources "API resources" blocks Cluster | Kubernetes | Cluster |
rbac | /rbac "RBAC" shield-check Cluster | Kubernetes | Cluster |
apply | /apply "Apply YAML" file-code-corner Cluster | Kubernetes | Cluster |
helm | /helm "Helm" ship-wheel Cluster | Kubernetes | Cluster |
port-forwards | /port-forwards "Port forwards" arrow-right-left Cluster | Kubernetes | Open |
nomad-jobs | /nomad/jobs "Jobs" briefcase Cluster | Nomad | Cluster |
nomad-nodes | /nomad/nodes "Nodes" server Cluster | Nomad | Cluster |
events | /events "Events" activity Host | Docker | Host |
settings | /settings "Settings" settings App | - | Open | settings:theme:light,settings:theme:dark,settings:theme:system
hosts | - | - | Open | hosts:add,hosts:manage
clusters | - | - | Open | clusters:add,clusters:manage
updates | - | - | Open | updates:check
"#;

#[test]
fn the_list_is_the_react_app_s_in_sidebar_order() {
    let _english = in_language("en");
    let lines: Vec<String> = FEATURES
        .iter()
        .map(|f| line(f).trim_end().to_owned())
        .collect();
    let pinned: Vec<&str> = PINNED.trim().lines().map(str::trim_end).collect();
    assert_eq!(lines, pinned);
}

/// A path for `pattern` with a value for each parameter; an optional one
/// is given only when `optional` says so.
fn sample(pattern: &str, optional: bool) -> String {
    let segments: Vec<&str> = pattern
        .split('/')
        .filter(|segment| !segment.is_empty())
        .filter(|segment| optional || !segment.ends_with('?'))
        .map(|segment| {
            if segment.starts_with(':') {
                "x1"
            } else {
                segment
            }
        })
        .collect();
    format!("/{}", segments.join("/"))
}

#[test]
fn every_route_of_a_feature_is_a_route_of_the_enum_and_its_own() {
    let mut ids = std::collections::HashSet::new();
    for feature in FEATURES {
        assert!(
            ids.insert(feature.id),
            "{} is listed twice",
            feature.id.name()
        );
        for pattern in feature.routes {
            for optional in [false, true] {
                let path = sample(pattern, optional);
                let route =
                    Route::parse(&path).unwrap_or_else(|| panic!("{path} ({pattern}) is no route"));
                assert_eq!(route.feature(), feature.id, "{path}");
                assert_eq!(route.path(), path);
            }
        }
        if let Some(nav) = &feature.nav {
            assert_eq!(nav.route.feature(), feature.id);
        }
        for command in feature.commands {
            assert!(
                !(command.label)().contains('{'),
                "{} takes arguments",
                command.id
            );
            if command.needs_host {
                assert_ne!(feature.scope(), Scope::Cluster, "{}", command.id);
            }
        }
        for shortcut in feature.shortcuts {
            assert!(!(shortcut.label)().is_empty());
        }
        assert_eq!(super::feature(feature.id).id, feature.id);
    }
    assert_eq!(FEATURES.len(), 37);
}

// What follows reads the React manifests while they are still in the tree.

fn manifests() -> Option<std::path::PathBuf> {
    let features = Path::new(env!("CARGO_MANIFEST_DIR")).join("../src/features");
    features.is_dir().then_some(features)
}

/// The text between the quotes after each `marker` of `text`.
fn quoted<'a>(text: &'a str, marker: &str) -> Vec<&'a str> {
    text.match_indices(marker)
        .filter_map(|(at, _)| {
            let rest = text[at + marker.len()..].trim_start();
            rest.strip_prefix('\'')?.split('\'').next()
        })
        .collect()
}

/// The word after each `marker` of `text`: an icon's name, `true`.
fn word<'a>(text: &'a str, marker: &str) -> Option<&'a str> {
    let at = text.find(marker)?;
    let rest = text[at + marker.len()..].trim_start();
    let end = rest
        .find(|c: char| !c.is_alphanumeric())
        .unwrap_or(rest.len());
    Some(&rest[..end])
}

/// Lucide's file for the name React imported: `LayoutDashboardIcon`.
fn lucide(import: &str) -> String {
    let name = import.trim_end_matches("Icon");
    let mut file = String::new();
    for (at, c) in name.char_indices() {
        if at > 0 && (c.is_uppercase() || c.is_ascii_digit()) {
            file.push('-');
        }
        file.push(c.to_ascii_lowercase());
    }
    match file.as_str() {
        "file-code-2" => "file-code-corner".to_owned(),
        _ => file,
    }
}

fn english(key: &str) -> String {
    i18n::current().text(key, None)
}

fn check_commands(feature: &Feature, text: &str, problems: &mut Vec<String>) {
    let name = feature.id.name();
    let mut found = vec![];
    let starts: Vec<usize> = text.match_indices("id: '").map(|(at, _)| at).collect();
    for (index, at) in starts.iter().enumerate() {
        let end = starts.get(index + 1).copied().unwrap_or(text.len());
        found.push(text[*at..end].to_owned());
    }
    // The Settings' themes are built by a helper: `theme('light', key, Icon)`.
    for (at, _) in text.match_indices("\n    theme('") {
        let call = &text[at..];
        let parts = quoted(call.lines().nth(1).unwrap_or_default(), "theme(");
        let value = parts.first().copied().unwrap_or_default();
        let line = call.lines().nth(1).unwrap_or_default();
        let label = line.split('\'').nth(3).unwrap_or_default();
        let icon = line
            .rsplit(", ")
            .next()
            .unwrap_or_default()
            .trim_end_matches("),");
        found.push(format!(
            "id: 'settings:theme:{value}', label: '{label}', icon: {icon}, \
             keywords: ['theme', 'appearance', '{value}'], run: theme"
        ));
    }
    if found.len() != feature.commands.len() {
        problems.push(format!(
            "{name}: {} commands in manifest.ts, {} here",
            found.len(),
            feature.commands.len()
        ));
        return;
    }
    for (block, command) in found.iter().zip(feature.commands) {
        let id = quoted(block, "id:")[0];
        let say = |what: &str| format!("{name} {id}: {what}");
        if id != command.id {
            problems.push(say(&format!("is {} here", command.id)));
        }
        if english(quoted(block, "label:")[0]) != (command.label)() {
            problems.push(say("another label"));
        }
        if word(block, "icon:").map(lucide).as_deref() != Some(command.icon.name()) {
            problems.push(say("another icon"));
        }
        let list = block
            .split("keywords: [")
            .nth(1)
            .and_then(|rest| rest.split(']').next())
            .map(|list| format!(", {list}"))
            .unwrap_or_default();
        let keywords = quoted(&list, ",");
        if keywords != command.keywords {
            problems.push(say(&format!("keywords {keywords:?}")));
        }
        if block.contains("needsHost: true") != command.needs_host {
            problems.push(say("needsHost differs"));
        }
        let opened = quoted(block, "ctx.open(");
        let action = block
            .split("ctx.open(")
            .nth(1)
            .and_then(|rest| rest.split(')').next())
            .and_then(|args| args.split(", '").nth(1))
            .map(|action| action.trim_end_matches('\''));
        let same = match &command.run {
            Run::Open(route, wanted) => {
                opened.first() == Some(&route.path().as_str()) && action == *wanted
            }
            Run::Theme(_) => block.ends_with("run: theme"),
            Run::CheckForUpdates => block.contains("checkForUpdatesNow"),
            Run::Request(_) | Run::ShortcutsHelp => false,
        };
        if !same {
            problems.push(say("runs something else"));
        }
    }
}

#[test]
fn every_manifest_of_the_react_app_is_here_as_it_was_written() {
    let Some(folder) = manifests() else {
        // The React app is gone: the pinned list above is what remains.
        return;
    };
    let _english = in_language("en");
    let mut problems = vec![];
    let mut seen = 0;
    for entry in fs::read_dir(&folder).unwrap() {
        let path = entry.unwrap().path();
        let Ok(text) = fs::read_to_string(path.join("manifest.ts")) else {
            continue;
        };
        seen += 1;
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        let Some(feature) = FEATURES.iter().find(|f| f.id.name() == name) else {
            problems.push(format!("{name} has a manifest and no line in the registry"));
            continue;
        };
        let mut routes = quoted(&text, "{ path:");
        routes.extend(quoted(&text, "\n      path:"));
        // A sidebar entry's path starts at the root; a route's doesn't.
        routes.retain(|route| !route.starts_with('/'));
        if text.contains("index: true") {
            routes.insert(0, "");
        }
        let mut own: Vec<&str> = feature.routes.to_vec();
        routes.sort_unstable();
        own.sort_unstable();
        if routes != own {
            problems.push(format!("{name}: routes {routes:?}, here {own:?}"));
        }
        let nav = text
            .split("nav: {")
            .nth(1)
            .and_then(|rest| rest.split('}').next());
        match (nav, &feature.nav) {
            (None, None) => {}
            (Some(nav), Some(own)) => {
                let scope = quoted(nav, "scope:")[0];
                let same = quoted(nav, "path:")[0] == own.route.path()
                    && english(quoted(nav, "label:")[0]) == (own.label)()
                    && word(nav, "icon:").map(lucide).as_deref() == Some(own.icon.name())
                    && scope == format!("{:?}", own.scope).to_lowercase();
                if !same {
                    problems.push(format!("{name}: its sidebar entry differs: {nav}"));
                }
            }
            _ => problems.push(format!("{name}: a sidebar entry on one side only")),
        }
        let requires = quoted(&text, "requires:").first().map(|r| (*r).to_owned());
        if requires != feature.requires.map(|r| format!("{r:?}").to_lowercase()) {
            problems.push(format!("{name}: requires {requires:?}"));
        }
        let kind = quoted(&text, "clusterKind:")
            .first()
            .map(|k| (*k).to_owned());
        if kind != feature.cluster_kind.map(|k| k.as_str().to_owned()) {
            problems.push(format!("{name}: clusterKind {kind:?}"));
        }
        check_commands(feature, &text, &mut problems);
    }
    assert_eq!(seen, FEATURES.len(), "manifests read");
    // The order is `features.ts`'s.
    let order = fs::read_to_string(folder.join("../app/router/features.ts")).unwrap();
    let listed: Vec<String> = order
        .split("FeatureManifest[] = [")
        .nth(1)
        .and_then(|rest| rest.split(']').next())
        .unwrap_or_default()
        .split(',')
        .map(|name| name.trim().trim_end_matches("Feature").to_lowercase())
        .filter(|name| !name.is_empty())
        .collect();
    let own: Vec<String> = FEATURES
        .iter()
        .map(|f| f.id.name().replace('-', ""))
        .collect();
    assert_eq!(listed, own, "the order of features.ts");
    assert!(problems.is_empty(), "{problems:#?}");
}

#[test]
fn a_gate_stands_before_the_pages_that_had_one_in_react() {
    let gate = |id| super::feature(id).gate;
    assert_eq!(gate(FeatureId::Overview), Gate::Selection);
    assert_eq!(gate(FeatureId::Board), Gate::Open);
    assert_eq!(gate(FeatureId::ContainerdNamespaces), Gate::Host);
    assert_eq!(gate(FeatureId::ContainerdImages), Gate::Containerd);
    assert_eq!(gate(FeatureId::IncusStorage), Gate::Incus);
    assert_eq!(gate(FeatureId::Swarm), Gate::Swarm);
    assert_eq!(gate(FeatureId::PortForwards), Gate::Open);
}
