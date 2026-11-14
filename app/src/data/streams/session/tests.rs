//! A terminal's session against the fake Engine: the real use cases, the
//! real crate and the Engine's client, over a socket. What is typed
//! reaches the container in order, a size is told, the output comes in
//! batches, the end says how it ended, and letting go of the stream
//! closes the session. The other backends need a Host or a Cluster this
//! test can't have: their sessions are held to failing as the use case
//! says, and to closing nothing that is not theirs.

use std::{sync::Arc, time::Duration};

use iced::futures::{Stream, StreamExt};

use homeport_containerd::containers::exec::ContainerdExecOptions;
use homeport_core::{error::ErrorCode, terminal::ExecEvent};
use homeport_engine::containers::exec::ExecOptions;
use homeport_incus::instances::terminal::IncusExecOptions;
use homeport_kubernetes::streams::terminal::PodExecOptions;
use homeport_nomad::allocations::exec::NomadExecOptions;

use super::{output, Backend, Exit, Kind, Opened, Output, SessionUpdate, TerminalSession, Written};
use crate::{
    app::{testing, AppState},
    data::streams::Update,
    test_support::engine::{Engine, Stream as Open},
};

fn state() -> (Arc<AppState>, impl Sized) {
    let testing::TestApp { state, events, dir } = testing::app();
    (Arc::new(state), (events, dir))
}

/// A shell in the container.
fn shell(host_id: &str) -> Backend {
    Backend::Engine {
        host_id: host_id.to_owned(),
        container_id: "c1".to_owned(),
        options: ExecOptions::default(),
    }
}

/// A session of `backend` for a terminal of 100 by 30.
fn session(backend: Backend) -> TerminalSession {
    TerminalSession::new(backend, (100, 30))
}

/// What `session`'s stream says, without the mark of the session.
fn updates(
    session: &TerminalSession,
    state: &Arc<AppState>,
) -> impl Stream<Item = Update<ExecEvent>> {
    session.opened(state.clone()).map(|said| said.update)
}

/// The sizes the fake's terminal was told, in order.
fn sizes(engine: &Engine) -> Vec<String> {
    engine
        .terminal()
        .into_iter()
        .filter(|told| !told.starts_with("input "))
        .collect()
}

/// Reads the stream until `done` says what came is enough, for two
/// seconds at most; what came, as one output.
async fn read_until(
    stream: &mut (impl Stream<Item = Update<ExecEvent>> + Unpin),
    done: impl Fn(&Output) -> bool,
) -> Output {
    let mut all = Output::default();
    let reading = async {
        while let Some(update) = stream.next().await {
            if let Update::Batch { items, .. } = update {
                let batch = output(items);
                all.text.push_str(&batch.text);
                all.exit = all.exit.take().or(batch.exit);
                all.error = all.error.take().or(batch.error);
            }
            if done(&all) {
                break;
            }
        }
    };
    let _ = tokio::time::timeout(Duration::from_secs(2), reading).await;
    all
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_shell_in_a_container_takes_keys_in_order_is_resized_and_says_its_end() {
    let (state, _app) = state();
    let engine = Engine::start();
    let host_id = engine.host(&state).await;
    let mut session = session(shell(&host_id));
    let mut stream = Box::pin(updates(&session, &state));
    // Typed and resized before the session started: kept, the last size only.
    session.input("ec".to_owned());
    session.resize(90, 20);
    session.resize(120, 40);
    let Some(Update::Started(id)) = stream.next().await else {
        panic!("the terminal started");
    };
    assert_eq!(session.id(), None);
    let writer = tokio::spawn(session.started(state.clone(), id));
    assert_eq!(session.id(), Some(id));
    // The exec was asked for a terminal of the first size, with a shell.
    let created = engine.execs();
    assert_eq!(created.len(), 1);
    assert!(created[0].contains(r#""Tty":true"#), "{}", created[0]);
    assert!(
        created[0].contains(r#""ConsoleSize":[30,100]"#),
        "{}",
        created[0]
    );
    for key in ["ho", " hi", "\r"] {
        session.input(key.to_owned());
    }
    // The fake's line discipline echoes what came, in the order it came.
    let echoed = read_until(&mut stream, |out| out.text.contains("echo hi\r")).await;
    assert!(echoed.text.contains("echo hi\r"), "{echoed:?}");
    // Every key, in the order typed, however many writes they took.
    assert_eq!(engine.typed(), "echo hi\r");
    // Of the sizes asked for before the start, the last only. (The keys
    // and the sizes go to the Engine by two ways, its stream and its API,
    // so the fake may hear them in either order.)
    let sizes = sizes(&engine);
    assert_eq!(sizes.len(), 1, "{sizes:?}");
    assert!(
        sizes[0].contains("h=40") && sizes[0].contains("w=120"),
        "{sizes:?}"
    );
    engine.print("hi\r\n$ ");
    let printed = read_until(&mut stream, |out| out.text.contains("hi\r\n$ ")).await;
    assert!(printed.text.contains("hi\r\n$ "));
    // The shell exits: the end says its code, and the stream ends.
    engine.exit();
    let ended = read_until(&mut stream, |out| out.exit.is_some()).await;
    assert_eq!(
        ended.exit,
        Some(Exit {
            code: Some(0),
            error: None
        })
    );
    session.ended();
    let written = tokio::time::timeout(Duration::from_secs(2), writer).await;
    assert!(
        matches!(&written, Ok(Ok(Written { result: Ok(()), .. }))),
        "the writer stops: {written:?}"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn letting_go_of_the_stream_closes_the_session_and_its_writer_says_why() {
    let (state, _app) = state();
    let engine = Engine::start();
    let host_id = engine.host(&state).await;
    let mut stream = Box::pin(updates(&session(shell(&host_id)), &state));
    let Some(Update::Started(id)) = stream.next().await else {
        panic!("the terminal started");
    };
    assert!(engine.settles_at(Open::Exec, 1).await);
    drop(stream);
    assert!(
        engine.settles_at(Open::Exec, 0).await,
        "the Engine's side is let go of"
    );
    // What is typed after goes nowhere: the writer says why, once...
    let mut session = session(shell(&host_id));
    session.input("ls\r".to_owned());
    let refused = session.started(state.clone(), id).await;
    assert_eq!(
        refused.result.clone().map_err(|e| e.code),
        Err(ErrorCode::NotFound)
    );
    let shown = session.written(refused).map(|e| e.code);
    assert_eq!(shown, Some(ErrorCode::NotFound));
    assert_eq!(session.failure().map(|e| e.code), Some(ErrorCode::NotFound));
    // ...and what is sent after is let go of.
    session.input("pwd\r".to_owned());
    session.resize(80, 24);
    assert_eq!(engine.typed(), "");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_session_is_told_its_size_when_it_starts_and_says_how_its_program_ended() {
    let (state, _app) = state();
    let engine = Engine::start();
    let host_id = engine.host(&state).await;
    engine.exit_with(127);
    // A terminal that kept its size from a session before: nothing new is
    // resized, yet the program is told it.
    let mut session = session(shell(&host_id));
    let mut stream = Box::pin(updates(&session, &state));
    let Some(Update::Started(id)) = stream.next().await else {
        panic!("the terminal started");
    };
    let writer = tokio::spawn(session.started(state.clone(), id));
    let created = engine.execs();
    assert!(
        created[0].contains(r#""ConsoleSize":[30,100]"#),
        "{}",
        created[0]
    );
    for _ in 0..100 {
        if !engine.terminal().is_empty() {
            break;
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    let told = engine.terminal();
    assert!(
        told[0].contains("h=30") && told[0].contains("w=100"),
        "{told:?}"
    );
    // A command not found: 127, and why.
    engine.exit();
    let ended = read_until(&mut stream, |out| out.exit.is_some()).await;
    let exit = ended.exit.expect("an end");
    assert_eq!(exit.code, Some(127));
    assert!(exit.error.is_some(), "127 says the command could not run");
    session.ended();
    let _ = tokio::time::timeout(Duration::from_secs(2), writer).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_session_opens_its_terminal_at_the_size_it_was_made_with() {
    let (state, _app) = state();
    let engine = Engine::start();
    let host_id = engine.host(&state).await;
    // Options left at their default: no size of their own.
    let backend = Backend::Engine {
        host_id,
        container_id: "c1".to_owned(),
        options: ExecOptions::default(),
    };
    let mut session = TerminalSession::new(backend, (132, 43));
    let mut stream = Box::pin(updates(&session, &state));
    let Some(Update::Started(id)) = stream.next().await else {
        panic!("the terminal started");
    };
    let writer = tokio::spawn(session.started(state.clone(), id));
    let created = engine.execs();
    assert!(
        created[0].contains(r#""ConsoleSize":[43,132]"#),
        "{}",
        created[0]
    );
    engine.exit();
    let _ = read_until(&mut stream, |out| out.exit.is_some()).await;
    session.ended();
    let _ = tokio::time::timeout(Duration::from_secs(2), writer).await;
}

/// Waits up to two seconds for `typed` to have reached the fake.
async fn typed_reaches(engine: &Engine, typed: &str) -> bool {
    for _ in 0..100 {
        if engine.typed().contains(typed) {
            return true;
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    false
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn keys_typed_after_a_resize_reach_the_program_after_it() {
    let (state, _app) = state();
    let engine = Engine::start();
    let host_id = engine.host(&state).await;
    let mut session = session(shell(&host_id));
    let mut stream = Box::pin(updates(&session, &state));
    let Some(Update::Started(id)) = stream.next().await else {
        panic!("the terminal started");
    };
    let writer = tokio::spawn(session.started(state.clone(), id));
    // The writer waits for the Engine's answer to a resize before it
    // writes what was typed after.
    session.resize(80, 24);
    session.input("x".to_owned());
    assert!(typed_reaches(&engine, "x").await);
    let told = engine.terminal();
    let resized = told
        .iter()
        .position(|told| told.contains("h=24") && told.contains("w=80"))
        .expect("the resize reached it");
    let typed = told
        .iter()
        .position(|told| told == "input x")
        .expect("the key reached it");
    assert!(resized < typed, "{told:?}");
    engine.exit();
    session.ended();
    let _ = tokio::time::timeout(Duration::from_secs(2), writer).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_late_word_from_the_session_before_does_not_silence_the_next() {
    let (state, _app) = state();
    let engine = Engine::start();
    let host_id = engine.host(&state).await;
    let mut first = session(shell(&host_id));
    let mut stream = Box::pin(updates(&first, &state));
    let Some(Update::Started(id)) = stream.next().await else {
        panic!("the terminal started");
    };
    let first_writes = first.started(state.clone(), id);
    // The page opens a terminal again, on a new session...
    let mut next = session(shell(&host_id));
    let mut again = Box::pin(updates(&next, &state));
    let Some(Update::Started(id)) = again.next().await else {
        panic!("the terminal started again");
    };
    let writer = tokio::spawn(next.started(state.clone(), id));
    // ...and the first one's writer ends after: its word comes late.
    drop(first);
    drop(stream);
    let late = first_writes.await;
    assert_eq!(
        next.written(late),
        None,
        "an old writer's word says nothing"
    );
    next.input("ls\r".to_owned());
    assert!(typed_reaches(&engine, "ls\r").await, "the keys still go");
    engine.exit();
    next.ended();
    let _ = tokio::time::timeout(Duration::from_secs(2), writer).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_session_started_again_hears_its_own_writer_only() {
    let (state, _app) = state();
    let engine = Engine::start();
    let host_id = engine.host(&state).await;
    let mut session = session(shell(&host_id));
    let mut stream = Box::pin(updates(&session, &state));
    let Some(Update::Started(id)) = stream.next().await else {
        panic!("the terminal started");
    };
    let old_writes = session.started(state.clone(), id);
    // Started again (its subscription ran again): the old writer ends.
    let writer = tokio::spawn(session.started(state.clone(), id));
    let late = old_writes.await;
    assert_eq!(
        session.written(late),
        None,
        "an old writer's word says nothing"
    );
    session.input("pwd\r".to_owned());
    assert!(typed_reaches(&engine, "pwd\r").await, "the keys still go");
    engine.exit();
    session.ended();
    let _ = tokio::time::timeout(Duration::from_secs(2), writer).await;
}

#[test]
fn a_backend_takes_the_terminal_s_size_and_never_none() {
    let backend = Backend::Engine {
        host_id: "h".to_owned(),
        container_id: "c".to_owned(),
        options: ExecOptions::default(),
    };
    assert_eq!(backend.size(), (1, 1), "0 by 0 is asked as 1 by 1 at least");
    assert_eq!(backend.sized(132, 43).size(), (132, 43));
    let console = Backend::IncusConsole {
        host_id: "h".to_owned(),
        project: "p".to_owned(),
        name: "n".to_owned(),
        cols: 80,
        rows: 24,
    };
    assert_eq!(console.sized(0, 50).size(), (1, 50));
}

#[tokio::test]
async fn every_backend_without_its_host_or_cluster_fails_to_start() {
    let (state, _app) = state();
    let host = || "no-such-host".to_owned();
    let backends = [
        shell("no-such-host"),
        Backend::Containerd {
            host_id: host(),
            namespace: "default".to_owned(),
            id: "web".to_owned(),
            options: ContainerdExecOptions::default(),
        },
        Backend::IncusExec {
            host_id: host(),
            project: "default".to_owned(),
            name: "web".to_owned(),
            options: IncusExecOptions {
                command: None,
                user: None,
                cwd: None,
                cols: 80,
                rows: 24,
            },
        },
        Backend::IncusConsole {
            host_id: host(),
            project: "default".to_owned(),
            name: "web".to_owned(),
            cols: 80,
            rows: 24,
        },
        Backend::Pod {
            cluster_id: "no-such-cluster".to_owned(),
            namespace: "default".to_owned(),
            pod: "web-0".to_owned(),
            options: PodExecOptions::default(),
        },
        Backend::Nomad {
            cluster_id: "no-such-cluster".to_owned(),
            alloc_id: "a1".to_owned(),
            options: NomadExecOptions {
                task: "web".to_owned(),
                command: None,
                cols: 80,
                rows: 24,
            },
        },
    ];
    let kinds: Vec<Kind> = backends.iter().map(Backend::kind).collect();
    assert_eq!(
        kinds,
        [
            Kind::Engine,
            Kind::Containerd,
            Kind::Incus,
            Kind::Incus,
            Kind::Pod,
            Kind::Nomad
        ]
    );
    for backend in backends {
        let mut session = session(backend.clone());
        let said: Vec<_> = updates(&session, &state).collect().await;
        assert!(
            matches!(said.as_slice(), [Update::Failed(_)]),
            "{backend:?}: {said:?}"
        );
        // A session of that kind that ended takes no input.
        session.input("x".to_owned());
        let refused = session.started(state.clone(), 7).await;
        assert!(refused.result.is_err(), "{backend:?}");
    }
}

#[test]
fn what_is_sent_to_a_session_that_ended_is_let_go_of_quietly() {
    let mut session = session(shell("h"));
    session.ended();
    session.input("x".to_owned());
    session.resize(80, 24);
    assert_eq!(session.failure(), None, "an end is not a failure");
}

#[test]
fn what_a_session_before_said_late_is_let_go_of() {
    let (first, next) = (session(shell("h")), session(shell("h")));
    let late = SessionUpdate {
        key: first.key,
        update: Update::Ended,
    };
    assert_eq!(next.take(late.clone()), None, "not the session's own");
    assert_eq!(first.take(late), Some(Update::Ended));
}

#[test]
fn a_batch_is_its_output_in_order_with_its_end() {
    let batch = output(vec![
        ExecEvent::Output { data: "a".into() },
        ExecEvent::Output { data: "b".into() },
        ExecEvent::Exit {
            code: Some(127),
            error: None,
        },
    ]);
    assert_eq!(batch.text, "ab");
    assert_eq!(batch.exit.map(|exit| exit.code), Some(Some(127)));
    assert_eq!(batch.error, None);
}

#[test]
fn two_terminals_on_one_container_are_two_subscriptions() {
    use std::hash::{BuildHasher, RandomState};
    let hasher = RandomState::new();
    let (one, two) = (session(shell("h1")), session(shell("h1")));
    assert_ne!(one.key, two.key, "each session its own");
    let opened = |key| Opened {
        key,
        backend: shell("h1"),
    };
    assert_eq!(hasher.hash_one(opened(1)), hasher.hash_one(opened(1)));
    assert_ne!(hasher.hash_one(opened(1)), hasher.hash_one(opened(2)));
    let other = Opened {
        key: 1,
        backend: Backend::Engine {
            host_id: "h1".to_owned(),
            container_id: "c2".to_owned(),
            options: ExecOptions::default(),
        },
    };
    assert_ne!(hasher.hash_one(opened(1)), hasher.hash_one(other));
}

/// A page with a terminal, its code as the module's docs write it (the
/// window's context aside: what is copied or opened is let go of).
mod page {
    use std::sync::Arc;

    use homeport_core::error::AppError;
    use iced::Task;

    use crate::{
        app::AppState,
        data::streams::{
            session::{self, Backend, SessionUpdate, TerminalSession, Written},
            Update,
        },
        ui::widgets::terminal,
    };

    #[derive(Debug, Clone)]
    pub(super) enum Message {
        Session(SessionUpdate),
        Written(Written),
        Terminal(terminal::Message),
    }

    pub(super) struct Page {
        pub(super) terminal: terminal::State,
        pub(super) session: TerminalSession,
        pub(super) failure: Option<AppError>,
        state: Arc<AppState>,
    }

    impl Page {
        pub(super) fn new(state: Arc<AppState>, backend: Backend) -> Self {
            let terminal = terminal::State::new();
            Self {
                session: TerminalSession::new(backend, terminal.size()),
                terminal,
                failure: None,
                state,
            }
        }

        pub(super) fn update(&mut self, message: Message) -> Task<Message> {
            match message {
                Message::Session(said) => {
                    let Some(update) = self.session.take(said) else {
                        return Task::none();
                    };
                    match update {
                        Update::Started(id) => {
                            let writes = self.session.started(self.state.clone(), id);
                            return Task::perform(writes, Message::Written);
                        }
                        Update::Batch { items, .. } => {
                            let output = session::output(items);
                            let events = self.terminal.feed(&output.text);
                            if let Some(error) = output.error {
                                self.failure = Some(error);
                            }
                            if let Some(exit) = output.exit {
                                self.session.ended();
                                self.terminal.notice(&format!("exited: {:?}", exit.code));
                            }
                            return self.told(events);
                        }
                        Update::Failed(error) => {
                            self.session.ended();
                            self.failure = Some(error);
                        }
                        Update::Ended => self.session.ended(),
                    }
                    Task::none()
                }
                Message::Written(written) => {
                    if let Some(error) = self.session.written(written) {
                        self.failure = Some(error.clone());
                    }
                    Task::none()
                }
                Message::Terminal(message) => {
                    let (task, events) = self.terminal.update(message);
                    Task::batch([task.map(Message::Terminal), self.told(events)])
                }
            }
        }

        fn told(&mut self, events: Vec<terminal::Event>) -> Task<Message> {
            for event in events {
                match event {
                    terminal::Event::Input(bytes) => self.session.input(bytes),
                    terminal::Event::Resize { cols, rows } => self.session.resize(cols, rows),
                    terminal::Event::Copy(_)
                    | terminal::Event::Open(_)
                    | terminal::Event::Title(_)
                    | terminal::Event::Bell => {}
                }
            }
            Task::none()
        }
    }
}

/// Runs what `update` returned on the runtime: its messages come later.
fn spawned(task: iced::Task<page::Message>) -> tokio::task::JoinHandle<Vec<page::Message>> {
    tokio::spawn(crate::test_support::tasks::messages(task))
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn the_documented_page_runs_a_shell_and_opens_it_again() {
    use crate::ui::widgets::terminal::Message as Typed;
    let (state, _app) = state();
    let engine = Engine::start();
    let host_id = engine.host(&state).await;
    let mut page = page::Page::new(state.clone(), shell(&host_id));
    let mut stream = Box::pin(page.session.opened(state.clone()));
    // Started: the writer runs until the session ends.
    let said = stream.next().await.expect("the terminal started");
    let writer = spawned(page.update(page::Message::Session(said)));
    let _ = page.update(page::Message::Terminal(Typed::Input("ls\r".to_owned())));
    assert!(typed_reaches(&engine, "ls\r").await);
    engine.print("README.md\r\n$ ");
    engine.exit();
    while let Some(said) = stream.next().await {
        let _ = page.update(page::Message::Session(said));
    }
    // The shell exited: the writer ends, and that is no failure.
    for message in writer.await.unwrap() {
        let _ = page.update(message);
    }
    assert!(page.failure.is_none(), "{:?}", page.failure);
    // Opened again, on a new session; the old one's words come late.
    let old = std::mem::replace(
        &mut page.session,
        TerminalSession::new(shell(&host_id), page.terminal.size()),
    );
    let late = SessionUpdate {
        key: old.key,
        update: Update::Ended,
    };
    let mut again = Box::pin(page.session.opened(state.clone()));
    let said = again.next().await.expect("the terminal started again");
    let writer = spawned(page.update(page::Message::Session(said)));
    let _ = page.update(page::Message::Session(late));
    let _ = page.update(page::Message::Terminal(Typed::Input("pwd\r".to_owned())));
    assert!(
        typed_reaches(&engine, "pwd\r").await,
        "the new session takes keys"
    );
    engine.exit();
    while let Some(said) = again.next().await {
        let _ = page.update(page::Message::Session(said));
    }
    let _ = tokio::time::timeout(Duration::from_secs(2), writer).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_failure_the_user_closed_stays_closed_when_an_old_writer_ends() {
    use homeport_core::error::AppError;
    let (state, _app) = state();
    let mut page = page::Page::new(state.clone(), shell("h"));
    // Started, then started again (its subscription ran again): the first
    // writer is an old one, whose word comes late.
    drop(page.session.started(state.clone(), 1));
    let old = page.session.writer;
    drop(page.session.started(state.clone(), 1));
    let current = page.session.writer;
    let refused = AppError::new(ErrorCode::NotFound, homeport_core::messages::IO_FAILED);
    let failed = Written {
        writer: current,
        result: Err(refused),
    };
    let _ = page.update(page::Message::Written(failed));
    assert_eq!(
        page.failure.as_ref().map(|e| e.code),
        Some(ErrorCode::NotFound)
    );
    // The user closes it; the old writer's end says nothing of it.
    page.failure = None;
    let late = Written {
        writer: old,
        result: Ok(()),
    };
    let _ = page.update(page::Message::Written(late));
    assert!(page.failure.is_none(), "{:?}", page.failure);
}
