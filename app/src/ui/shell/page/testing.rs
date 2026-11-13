//! A page's surroundings for its tests: an app on a new database, an empty
//! data layer to fill, and a [`Context`] on a route, without a shell.

use std::sync::Arc;

use homeport_core::error::AppError;
use homeport_hosts::service::HostView;
use homeport_settings::Settings;
use tokio::runtime::Runtime;

use super::{context::Effect, Context, Effects, Outcomes, World};
use crate::{
    app::{testing, AppState},
    data::{keys, Data, Key},
    i18n,
    platform::{fake::Fake, Platform},
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
    /// The system the page is given: `os` answers its dialogs.
    pub platform: Platform,
    /// What stands for the OS: `bench.os.answer(path)` is what the next
    /// dialog answers (none is a cancel), `bench.os.took()` what was
    /// asked of it. No dialog of the OS ever opens.
    pub os: Arc<Fake>,
    pickers: pickers::State,
    effects: Effects,
    outcomes: Outcomes,
}

impl Bench {
    /// A page of `route`, about the app, with nothing read yet.
    pub fn at(route: Route) -> Self {
        let os = Fake::new();
        Self {
            platform: os.platform(),
            os,
            app: TestApp::start(),
            data: Data::default(),
            world: World::new(Settings::default()),
            history: History::starting_at(Request::to(route)),
            subject: Subject::App,
            pickers: pickers::State::default(),
            effects: Effects::default(),
            outcomes: Outcomes::default(),
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
            platform: &self.platform,
            pickers: &self.pickers,
            effects: &self.effects,
            outcomes: &self.outcomes,
        }
    }

    /// Runs what an `update` returned and gives the messages it produced.
    pub fn run<Message>(&self, task: iced::Task<Message>) -> Vec<Message> {
        self.app
            .runtime
            .block_on(crate::test_support::tasks::messages(task))
    }

    /// An action the page started ended with `outcome`, as if its use
    /// case had answered: a page has no Engine in its tests.
    pub fn told(&self, outcome: super::Outcome) {
        let _ = self.outcomes.sender().send(outcome);
    }

    /// What the page asked of the shell since the last call, in order,
    /// then what the actions it started (`ctx.act`) told the shell once
    /// they ended: what they said, how they failed, what they filed and
    /// what they made out of date. What was filed is in the data layer,
    /// and what was kept (`ctx.keep`) in the location, as the shell would
    /// have done.
    pub fn took(&mut self) -> Vec<Asked> {
        let mut asked = vec![];
        for effect in self.effects.take() {
            asked.push(match effect {
                Effect::Open(request) => Asked::Open(request),
                Effect::Back => Asked::Back,
                Effect::Select(selection) => Asked::Select(selection),
                Effect::Watch(key, _) => Asked::Watch(key),
                Effect::Unwatch(key) => Asked::Unwatch(key),
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
                Effect::KeepRoute(route) => {
                    self.history.retarget(route.clone());
                    Asked::KeepRoute(route)
                }
            });
        }
        for outcome in self.outcomes.take() {
            asked.extend(outcome.said.map(|said| Asked::Said(said.now())));
            asked.extend(outcome.failed.map(|(title, surfaced)| {
                Asked::Failed(title.map(|title| title.now()), surfaced.error)
            }));
            for (key, put) in outcome.puts {
                put(&mut self.data);
                asked.push(Asked::Put(key));
            }
            asked.extend(outcome.invalidates.into_iter().map(Asked::Invalidate));
        }
        asked
    }

    /// Reads what the page asked to watch since the last call, as the
    /// shell does, until every read settled. A page has no Engine in its
    /// tests, so what is read of a Host fails with `HostNotConnected`: the
    /// failure a page shows in place of what it could not read. Whatever
    /// else the page asked for since the last call is let go of.
    pub fn read_watched(&mut self) {
        let mut tasks = vec![];
        for effect in self.effects.take() {
            if let Effect::Watch(_, start) = effect {
                tasks.push(start(&mut self.data, &self.app.state));
            }
        }
        while let Some(task) = tasks.pop() {
            for message in self.run(task) {
                tasks.push(self.data.update(&self.app.state, message));
            }
        }
    }

    /// The keys the page let go of since the last call, the other
    /// effects being let go of too.
    pub fn unwatched(&self) -> Vec<Key> {
        self.effects
            .take()
            .into_iter()
            .filter_map(|effect| match effect {
                Effect::Unwatch(key) => Some(key),
                _ => None,
            })
            .collect()
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
                Effect::Unwatch(_) => "unwatch",
                Effect::Invalidate(_) => "invalidate",
                Effect::Failed(..) => "failed",
                Effect::Details(_) => "details",
                Effect::Copy(_) => "copy",
                Effect::Picker(_) => "picker",
                Effect::Said(..) => "said",
                Effect::OpenUrl(_) => "open_url",
                Effect::Put(..) => "put",
                Effect::Keep { .. } => "keep",
                Effect::KeepRoute(_) => "keep_route",
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
    Unwatch(Key),
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
    /// The route the page's location has from now on.
    KeepRoute(Route),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        platform::{
            dialogs::SaveFile,
            fake::Asked as AskedOfOs,
            files::{self, Saved},
        },
        ui::{shell::Outcome, widgets::feedback::Surfaced},
    };

    /// What a page does of a download, as `crate::platform` documents it:
    /// the dialog, the write off the window's thread, the toast by the shell.
    fn save(bench: &Bench, contents: &'static str) -> Vec<Result<Saved, AppError>> {
        let ctx = bench.ctx();
        let request = SaveFile::new("Save logs", "logs.txt");
        let path = match bench.run(ctx.platform.save_file(request)).pop() {
            Some(Ok(Some(path))) => path,
            // Cancelled: nothing follows.
            Some(Ok(None)) | None => return vec![],
            // No dialog could be shown: the page says so.
            Some(Err(error)) => {
                ctx.failed(Surfaced::at(
                    error,
                    Some("save_file"),
                    jiff::Timestamp::UNIX_EPOCH,
                ));
                return vec![];
            }
        };
        let saving = files::save(path, move |out, _| out.write_all(contents.as_bytes()));
        bench.run(ctx.act(
            saving.done,
            |saved| match saved {
                Ok(Saved::Whole) => Outcome::quiet().saying(|| "Logs saved".to_owned()),
                Ok(Saved::Stopped) => Outcome::quiet(),
                Err(e) => Outcome::quiet().failing(Surfaced::at(
                    e.clone(),
                    Some("write_text_file"),
                    jiff::Timestamp::UNIX_EPOCH,
                )),
            },
            |saved| saved,
        ))
    }

    #[test]
    fn a_page_s_dialog_is_answered_by_the_bench_and_its_save_is_said_by_the_shell() {
        let mut bench = Bench::at(Route::Hosts);
        // Nothing was answered: the dialog is cancelled, and nothing follows.
        assert!(save(&bench, "one\n").is_empty());
        assert_eq!(
            bench.os.took(),
            [AskedOfOs::Save(SaveFile::new("Save logs", "logs.txt"))]
        );
        assert!(bench.took().is_empty());

        let path = bench.os.dir().join("logs.txt");
        bench.os.answer(&path);
        assert_eq!(save(&bench, "one\n"), [Ok(Saved::Whole)]);
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "one\n");
        assert_eq!(bench.took(), [Asked::Said("Logs saved".to_owned())]);

        // A path that can't be written: the failure is the shell's to say.
        bench.os.answer(bench.os.dir().join("missing/logs.txt"));
        let failed = save(&bench, "two\n");
        let [Err(error)] = failed.as_slice() else {
            panic!("{failed:?}");
        };
        assert_eq!(bench.took(), [Asked::Failed(None, error.clone())]);
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "one\n");

        // A desktop with nothing to show a dialog with: said, not taken
        // for a cancel.
        bench.os.without_dialogs();
        assert!(save(&bench, "three\n").is_empty());
        let said = bench.took();
        let [Asked::Failed(None, error)] = said.as_slice() else {
            panic!("{said:?}");
        };
        assert_eq!(error.key, "errors.file.dialogUnavailable");
    }
}
