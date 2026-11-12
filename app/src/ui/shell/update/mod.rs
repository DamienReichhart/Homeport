//! What a message changes in the shell, and how the shell keeps in step
//! with the data layer: the selection, the gate and the page that shows.

mod gate;

use iced::Task;

use homeport_clusters::model::ClusterKind;

use super::{
    gates::{self, Verdict},
    page::{
        context::{Effect, Start},
        Context, Pages,
    },
    palette::{self, items::Act},
    pickers::{self, Picked},
    registry::{self, Capability, Run},
    router::{Request, Route},
    selection::{
        self,
        remember::{Remember, Saved},
        Selection,
    },
    shortcuts::{
        keymap::{self, Command},
        keys::Target,
    },
    sidebar, status, Event, Message, Open, Shell,
};
use crate::{
    api,
    data::{self, keys, resources, Data, Key},
    i18n, t,
    ui::widgets::feedback::Surfaced,
};

/// A read the shell wants kept fresh: its key and how to start it.
type Wanted = (Key, Start);

fn wanted<T: Send + Sync + 'static>(query: data::Query<T>) -> Wanted {
    let key = query.key().clone();
    (key, Box::new(move |data, state| data.watch(state, &query)))
}

/// How many times a change may lead to another before the shell stops
/// following: a page that opens another that opens another.
const ROUNDS: usize = 6;

impl<P: Pages> Shell<P> {
    pub fn update(
        &mut self,
        message: Message<P::Message>,
        data: &mut Data,
    ) -> Task<Message<P::Message>> {
        let mut tasks = vec![self.apply(message, data)];
        // What a page asked for may open another page, which asks in turn.
        for _ in 0..ROUNDS {
            tasks.push(self.run_effects(data));
            tasks.push(self.sync(data));
            if self.effects.is_empty() {
                break;
            }
        }
        Task::batch(tasks)
    }

    /// A page's context and the page itself, to call it with.
    fn with_page<R>(
        &mut self,
        data: &Data,
        call: impl FnOnce(&mut P, &Context<'_>) -> R,
    ) -> Option<R> {
        let Open { key, page, .. } = self.page.as_mut()?;
        let ctx = Context {
            state: &self.state,
            data,
            world: &self.world,
            location: self.history.current(),
            subject: &key.1,
            direction: i18n::direction(),
            pickers: &self.pickers,
            effects: &self.effects,
        };
        Some(call(page, &ctx))
    }

    fn apply(
        &mut self,
        message: Message<P::Message>,
        data: &mut Data,
    ) -> Task<Message<P::Message>> {
        match message {
            // A page that was closed meanwhile no longer hears its messages:
            // the page in its place is another one.
            Message::Page(id, _) if self.page.as_ref().map(|open| open.id) != Some(id) => {
                Task::none()
            }
            Message::Page(id, message) => self
                .with_page(data, |page, ctx| page.update(message, ctx))
                .map_or_else(Task::none, |task| task.map(move |m| Message::Page(id, m))),
            Message::Go(request) => self.go(request, data),
            Message::Back => {
                self.history.back();
                Task::none()
            }
            Message::Forward => {
                self.history.forward();
                Task::none()
            }
            Message::Select(selection) => self.select(selection),
            Message::Key { press, captured } => {
                // What used the key is a field or a dialog: only the
                // shortcuts with Mod work from there.
                let target = if captured { Target::Text } else { Target::Page };
                let map = self.keymap();
                match keymap::resolve(&map, &press, self.mac, target) {
                    Some(binding) => self.command(binding.command.clone(), data),
                    None => Task::none(),
                }
            }
            Message::Palette(message) => {
                let items = self.palette_items();
                let stepped = matches!(message, palette::Message::Step(_));
                if matches!(message, palette::Message::Toggle) {
                    self.help = false;
                }
                match self.palette.update(message, &items) {
                    Some(Act::Open(route)) => self.go(route.into(), data),
                    Some(Act::SelectHost(id)) => self.select(Selection::Host { id }),
                    Some(Act::Run(run)) => self.run(run, data),
                    None if stepped => self.palette.reveal(&items),
                    None => Task::none(),
                }
            }
            Message::CloseHelp => {
                self.help = false;
                Task::none()
            }
            Message::Sidebar(message) => self.sidebar(message, data),
            Message::Gate(action) => self.gate(action, data),
            Message::Reconnected(finished) => {
                self.reconnect.finish(finished);
                if let Some(error) = self.reconnect.error().cloned() {
                    self.events
                        .push(Event::Failed(None, Surfaced::new(error, "reconnect")));
                    self.reconnect.reset();
                }
                Task::none()
            }
            Message::Added(finished) => self.added(finished, data),
            Message::Saved(Saved { what, error }) => {
                if let Some(error) = error {
                    let title: fn() -> String = match what {
                        Remember::Selection(_) => || t!("core.selection.rememberFailed"),
                        Remember::ClusterNamespace { .. } => || t!("core.namespace.rememberFailed"),
                        Remember::ContainerdNamespace { .. } => {
                            || t!("core.containerd.rememberFailed")
                        }
                        Remember::IncusProject { .. } => || t!("core.incus.rememberFailed"),
                    };
                    let surfaced = Surfaced::new(*error, "update_settings");
                    self.events.push(Event::Failed(Some(title), surfaced));
                }
                // The next one waiting, now that this one is written.
                match self.to_save.pop_front() {
                    Some(next) => next.save(&self.state).map(Message::Saved),
                    None => {
                        self.saving = false;
                        Task::none()
                    }
                }
            }
            Message::DismissUpdate => {
                self.dismissed = self.world.update.as_ref().map(|u| u.version.clone());
                Task::none()
            }
            Message::UpdateFound(update) => {
                self.world.update = update;
                Task::none()
            }
            Message::Data(message) => data.update(&self.state, message).map(Message::Data),
            Message::Changed => Task::none(),
        }
    }

    /// Goes to a page. Asked for the page that shows, it stays as it is,
    /// and starts the action the request carries.
    fn go(&mut self, request: Request, data: &mut Data) -> Task<Message<P::Message>> {
        self.palette.update(palette::Message::Close, &[]);
        self.help = false;
        // An action asked of a page its gate still hides is a navigation
        // of its own: the page starts it when the gate opens.
        let waits = request.action.is_some() && self.page.is_none();
        if !self.at(&request.route) || request.query.is_some() || waits {
            self.history.push(request);
            return Task::none();
        }
        request
            .action
            .map_or_else(Task::none, |action| self.request(action, data))
    }

    /// Asks the page that shows to start `action`.
    fn request(&mut self, action: &str, data: &Data) -> Task<Message<P::Message>> {
        let Some(id) = self.page.as_ref().map(|open| open.id) else {
            return Task::none();
        };
        self.with_page(data, |page, ctx| page.request(action, ctx))
            .map_or_else(Task::none, |task| task.map(move |m| Message::Page(id, m)))
    }

    /// Shows another Host or Cluster, and remembers it.
    fn select(&mut self, selection: Selection) -> Task<Message<P::Message>> {
        self.chosen = Some(selection.clone());
        self.remember(Remember::Selection(selection))
    }

    /// Shows what is remembered at once, and saves it behind.
    /// Saves go one at a time, in the order they were asked for: the last
    /// Host picked is the one a new launch opens on, however long a save
    /// takes.
    fn remember(&mut self, what: Remember) -> Task<Message<P::Message>> {
        what.apply(&mut self.world.settings);
        if self.saving {
            self.to_save.push_back(what);
            return Task::none();
        }
        self.saving = true;
        what.save(&self.state).map(Message::Saved)
    }

    fn command(&mut self, command: Command, data: &mut Data) -> Task<Message<P::Message>> {
        match command {
            Command::TogglePalette => {
                self.help = false;
                self.palette.update(palette::Message::Toggle, &[]);
                Task::none()
            }
            Command::StepHost(by) => {
                let hosts = self.world.host_list();
                if hosts.len() < 2 {
                    return Task::none();
                }
                let count = hosts.len().cast_signed();
                let at = self
                    .world
                    .host_id()
                    .and_then(|id| hosts.iter().position(|view| view.host.id == id))
                    .map_or(-1, usize::cast_signed);
                let next = (at + by).rem_euclid(count).cast_unsigned();
                let id = hosts[next].host.id.clone();
                self.select(Selection::Host { id })
            }
            Command::Refresh => {
                let mut tasks = vec![data.invalidate(&self.state, &keys::host::list())];
                if let Some(id) = self.world.host_id() {
                    tasks.push(data.invalidate(&self.state, &keys::host::scope(id)));
                }
                Task::batch(tasks).map(Message::Data)
            }
            Command::Run(run) => self.run(run, data),
        }
    }

    /// Does what a palette command or a shortcut stands for.
    fn run(&mut self, run: Run, data: &mut Data) -> Task<Message<P::Message>> {
        match run {
            Run::Open(route, action) => {
                let request = Request {
                    route,
                    action,
                    query: None,
                };
                self.go(request, data)
            }
            Run::Request(action) => self.request(action, data),
            Run::Theme(theme) => {
                self.events.push(Event::Theme(theme));
                Task::none()
            }
            Run::CheckForUpdates => {
                self.events.push(Event::CheckForUpdates);
                self.go(Route::Settings.into(), data)
            }
            Run::ShortcutsHelp => {
                self.palette.update(palette::Message::Close, &[]);
                self.help = true;
                Task::none()
            }
        }
    }

    fn sidebar(&mut self, message: sidebar::Message, data: &mut Data) -> Task<Message<P::Message>> {
        match message {
            sidebar::Message::Go(route) => self.go(route.into(), data),
            sidebar::Message::Switcher(event) => {
                self.switcher.update(event);
                Task::none()
            }
            sidebar::Message::Select(selection) => self.select(selection),
            sidebar::Message::Picker(message) => self.pick(message, data),
            sidebar::Message::WhyUnusable => {
                if let Some(unusable) = sidebar::switcher::unusable(&self.world) {
                    // Logged where the connection failed.
                    let surfaced =
                        Surfaced::at(unusable.error.clone(), None, jiff::Timestamp::now());
                    self.events.push(Event::Details(surfaced));
                }
                Task::none()
            }
            sidebar::Message::External(url) => {
                self.events.push(Event::OpenUrl(url));
                Task::none()
            }
        }
    }

    /// A namespace or a project was picked: it shows at once and is
    /// remembered for its Host or Cluster.
    fn pick(&mut self, message: pickers::Message, data: &mut Data) -> Task<Message<P::Message>> {
        match self.pickers.update(message, &self.world) {
            Some(Picked::Remember(what)) => self.remember(what),
            Some(Picked::Go(route)) => self.go(route.into(), data),
            None => Task::none(),
        }
    }

    /// Does what the page asked for during its last call.
    fn run_effects(&mut self, data: &mut Data) -> Task<Message<P::Message>> {
        let mut tasks = vec![];
        for effect in self.effects.take() {
            tasks.push(match effect {
                Effect::Open(request) => self.go(request, data),
                Effect::Back => {
                    self.history.back();
                    Task::none()
                }
                Effect::Select(selection) => self.select(selection),
                Effect::Watch(key, start) => {
                    let Some(open) = &mut self.page else {
                        continue;
                    };
                    if open.watched.contains(&key) {
                        continue;
                    }
                    open.watched.push(key);
                    start(data, &self.state).map(Message::Data)
                }
                Effect::Invalidate(scope) => {
                    data.invalidate(&self.state, &scope).map(Message::Data)
                }
                Effect::Failed(surfaced) => {
                    self.events.push(Event::Failed(None, surfaced));
                    Task::none()
                }
                Effect::Details(surfaced) => {
                    self.events.push(Event::Details(surfaced));
                    Task::none()
                }
                Effect::Copy(text) => {
                    self.events.push(Event::Copy(text));
                    Task::none()
                }
                Effect::Picker(message) => self.pick(message, data),
            });
        }
        Task::batch(tasks)
    }

    /// What the shell itself reads, given what is selected and where the
    /// window is: the lists, the namespaces or projects of what is
    /// selected, and the Swarm of a Swarm page.
    fn wants(&self) -> Vec<Wanted> {
        let mut wants = vec![
            wanted(resources::hosts()),
            wanted(resources::clusters()),
            wanted(resources::swarms()),
            wanted(resources::capabilities()),
        ];
        if let Some(host) = self.world.host() {
            let connected = status::host::is_connected(&host.status);
            let offers = |capability| status::engine::supports(Some(host), capability);
            if connected && offers(Capability::Containerd) {
                wants.push(wanted(resources::containerd_namespaces(&host.host.id)));
            }
            if connected && offers(Capability::Incus) {
                wants.push(wanted(resources::incus_projects(&host.host.id)));
            }
        }
        if let Some(cluster) = self.world.cluster().filter(|c| c.status.is_connected()) {
            let id = &cluster.cluster.id;
            wants.push(match cluster.cluster.kind {
                ClusterKind::Kubernetes => wanted(resources::cluster_namespaces(id)),
                ClusterKind::Nomad => wanted(resources::nomad_namespaces(id)),
            });
        }
        // What this machine runs is looked for while the welcome shows.
        if gates::decide(&self.location().route, &self.world) == Verdict::Welcome {
            wants.push(wanted(resources::local_engines()));
            wants.push(wanted(resources::local_clusters()));
        }
        if let Some(id) = self.location().route.swarm_id() {
            if self.world.swarm_list().iter().any(|swarm| swarm.id == id) {
                wants.push(wanted(resources::swarm_summary(id)));
            }
        }
        wants
    }

    /// Takes the lists from the data layer as they are now.
    fn read(&mut self, data: &Data) {
        let world = &mut self.world;
        world.hosts = data.read(&keys::host::list());
        world.clusters = data.read(&keys::cluster::list());
        world.swarms = data.read(&keys::swarm::list());
        world.capabilities = data
            .read::<api::app::Capabilities>(&keys::app::capabilities())
            .data()
            .cloned();
    }

    /// Takes what is read about the selection and the route.
    fn read_selected(&mut self, data: &Data) {
        let host = self.world.host_id().unwrap_or_default().to_owned();
        let cluster = self.world.cluster_id().unwrap_or_default().to_owned();
        let swarm = self
            .location()
            .route
            .swarm_id()
            .unwrap_or_default()
            .to_owned();
        let world = &mut self.world;
        world.containerd = data.read(&keys::host::containerd_namespaces(&host));
        world.incus = data.read(&keys::host::incus_projects(&host));
        world.namespaces = data.read(&resources::cluster_namespaces(&cluster).key().clone());
        world.nomad = data.read(&keys::nomad::namespaces(&cluster));
        world.swarm = data.read(&keys::swarm::summary(&swarm));
        world.local_engines = data.read(&keys::host::local_engines());
        world.local_clusters = data.read(&keys::cluster::local_clusters());
    }

    /// Lets go of a page's reads.
    fn close_page(&mut self, data: &mut Data) {
        if let Some(open) = self.page.take() {
            for key in &open.watched {
                data.unwatch(key);
            }
        }
    }

    /// Brings the shell in step with the data layer and the route: the
    /// selection (letting go of what the last one held), the page a
    /// selection of the other kind sends away from, what the shell reads,
    /// and the page behind its gate.
    fn sync(&mut self, data: &mut Data) -> Task<Message<P::Message>> {
        let mut tasks = vec![];
        self.read(data);
        let resolved = self.world.resolve(self.chosen.as_ref());
        if resolved != self.world.selection {
            let mut stale = false;
            if let Some(old) = &self.world.selection {
                for scope in selection::scopes(old) {
                    data.release(&scope);
                    // Released with the rest: nothing more to let go of.
                    self.held.retain(|key| !key.is_in(&scope));
                    if let Some(open) = &mut self.page {
                        let before = open.watched.len();
                        open.watched.retain(|key| !key.is_in(&scope));
                        stale |= open.watched.len() != before;
                    }
                }
            }
            self.world.selection = resolved;
            // A page that stays (the Board, a Swarm) and read something of
            // the old selection is opened anew, to read the new one's.
            if stale {
                self.close_page(data);
            }
        }
        let scope = registry::feature(self.location().route.feature()).scope();
        if !self.allows(scope) {
            self.history.replace(Route::Overview.into());
        }
        self.entries = self.visible_entries();
        let wants = self.wants();
        let keys: Vec<Key> = wants.iter().map(|(key, _)| key.clone()).collect();
        for key in self.held.iter().filter(|key| !keys.contains(key)) {
            data.unwatch(key);
        }
        for (key, start) in wants {
            if !self.held.contains(&key) {
                tasks.push(start(data, &self.state).map(Message::Data));
            }
        }
        self.held = keys;
        self.read(data);
        self.read_selected(data);
        let location = self.history.current().clone();
        let subject = match gates::decide(&location.route, &self.world) {
            Verdict::Open(subject) => Some(subject),
            _ => None,
        };
        let key = subject.map(|subject| (location.key, subject));
        if self.page.as_ref().map(|open| &open.key) != key.as_ref() {
            self.close_page(data);
            if let Some(key) = key {
                let ctx = Context {
                    state: &self.state,
                    data,
                    world: &self.world,
                    location: &location,
                    subject: &key.1,
                    direction: i18n::direction(),
                    pickers: &self.pickers,
                    effects: &self.effects,
                };
                self.opened += 1;
                let id = self.opened;
                let (mut page, opened) = P::open(&ctx);
                tasks.push(opened.map(move |m| Message::Page(id, m)));
                // The action a navigation carries is started once.
                if let Some(action) = location.action.filter(|_| self.acted != Some(location.key)) {
                    tasks.push(
                        page.request(action, &ctx)
                            .map(move |m| Message::Page(id, m)),
                    );
                    self.acted = Some(location.key);
                }
                self.page = Some(Open {
                    id,
                    key,
                    page,
                    watched: vec![],
                });
            }
        }
        Task::batch(tasks)
    }
}
