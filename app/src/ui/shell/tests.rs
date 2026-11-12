//! The shell as a whole: what its messages change, what it draws for each
//! selection and gate, and that the keyboard alone drives it.

use iced::{
    keyboard::{self, key::Named, Key, Location, Modifiers},
    Task,
};

use homeport_clusters::{registry::ClusterStatus, service::ClusterView};
use homeport_core::{
    error::{AppError, ErrorCode, Message as Text},
    host::EngineKind,
};
use homeport_hosts::{
    registry::{HostStatus, SwarmView},
    service::HostView,
};
use homeport_settings::Settings;

use super::{
    gates::view::Action as GateAction,
    page::{stub, testing::TestApp, Context, Pages, Update},
    palette,
    registry::FeatureId,
    router::{Request, Route},
    selection::{self, Selection},
    shortcuts::keys::Press,
    sidebar, Event, Message, Shell,
};
use crate::{
    api::app::Capabilities,
    data::{keys, resources, Data},
    test_support::{
        fixtures::{cluster_view, connected, connected_host, host_view, nomad_cluster_view},
        snapshot::{assert_snapshot, in_language},
        stage::{Frame, Stage},
        tasks,
    },
    ui::theme::{Element, Theme},
};

/// Every page as the stub the features re-export.
#[derive(Debug)]
struct Blank {
    state: stub::State,
    asked: Vec<String>,
}

impl Pages for Blank {
    type Message = stub::Message;

    fn open(ctx: &Context<'_>) -> (Self, Task<stub::Message>) {
        let (state, task) = stub::open(ctx);
        let asked = vec![];
        (Self { state, asked }, task)
    }

    fn update(&mut self, message: stub::Message, ctx: &Context<'_>) -> Task<stub::Message> {
        stub::update(&mut self.state, message, ctx)
    }

    fn view<'a>(&'a self, ctx: &Context<'a>) -> Element<'a, stub::Message> {
        stub::view(&self.state, ctx)
    }

    fn request(&mut self, action: &str, ctx: &Context<'_>) -> Task<stub::Message> {
        self.asked.push(action.to_owned());
        stub::request(&mut self.state, action, ctx)
    }
}

type Msg = Message<stub::Message>;

struct Rig {
    app: TestApp,
    data: Data,
    shell: Shell<Blank>,
}

const SIZE: (f32, f32) = (1100.0, 720.0);

fn refused() -> AppError {
    AppError::new(
        ErrorCode::ConnectionRefused,
        Text {
            key: "errors.test.refused",
            en: "connection refused",
        },
    )
    .with_detail("dial tcp 10.0.0.4:2376: connect: connection refused")
}

fn second_host() -> HostView {
    host_view("prod", connected())
}

impl Rig {
    fn new(hosts: Vec<HostView>, clusters: Vec<ClusterView>, settings: Settings) -> Self {
        let app = TestApp::start();
        let mut data = Data::default();
        data.set(&keys::host::list(), hosts);
        data.set(&keys::cluster::list(), clusters);
        data.set(&keys::swarm::list(), Vec::<SwarmView>::new());
        let mut shell = Shell::new(app.state.clone(), settings);
        shell.mac = false;
        let mut rig = Self { app, data, shell };
        rig.send(Message::Changed);
        rig
    }

    fn with_hosts(hosts: Vec<HostView>) -> Self {
        Self::new(hosts, vec![], Settings::default())
    }

    fn with_cluster(cluster: ClusterView) -> Self {
        Self::new(vec![], vec![cluster], Settings::default())
    }

    /// Sends a message; what it started is dropped, not run.
    fn send(&mut self, message: Msg) {
        drop(self.shell.update(message, &mut self.data));
    }

    /// Sends a message and runs what it started, feeding the answers back.
    fn run(&mut self, message: Msg) {
        let task = self.shell.update(message, &mut self.data);
        for answer in self.app.runtime.block_on(tasks::messages(task)) {
            self.run(answer);
        }
    }

    fn go(&mut self, route: Route) {
        self.send(Message::Go(route.into()));
    }

    fn route(&self) -> &Route {
        &self.shell.location().route
    }

    fn view(&self) -> Element<'_, Msg> {
        self.shell.view(&self.data)
    }

    fn snapshot(&self, name: &str, language: &str, theme: Theme) {
        assert_snapshot(name, language, &theme, SIZE, || self.view());
    }

    /// Draws the shell on `stage`, acts on it, and applies what that sent.
    fn frame(&mut self, stage: &mut Stage, act: impl FnOnce(&mut Frame<'_, '_, Msg>)) {
        let messages = stage.frame(self.shell.view(&self.data), act);
        for message in messages {
            self.send(message);
        }
    }
}

fn pressed(key: Key, modifiers: Modifiers) -> [iced::Event; 2] {
    let event = |pressed: bool| {
        iced::Event::Keyboard(if pressed {
            keyboard::Event::KeyPressed {
                key: key.clone(),
                modified_key: key.clone(),
                physical_key: keyboard::key::Physical::Unidentified(
                    keyboard::key::NativeCode::Unidentified,
                ),
                location: Location::Standard,
                modifiers,
                text: None,
                repeat: false,
            }
        } else {
            keyboard::Event::KeyReleased {
                key: key.clone(),
                modified_key: key.clone(),
                physical_key: keyboard::key::Physical::Unidentified(
                    keyboard::key::NativeCode::Unidentified,
                ),
                location: Location::Standard,
                modifiers,
            }
        })
    };
    [event(true), event(false)]
}

fn ctrl(key: &str) -> [iced::Event; 2] {
    pressed(Key::Character(key.into()), Modifiers::CTRL)
}

fn key(press: Press) -> Msg {
    Message::Key {
        press,
        captured: false,
    }
}

fn docker_and_prod() -> Rig {
    Rig::with_hosts(vec![connected_host(), second_host()])
}

// What its messages change.

#[test]
fn it_opens_on_the_first_connected_host_and_its_pages() {
    let rig = Rig::with_hosts(vec![
        host_view("down", HostStatus::Disabled),
        connected_host(),
    ]);
    assert_eq!(rig.shell.world().host_id(), Some("h1"));
    assert_eq!(rig.route(), &Route::Overview);
    let shown: Vec<FeatureId> = rig.shell.entries.iter().map(|e| e.feature).collect();
    assert!(shown.contains(&FeatureId::Containers));
    assert!(!shown.contains(&FeatureId::Pods) && !shown.contains(&FeatureId::Quadlets));
    assert!(rig.shell.page.is_some(), "the Overview is open");
}

#[test]
fn navigating_opens_the_page_and_back_and_forward_move() {
    let mut rig = docker_and_prod();
    rig.go(Route::Containers);
    assert_eq!(rig.route(), &Route::Containers);
    let first = rig.shell.page.as_ref().unwrap().key.clone();
    rig.go(Route::Images);
    assert_ne!(rig.shell.page.as_ref().unwrap().key, first, "another page");
    rig.send(Message::Back);
    assert_eq!(rig.route(), &Route::Containers);
    rig.send(Message::Forward);
    assert_eq!(rig.route(), &Route::Images);
    // The page that shows stays as it is when asked for again.
    let images = rig.shell.page.as_ref().unwrap().key.clone();
    rig.go(Route::Images);
    assert_eq!(rig.shell.page.as_ref().unwrap().key, images);
    assert_eq!(rig.shell.history.count(), 3);
}

#[test]
fn a_navigation_s_action_is_started_once_on_the_page_it_opens() {
    let mut rig = docker_and_prod();
    rig.send(Message::Go(Request::to(Route::Containers).starting("run")));
    let asked = |rig: &Rig| rig.shell.page.as_ref().unwrap().page.asked.clone();
    assert_eq!(asked(&rig), ["run"]);
    // Asked again while the page shows: it starts again, on the same page.
    rig.send(Message::Go(Request::to(Route::Containers).starting("run")));
    assert_eq!(asked(&rig), ["run", "run"]);
    // `n` asks the page for its main action, `/` for its search.
    rig.send(key(Press::of("n")));
    rig.send(key(Press::of("/")));
    assert_eq!(asked(&rig), ["run", "run", "run", "search"]);
}

#[test]
fn selecting_a_cluster_sends_a_host_page_to_the_overview_and_lets_go_of_the_host() {
    let mut rig = Rig::new(
        vec![connected_host()],
        vec![cluster_view("kind-dev")],
        Settings::default(),
    );
    rig.go(Route::Containers);
    // What a Host page would hold of its Host.
    let before = rig.data.in_flight();
    drop(rig.data.watch(&rig.app.state, &resources::containers("h1")));
    assert_eq!(rig.data.in_flight(), before + 1);
    rig.send(Message::Select(Selection::Cluster {
        id: "c1".to_owned(),
    }));
    assert_eq!(rig.route(), &Route::Overview);
    assert_eq!(rig.shell.world().cluster_id(), Some("c1"));
    // The Host's read is let go of; the Cluster's namespaces are read.
    assert_eq!(rig.data.in_flight(), before + 1);
    assert!(rig
        .data
        .read::<Vec<()>>(&keys::host::containers("h1"))
        .is_pending());
    let shown: Vec<FeatureId> = rig.shell.entries.iter().map(|e| e.feature).collect();
    assert!(shown.contains(&FeatureId::Pods) && !shown.contains(&FeatureId::Containers));
    // A page about the app stays where it is.
    rig.go(Route::Settings);
    rig.send(Message::Select(Selection::Host {
        id: "h1".to_owned(),
    }));
    assert_eq!(rig.route(), &Route::Settings);
}

#[test]
fn the_selection_is_remembered_across_a_restart() {
    let mut rig = Rig::new(
        vec![connected_host(), second_host()],
        vec![cluster_view("kind-dev")],
        Settings::default(),
    );
    rig.run(Message::Select(Selection::Host {
        id: "prod-id".to_owned(),
    }));
    assert!(
        rig.shell.take_events().is_empty(),
        "saved without a failure"
    );
    let stored = rig.app.state.settings.get().unwrap();
    assert_eq!(stored.last_host_id.as_deref(), Some("prod-id"));
    // A new launch reads the settings and opens on the same Host.
    let mut again: Shell<Blank> = Shell::new(rig.app.state.clone(), stored);
    drop(again.update(Message::Changed, &mut rig.data));
    assert_eq!(again.world().host_id(), Some("prod-id"));
    rig.run(Message::Select(Selection::Cluster {
        id: "c1".to_owned(),
    }));
    let stored = rig.app.state.settings.get().unwrap();
    assert_eq!(
        selection::remembered(&stored),
        Some(Selection::Cluster {
            id: "c1".to_owned()
        })
    );
    assert_eq!(
        stored.last_host_id.as_deref(),
        Some("prod-id"),
        "kept for older versions"
    );
}

#[test]
fn a_remembered_host_that_is_gone_falls_back_and_one_still_loading_is_kept() {
    let settings = Settings {
        last_host_id: Some("gone".to_owned()),
        ..Settings::default()
    };
    let rig = Rig::new(vec![connected_host()], vec![], settings.clone());
    assert_eq!(rig.shell.world().host_id(), Some("h1"));
    let app = TestApp::start();
    let mut shell: Shell<Blank> = Shell::new(app.state.clone(), settings);
    drop(shell.update(Message::Changed, &mut Data::default()));
    assert_eq!(
        shell.world().host_id(),
        Some("gone"),
        "the Hosts are still read"
    );
}

#[test]
fn alt_and_the_arrows_step_through_the_hosts_round_the_ends() {
    let mut rig = docker_and_prod();
    let alt = |key: &str| Press {
        alt: true,
        ..Press::of(key)
    };
    rig.send(key(alt("ArrowDown")));
    assert_eq!(rig.shell.world().host_id(), Some("prod-id"));
    rig.send(key(alt("ArrowDown")));
    assert_eq!(rig.shell.world().host_id(), Some("h1"));
    rig.send(key(alt("ArrowUp")));
    assert_eq!(rig.shell.world().host_id(), Some("prod-id"));
}

#[test]
fn the_palette_opens_runs_a_row_and_closes() {
    let _english = in_language("en");
    let mut rig = docker_and_prod();
    let ctrl_k = Press {
        ctrl: true,
        ..Press::of("k")
    };
    rig.send(key(ctrl_k.clone()));
    assert!(rig.shell.palette.is_open());
    rig.send(Message::Palette(palette::Message::Query("prod".to_owned())));
    rig.send(Message::Palette(palette::Message::Submit));
    assert!(!rig.shell.palette.is_open());
    assert_eq!(rig.shell.world().host_id(), Some("prod-id"));
    // A command opens its page and starts its action there.
    rig.send(key(ctrl_k.clone()));
    rig.send(Message::Palette(palette::Message::Query(
        "pull an".to_owned(),
    )));
    rig.send(Message::Palette(palette::Message::Submit));
    assert_eq!(rig.route(), &Route::Images);
    assert_eq!(rig.shell.page.as_ref().unwrap().page.asked, ["pull"]);
    // The theme is the window's to change; Mod+K closes the palette too.
    rig.send(key(ctrl_k.clone()));
    rig.send(Message::Palette(palette::Message::Query("dark".to_owned())));
    rig.send(Message::Palette(palette::Message::Submit));
    assert!(matches!(
        rig.shell.take_events().as_slice(),
        [Event::Theme(homeport_settings::Theme::Dark)]
    ));
    rig.send(key(ctrl_k.clone()));
    rig.send(Message::Key {
        press: ctrl_k,
        captured: true,
    });
    assert!(!rig.shell.palette.is_open());
}

#[test]
fn the_shortcuts_sheet_opens_with_a_question_mark_and_from_the_palette() {
    let _english = in_language("en");
    let mut rig = docker_and_prod();
    rig.send(key(Press::of("?")));
    assert!(rig.shell.help);
    rig.send(Message::CloseHelp);
    assert!(!rig.shell.help);
    // A key a field used types; it opens nothing.
    rig.send(Message::Key {
        press: Press::of("?"),
        captured: true,
    });
    assert!(!rig.shell.help);
    rig.send(Message::Palette(palette::Message::Toggle));
    rig.send(Message::Palette(palette::Message::Query(
        "keyboard".to_owned(),
    )));
    rig.send(Message::Palette(palette::Message::Submit));
    assert!(rig.shell.help && !rig.shell.palette.is_open());
}

#[test]
fn a_gate_s_buttons_go_where_they_say_and_show_the_failure_s_details() {
    let down = HostStatus::Unreachable {
        error: refused(),
        retry_in_secs: 8,
    };
    let mut rig = Rig::with_hosts(vec![host_view("prod", down)]);
    rig.go(Route::Containers);
    assert!(rig.shell.page.is_none(), "the gate stands before the page");
    rig.send(Message::Gate(GateAction::Details));
    rig.send(Message::Gate(GateAction::Copy("ssh prod".to_owned())));
    assert!(matches!(
        rig.shell.take_events().as_slice(),
        [Event::Details(_), Event::Copy(_)]
    ));
    rig.send(Message::Gate(GateAction::Retry));
    assert!(rig.shell.reconnect.is_pending());
    rig.send(Message::Gate(GateAction::Go(Route::Host {
        host_id: "prod-id".to_owned(),
    })));
    assert!(matches!(rig.route(), Route::Host { .. }));
    // The switcher says why the Host can't be used.
    rig.send(Message::Sidebar(sidebar::Message::WhyUnusable));
    assert!(matches!(
        rig.shell.take_events().as_slice(),
        [Event::Details(_)]
    ));
}

#[test]
fn a_page_opens_when_its_host_connects() {
    let mut rig = Rig::with_hosts(vec![host_view("prod", HostStatus::Connecting)]);
    rig.go(Route::Containers);
    assert!(rig.shell.page.is_none());
    rig.data
        .set(&keys::host::list(), vec![host_view("prod", connected())]);
    rig.send(Message::Changed);
    assert!(rig.shell.page.is_some());
}

#[test]
fn a_picked_namespace_shows_at_once_and_is_remembered_for_its_cluster() {
    use crate::ui::widgets::form::select;
    let mut rig = Rig::with_cluster(cluster_view("kind-dev"));
    rig.go(Route::Pods);
    let namespaces = ["default", "kube-system", "shop"].map(|name| {
        homeport_kubernetes::resources::kinds::pods::NamespaceView {
            name: name.to_owned(),
            phase: "Active".to_owned(),
            created: None,
            labels: std::collections::BTreeMap::new(),
        }
    });
    let key = resources::cluster_namespaces("c1").key().clone();
    rig.data.set(&key, namespaces.to_vec());
    rig.send(Message::Changed);
    let subject = |rig: &Rig| rig.shell.page.as_ref().unwrap().key.1.clone();
    let before = subject(&rig);
    // The page hands the picker's messages back to the shell.
    let pick = |index| {
        Message::Page(stub::Message::Picker(super::pickers::Message::Namespace(
            select::Message::Choose(index),
        )))
    };
    rig.run(pick(3));
    assert_ne!(
        subject(&rig),
        before,
        "the page is opened on the new Namespace"
    );
    let stored = rig.app.state.settings.get().unwrap();
    assert_eq!(stored.cluster_namespaces["c1"], "shop");
    // The first line is every Namespace, stored as "".
    rig.run(pick(0));
    assert_eq!(
        rig.app.state.settings.get().unwrap().cluster_namespaces["c1"],
        ""
    );
}

// What it draws.

#[test]
fn the_shell_with_a_host_selected() {
    let mut rig = docker_and_prod();
    rig.go(Route::Containers);
    rig.snapshot("shell-host-en", "en", Theme::Light);
    rig.snapshot("shell-host-dark-en", "en", Theme::Dark);
    rig.snapshot("shell-host-ar", "ar", Theme::Light);
}

#[test]
fn the_shell_with_a_cluster_selected() {
    let mut rig = Rig::with_cluster(cluster_view("kind-dev"));
    rig.go(Route::Pods);
    rig.snapshot("shell-cluster-en", "en", Theme::Light);
}

#[test]
fn the_shell_with_nothing_selected_welcomes() {
    let rig = Rig::with_hosts(vec![]);
    rig.snapshot("shell-none-en", "en", Theme::Light);
}

#[test]
fn the_bars_over_the_page_and_the_swarms_of_the_sidebar() {
    let mut rig = docker_and_prod();
    let swarm = SwarmView {
        id: "swarm123456789abc".to_owned(),
        managers: vec!["h1".to_owned()],
        workers: vec![],
        reachable: true,
    };
    rig.data.set(&keys::swarm::list(), vec![swarm]);
    let app = rig
        .app
        .runtime
        .block_on(crate::api::app::get_capabilities(&rig.app.state, false));
    let capabilities = Capabilities {
        keychain: false,
        keychain_entry_missing: false,
        ..app
    };
    rig.data.set(&keys::app::capabilities(), capabilities);
    rig.send(Message::UpdateFound(Some(Update {
        version: "2.1.0".to_owned(),
        via_package_manager: false,
    })));
    rig.snapshot("shell-banners-en", "en", Theme::Light);
    rig.send(Message::DismissUpdate);
    assert!(rig.shell.update_notice().is_none());
}

#[test]
fn the_host_s_gate_for_a_host_that_is_down() {
    let down = HostStatus::Unreachable {
        error: refused(),
        retry_in_secs: 8,
    };
    let mut rig = Rig::with_hosts(vec![host_view("prod", down)]);
    rig.go(Route::Containers);
    rig.snapshot("shell-gate-host-down-en", "en", Theme::Light);
    let mut rig = Rig::with_hosts(vec![host_view("prod", HostStatus::CredentialsLost)]);
    rig.go(Route::Images);
    rig.snapshot("shell-gate-host-credentials-en", "en", Theme::Light);
}

#[test]
fn the_guard_of_an_engine_that_doesn_t_offer_the_page() {
    let mut rig = docker_and_prod();
    rig.go(Route::IncusInstances);
    rig.snapshot("shell-gate-unsupported-en", "en", Theme::Light);
}

#[test]
fn the_containerd_and_incus_gates_open_on_their_namespace_and_project() {
    let of = |engine| {
        let mut host = connected_host();
        host.host.engine = engine;
        Rig::with_hosts(vec![host])
    };
    let mut rig = of(EngineKind::Containerd);
    let namespace = |name: &str, containers| homeport_containerd::namespaces::ContainerdNamespace {
        name: name.to_owned(),
        labels: std::collections::HashMap::new(),
        containers,
        images: 30,
    };
    rig.data.set(
        &keys::host::containerd_namespaces("h1"),
        vec![namespace("default", 0), namespace("k8s.io", 12)],
    );
    rig.go(Route::ContainerdContainers);
    assert!(rig.shell.page.is_some());
    rig.snapshot("shell-gate-containerd-en", "en", Theme::Light);
    let mut rig = of(EngineKind::Incus);
    rig.go(Route::IncusInstances);
    assert!(rig.shell.page.is_none(), "it waits for the projects");
    rig.snapshot("shell-gate-incus-en", "en", Theme::Light);
}

#[test]
fn the_cluster_s_gate_and_the_guard_of_its_kind() {
    let mut down = cluster_view("kind-dev");
    down.status = ClusterStatus::Unauthorized { error: refused() };
    let mut rig = Rig::with_cluster(down);
    rig.go(Route::Pods);
    rig.snapshot("shell-gate-cluster-down-en", "en", Theme::Light);
    let mut rig = Rig::with_cluster(nomad_cluster_view("lab"));
    rig.go(Route::Pods);
    rig.snapshot("shell-gate-wrong-kind-en", "en", Theme::Light);
}

#[test]
fn the_swarm_s_gate_for_a_swarm_that_is_gone() {
    let mut rig = docker_and_prod();
    rig.go(Route::Swarm {
        swarm_id: "gone".to_owned(),
        tab: None,
    });
    rig.snapshot("shell-gate-swarm-en", "en", Theme::Light);
}

#[test]
fn the_palette_open_with_results() {
    let mut rig = docker_and_prod();
    rig.go(Route::Containers);
    rig.send(Message::Palette(palette::Message::Toggle));
    rig.send(Message::Palette(palette::Message::Query("co".to_owned())));
    rig.send(Message::Palette(palette::Message::Step(1)));
    rig.snapshot("shell-palette-en", "en", Theme::Light);
    rig.snapshot("shell-palette-ar", "ar", Theme::Light);
}

#[test]
fn the_shortcuts_sheet() {
    let mut rig = docker_and_prod();
    rig.go(Route::Containers);
    rig.send(key(Press::of("?")));
    rig.snapshot("shell-shortcuts-en", "en", Theme::Light);
}

// With the keyboard alone.

#[test]
fn mod_k_opens_the_palette_typing_finds_and_enter_goes_there() {
    let _english = in_language("en");
    let mut rig = docker_and_prod();
    let mut stage = Stage::new(SIZE);
    rig.frame(&mut stage, |frame| frame.send(ctrl("k")));
    assert!(rig.shell.palette.is_open());
    // The field has focus: what is typed is searched, one key at a time.
    let mut typed = String::new();
    for letter in ["v", "o", "l"] {
        typed.push_str(letter);
        rig.frame(&mut stage, |frame| frame.typewrite(letter));
        assert_eq!(rig.shell.palette.query(), typed);
    }
    rig.frame(&mut stage, |frame| frame.tap_key(Named::Enter));
    assert_eq!(rig.route(), &Route::Volumes);
    assert!(!rig.shell.palette.is_open());
    // The arrows move the highlight: the second row is the Board.
    rig.frame(&mut stage, |frame| frame.send(ctrl("k")));
    for arrow in [
        Named::ArrowDown,
        Named::ArrowDown,
        Named::ArrowDown,
        Named::ArrowUp,
    ] {
        rig.frame(&mut stage, |frame| frame.tap_key(arrow));
    }
    assert_eq!(rig.shell.palette.active(), 2, "under Recent's one row");
    rig.frame(&mut stage, |frame| frame.tap_key(Named::Enter));
    assert_eq!(rig.route(), &Route::Board);
    // Escape closes it and runs nothing.
    rig.frame(&mut stage, |frame| frame.send(ctrl("k")));
    rig.frame(&mut stage, |frame| frame.tap_key(Named::Escape));
    assert!(!rig.shell.palette.is_open());
    assert_eq!(rig.route(), &Route::Board);
}

#[test]
fn a_question_mark_opens_the_sheet_and_escape_gives_focus_back() {
    let _english = in_language("en");
    let mut rig = docker_and_prod();
    let mut stage = Stage::new(SIZE);
    let mut before = None;
    rig.frame(&mut stage, |frame| {
        frame.tab();
        frame.tab();
        before = frame.focused();
    });
    assert!(before.is_some(), "Tab reached something in the sidebar");
    rig.frame(&mut stage, |frame| frame.typewrite("?"));
    assert!(rig.shell.help);
    rig.frame(&mut stage, |frame| {
        assert!(frame.find("Keyboard shortcuts").is_some());
        frame.tap_key(Named::Escape);
    });
    assert!(!rig.shell.help);
    rig.frame(&mut stage, |frame| assert_eq!(frame.focused(), before));
}

#[test]
fn mod_and_a_number_opens_that_entry_of_the_sidebar() {
    let _english = in_language("en");
    let mut rig = docker_and_prod();
    let mut stage = Stage::new(SIZE);
    rig.frame(&mut stage, |frame| frame.send(ctrl("2")));
    assert_eq!(rig.route(), &Route::Board);
    rig.frame(&mut stage, |frame| frame.send(ctrl("4")));
    assert_eq!(rig.route(), &Route::Images);
    rig.frame(&mut stage, |frame| frame.send(ctrl(",")));
    assert_eq!(rig.route(), &Route::Settings);
    // Alt and the down arrow select the second Host.
    rig.frame(&mut stage, |frame| {
        frame.send(pressed(Key::Named(Named::ArrowDown), Modifiers::ALT));
    });
    assert_eq!(rig.shell.world().host_id(), Some("prod-id"));
}

#[test]
fn the_sidebar_is_on_the_right_in_arabic() {
    let _arabic = in_language("ar");
    let rig = docker_and_prod();
    let mut stage = Stage::new(SIZE);
    let mut logo = None;
    stage.frame(rig.view(), |frame| logo = frame.find("Homeport"));
    let logo = logo.expect("the mark's name");
    assert!(logo.x > SIZE.0 - sidebar::WIDTH, "it starts on the right");
}
