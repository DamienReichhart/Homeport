//! The shapes the backend's streams come in, each with what it does when
//! the window is behind or gone:
//!
//! | Shape | Examples | Behind | Page closed |
//! | --- | --- | --- | --- |
//! | [`lines`] | container, Pod, task and allocation logs | oldest lines dropped past [`LINES_BACKLOG`] | cancelled |
//! | [`view`] | stats, usage, metrics, watches | oldest dropped past [`VIEW_BACKLOG`] | cancelled |
//! | [`job`] | pull, push, build, save, load, import, export, deploy, backup, restore | everything kept | **goes on** |
//! | [`terminal`] | exec, consoles | oldest output dropped past [`TERMINAL_BACKLOG`] | session closed |
//!
//! Each takes the `*_subscribe` (or `*_start`) use case to run, given the
//! [`Sink`] as its callback. The Engine's own first users are here too:
//! [`logs`], [`stats`], [`usage`], [`pull`] and [`exec`].
//!
//! A job says how it ended in its last item (`Done`, `Error`), which comes
//! in the last batch before [`Update::Ended`]; nothing of a job is dropped.

use std::{future::Future, sync::Arc};

use iced::futures::Stream;

use homeport_core::{
    error::{AppError, AppResult},
    logs::{LogEvent, LogLine},
};
use homeport_engine::{
    containers::{
        exec::{ExecEvent, ExecOptions},
        logs::{LogOptions, LogSource},
        stats::{ContainerUsage, StatsSample},
    },
    images::PullEvent,
};

use super::{open, Backlog, Options, Sink, Stop, Update};
use crate::{
    api::{
        app::streams::stream_cancel,
        engine::{
            containers::{logs_subscribe, stats_subscribe},
            exec::{exec_close, exec_start},
            images::images_pull_subscribe,
            system::usage_subscribe,
        },
    },
    app::AppState,
};

/// How many log lines may wait for the window: more than a log view keeps,
/// so nothing it would show is dropped.
pub const LINES_BACKLOG: usize = 50_000;

/// How many samples or changes may wait for the window.
pub const VIEW_BACKLOG: usize = 1_024;

/// How many bytes of a terminal's output may wait for the window.
pub const TERMINAL_BACKLOG: usize = 16 * 1024 * 1024;

/// A stream a page shows and nothing else needs: samples, usage, metrics,
/// the changes of a watch. Cancelled when the page lets go of it.
pub fn view<T, F, Fut>(state: Arc<AppState>, start: F) -> impl Stream<Item = Update<T>> + Send
where
    T: Send + 'static,
    F: FnOnce(Arc<AppState>, Sink<T>) -> Fut + Send + 'static,
    Fut: Future<Output = AppResult<u32>> + Send + 'static,
{
    let backlog = Backlog {
        max: VIEW_BACKLOG,
        weight: |_| 1,
    };
    open(state, viewed(backlog, stream_cancel), start)
}

/// A stream of log lines. Cancelled when the page lets go of it; behind, it
/// drops the oldest lines and says how many.
pub fn lines<F, Fut>(state: Arc<AppState>, start: F) -> impl Stream<Item = Update<LogEvent>> + Send
where
    F: FnOnce(Arc<AppState>, Sink<LogEvent>) -> Fut + Send + 'static,
    Fut: Future<Output = AppResult<u32>> + Send + 'static,
{
    let backlog = Backlog {
        max: LINES_BACKLOG,
        weight: |event| match event {
            LogEvent::Lines { lines } => lines.len(),
            // An error or the end is never worth dropping lines for.
            LogEvent::Error { .. } | LogEvent::End => 0,
        },
    };
    open(state, viewed(backlog, stream_cancel), start)
}

/// A job with its progress: a pull, a push, a build, a save, a load, an
/// import, an export, a deploy, a backup. Every item is kept, and the job
/// goes on when its page closes: only `stream_cancel` with the id of
/// [`Update::Started`] stops it.
pub fn job<T, F, Fut>(state: Arc<AppState>, start: F) -> impl Stream<Item = Update<T>> + Send
where
    T: Send + 'static,
    F: FnOnce(Arc<AppState>, Sink<T>) -> Fut + Send + 'static,
    Fut: Future<Output = AppResult<u32>> + Send + 'static,
{
    let options = Options {
        backlog: None,
        stop: None,
    };
    open(state, options, start)
}

/// A terminal: `start` opens the session ([`Update::Started`] has its id,
/// for the `*_input` and `*_resize` use cases) and `close` ends it when the
/// page lets go of the stream.
pub fn terminal<F, Fut>(
    state: Arc<AppState>,
    close: Stop,
    start: F,
) -> impl Stream<Item = Update<ExecEvent>> + Send
where
    F: FnOnce(Arc<AppState>, Sink<ExecEvent>) -> Fut + Send + 'static,
    Fut: Future<Output = AppResult<u32>> + Send + 'static,
{
    let backlog = Backlog {
        max: TERMINAL_BACKLOG,
        weight: |event| match event {
            ExecEvent::Output { data } => data.len(),
            ExecEvent::Exit { .. } | ExecEvent::Error { .. } => 0,
        },
    };
    open(state, viewed(backlog, close), start)
}

fn viewed<T>(backlog: Backlog<T>, stop: Stop) -> Options<T> {
    Options {
        backlog: Some(backlog),
        stop: Some(stop),
    }
}

/// What a batch of a log stream holds, ready for a log view.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Lines {
    pub lines: Vec<LogLine>,
    /// Why the stream broke.
    pub error: Option<AppError>,
    /// The source said it has no more.
    pub ended: bool,
}

/// The lines of a batch of [`lines`], in order.
pub fn log_lines(items: Vec<LogEvent>) -> Lines {
    let mut batch = Lines::default();
    for event in items {
        match event {
            LogEvent::Lines { lines } => batch.lines.extend(lines),
            LogEvent::Error { error } => batch.error = Some(error),
            LogEvent::End => batch.ended = true,
        }
    }
    batch
}

/// The logs of containers of a Host, merged.
pub fn logs(
    state: Arc<AppState>,
    host_id: String,
    sources: Vec<LogSource>,
    options: LogOptions,
) -> impl Stream<Item = Update<LogEvent>> + Send {
    lines(state, move |state, sink| async move {
        logs_subscribe(&state, host_id, sources, options, move |event| {
            sink.send(event)
        })
        .await
    })
}

/// A container's CPU, memory, network and disk use, about once a second.
pub fn stats(
    state: Arc<AppState>,
    host_id: String,
    container_id: String,
) -> impl Stream<Item = Update<StatsSample>> + Send {
    view(state, move |state, sink| async move {
        stats_subscribe(&state, host_id, container_id, move |sample| {
            sink.send(sample)
        })
        .await
    })
}

/// The use of every running container of a Host, every two seconds.
pub fn usage(
    state: Arc<AppState>,
    host_id: String,
) -> impl Stream<Item = Update<Vec<ContainerUsage>>> + Send {
    view(state, move |state, sink| async move {
        usage_subscribe(&state, host_id, move |usage| sink.send(usage)).await
    })
}

/// Pulls an image on a Host, with its progress; the last item is `Done` or
/// the error.
pub fn pull(
    state: Arc<AppState>,
    host_id: String,
    reference: String,
) -> impl Stream<Item = Update<PullEvent>> + Send {
    job(state, move |state, sink| async move {
        images_pull_subscribe(&state, host_id, reference, move |event| sink.send(event)).await
    })
}

/// A shell in a container. Its input and resizes go through `exec_input`
/// and `exec_resize` with the session of [`Update::Started`].
pub fn exec(
    state: Arc<AppState>,
    host_id: String,
    container_id: String,
    options: ExecOptions,
) -> impl Stream<Item = Update<ExecEvent>> + Send {
    terminal(state, exec_close, move |state, sink| async move {
        exec_start(&state, host_id, container_id, options, move |event| {
            sink.send(event)
        })
        .await
    })
}
