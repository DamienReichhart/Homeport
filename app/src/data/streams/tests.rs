//! Streams from a backend that is a task of the test, on a paused clock;
//! then one from a real Engine on this machine's socket, when there is
//! one; and the soak test of a source logging 50,000 lines a second.

use std::{
    collections::VecDeque,
    path::Path,
    sync::{
        atomic::{AtomicBool, AtomicU32, Ordering},
        Arc,
    },
    time::Duration,
};

use iced::futures::{pin_mut, Stream, StreamExt};

use homeport_core::{
    error::ErrorCode,
    logs::{LogEvent, LogLine, LogOptions},
};
use homeport_engine::containers::{exec::ExecOptions, logs::LogSource, ContainerAction};
use homeport_hosts::{service::HostInput, EngineKind, HostConnection};

use super::{
    shapes::{self, log_lines, Lines},
    Sink, Update, FRAME, UPDATES_PER_SECOND,
};
use crate::{
    api::{
        app::{lifecycle::shutdown, streams::stream_cancel},
        engine::containers::containers_act,
        hosts::hosts_create,
    },
    app::{testing, AppState},
};

fn state() -> (Arc<AppState>, impl Sized) {
    let testing::TestApp { state, events, dir } = testing::app();
    (Arc::new(state), (events, dir))
}

/// A backend stream that sends `count` numbers, one every `every`, then
/// ends; `stopped` is set if it is told to stop or dropped before.
async fn counting(
    state: Arc<AppState>,
    sink: Sink<u32>,
    count: u32,
    every: Duration,
    stopped: Arc<AtomicBool>,
) -> homeport_core::error::AppResult<u32> {
    Ok(state.streams.spawn(async move {
        let mut stopped = Stopped(stopped, true);
        for n in 0..count {
            if !sink.send(n) {
                return;
            }
            tokio::time::sleep(every).await;
        }
        stopped.1 = false;
    }))
}

/// Sets its flag when dropped while its second field is still `true`.
struct Stopped(Arc<AtomicBool>, bool);

impl Drop for Stopped {
    fn drop(&mut self) {
        if self.1 {
            self.0.store(true, Ordering::SeqCst);
        }
    }
}

/// Waits, on the real or the paused clock, until `done` or two seconds.
async fn until(done: impl Fn() -> bool) -> bool {
    for _ in 0..200 {
        if done() {
            return true;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    done()
}

fn items<T: Clone>(updates: &[Update<T>]) -> Vec<T> {
    updates
        .iter()
        .flat_map(|update| match update {
            Update::Batch { items, .. } => items.clone(),
            _ => vec![],
        })
        .collect()
}

#[tokio::test(start_paused = true)]
async fn a_stream_starts_gives_its_items_in_batches_and_ends() {
    let (state, _app) = state();
    let stopped = Arc::new(AtomicBool::new(false));
    let flag = stopped.clone();
    let stream = shapes::view(state.clone(), move |state, sink| {
        counting(state, sink, 5, Duration::from_millis(100), flag)
    });
    // Nothing starts before the window asks.
    assert_eq!(state.streams.running(), 0);

    let updates: Vec<Update<u32>> = stream.collect().await;
    assert!(matches!(updates[0], Update::Started(_)));
    assert_eq!(updates.last(), Some(&Update::Ended));
    assert_eq!(items(&updates), [0, 1, 2, 3, 4]);
    // It ended by itself: nothing was stopped, nothing is left running.
    assert!(!stopped.load(Ordering::SeqCst));
    assert_eq!(state.streams.running(), 0);
}

#[tokio::test(start_paused = true)]
async fn a_burst_is_coalesced_to_sixty_updates_a_second() {
    let (state, _app) = state();
    let flag = Arc::new(AtomicBool::new(false));
    // 2,000 items, one a millisecond: two seconds of a busy source.
    let stream = shapes::view(state.clone(), move |state, sink| {
        counting(state, sink, 2_000, Duration::from_millis(1), flag)
    });
    let started = tokio::time::Instant::now();
    let mut batches = vec![];
    let mut times = vec![];
    pin_mut!(stream);
    while let Some(update) = stream.next().await {
        if let Update::Batch { items, dropped } = update {
            assert_eq!(dropped, 0);
            times.push(started.elapsed());
            batches.push(items);
        }
    }
    let received: Vec<u32> = batches.concat();
    assert_eq!(received, (0..2_000).collect::<Vec<_>>());
    // Two seconds: at most 120 updates, each a frame after the last.
    assert!(batches.len() <= 2 * UPDATES_PER_SECOND as usize + 1);
    assert!(batches.len() >= 100, "{} batches", batches.len());
    assert!(times.windows(2).all(|pair| pair[1] - pair[0] >= FRAME));
}

#[tokio::test(start_paused = true)]
async fn a_closed_page_leaves_no_stream_running() {
    let (state, _app) = state();
    let stopped = Arc::new(AtomicBool::new(false));
    let flag = stopped.clone();
    {
        let stream = shapes::view(state.clone(), move |state, sink| {
            counting(state, sink, u32::MAX, Duration::from_millis(5), flag)
        });
        pin_mut!(stream);
        assert!(matches!(stream.next().await, Some(Update::Started(_))));
        assert!(matches!(stream.next().await, Some(Update::Batch { .. })));
        assert_eq!(state.streams.running(), 1);
        // The page closes here: the stream is dropped.
    }
    assert_eq!(state.streams.running(), 0);
    assert!(until(|| stopped.load(Ordering::SeqCst)).await);
}

#[tokio::test(start_paused = true)]
async fn a_job_goes_on_without_its_page_until_it_is_cancelled() {
    let (state, _app) = state();
    let stopped = Arc::new(AtomicBool::new(false));
    let flag = stopped.clone();
    let id = {
        let stream = shapes::job(state.clone(), move |state, sink| {
            counting(state, sink, u32::MAX, Duration::from_millis(5), flag)
        });
        pin_mut!(stream);
        let Some(Update::Started(id)) = stream.next().await else {
            panic!("the job should start");
        };
        id
    };
    // The user left the page: the job is still told to go on.
    tokio::time::sleep(Duration::from_secs(1)).await;
    assert_eq!(state.streams.running(), 1);
    assert!(!stopped.load(Ordering::SeqCst));

    stream_cancel(&state, id);
    assert!(until(|| stopped.load(Ordering::SeqCst)).await);
    assert_eq!(state.streams.running(), 0);
}

#[tokio::test(start_paused = true)]
async fn quitting_stops_the_jobs_too() {
    let (state, _app) = state();
    let stopped = Arc::new(AtomicBool::new(false));
    let flag = stopped.clone();
    let stream = shapes::job(state.clone(), move |state, sink| {
        counting(state, sink, u32::MAX, Duration::from_millis(5), flag)
    });
    pin_mut!(stream);
    assert!(matches!(stream.next().await, Some(Update::Started(_))));
    assert!(matches!(stream.next().await, Some(Update::Batch { .. })));

    shutdown(&state).await;
    assert_eq!(state.streams.running(), 0);
    // The window is told the stream is over.
    let rest: Vec<Update<u32>> = stream.collect().await;
    assert_eq!(rest.last(), Some(&Update::Ended));
    assert!(stopped.load(Ordering::SeqCst));
}

static CLOSED: AtomicU32 = AtomicU32::new(0);

#[tokio::test(start_paused = true)]
async fn dropping_a_terminal_closes_its_session() {
    let (state, _app) = state();
    {
        let stream = shapes::terminal(
            state.clone(),
            |_, session| CLOSED.store(session, Ordering::SeqCst),
            |_, sink| async move {
                // The session's output goes on for as long as the sink lives.
                tokio::spawn(async move {
                    let _sink = sink;
                    std::future::pending::<()>().await;
                });
                Ok(41)
            },
        );
        pin_mut!(stream);
        assert_eq!(stream.next().await, Some(Update::Started(41)));
        assert_eq!(CLOSED.load(Ordering::SeqCst), 0);
    }
    assert_eq!(CLOSED.load(Ordering::SeqCst), 41);
}

async fn first<T>(stream: impl Stream<Item = Update<T>>) -> Vec<Update<T>> {
    stream.collect().await
}

#[tokio::test]
async fn a_stream_that_cannot_start_says_why_and_ends() {
    fn codes<T>(updates: Vec<Update<T>>) -> Vec<ErrorCode> {
        updates
            .into_iter()
            .map(|update| match update {
                Update::Failed(error) => error.code,
                _ => panic!("only a failure was expected"),
            })
            .collect()
    }
    let (state, _app) = state();
    let host = || "no-such-host".to_owned();
    let options = LogOptions {
        tail: Some(10),
        timestamps: false,
        since: None,
        stdout: true,
        stderr: true,
        follow: true,
    };
    let refused = |updates: Vec<ErrorCode>| assert_eq!(updates, [ErrorCode::HostNotConnected]);
    refused(codes(
        first(shapes::logs(state.clone(), host(), vec![], options)).await,
    ));
    refused(codes(
        first(shapes::stats(state.clone(), host(), "c1".to_owned())).await,
    ));
    refused(codes(first(shapes::usage(state.clone(), host())).await));
    refused(codes(
        first(shapes::pull(state.clone(), host(), "nginx".to_owned())).await,
    ));
    let shell = ExecOptions::default();
    refused(codes(
        first(shapes::exec(state.clone(), host(), "c1".to_owned(), shell)).await,
    ));
    assert_eq!(state.streams.running(), 0);
}

fn line(text: &str) -> LogLine {
    LogLine {
        source: None,
        stream: "stdout".to_owned(),
        text: text.to_owned(),
    }
}

#[test]
fn a_batch_of_log_events_is_its_lines_in_order() {
    let batch = log_lines(vec![
        LogEvent::Lines {
            lines: vec![line("one"), line("two")],
        },
        LogEvent::Lines {
            lines: vec![line("three")],
        },
        LogEvent::End,
    ]);
    assert_eq!(
        batch,
        Lines {
            lines: vec![line("one"), line("two"), line("three")],
            error: None,
            ended: true,
        }
    );
    assert_eq!(log_lines(vec![]), Lines::default());
}

/// This machine's Engine as a connected Host of `state`; `None`, with a
/// note, where no Engine listens (CI on macOS and Windows).
async fn local_host(state: &Arc<AppState>) -> Option<String> {
    let socket = "/var/run/docker.sock";
    if !Path::new(socket).exists() {
        eprintln!("skipped: no Engine on {socket}");
        return None;
    }
    let input = HostInput {
        name: "local".to_owned(),
        connection: HostConnection::Local {
            socket_path: socket.to_owned(),
        },
        engine: EngineKind::Docker,
        credentials: None,
        bastion_keys: Vec::new(),
    };
    let id = hosts_create(state, input)
        .await
        .expect("a local Host is created")
        .host
        .id;
    for _ in 0..100 {
        if state.hosts.registry.docker(&id).is_ok() {
            return Some(id);
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    eprintln!("skipped: the Engine on {socket} doesn't answer");
    None
}

#[tokio::test(flavor = "multi_thread")]
async fn logs_of_a_real_container_come_in_batches_and_stop_when_dropped() {
    let (state, _app) = state();
    let Some(host_id) = local_host(&state).await else {
        return;
    };
    let docker = state.hosts.registry.docker(&host_id).unwrap();
    let name = homeport_engine::testing::unique("stream");
    let chatty = ["sh", "-c", "while true; do echo line; done"];
    let container = homeport_engine::testing::container(&docker, &name, &chatty).await;
    containers_act(
        &state,
        host_id.clone(),
        container.id.clone(),
        ContainerAction::Start,
    )
    .await
    .unwrap();

    let sources = vec![LogSource {
        container_id: container.id.clone(),
        label: None,
    }];
    let options = LogOptions {
        tail: Some(10),
        timestamps: false,
        since: None,
        stdout: true,
        stderr: true,
        follow: true,
    };
    {
        let stream = shapes::logs(state.clone(), host_id, sources, options);
        pin_mut!(stream);
        assert!(matches!(stream.next().await, Some(Update::Started(_))));
        let started = std::time::Instant::now();
        let mut lines = 0;
        let mut batches = 0;
        while batches < 30 {
            match stream.next().await {
                Some(Update::Batch { items, .. }) => {
                    batches += 1;
                    lines += log_lines(items).lines.len();
                }
                other => panic!("the logs should go on, got {other:?}"),
            }
        }
        assert!(lines >= 30, "{lines} lines");
        // Thirty batches take at least twenty-nine frames.
        assert!(started.elapsed() >= FRAME * 29);
        assert_eq!(state.streams.running(), 1);
    }
    // The page closed: the backend's stream is cancelled.
    assert!(until(|| state.streams.running() == 0).await);
}

/// What a log view keeps, and so what each update costs the window: the
/// batch's lines appended, the oldest dropped past 10,000.
fn keep(buffer: &mut VecDeque<LogLine>, items: Vec<LogEvent>) {
    const KEPT: usize = 10_000;
    buffer.extend(log_lines(items).lines);
    let extra = buffer.len().saturating_sub(KEPT);
    buffer.drain(..extra);
}

/// The soak test of #233: a source logging 50,000 lines a second for ten
/// seconds. Run it with
/// `cargo test -p homeport-app --release soak -- --ignored --nocapture`.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "takes ten seconds; run on demand"]
async fn soak_fifty_thousand_lines_a_second_stay_under_a_frame() {
    const RATE: u64 = 50_000;
    const SECONDS: u64 = 10;
    let (state, _app) = state();
    let sent = Arc::new(std::sync::atomic::AtomicU64::new(0));
    let counter = sent.clone();
    let stream = shapes::lines(state.clone(), move |state, sink| async move {
        Ok(state.streams.spawn(async move {
            let started = std::time::Instant::now();
            let mut tick = tokio::time::interval(Duration::from_millis(1));
            let mut total = 0_u64;
            while started.elapsed() < Duration::from_secs(SECONDS) {
                tick.tick().await;
                // As many lines as the rate asks for by now.
                let due =
                    (started.elapsed().as_micros() as u64 * RATE / 1_000_000).min(RATE * SECONDS);
                let lines = (total..due)
                    .map(|n| line(&format!("2026-11-11T10:00:00Z request {n} served in 3 ms")))
                    .collect();
                total = due;
                counter.store(total, Ordering::SeqCst);
                if !sink.send(LogEvent::Lines { lines }) {
                    return;
                }
            }
        }))
    });

    let mut buffer = VecDeque::new();
    let (mut updates, mut received, mut skipped) = (0_u64, 0_u64, 0_u64);
    let mut worst = Duration::ZERO;
    let mut total = Duration::ZERO;
    pin_mut!(stream);
    while let Some(update) = stream.next().await {
        let Update::Batch { items, dropped } = update else {
            continue;
        };
        updates += 1;
        skipped += dropped;
        received += items
            .iter()
            .map(|event| match event {
                LogEvent::Lines { lines } => lines.len() as u64,
                _ => 0,
            })
            .sum::<u64>();
        // What the window's `update` does with a batch.
        let began = std::time::Instant::now();
        keep(&mut buffer, items);
        let took = began.elapsed();
        worst = worst.max(took);
        total += took;
    }
    let sent = sent.load(Ordering::SeqCst);
    println!(
        "soak: {sent} lines sent, {received} received, {skipped} skipped, {updates} updates \
         ({:.1}/s), update time mean {:?}, worst {worst:?}",
        updates as f64 / SECONDS as f64,
        total / updates.max(1) as u32,
    );
    assert_eq!(sent, RATE * SECONDS);
    assert_eq!(received + skipped, sent);
    assert!(updates <= (SECONDS + 1) * u64::from(UPDATES_PER_SECOND));
    assert!(
        worst < Duration::from_millis(8),
        "worst update took {worst:?}"
    );
    assert_eq!(state.streams.running(), 0);
}
