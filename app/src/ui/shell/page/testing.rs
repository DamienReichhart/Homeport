//! A page's surroundings for its tests: an app on a new database, an empty
//! data layer to fill, and a [`Context`] on a route, without a shell.

use std::sync::Arc;

use homeport_core::error::AppError;
use homeport_hosts::service::HostView;
use homeport_settings::Settings;
use tokio::runtime::Runtime;

use super::{context::Effect, Context, Effects, World};
use crate::{
    app::{testing, AppState},
    data::{keys, Data, Key},
    i18n,
    ui::shell::{
        gates::Subject,
        pickers,
        router::{History, Request, Route},
        selection::Selection,
    },
};

/// An app with no Host, and the runtime its state was built on.
pub struct TestApp {
    pub state: Arc<AppState>,
    /// Kept so the database's folder and the runtime outlive the test.
    _dir: tempfile::TempDir,
    pub runtime: Runtime,
}

impl TestApp {
    pub fn start() -> Self {
        let runtime = Runtime::new().expect("a runtime");
        let app = runtime.block_on(async { testing::app() });
        Self {
            state: Arc::new(app.state),
            _dir: app.dir,
            runtime,
        }
    }
}

/// What a page is given, owned, for a test to fill and hand over.
pub struct Bench {
    pub app: TestApp,
    pub data: Data,
    pub world: World,
    pub history: History,
    pub subject: Subject,
    pickers: pickers::State,
    effects: Effects,
}

impl Bench {
    /// A page of `route`, about the app, with nothing read yet.
    pub fn at(route: Route) -> Self {
        Self {
            app: TestApp::start(),
            data: Data::default(),
            world: World::new(Settings::default()),
            history: History::starting_at(Request::to(route)),
            subject: Subject::App,
            pickers: pickers::State::default(),
            effects: Effects::default(),
        }
    }

    /// A page of `route` about `host`, connected and selected.
    pub fn on_host(route: Route, host: HostView) -> Self {
        let mut bench = Self::at(route);
        bench.subject = Subject::Host {
            host_id: host.host.id.clone(),
        };
        bench.world.selection = Some(Selection::Host {
            id: host.host.id.clone(),
        });
        bench.set_hosts(vec![host]);
        bench
    }

    /// The Hosts, as read: in the data layer and in what the shell knows.
    pub fn set_hosts(&mut self, hosts: Vec<HostView>) {
        self.data.set(&keys::host::list(), hosts);
        self.world.hosts = self.data.read(&keys::host::list());
    }

    pub fn ctx(&self) -> Context<'_> {
        Context {
            state: &self.app.state,
            data: &self.data,
            world: &self.world,
            location: self.history.current(),
            subject: &self.subject,
            direction: i18n::direction(),
            pickers: &self.pickers,
            effects: &self.effects,
        }
    }

    /// Runs what an `update` returned and gives the messages it produced.
    pub fn run<Message>(&self, task: iced::Task<Message>) -> Vec<Message> {
        self.app
            .runtime
            .block_on(crate::test_support::tasks::messages(task))
    }

    /// What the page asked of the shell since the last call, in order.
    /// What it filed (`ctx.put`) is in the data layer, and what it kept
    /// (`ctx.keep`) in its location, as the shell would have done.
    pub fn took(&mut self) -> Vec<Asked> {
        let mut asked = vec![];
        for effect in self.effects.take() {
            asked.push(match effect {
                Effect::Open(request) => Asked::Open(request),
                Effect::Back => Asked::Back,
                Effect::Select(selection) => Asked::Select(selection),
                Effect::Watch(key, _) => Asked::Watch(key),
                Effect::Invalidate(key) => Asked::Invalidate(key),
                Effect::Failed(title, surfaced) => {
                    Asked::Failed(title.map(|title| title.now()), surfaced.error)
                }
                Effect::Details(surfaced) => Asked::Details(surfaced.error),
                Effect::Copy(text) => Asked::Copy(text),
                Effect::Picker(_) => Asked::Picker,
                Effect::Said(_, said) => Asked::Said(said.now()),
                Effect::OpenUrl(url) => Asked::OpenUrl(url),
                Effect::Put(key, put) => {
                    put(&mut self.data);
                    Asked::Put(key)
                }
                Effect::Keep { query, params } => {
                    self.history.amend(query, params);
                    Asked::Keep
                }
            });
        }
        asked
    }

    /// The keys the page asked to watch since the last call.
    pub fn watched(&self) -> Vec<Key> {
        self.effects
            .take()
            .into_iter()
            .filter_map(|effect| match effect {
                Effect::Watch(key, _) => Some(key),
                _ => None,
            })
            .collect()
    }

    /// What the page asked of the shell since the last call, by name.
    pub fn asked(&self) -> Vec<&'static str> {
        self.effects
            .take()
            .iter()
            .map(|effect| match effect {
                Effect::Open(_) => "open",
                Effect::Back => "back",
                Effect::Select(_) => "select",
                Effect::Watch(..) => "watch",
                Effect::Invalidate(_) => "invalidate",
                Effect::Failed(..) => "failed",
                Effect::Details(_) => "details",
                Effect::Copy(_) => "copy",
                Effect::Picker(_) => "picker",
                Effect::Said(..) => "said",
                Effect::OpenUrl(_) => "open_url",
                Effect::Put(..) => "put",
                Effect::Keep { .. } => "keep",
            })
            .collect()
    }
}

/// Something a page asked of the shell, as a test reads it.
#[derive(Debug, Clone, PartialEq)]
pub enum Asked {
    Open(Request),
    Back,
    Select(Selection),
    Watch(Key),
    Invalidate(Key),
    /// A failure's toast: its title, if it names what failed, and why.
    Failed(Option<String>, AppError),
    Details(AppError),
    Copy(String),
    Picker,
    /// A toast that says what was done, in the language of the moment.
    Said(String),
    OpenUrl(String),
    Put(Key),
    Keep,
}
