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

use super::{output, Backend, Exit, Kind, Opened, Output, TerminalSession};
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
    TerminalSession::new(backend, 100, 30)
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
    let mut stream = Box::pin(session.opened(state.clone()));
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
        matches!(written, Ok(Ok(Ok(())))),
        "the writer stops: {written:?}"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn letting_go_of_the_stream_closes_the_session_and_its_writer_says_why() {
    let (state, _app) = state();
    let engine = Engine::start();
    let host_id = engine.host(&state).await;
    let mut stream = Box::pin(session(shell(&host_id)).opened(state.clone()));
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
        refused.clone().map_err(|e| e.code),
        Err(ErrorCode::NotFound)
    );
    session.written(refused);
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
    let mut stream = Box::pin(session.opened(state.clone()));
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
    let mut session = TerminalSession::new(backend, 132, 43);
    let mut stream = Box::pin(session.opened(state.clone()));
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
        let updates: Vec<_> = session.opened(state.clone()).collect().await;
        assert!(
            matches!(updates.as_slice(), [Update::Failed(_)]),
            "{backend:?}: {updates:?}"
        );
        // A session of that kind that ended takes no input.
        session.input("x".to_owned());
        let refused = session.started(state.clone(), 7).await;
        assert!(refused.is_err(), "{backend:?}");
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
    let opened = |number| Opened {
        number,
        backend: shell("h1"),
    };
    assert_eq!(hasher.hash_one(opened(1)), hasher.hash_one(opened(1)));
    assert_ne!(hasher.hash_one(opened(1)), hasher.hash_one(opened(2)));
    let other = Opened {
        number: 1,
        backend: Backend::Engine {
            host_id: "h1".to_owned(),
            container_id: "c2".to_owned(),
            options: ExecOptions::default(),
        },
    };
    assert_ne!(hasher.hash_one(opened(1)), hasher.hash_one(other));
}
