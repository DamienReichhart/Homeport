//! The page's streams, against an Engine that counts what is open on it:
//! a stream starts when its tab shows, and is cancelled when the tab is
//! left, when the page is, when another Host is selected and when the
//! container is gone. Nothing is faked between the page and the socket:
//! the use cases, the crates and the Engine's client are the app's.

use std::time::Duration;

use homeport_core::logs::{LogEvent, LogLine};

use super::*;
use crate::{
    data::streams::Update,
    test_support::{
        engine::{Engine, Stream},
        fixtures::{connected, host_view},
        following::Following,
    },
    ui::{
        features::docker::containers::detail::{subscription, tabs::logs},
        shell::{gates::Subject, selection::Selection},
        widgets::{form::select, log},
    },
};

/// A container's page on a Host whose Engine is the fake.
struct Live {
    rig: Rig,
    engine: Engine,
    following: Following<Message>,
}

impl Live {
    fn at(tab: &str) -> Self {
        let mut bench = Bench::at(Route::Containers);
        let (engine, id) = bench.app.runtime.block_on(async {
            let engine = Engine::start();
            let id = engine.host(&bench.app.state).await;
            (engine, id)
        });
        let mut host = host_view("fake", connected());
        host.host.id.clone_from(&id);
        bench.subject = Subject::Host {
            host_id: id.clone(),
        };
        bench.world.selection = Some(Selection::Host { id: id.clone() });
        bench.set_hosts(vec![host]);
        bench.data.set(&keys::host::containers(&id), sample());
        bench.history.push(route(WEB, Some(tab)).into());
        let mut live = Self {
            rig: Rig::open_on(bench),
            engine,
            following: Following::default(),
        };
        live.sync();
        live
    }

    /// What the runtime does after each `update`: asks the page what it
    /// follows, starts what is new and drops the rest.
    fn sync(&mut self) {
        let follows = subscription(&self.rig.state, &self.rig.bench.ctx());
        let _runtime = self.rig.bench.app.runtime.enter();
        self.following.sync(follows);
    }

    fn send(&mut self, message: Message) {
        self.rig.send(message);
        self.sync();
    }

    /// Hands the page what its streams send, until `done` or two seconds.
    fn until(&mut self, done: impl Fn(&Self) -> bool) -> bool {
        for _ in 0..100 {
            if done(self) {
                return true;
            }
            let wait = Duration::from_millis(20);
            let next = self.following.next(wait);
            if let Some(message) = self.rig.bench.app.runtime.block_on(next) {
                self.send(message);
            }
        }
        done(self)
    }

    /// Whether `count` streams of this kind are open on the Engine,
    /// within two seconds.
    fn settles(&mut self, stream: Stream, count: usize) -> bool {
        self.until(|live| live.engine.open(stream) == count)
    }

    fn lines(&self) -> usize {
        self.rig.state.logs.view.len()
    }
}

#[test]
fn the_logs_stream_while_their_tab_shows_and_stop_when_it_is_left() {
    let mut live = Live::at("inspect");
    // Another tab shows: nothing was asked of the Engine.
    assert!(live.settles(Stream::Logs, 0));
    assert!(live.engine.asked("/logs").is_empty());

    live.send(Message::Tab(Tab::Logs));
    assert!(live.settles(Stream::Logs, 1));
    live.engine.log("listening on :8080");
    live.engine.log("ready");
    assert!(live.until(|live| live.lines() == 2));

    // The tab is left: its stream is cancelled, and what it kept goes.
    live.send(Message::Tab(Tab::Inspect));
    assert!(live.settles(Stream::Logs, 0));
    live.send(Message::Tab(Tab::Logs));
    assert!(live.settles(Stream::Logs, 1));
    assert_eq!(live.lines(), 0, "a tab that comes back starts over");
    assert_eq!(live.engine.asked("/logs").len(), 2);
}

#[test]
fn the_log_stream_is_cancelled_with_the_page_the_host_and_the_container() {
    // The page is left: the runtime is given nothing to follow.
    let mut live = Live::at("logs");
    assert!(live.settles(Stream::Logs, 1));
    live.following.sync(iced::Subscription::none());
    assert!(live.settles(Stream::Logs, 0));

    // The container is no longer in the Host's list.
    let mut live = Live::at("logs");
    assert!(live.settles(Stream::Logs, 1));
    live.rig.list(vec![sample().remove(2)]);
    live.sync();
    assert!(live.settles(Stream::Logs, 0));

    // Another Host is selected: the shell opens the page anew under it,
    // and that Host has no such container.
    let mut live = Live::at("logs");
    assert!(live.settles(Stream::Logs, 1));
    live.rig.bench.subject = Subject::Host {
        host_id: "other".to_owned(),
    };
    let (other, _) = open(&live.rig.bench.ctx());
    live.rig.state = other;
    live.sync();
    assert!(live.settles(Stream::Logs, 0));
    assert_eq!(live.following.count(), 0);
}

#[test]
fn a_stream_that_ends_says_so_and_a_container_that_starts_again_is_followed_again() {
    let mut live = Live::at("logs");
    assert!(live.settles(Stream::Logs, 1));
    live.engine.log("bye");
    live.engine.end_logs();
    assert!(live.until(|live| live.rig.state.logs.is_over()));
    assert!(live.settles(Stream::Logs, 0));
    // The list says it stopped, then that it runs again.
    let mut stopped = sample();
    "exited".clone_into(&mut stopped[0].state);
    live.rig.list(stopped);
    live.sync();
    assert!(live.settles(Stream::Logs, 0));
    live.rig.list(sample());
    live.sync();
    assert!(live.settles(Stream::Logs, 1));
    assert_eq!(live.engine.asked("/logs").len(), 2);
}

fn chunk(text: &str) -> Update<LogEvent> {
    let line = LogLine {
        source: None,
        stream: "stdout".to_owned(),
        text: text.to_owned(),
    };
    Update::Batch {
        items: vec![LogEvent::Lines { lines: vec![line] }],
        dropped: 0,
    }
}

#[test]
fn what_the_engine_applies_opens_the_stream_again_and_a_late_chunk_is_ignored() {
    let mut live = Live::at("logs");
    assert!(live.settles(Stream::Logs, 1));
    let first = live.engine.asked("/logs").pop().expect("a request");
    let parts: Vec<&str> = first.split(['?', '&']).skip(1).collect();
    for part in [
        "tail=1000",
        "follow=true",
        "stdout=true",
        "stderr=true",
        "timestamps=false",
        "since=0",
    ] {
        assert!(parts.contains(&part), "{part} in {first}");
    }
    live.engine.log("old");
    assert!(live.until(|live| live.lines() == 1));
    let old = live.rig.state.logs.generation();

    let logs = |message| Message::Logs(message);
    let asked_with = |live: &mut Live, message: logs::Message, part: &str| {
        let before = live.engine.asked("/logs").len();
        live.send(logs(message));
        assert_eq!(live.lines(), 0, "the viewer starts over");
        assert!(live.until(|live| live.engine.asked("/logs").len() == before + 1));
        let request = live.engine.asked("/logs").pop().unwrap_or_default();
        let asked = |part: &str| {
            let mut said = request.split(['?', '&']);
            // `tail=100` is not `tail=1000`, and `since=1…` is any time
            // of this century.
            said.any(|said| said == part || (part == "since=1" && said.starts_with(part)))
        };
        assert!(asked(part), "{part} in {request}");
        // The stream before is let go of: one is open, not two.
        assert!(live.settles(Stream::Logs, 1));
    };
    asked_with(
        &mut live,
        logs::Message::Tail(select::Message::Choose(0)),
        "tail=100",
    );
    // What the stream of before the change still sends is not shown.
    live.send(logs(logs::Message::Lines(old, chunk("late"))));
    assert_eq!(live.lines(), 0);
    let current = live.rig.state.logs.generation();
    live.send(logs(logs::Message::Lines(current, chunk("on time"))));
    assert_eq!(live.lines(), 1);

    asked_with(
        &mut live,
        logs::Message::Since(select::Message::Choose(1)),
        "since=1",
    );
    asked_with(
        &mut live,
        logs::Message::Viewer(log::Message::Timestamps(true)),
        "timestamps=true",
    );
    asked_with(
        &mut live,
        logs::Message::Toggle(logs::Channel::Stderr),
        "stderr=false",
    );
    // The same choice again changes nothing: the stream goes on.
    let asked = live.engine.asked("/logs").len();
    live.send(logs(logs::Message::Tail(select::Message::Choose(0))));
    assert_eq!(live.engine.asked("/logs").len(), asked);
}

#[test]
fn the_stats_stream_while_their_tab_shows_a_running_container() {
    let mut live = Live::at("logs");
    assert!(live.settles(Stream::Logs, 1));
    assert!(live.engine.asked("/stats").is_empty());

    // The tab changes: the logs are let go of, the stats are followed.
    live.send(Message::Tab(Tab::Stats));
    assert!(live.settles(Stream::Stats, 1) && live.settles(Stream::Logs, 0));
    live.engine.sample(64 * 1024 * 1024, 1000);
    assert!(live.until(|live| live.rig.state.stats.last.is_some()));
    let last = live.rig.state.stats.last.clone().expect("a sample");
    assert_eq!((last.memory_usage, last.net_rx), (64 * 1024 * 1024, 1000));

    // The container stops: nothing is left to follow.
    let mut stopped = sample();
    "exited".clone_into(&mut stopped[0].state);
    live.rig.list(stopped);
    live.sync();
    assert!(live.settles(Stream::Stats, 0));
    // It runs again: the same charts go on.
    live.rig.list(sample());
    live.sync();
    assert!(live.settles(Stream::Stats, 1));
    assert!(live.rig.state.stats.last.is_some());

    // The tab is left, then the page.
    live.send(Message::Tab(Tab::Processes));
    assert!(live.settles(Stream::Stats, 0));
    live.send(Message::Tab(Tab::Stats));
    assert!(live.settles(Stream::Stats, 1));
    live.following.sync(iced::Subscription::none());
    assert!(live.settles(Stream::Stats, 0));
    assert_eq!(live.engine.asked("/stats").len(), 3);
}

#[test]
fn a_stopped_container_s_stats_are_never_asked_for() {
    let mut bench_page = Live::at("stats");
    let mut stopped = sample();
    "exited".clone_into(&mut stopped[0].state);
    bench_page.rig.list(stopped);
    bench_page.sync();
    assert!(bench_page.settles(Stream::Stats, 0));
    // Only the window's size is listened to.
    assert_eq!(bench_page.following.count(), 1);
}
