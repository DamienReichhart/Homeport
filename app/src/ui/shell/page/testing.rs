//! A page's surroundings for its tests: an app on a new database, an empty
//! data layer to fill, and a [`Context`] on a route, without a shell.

use std::sync::Arc;

use homeport_settings::Settings;
use tokio::runtime::Runtime;

use super::{context::Effect, Context, Effects, World};
use crate::{
    app::{testing, AppState},
    data::{Data, Key},
    i18n,
    ui::shell::{
        gates::Subject,
        pickers,
        router::{History, Request, Route},
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
                Effect::Said(..) => "said",
                Effect::OpenUrl(_) => "open_url",
                Effect::Put(_) => "put",
                Effect::Keep { .. } => "keep",
                Effect::Details(_) => "details",
                Effect::Copy(_) => "copy",
                Effect::Picker(_) => "picker",
            })
            .collect()
    }
}
