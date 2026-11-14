//! A container's page: what it reads, what its messages change, how it
//! looks in each state.

mod drawn;
#[cfg(unix)]
mod exec;
mod files;
mod health;
mod inspect;
mod logs;
mod processes;
mod stats;
#[cfg(unix)]
mod streams;
#[cfg(unix)]
mod transfers;
mod updates;

use homeport_engine::containers::Container;
use homeport_hosts::service::HostView;

use super::{dialog, open, refresh, state::Tab, update, view, Message, State};
use crate::{
    data::keys,
    test_support::{
        fixtures::{connected_host, container},
        stage::{Frame, Stage},
        window::window,
    },
    ui::{
        shell::{
            page::testing::Bench,
            router::{Request, Route},
        },
        theme::Element,
        widgets::overlay,
    },
};

/// The page as it shows in the window: 1100 wide less the sidebar.
const SIZE: (f32, f32) = (828.0, 700.0);

/// The window that page is in: what the page lays itself out by.
const WINDOW: (f32, f32) = (1100.0, 748.0);

/// The id of the sample's `web` container.
const WEB: &str = "shop-web-1-0123456789abcdef";

/// When the sample's containers were created: 2026-05-01 10:00:00 UTC.
const THEN: i64 = 1_777_629_600;

fn route(id: &str, tab: Option<&str>) -> Route {
    Route::Container {
        id: id.to_owned(),
        tab: tab.map(str::to_owned),
    }
}

/// A running container of a Stack with a healthcheck, a stopped one of
/// the same Stack, and one of no Stack.
fn sample() -> Vec<Container> {
    let mut web = container("shop-web-1", Some("shop"), "running", THEN);
    web.health = Some("healthy".to_owned());
    "Up 3 hours (healthy)".clone_into(&mut web.status);
    let mut db = container("shop-db-1", Some("shop"), "exited", THEN);
    "Exited (0) 2 days ago".clone_into(&mut db.status);
    let redis = container("redis", None, "running", THEN);
    vec![web, db, redis]
}

/// The page on a bench, drawn on a stage that keeps its tree.
struct Rig {
    bench: Bench,
    state: State,
    stage: Stage,
}

impl Rig {
    fn on(host: HostView, request: Request, list: Option<Vec<Container>>) -> Self {
        let id = host.host.id.clone();
        let mut bench = Bench::on_host(Route::Containers, host);
        bench.history.push(request);
        if let Some(list) = list {
            bench.data.set(&keys::host::containers(&id), list);
        }
        Self::open_on(bench)
    }

    /// The page of where `bench` is, opened.
    fn open_on(bench: Bench) -> Self {
        let (state, task) = open(&bench.ctx());
        let mut rig = Self {
            bench,
            state,
            stage: Stage::new(SIZE),
        };
        for message in rig.bench.run(task) {
            rig.send(message);
        }
        // The window says its size once it is asked: no window is there
        // in a test, so the test says it.
        rig.send(Message::Resized(WINDOW.into()));
        let _ = rig.bench.took();
        rig
    }

    /// The page whose reads were tried and failed: no Engine answers a
    /// test, so each fails as a Host that is not connected.
    fn unread(request: Request) -> Self {
        let mut bench = Bench::on_host(Route::Containers, connected_host());
        bench.history.push(request);
        let (state, _) = open(&bench.ctx());
        bench.read_watched();
        Self {
            bench,
            state,
            stage: Stage::new(SIZE),
        }
    }

    /// The page of the sample's container `id`, on `tab`.
    fn at(id: &str, tab: &str) -> Self {
        Self::on(
            connected_host(),
            route(id, Some(tab)).into(),
            Some(sample()),
        )
    }

    /// The container the page is about, as the list has it now.
    fn container(&self) -> Container {
        let list = self.state.containers(&self.bench.ctx());
        let list = list.data().map_or(&[][..], |list| list.as_slice());
        self.state.find(list).expect("the container").clone()
    }

    /// The Host's containers were read again.
    fn list(&mut self, list: Vec<Container>) {
        let key = keys::host::containers(&self.state.host_id);
        self.bench.data.set(&key, list);
        self.refresh();
    }

    fn refresh(&mut self) {
        let task = refresh(&mut self.state, &self.bench.ctx());
        for message in self.bench.run(task) {
            self.send(message);
        }
    }

    /// Sends a message and what it started, until nothing more comes.
    fn send(&mut self, message: Message) {
        // What `update` starts (a timer, a task) needs the runtime the
        // window has.
        let runtime = self.bench.app.runtime.handle().clone();
        let _inside = runtime.enter();
        let task = update(&mut self.state, message, &self.bench.ctx());
        for answer in self.bench.run(task) {
            self.send(answer);
        }
    }

    /// The page with its dialog over it, as the shell lays them.
    fn view(&self) -> Element<'_, Message> {
        let ctx = self.bench.ctx();
        overlay::over(view(&self.state, &ctx), dialog(&self.state, &ctx))
    }

    /// Draws the page on the stage, acts on it, and applies what that sent.
    fn frame(&mut self, act: impl FnOnce(&mut Frame<'_, '_, Message>)) {
        let ctx = self.bench.ctx();
        let page = overlay::over(view(&self.state, &ctx), dialog(&self.state, &ctx));
        let messages = self.stage.frame(page, act);
        for message in messages {
            self.send(message);
        }
    }

    /// Whether the page says `label` somewhere in view.
    fn shows(&self, label: &str) -> bool {
        self.shows_in(SIZE, label)
    }

    /// The same in a window of `size`: a page taller than the window
    /// shows only its top.
    fn shows_in(&self, size: (f32, f32), label: &str) -> bool {
        window(size, self.view()).find(label).is_ok()
    }
}
