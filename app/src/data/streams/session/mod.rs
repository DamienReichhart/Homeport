//! A terminal's session, whatever it runs in: one [`TerminalSession`] over
//! the start, the input, the size and the end of each backend's terminals,
//! as `startExec`, `startContainerdExec`, `startIncusExec`,
//! `startIncusConsole`, `startPodExec` and `startNomadExec` were in React's
//! `api/streams.ts`.
//!
//! | [`Backend`] | Start | Input, size | Closed by |
//! | --- | --- | --- | --- |
//! | `Engine` | `exec_start` | `exec_input`, `exec_resize` | `exec_close` |
//! | `Containerd` | `containerd_exec_start` | `containerd_exec_input`, `_resize` | `containerd_exec_close` |
//! | `IncusExec`, `IncusConsole` | `incus_exec_start`, `incus_console_start` | `incus_terminal_input`, `_resize` | `incus_terminal_close` |
//! | `Pod` | `clusters_exec_start` | `clusters_exec_input`, `_resize` | `clusters_exec_close` |
//! | `Nomad` | `clusters_nomad_exec_start` | `clusters_nomad_exec_input`, `_resize` | `clusters_nomad_exec_close` |
//!
//! The output is a [`shapes::terminal`] stream: batched to a frame, its
//! backlog bounded, and the session closed when the stream is dropped
//! (the page closed or let go of it). A page follows it with
//! [`subscription`] and keeps a [`TerminalSession`] for what it sends:
//!
//! ```ignore
//! // in subscription:
//! session::subscription(ctx.state, self.number, self.backend.clone()).map(Message::Session)
//!
//! // in update:
//! Message::Session(Update::Started(id)) => {
//!     let writes = self.session.started(ctx.state.clone(), id);
//!     return Task::perform(writes, Message::Written);
//! }
//! Message::Written(result) => self.session.written(result), // then show `session.failure()`
//! Message::Session(Update::Batch { items, .. }) => {
//!     let output = session::output(items);
//!     for event in self.terminal.feed(&output.text) { /* as below */ }
//! }
//! Message::Terminal(message) => match self.terminal.update(message) {
//!     (_, Some(terminal::Event::Input(bytes))) => self.session.input(bytes),
//!     (_, Some(terminal::Event::Resize { cols, rows })) => self.session.resize(cols, rows),
//!     ..
//! }
//! ```
//!
//! What is typed and the new sizes go in order through one writer, the
//! future [`TerminalSession::started`] gives, so two keys are never
//! reordered and `update` never waits on the backend: [`input`] and
//! [`resize`] only queue. Keys typed before the session started are sent
//! once it does; a size asked for meanwhile replaces the one before it.
//! A backend is opened at the terminal's size ([`Backend::sized`]), and
//! that size is told again when the session starts, so a terminal opened
//! again on a new session is the program's size even though its own did
//! not change. A write that failed ends the writer: the page is told by
//! its task, [`TerminalSession::written`] keeps the failure, and [`input`]
//! and [`resize`] say it again rather than dropping what they were given.
//!
//! [`input`]: TerminalSession::input
//! [`resize`]: TerminalSession::resize

use std::{
    future::Future,
    hash::{Hash, Hasher},
    sync::Arc,
};

use iced::{futures::Stream, Subscription};
use tokio::sync::mpsc;

use homeport_core::error::{AppError, AppResult, ErrorCode};
use homeport_engine::containers::exec::ExecEvent;

pub use self::backend::{Backend, Kind};
use super::{shapes, Source, Update};
use crate::{
    api::{
        clusters::{
            nomad::{
                clusters_nomad_exec_close, clusters_nomad_exec_input, clusters_nomad_exec_resize,
                clusters_nomad_exec_start,
            },
            streams::{
                clusters_exec_close, clusters_exec_input, clusters_exec_resize, clusters_exec_start,
            },
        },
        containerd::{
            containerd_exec_close, containerd_exec_input, containerd_exec_resize,
            containerd_exec_start,
        },
        engine::exec::{exec_close, exec_input, exec_resize, exec_start},
        incus::terminals::{
            incus_console_start, incus_exec_start, incus_terminal_close, incus_terminal_input,
            incus_terminal_resize,
        },
    },
    app::AppState,
};

/// Closes a session whose closing needs the runtime (it hands the end to
/// a task of its own): the window drops a stream on the runtime, and a
/// session dropped elsewhere is only let go of.
fn on_runtime(close: impl FnOnce()) {
    if tokio::runtime::Handle::try_current().is_ok() {
        close();
    } else {
        log::warn!("a terminal was let go of outside the runtime; its session ends with the app");
    }
}

/// The stops of the stores whose `*_close` use case is a future: each
/// does its work before its first await, so it is polled once here.
fn closed<F: Future<Output = AppResult<()>>>(closing: F) {
    let mut closing = std::pin::pin!(closing);
    let waker = std::task::Waker::noop();
    let _ = closing
        .as_mut()
        .poll(&mut std::task::Context::from_waker(waker));
}

fn close_containerd(state: &AppState, id: u32) {
    on_runtime(|| closed(containerd_exec_close(state, id)));
}

fn close_incus(state: &AppState, id: u32) {
    on_runtime(|| closed(incus_terminal_close(state, id)));
}

fn close_nomad(state: &AppState, id: u32) {
    on_runtime(|| closed(clusters_nomad_exec_close(state, id)));
}

/// Opens the terminal `backend` says: [`Update::Started`] with the
/// session's id (for [`TerminalSession::started`]), then its output.
/// Dropping the stream closes the session.
pub fn open(
    state: Arc<AppState>,
    backend: Backend,
) -> impl Stream<Item = Update<ExecEvent>> + Send {
    use iced::futures::StreamExt;
    match backend {
        Backend::Engine {
            host_id,
            container_id,
            options,
        } => shapes::terminal(state, exec_close, move |state, sink| async move {
            exec_start(&state, host_id, container_id, options, move |event| {
                sink.send(event)
            })
            .await
        })
        .boxed(),
        Backend::Containerd {
            host_id,
            namespace,
            id,
            options,
        } => shapes::terminal(state, close_containerd, move |state, sink| async move {
            containerd_exec_start(&state, host_id, namespace, id, options, move |event| {
                sink.send(event)
            })
            .await
        })
        .boxed(),
        Backend::IncusExec {
            host_id,
            project,
            name,
            options,
        } => shapes::terminal(state, close_incus, move |state, sink| async move {
            incus_exec_start(&state, host_id, project, name, options, move |event| {
                sink.send(event)
            })
            .await
        })
        .boxed(),
        Backend::IncusConsole {
            host_id,
            project,
            name,
            cols,
            rows,
        } => shapes::terminal(state, close_incus, move |state, sink| async move {
            incus_console_start(&state, host_id, project, name, cols, rows, move |event| {
                sink.send(event)
            })
            .await
        })
        .boxed(),
        Backend::Pod {
            cluster_id,
            namespace,
            pod,
            options,
        } => shapes::terminal(state, clusters_exec_close, move |state, sink| async move {
            clusters_exec_start(&state, cluster_id, namespace, pod, options, move |event| {
                sink.send(event)
            })
            .await
        })
        .boxed(),
        Backend::Nomad {
            cluster_id,
            alloc_id,
            options,
        } => shapes::terminal(state, close_nomad, move |state, sink| async move {
            clusters_nomad_exec_start(&state, cluster_id, alloc_id, options, move |event| {
                sink.send(event)
            })
            .await
        })
        .boxed(),
    }
}

/// What a page follows a terminal by: its own number for it (two
/// terminals on the same container are two sessions) and the backend.
#[derive(Debug, Clone)]
pub struct Opened {
    pub number: u64,
    pub backend: Backend,
}

impl Hash for Opened {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.number.hash(hasher);
        self.backend.kind().hash(hasher);
        self.backend.identity().hash(hasher);
    }
}

/// The terminal as a subscription: it runs while the page returns it, and
/// its session is closed when the page no longer does.
pub fn subscription(
    state: &Arc<AppState>,
    number: u64,
    backend: Backend,
) -> Subscription<Update<ExecEvent>> {
    super::subscription(
        state,
        Opened { number, backend },
        |source: &Source<Opened>| open(source.state.clone(), source.params.backend.clone()),
    )
}

/// How a session ended, as its last item said.
#[derive(Debug, Clone, PartialEq)]
pub struct Exit {
    pub code: Option<i64>,
    /// Why the command could not run (126, 127: a minimal image).
    pub error: Option<AppError>,
}

/// What a batch of a terminal's stream holds.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Output {
    /// What the program printed, in order: for `terminal::State::feed`.
    pub text: String,
    pub exit: Option<Exit>,
    /// The session broke.
    pub error: Option<AppError>,
}

/// The output of a batch, its end and its failure.
pub fn output(items: Vec<ExecEvent>) -> Output {
    let mut output = Output::default();
    for item in items {
        match item {
            ExecEvent::Output { data } => output.text.push_str(&data),
            ExecEvent::Exit { code, error } => output.exit = Some(Exit { code, error }),
            ExecEvent::Error { error } => output.error = Some(error),
        }
    }
    output
}

/// What is sent to a session, in order.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Op {
    Input(String),
    Resize(u16, u16),
}

/// What a page keeps of a terminal's session: what it sends goes in
/// order, from `update`, without waiting.
#[derive(Debug)]
pub struct TerminalSession {
    kind: Kind,
    id: Option<u32>,
    /// The size the program's terminal is to be: told first when the
    /// session starts, whatever it was asked with.
    size: (u16, u16),
    /// What was asked for before the session started.
    pending: Vec<Op>,
    sender: Option<mpsc::UnboundedSender<Op>>,
    /// Why nothing more can be sent: a write that failed, or the end.
    failure: Option<AppError>,
}

/// What is said of a session that ended when something is sent to it.
fn ended() -> AppError {
    AppError::new(
        ErrorCode::NotFound,
        homeport_engine::messages::TERMINAL_ENDED,
    )
}

impl TerminalSession {
    pub fn new(backend: &Backend) -> Self {
        Self {
            kind: backend.kind(),
            id: None,
            size: backend.size(),
            pending: vec![],
            sender: None,
            failure: None,
        }
    }

    /// The session's id, once it started.
    pub fn id(&self) -> Option<u32> {
        self.id
    }

    /// The session `id` started: its size is told first (a session opened
    /// again on a terminal that kept its size is told it too), then what
    /// was asked for meanwhile, and everything after, in order, by the
    /// future this returns (for `Task::perform`). It ends with the first
    /// write that fails, or once the session ended or this was dropped;
    /// give what it ended with to [`TerminalSession::written`].
    pub fn started(
        &mut self,
        state: Arc<AppState>,
        id: u32,
    ) -> impl Future<Output = AppResult<()>> + Send + 'static {
        let (sender, receiver) = mpsc::unbounded_channel();
        if !self.pending.iter().any(|op| matches!(op, Op::Resize(..))) {
            let (cols, rows) = self.size;
            let _ = sender.send(Op::Resize(cols, rows));
        }
        for op in self.pending.drain(..) {
            let _ = sender.send(op);
        }
        self.id = Some(id);
        self.sender = Some(sender);
        self.failure = None;
        write(state, self.kind, id, receiver)
    }

    /// What the writer of [`TerminalSession::started`] ended with: a
    /// failure is kept, and said again for anything sent after.
    pub fn written(&mut self, result: AppResult<()>) {
        self.sender = None;
        self.failure = Some(result.err().unwrap_or_else(ended));
    }

    /// Why nothing can be sent any more, once that is so.
    pub fn failure(&self) -> Option<&AppError> {
        self.failure.as_ref()
    }

    fn send(&mut self, op: Op) -> AppResult<()> {
        if let Some(failure) = &self.failure {
            return Err(failure.clone());
        }
        match &self.sender {
            Some(sender) => {
                if sender.send(op).is_err() {
                    // The writer stopped: its session ended or a write
                    // failed, which the page is told by its task too.
                    self.sender = None;
                    let failure = ended();
                    self.failure = Some(failure.clone());
                    return Err(failure);
                }
            }
            None => {
                // A new size before the start replaces the one before.
                if let (Op::Resize(..), Some(Op::Resize(..))) = (&op, self.pending.last()) {
                    self.pending.pop();
                }
                self.pending.push(op);
            }
        }
        Ok(())
    }

    /// Bytes for the program: a key, a paste, an answer. A session that
    /// can take no more says why.
    pub fn input(&mut self, data: String) -> AppResult<()> {
        if data.is_empty() {
            return Ok(());
        }
        self.send(Op::Input(data))
    }

    /// The terminal has this many cells now.
    pub fn resize(&mut self, cols: u16, rows: u16) -> AppResult<()> {
        self.size = (cols, rows);
        self.send(Op::Resize(cols, rows))
    }

    /// The session ended: nothing more is sent, and the writer stops.
    pub fn ended(&mut self) {
        self.sender = None;
        self.pending.clear();
        self.failure.get_or_insert_with(ended);
    }
}

async fn send_input(state: &AppState, kind: Kind, id: u32, data: String) -> AppResult<()> {
    match kind {
        Kind::Engine => exec_input(state, id, data).await,
        Kind::Containerd => containerd_exec_input(state, id, data).await,
        Kind::Incus => incus_terminal_input(state, id, data).await,
        Kind::Pod => clusters_exec_input(state, id, data).await,
        Kind::Nomad => clusters_nomad_exec_input(state, id, data).await,
    }
}

async fn send_size(state: &AppState, kind: Kind, id: u32, cols: u16, rows: u16) -> AppResult<()> {
    match kind {
        Kind::Engine => exec_resize(state, id, cols, rows).await,
        Kind::Containerd => containerd_exec_resize(state, id, cols, rows).await,
        Kind::Incus => incus_terminal_resize(state, id, cols, rows).await,
        Kind::Pod => clusters_exec_resize(state, id, cols, rows).await,
        Kind::Nomad => clusters_nomad_exec_resize(state, id, cols, rows).await,
    }
}

/// Sends what comes, in order: keys that came together go as one write,
/// and of sizes that came together only the last is sent.
async fn write(
    state: Arc<AppState>,
    kind: Kind,
    id: u32,
    mut receiver: mpsc::UnboundedReceiver<Op>,
) -> AppResult<()> {
    let mut next = receiver.recv().await;
    while let Some(op) = next.take() {
        match op {
            Op::Input(mut data) => {
                while let Ok(op) = receiver.try_recv() {
                    match op {
                        Op::Input(more) => data.push_str(&more),
                        other => {
                            next = Some(other);
                            break;
                        }
                    }
                }
                send_input(&state, kind, id, data).await?;
            }
            Op::Resize(mut cols, mut rows) => {
                while let Ok(op) = receiver.try_recv() {
                    match op {
                        Op::Resize(c, r) => (cols, rows) = (c, r),
                        other => {
                            next = Some(other);
                            break;
                        }
                    }
                }
                send_size(&state, kind, id, cols, rows).await?;
            }
        }
        if next.is_none() {
            next = receiver.recv().await;
        }
    }
    Ok(())
}

mod backend;
#[cfg(test)]
mod tests;
