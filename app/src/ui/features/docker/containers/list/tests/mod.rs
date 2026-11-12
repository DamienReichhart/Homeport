//! The Containers page: what it reads, what its messages change, how it
//! looks in each state and that the keyboard alone drives it.

mod drawn;
mod keyboard;
mod updates;

use std::collections::HashMap;

use homeport_engine::{
    containers::{stats::ContainerUsage, Container, PortBinding},
    images::freshness::{Freshness, FreshnessResult},
};
use homeport_hosts::service::HostView;

use super::{dialog, open, refresh, update, view, Message, State};
use crate::{
    data::keys,
    test_support::{
        fixtures::{connected_host, container},
        stage::{Frame, Stage},
        window::window,
    },
    ui::{
        shell::{page::testing::Bench, router::Route},
        theme::Element,
        widgets::overlay,
    },
};

/// The page as it shows in the window: 1100 wide less the sidebar.
const SIZE: (f32, f32) = (828.0, 700.0);

fn now() -> i64 {
    jiff::Timestamp::now().as_second()
}

fn port(private: u16, public: Option<u16>) -> PortBinding {
    PortBinding {
        ip: None,
        private_port: private,
        public_port: public,
        protocol: "tcp".to_owned(),
    }
}

/// Two Stacks and two containers of none, as the React tests had them.
fn sample() -> Vec<Container> {
    let now = now();
    let mut web = container("shop-web-1", Some("shop"), "running", now);
    web.ports = vec![port(443, None), port(80, Some(8080))];
    web.health = Some("healthy".to_owned());
    "Up 1 minute (healthy)".clone_into(&mut web.status);
    let db = container("shop-db-1", Some("shop"), "exited", now);
    let app = container("blog-app-1", Some("blog"), "running", now);
    let mut redis = container("redis", None, "running", now);
    redis.ports = vec![port(6379, None)];
    let job = container("backup-job", None, "exited", now);
    vec![web, db, app, redis, job]
}

fn usage(list: &[Container]) -> HashMap<String, ContainerUsage> {
    list.iter()
        .filter(|c| c.state == "running")
        .enumerate()
        .map(|(at, c)| {
            let usage = ContainerUsage {
                id: c.id.clone(),
                cpu_percent: 1.5 + 2.0 * f64::from(u8::try_from(at).unwrap_or(0)),
                memory_usage: 64 * 1024 * 1024 * (at as u64 + 1),
                memory_limit: 1024 * 1024 * 1024,
            };
            (c.id.clone(), usage)
        })
        .collect()
}

/// The page on a bench, drawn on a stage that keeps its tree.
struct Rig {
    bench: Bench,
    state: State,
    stage: Stage,
}

impl Rig {
    fn on(host: HostView, list: Option<Vec<Container>>) -> Self {
        let mut bench = Bench::on_host(Route::Containers, host);
        if let Some(list) = list {
            bench.data.set(&keys::host::containers("h1"), list);
        }
        let (state, _) = open(&bench.ctx());
        let _ = bench.took();
        Self {
            bench,
            state,
            stage: Stage::new(SIZE),
        }
    }

    fn with(list: Vec<Container>) -> Self {
        Self::on(connected_host(), Some(list))
    }

    /// The sample containers, with their usage and one newer image.
    fn populated() -> Self {
        let list = sample();
        let mut rig = Self::with(list.clone());
        let newer = FreshnessResult {
            reference: "app:latest".to_owned(),
            image_id: "sha256:abc".to_owned(),
            freshness: Freshness::Newer,
        };
        rig.bench
            .data
            .set(&keys::host::freshness("h1"), vec![newer]);
        rig.refresh();
        rig.send(Message::Usage(("h1".to_owned(), usage(&list))));
        rig
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

    /// Whether the page says `label` somewhere in view.
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
}
