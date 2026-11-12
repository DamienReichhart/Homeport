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
    /// How many of its messages reached it.
    heard: usize,
    /// How many times it was told that something may have changed.
    refreshed: usize,
}

impl Pages for Blank {
    type Message = stub::Message;

    fn open(ctx: &Context<'_>) -> (Self, Task<stub::Message>) {
        let (state, task) = stub::open(ctx);
        // As a real page: Containers reads its Host's containers, and the
        // Board, which shows whatever is selected, the selected Host's.
        let reads = matches!(ctx.location.route, Route::Containers | Route::Board);
        if let (true, Some(host)) = (reads, ctx.world.host_id()) {
            ctx.watch(resources::containers(host));
        }
        let page = Self {
            state,
            asked: vec![],
            heard: 0,
            refreshed: 0,
        };
        (page, task)
    }

    fn update(&mut self, message: stub::Message, ctx: &Context<'_>) -> Task<stub::Message> {
        self.heard += 1;
        stub::update(&mut self.state, message, ctx)
    }

    fn view<'a>(&'a self, ctx: &Context<'a>) -> Element<'a, stub::Message> {
        stub::view(&self.state, ctx)
    }

    fn request(&mut self, action: &str, ctx: &Context<'_>) -> Task<stub::Message> {
        self.asked.push(action.to_owned());
        // What a page asks of the shell through its context.
        match action {
            "say" => ctx.done(|| "done".to_owned()),
            "blame" => ctx.failed_of(
                || "web".to_owned(),
                crate::ui::widgets::feedback::Surfaced::at(
                    refused(),
                    None,
                    jiff::Timestamp::UNIX_EPOCH,
                ),
            ),
            "link" => ctx.open_url("http://localhost:8080"),
            "put" => ctx.put(keys::host::freshness("h1"), vec![7_u8]),
            "keep" => ctx.keep(
                Some("api".to_owned()),
                vec![("host".to_owned(), "h1".to_owned())],
            ),
            _ => {}
        }
        stub::request(&mut self.state, action, ctx)
    }

    fn refresh(&mut self, ctx: &Context<'_>) -> Task<stub::Message> {
        self.refreshed += 1;
        stub::refresh(&mut self.state, ctx)
    }

    fn subscription(&self, ctx: &Context<'_>) -> iced::Subscription<stub::Message> {
        stub::subscription(&self.state, ctx)
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

fn open_page(rig: &Rig) -> &super::Open<Blank> {
    rig.shell.page.as_ref().expect("a page is open")
}

/// A message of the namespace picker, from the page numbered `id`.
fn picked(id: u64, index: usize) -> Msg {
    use crate::ui::widgets::form::select;
    let message = super::pickers::Message::Namespace(select::Message::Choose(index));
    Message::Page(id, stub::Message::Picker(message))
}

#[test]
fn selecting_a_cluster_sends_a_host_page_to_the_overview_and_lets_go_of_the_host() {
    let mut rig = Rig::new(
        vec![connected_host()],
        vec![cluster_view("kind-dev")],
        Settings::default(),
    );
    // Nothing of the Cluster is left to read: only the Host's reads move.
    let namespaces = resources::cluster_namespaces("c1").key().clone();
    rig.data.set(
        &namespaces,
        Vec::<homeport_kubernetes::resources::kinds::pods::NamespaceView>::new(),
    );
    let idle = rig.data.in_flight();
    rig.go(Route::Containers);
    // The page asked for its Host's containers through its context.
    assert_eq!(open_page(&rig).watched, [keys::host::containers("h1")]);
    assert_eq!(rig.data.in_flight(), idle + 1);
    rig.send(Message::Select(Selection::Cluster {
        id: "c1".to_owned(),
    }));
    assert_eq!(rig.route(), &Route::Overview);
    assert_eq!(rig.shell.world().cluster_id(), Some("c1"));
    assert!(open_page(&rig).watched.is_empty());
    assert_eq!(rig.data.in_flight(), idle, "the Host's read is let go of");
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
fn a_page_that_stays_reads_the_new_selection_and_lets_go_of_the_old() {
    let mut rig = docker_and_prod();
    rig.go(Route::Board);
    let first = open_page(&rig).id;
    assert_eq!(open_page(&rig).watched, [keys::host::containers("h1")]);
    let reading = rig.data.in_flight();
    rig.send(Message::Select(Selection::Host {
        id: "prod-id".to_owned(),
    }));
    // The Board is still the page, opened anew on the Host now selected:
    // the first Host's read was released, not left running beside it.
    assert_eq!(rig.route(), &Route::Board);
    assert_ne!(open_page(&rig).id, first);
    assert_eq!(open_page(&rig).watched, [keys::host::containers("prod-id")]);
    assert_eq!(rig.data.in_flight(), reading);
}

#[test]
fn a_message_of_a_page_that_was_left_does_not_reach_the_page_in_its_place() {
    let mut rig = Rig::with_cluster(cluster_view("kind-dev"));
    rig.go(Route::Pods);
    let first = open_page(&rig).id;
    rig.send(picked(first, 99));
    assert_eq!(open_page(&rig).page.heard, 1, "its own message reaches it");
    // Another page of the same feature takes its place.
    rig.go(Route::Pod {
        object: super::router::Namespaced::new("shop", "web"),
        tab: None,
    });
    assert_ne!(open_page(&rig).id, first);
    rig.send(picked(first, 99));
    assert_eq!(
        open_page(&rig).page.heard,
        0,
        "a late answer of the first is dropped"
    );
    // And once a gate closes the page, nothing hears it.
    let mut down = cluster_view("kind-dev");
    down.status = ClusterStatus::Connecting;
    rig.data.set(&keys::cluster::list(), vec![down]);
    let last = open_page(&rig).id;
    rig.send(Message::Changed);
    assert!(rig.shell.page.is_none());
    rig.send(picked(last, 99));
    assert!(rig.shell.page.is_none() && rig.shell.take_events().is_empty());
}

#[test]
fn an_action_asked_of_a_page_behind_its_gate_starts_when_the_gate_opens() {
    let mut rig = Rig::with_hosts(vec![host_view("prod", HostStatus::Connecting)]);
    rig.go(Route::Containers);
    assert!(rig.shell.page.is_none());
    // `n`, or the palette's Run a container, while the Host connects.
    rig.send(Message::Go(Request::to(Route::Containers).starting("run")));
    rig.send(key(Press::of("n")));
    assert!(rig.shell.page.is_none());
    rig.data
        .set(&keys::host::list(), vec![host_view("prod", connected())]);
    rig.send(Message::Changed);
    assert_eq!(open_page(&rig).page.asked, ["run"], "once, when it opens");
    rig.send(Message::Changed);
    assert_eq!(open_page(&rig).page.asked, ["run"]);
}

#[test]
fn what_is_remembered_is_saved_in_the_order_it_was_picked() {
    let mut rig = docker_and_prod();
    let select = |id: &str| Message::Select(Selection::Host { id: id.to_owned() });
    // Two Hosts picked before the first save is written.
    let first = rig.shell.update(select("prod-id"), &mut rig.data);
    let second = rig.shell.update(select("h1"), &mut rig.data);
    // Whichever task the runtime runs first, the last pick is stored last.
    for answer in rig.app.runtime.block_on(tasks::messages(second)) {
        rig.run(answer);
    }
    for answer in rig.app.runtime.block_on(tasks::messages(first)) {
        rig.run(answer);
    }
    let stored = rig.app.state.settings.get().unwrap();
    assert_eq!(stored.last_host_id.as_deref(), Some("h1"));
    assert!(!rig.shell.saving && rig.shell.to_save.is_empty());
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
    let subject = |rig: &Rig| open_page(rig).key.1.clone();
    let before = subject(&rig);
    // The page hands the picker's messages back to the shell.
    let id = open_page(&rig).id;
    rig.run(picked(id, 3));
    assert_ne!(
        subject(&rig),
        before,
        "the page is opened on the new Namespace"
    );
    let stored = rig.app.state.settings.get().unwrap();
    assert_eq!(stored.cluster_namespaces["c1"], "shop");
    // The first line is every Namespace, stored as "".
    let id = open_page(&rig).id;
    rig.run(picked(id, 0));
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

#[test]
fn alt_and_an_arrow_pressed_in_a_field_stay_the_field_s() {
    let _english = in_language("en");
    // A Cluster whose Namespaces can't be listed: its page has a field.
    let mut rig = Rig::new(
        vec![connected_host(), second_host()],
        vec![cluster_view("kind-dev")],
        Settings::default(),
    );
    rig.send(Message::Select(Selection::Cluster {
        id: "c1".to_owned(),
    }));
    let namespaces = resources::cluster_namespaces("c1").key().clone();
    rig.data.fail(&namespaces, &refused());
    rig.go(Route::Pods);
    let alt_down = || pressed(Key::Named(Named::ArrowDown), Modifiers::ALT);
    let mut stage = Stage::new(SIZE);
    rig.frame(&mut stage, |frame| {
        // The field is at the end of the page's header.
        frame.point_at((SIZE.0 - 80.0, 40.0));
        frame.press();
        assert!(frame.focused().is_some(), "the field has focus");
    });
    rig.frame(&mut stage, |frame| frame.typewrite("s"));
    rig.frame(&mut stage, |frame| frame.send(alt_down()));
    assert_eq!(
        rig.shell.world().cluster_id(),
        Some("c1"),
        "no Host was stepped to"
    );
    // Mod+K still opens the palette from the field; outside it, Alt steps.
    rig.frame(&mut stage, |frame| frame.send(ctrl("k")));
    assert!(rig.shell.palette.is_open());
    let mut rig = docker_and_prod();
    let mut stage = Stage::new(SIZE);
    rig.frame(&mut stage, |frame| frame.send(alt_down()));
    assert_eq!(rig.shell.world().host_id(), Some("prod-id"));
}

fn found_here() -> (
    Vec<homeport_hosts::detect::DetectedEngine>,
    homeport_kubernetes::kubeconfig::detect::LocalClusters,
) {
    use homeport_hosts::detect::DetectedEngine;
    use homeport_kubernetes::kubeconfig::detect::{
        LocalCluster, LocalClusters, LocalTool, MissingLocalCluster,
    };
    let engine = |label: &str, socket: &str, problem| DetectedEngine {
        label: label.to_owned(),
        socket_path: socket.to_owned(),
        engine: EngineKind::Docker,
        engine_version: if problem {
            String::new()
        } else {
            "29.1".to_owned()
        },
        problem: problem.then(refused),
    };
    let cluster = |tool, context: &str, version: Option<&str>| LocalCluster {
        tool,
        context: context.to_owned(),
        suggested_name: context.to_owned(),
        server: "https://127.0.0.1:6443".to_owned(),
        server_version: version.map(str::to_owned),
        added: None,
    };
    let engines = vec![
        engine("Docker Engine", "/var/run/docker.sock", false),
        engine("Podman", "/run/user/1000/podman/podman.sock", true),
    ];
    let clusters = LocalClusters {
        found: vec![
            cluster(LocalTool::Kind, "kind-dev", Some("v1.33.1")),
            cluster(LocalTool::Minikube, "minikube", None),
        ],
        missing: vec![MissingLocalCluster {
            tool: LocalTool::K3d,
            name: "lab".to_owned(),
            command: "k3d kubeconfig merge lab --kubeconfig-merge-default".to_owned(),
        }],
    };
    (engines, clusters)
}

fn welcome() -> Rig {
    let mut rig = Rig::with_hosts(vec![]);
    let (engines, clusters) = found_here();
    rig.data.set(&keys::host::local_engines(), engines);
    rig.data.set(&keys::cluster::local_clusters(), clusters);
    rig.send(Message::Changed);
    rig
}

#[test]
fn the_welcome_offers_what_runs_on_this_machine() {
    let rig = welcome();
    rig.snapshot("shell-welcome-local-en", "en", Theme::Light);
}

#[test]
fn the_welcome_looks_for_local_engines_and_clusters_only_while_it_shows() {
    let rig = Rig::with_hosts(vec![]);
    assert!(rig.shell.held.contains(&keys::host::local_engines()));
    assert!(rig.shell.held.contains(&keys::cluster::local_clusters()));
    let rig = docker_and_prod();
    assert!(!rig.shell.held.contains(&keys::host::local_engines()));
}

#[test]
fn an_engine_found_here_is_added_in_one_click_and_selected() {
    use super::gates::welcome;
    let mut rig = welcome();
    assert_eq!(welcome::engines(rig.shell.world()).len(), 2);
    // One that can't be added as it is says why, and adds nothing.
    let podman = "/run/user/1000/podman/podman.sock".to_owned();
    rig.send(Message::Gate(GateAction::AddEngine(podman.clone())));
    assert!(!rig.shell.adding.is_pending());
    rig.send(Message::Gate(GateAction::WhyNot(podman)));
    assert!(matches!(
        rig.shell.take_events().as_slice(),
        [Event::Details(_)]
    ));
    // A cluster that isn't running can't be added either.
    rig.send(Message::Gate(GateAction::AddCluster("minikube".to_owned())));
    assert!(!rig.shell.adding.is_pending());
    assert!(welcome::context_choice(rig.shell.world(), "kind-dev").is_some());
    let docker = "/var/run/docker.sock".to_owned();
    let task = rig.shell.update(
        Message::Gate(GateAction::AddEngine(docker.clone())),
        &mut rig.data,
    );
    assert!(rig.shell.adding.is_pending());
    let added = rig.app.runtime.block_on(tasks::messages(task));
    for message in added {
        rig.send(message);
    }
    let stored = rig.app.state.hosts.store.list().unwrap();
    assert_eq!(stored.len(), 1);
    assert_eq!(stored[0].name, "Docker Engine");
    assert_eq!(
        rig.shell.chosen,
        Some(Selection::Host {
            id: stored[0].id.clone()
        })
    );
    // Once it is a Host, its socket is no longer offered.
    let views = crate::api::hosts::hosts_list(&rig.app.state).unwrap();
    rig.data.set(&keys::host::list(), views);
    let (engines, _) = found_here();
    rig.data.set(&keys::host::local_engines(), engines);
    rig.send(Message::Changed);
    assert!(welcome::host_input(rig.shell.world(), &docker).is_none());
}

#[test]
fn a_page_that_stays_is_told_of_every_change() {
    let mut rig = docker_and_prod();
    rig.go(Route::Containers);
    let opened = open_page(&rig).id;
    let before = open_page(&rig).page.refreshed;
    // A read ended, a status came: the page hears of it once each.
    rig.send(Message::Changed);
    rig.send(Message::Changed);
    assert_eq!(open_page(&rig).id, opened);
    assert_eq!(open_page(&rig).page.refreshed, before + 2);
    // Another page is opened, not refreshed: it reads all there is.
    rig.go(Route::Images);
    assert_eq!(open_page(&rig).page.refreshed, 0);
}

#[test]
fn a_page_says_what_it_did_names_what_failed_and_opens_a_link() {
    let mut rig = docker_and_prod();
    rig.go(Route::Containers);
    for action in ["say", "blame", "link"] {
        rig.send(Message::Go(Request::to(Route::Containers).starting(action)));
    }
    let events = rig.shell.take_events();
    let [Event::Said(kind, said), Event::Failed(Some(title), surfaced), Event::OpenUrl(url)] =
        events.as_slice()
    else {
        panic!("{events:?}");
    };
    assert_eq!(
        (*kind, said.now()),
        (
            crate::ui::widgets::feedback::toast::Kind::Success,
            "done".to_owned()
        )
    );
    assert_eq!(
        (title.now(), &surfaced.error),
        ("web".to_owned(), &refused())
    );
    assert_eq!(url, "http://localhost:8080");
}

#[test]
fn what_a_page_keeps_in_its_location_is_there_when_the_window_comes_back() {
    let mut rig = docker_and_prod();
    rig.go(Route::Board);
    let page = open_page(&rig).id;
    rig.send(Message::Go(Request::to(Route::Board).starting("keep")));
    // The page stays as it is: its location carries more, under the same key.
    assert_eq!(open_page(&rig).id, page);
    rig.go(Route::Containers);
    rig.send(Message::Back);
    let location = rig.shell.location();
    assert_eq!(location.query.as_deref(), Some("api"));
    assert_eq!(location.params_of("host").collect::<Vec<_>>(), ["h1"]);
    assert_eq!(location.param("state"), None);
}

#[test]
fn a_page_files_what_an_action_answered_and_a_request_carries_parameters() {
    let mut rig = docker_and_prod();
    rig.go(Route::Containers);
    rig.send(Message::Go(Request::to(Route::Containers).starting("put")));
    let filed = rig.data.read::<Vec<u8>>(&keys::host::freshness("h1"));
    assert_eq!(filed.data().map(|list| list.as_slice()), Some(&[7_u8][..]));
    // `/board?host=h1`, as the Overview's card of a Host opens the Board.
    rig.send(Message::Go(
        Request::to(Route::Board).with_param("host", "h1"),
    ));
    assert_eq!(rig.shell.location().param("host"), Some("h1"));
    // Asked again with another Host, it is another navigation.
    let key = rig.shell.location().key;
    rig.send(Message::Go(
        Request::to(Route::Board).with_param("host", "prod-id"),
    ));
    assert_ne!(rig.shell.location().key, key);
}
