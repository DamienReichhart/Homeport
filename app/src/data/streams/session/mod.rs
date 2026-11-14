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

use homeport_containerd::containers::exec::ContainerdExecOptions;
use homeport_core::error::{AppError, AppResult};
use homeport_engine::containers::exec::{ExecEvent, ExecOptions};
use homeport_incus::instances::terminal::IncusExecOptions;
use homeport_kubernetes::streams::terminal::PodExecOptions;
use homeport_nomad::allocations::exec::NomadExecOptions;

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

/// What a terminal runs in, and what it runs.
#[derive(Debug, Clone)]
pub enum Backend {
    /// A shell in a container of a Docker or Podman Engine.
    Engine {
        host_id: String,
        container_id: String,
        options: ExecOptions,
    },
    /// A process in a containerd container's running task.
    Containerd {
        host_id: String,
        namespace: String,
        id: String,
        options: ContainerdExecOptions,
    },
    /// A command in an Incus Instance.
    IncusExec {
        host_id: String,
        project: String,
        name: String,
        options: IncusExecOptions,
    },
    /// An Incus Instance's text console.
    IncusConsole {
        host_id: String,
        project: String,
        name: String,
        cols: u16,
        rows: u16,
    },
    /// A shell in a Pod's container.
    Pod {
        cluster_id: String,
        namespace: String,
        pod: String,
        options: PodExecOptions,
    },
    /// A command in a Nomad allocation's task.
    Nomad {
        cluster_id: String,
        alloc_id: String,
        options: NomadExecOptions,
    },
}

/// Which store of terminals a session is in: what its input, size and
/// end go through.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    Engine,
    Containerd,
    Incus,
    Pod,
    Nomad,
}

impl Backend {
    pub fn kind(&self) -> Kind {
        match self {
            Self::Engine { .. } => Kind::Engine,
            Self::Containerd { .. } => Kind::Containerd,
            Self::IncusExec { .. } | Self::IncusConsole { .. } => Kind::Incus,
            Self::Pod { .. } => Kind::Pod,
            Self::Nomad { .. } => Kind::Nomad,
        }
    }

    /// What tells two terminals apart, past the page's own number: where
    /// they run and what.
    fn identity(&self) -> Vec<&str> {
        match self {
            Self::Engine {
                host_id,
                container_id,
                options,
            } => vec![
                "engine",
                host_id,
                container_id,
                options.command.as_deref().unwrap_or_default(),
            ],
            Self::Containerd {
                host_id,
                namespace,
                id,
                options,
            } => vec![
                "containerd",
                host_id,
                namespace,
                id,
                options.command.as_deref().unwrap_or_default(),
            ],
            Self::IncusExec {
                host_id,
                project,
                name,
                options,
            } => vec![
                "incus-exec",
                host_id,
                project,
                name,
                options.command.as_deref().unwrap_or_default(),
            ],
            Self::IncusConsole {
                host_id,
                project,
                name,
                ..
            } => vec!["incus-console", host_id, project, name],
            Self::Pod {
                cluster_id,
                namespace,
                pod,
                options,
            } => vec![
                "pod",
                cluster_id,
                namespace,
                pod,
                options.container.as_deref().unwrap_or_default(),
                options.command.as_deref().unwrap_or_default(),
            ],
            Self::Nomad {
                cluster_id,
                alloc_id,
                options,
            } => vec![
                "nomad",
                cluster_id,
                alloc_id,
                &options.task,
                options.command.as_deref().unwrap_or_default(),
            ],
        }
    }
}

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
    /// What was asked for before the session started.
    pending: Vec<Op>,
    sender: Option<mpsc::UnboundedSender<Op>>,
}

impl TerminalSession {
    pub fn new(backend: &Backend) -> Self {
        Self {
            kind: backend.kind(),
            id: None,
            pending: vec![],
            sender: None,
        }
    }

    /// The session's id, once it started.
    pub fn id(&self) -> Option<u32> {
        self.id
    }

    /// The session `id` started: what was asked for meanwhile is sent, and
    /// so is everything after, in order, by the future this returns (for
    /// `Task::perform`). It ends with the first write that fails, or once
    /// the session ended or this was dropped.
    pub fn started(
        &mut self,
        state: Arc<AppState>,
        id: u32,
    ) -> impl Future<Output = AppResult<()>> + Send + 'static {
        let (sender, receiver) = mpsc::unbounded_channel();
        for op in self.pending.drain(..) {
            let _ = sender.send(op);
        }
        self.id = Some(id);
        self.sender = Some(sender);
        write(state, self.kind, id, receiver)
    }

    fn send(&mut self, op: Op) {
        match &self.sender {
            Some(sender) => {
                let _ = sender.send(op);
            }
            None => {
                // A new size before the start replaces the one before.
                if let (Op::Resize(..), Some(Op::Resize(..))) = (&op, self.pending.last()) {
                    self.pending.pop();
                }
                self.pending.push(op);
            }
        }
    }

    /// Bytes for the program: a key, a paste, an answer.
    pub fn input(&mut self, data: String) {
        if !data.is_empty() {
            self.send(Op::Input(data));
        }
    }

    /// The terminal has this many cells now.
    pub fn resize(&mut self, cols: u16, rows: u16) {
        self.send(Op::Resize(cols, rows));
    }

    /// The session ended: nothing more is sent, and the writer stops.
    pub fn ended(&mut self) {
        self.sender = None;
        self.pending.clear();
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

#[cfg(test)]
mod tests;
