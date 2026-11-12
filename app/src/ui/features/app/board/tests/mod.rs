//! The Board: what it reads, what its messages change, how it looks in
//! each state, and what twenty thousand containers cost it.

mod drawn;
mod keyboard;
mod perf;
mod updates;

use std::collections::HashMap;

use homeport_core::error::{AppError, ErrorCode, Message as Text};
use homeport_engine::containers::{stats::ContainerUsage, Container};
use homeport_hosts::{registry::HostStatus, service::HostView};

use super::{dialog, open, refresh, update, view, Message, State};
use crate::{
    data::keys,
    test_support::{
        fixtures::{connected, connected_host, container, host_view},
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
const SIZE: (f32, f32) = (828.0, 640.0);

fn now() -> i64 {
    jiff::Timestamp::now().as_second()
}

fn refused() -> AppError {
    AppError::new(
        ErrorCode::ConnectionRefused,
        Text {
            key: "errors.test.refused",
            en: "connection refused",
        },
    )
}

/// This machine, a server, and a NAS that doesn't answer.
fn hosts() -> Vec<HostView> {
    let down = HostStatus::Unreachable {
        error: refused(),
        retry_in_secs: 8,
    };
    vec![
        connected_host(),
        host_view("prod-server", connected()),
        host_view("nas", down),
    ]
}

const PROD: &str = "prod-server-id";

fn local() -> Vec<Container> {
    let mut web = container("web", Some("shop"), "running", now());
    web.health = Some("healthy".to_owned());
    "Up 1 minute (healthy)".clone_into(&mut web.status);
    vec![web, container("db", Some("shop"), "exited", now())]
}

fn prod() -> Vec<Container> {
    let mut api = container("api", None, "running", now());
    api.health = Some("unhealthy".to_owned());
    vec![api, container("cache", Some("blog"), "running", now())]
}

fn usage(list: &[Container]) -> HashMap<String, ContainerUsage> {
    list.iter()
        .filter(|c| c.state == "running")
        .enumerate()
        .map(|(at, c)| {
            let usage = ContainerUsage {
                id: c.id.clone(),
                cpu_percent: 2.5 + 4.0 * f64::from(u8::try_from(at).unwrap_or(0)),
                memory_usage: 48 * 1024 * 1024 * (at as u64 + 1),
                memory_limit: 1024 * 1024 * 1024,
            };
            (c.id.clone(), usage)
        })
        .collect()
}

/// The Board on a bench, drawn on a stage that keeps its tree.
struct Rig {
    bench: Bench,
    state: State,
    stage: Stage,
}

impl Rig {
    /// The Board over `hosts`, opened by `request`.
    fn at(request: Request, hosts: Vec<HostView>) -> Self {
        let mut bench = Bench::at(Route::Board);
        bench.history.push(request);
        bench.set_hosts(hosts);
        let (state, _) = open(&bench.ctx());
        Self {
            bench,
            state,
            stage: Stage::new(SIZE),
        }
    }

    fn over(hosts: Vec<HostView>) -> Self {
        Self::at(Request::to(Route::Board), hosts)
    }

    /// The three Hosts, the two connected ones read, with their usage.
    fn populated() -> Self {
        Self::populated_at(Request::to(Route::Board))
    }

    fn populated_at(request: Request) -> Self {
        let mut rig = Self::at(request, hosts());
        rig.read("h1", local());
        rig.read(PROD, prod());
        rig.send(Message::Usage(("h1".to_owned(), usage(&local()))));
        rig.send(Message::Usage((PROD.to_owned(), usage(&prod()))));
        let _ = rig.bench.took();
        rig
    }

    /// A Host's containers were read.
    fn read(&mut self, host: &str, list: Vec<Container>) {
        self.bench.data.set(&keys::host::containers(host), list);
        self.refresh();
    }

    fn refresh(&mut self) {
        let _ = refresh(&mut self.state, &self.bench.ctx());
    }

    /// Sends a message and what it started, until nothing more comes.
    fn send(&mut self, message: Message) {
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

    fn shows(&self, label: &str) -> bool {
        window(SIZE, self.view()).find(label).is_ok()
    }

    /// How far down the page `label` is drawn.
    fn top(&self, label: &str) -> f32 {
        let mut window = window(SIZE, self.view());
        let found = window.find(label);
        found
            .unwrap_or_else(|_| panic!("nothing says {label:?}"))
            .bounds()
            .y
    }

    /// The names in `names` that show, from the top.
    fn order(&self, names: &[&'static str]) -> Vec<&'static str> {
        let mut shown: Vec<(f32, &str)> = names
            .iter()
            .filter(|name| self.shows(name))
            .map(|name| (self.top(name), *name))
            .collect();
        shown.sort_by(|a, b| a.0.total_cmp(&b.0));
        shown.into_iter().map(|(_, name)| name).collect()
    }
}
