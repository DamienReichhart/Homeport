//! The backend's streams as iced streams: what `subscribe()` and a Tauri
//! `Channel` were to the React app. A `*_subscribe` use case pushes items
//! through a callback from a task of [`Streams`]; [`open`] turns that into
//! a `Stream` of [`Update`]s for `Task::run` or a [`subscription`].
//!
//! - **Batched.** Whatever arrived since the last update comes as one
//!   [`Update::Batch`], and at most [`UPDATES_PER_SECOND`] of them a second:
//!   a container logging 50,000 lines a second costs the window 60 messages
//!   a second, not 50,000.
//! - **Bounded.** A view's stream keeps a [`Backlog`]; past it the oldest
//!   items are dropped and counted, so a window that can't keep up shows
//!   the latest lines and how many it skipped, and memory stays flat.
//! - **No leak.** Dropping the stream (the page closed, the selection
//!   moved, the subscription is no longer returned) stops a view's backend
//!   task and closes a terminal's session. A job (a pull, a build, a
//!   deploy, a backup) goes on: only `stream_cancel` with the id of its
//!   [`Update::Started`] ends it, as before.
//!
//! The shapes are in [`shapes`]: lines, samples, watches, jobs with their
//! progress, and terminals.
//!
//! [`Streams`]: homeport_core::streams::Streams

mod channel;
pub mod session;
pub mod shapes;
#[cfg(test)]
mod tests;

use std::{
    future::Future,
    hash::{Hash, Hasher},
    sync::Arc,
    time::Duration,
};

use iced::{
    futures::{future::BoxFuture, stream, FutureExt, Stream},
    Subscription,
};
use tokio::time::Instant;

use homeport_core::error::{AppError, AppResult};

use channel::{channel, Receiver};
pub use channel::{Backlog, Sink};

use crate::app::AppState;

/// The most updates a stream gives the window in a second: one per frame.
pub const UPDATES_PER_SECOND: u32 = 60;

/// The least time between two updates of a stream.
pub const FRAME: Duration = Duration::from_nanos(1_000_000_000 / UPDATES_PER_SECOND as u64);

/// What a stream tells the window.
#[derive(Debug, Clone, PartialEq)]
pub enum Update<T> {
    /// The backend started it, under this id: a stream's for
    /// `stream_cancel`, a terminal's session for its input and resizes.
    Started(u32),
    /// It couldn't start; nothing follows.
    Failed(AppError),
    /// Everything that arrived since the last update, oldest first, and
    /// how much was dropped before it because the window was behind.
    Batch { items: Vec<T>, dropped: u64 },
    /// The backend is done: the source ended, or it was cancelled. Nothing
    /// follows.
    Ended,
}

/// What stops the backend's side of a stream once the window dropped it.
pub type Stop = fn(&AppState, u32);

/// How a stream behaves when the window is behind or gone.
pub struct Options<T> {
    /// What may wait for the window; `None` keeps everything.
    pub backlog: Option<Backlog<T>>,
    /// Called with the stream's id when the window drops it; `None` for a
    /// job, which goes on without its page.
    pub stop: Option<Stop>,
}

type Start<T> =
    Box<dyn FnOnce(Arc<AppState>, Sink<T>) -> BoxFuture<'static, AppResult<u32>> + Send>;

enum Phase<T> {
    Starting(Start<T>, Sink<T>),
    Running,
    Over,
}

/// A stream being followed; dropping it is how the window lets go of it.
struct Following<T> {
    state: Arc<AppState>,
    receiver: Receiver<T>,
    phase: Phase<T>,
    /// What the backend started, once it did.
    id: Option<u32>,
    stop: Option<Stop>,
    /// When the last batch was given.
    last: Option<Instant>,
}

impl<T> Following<T> {
    async fn next(&mut self) -> Option<Update<T>> {
        match std::mem::replace(&mut self.phase, Phase::Running) {
            Phase::Starting(start, sink) => match start(self.state.clone(), sink).await {
                Ok(id) => {
                    self.id = Some(id);
                    Some(Update::Started(id))
                }
                Err(error) => {
                    self.phase = Phase::Over;
                    Some(Update::Failed(error))
                }
            },
            Phase::Running => {
                // Let a frame's worth gather before taking it.
                if let Some(last) = self.last {
                    tokio::time::sleep_until(last + FRAME).await;
                }
                let taken = self.receiver.take().await;
                self.last = Some(Instant::now());
                if taken.items.is_empty() {
                    // Only an ended stream gives nothing.
                    self.phase = Phase::Over;
                    return Some(Update::Ended);
                }
                Some(Update::Batch {
                    items: taken.items,
                    dropped: taken.dropped,
                })
            }
            Phase::Over => {
                self.phase = Phase::Over;
                None
            }
        }
    }
}

impl<T> Drop for Following<T> {
    fn drop(&mut self) {
        if let (Some(stop), Some(id)) = (self.stop, self.id) {
            stop(&self.state, id);
        }
    }
}

/// Starts a backend stream with `start` (a `*_subscribe` use case given the
/// [`Sink`] as its callback) and follows it: [`Update::Started`] or
/// [`Update::Failed`] first, then batches, then [`Update::Ended`].
///
/// Nothing starts before the stream is polled, and dropping it at any point
/// stops what was started, as `options` say.
pub fn open<T, F, Fut>(
    state: Arc<AppState>,
    options: Options<T>,
    start: F,
) -> impl Stream<Item = Update<T>> + Send + 'static
where
    T: Send + 'static,
    F: FnOnce(Arc<AppState>, Sink<T>) -> Fut + Send + 'static,
    Fut: Future<Output = AppResult<u32>> + Send + 'static,
{
    let (sink, receiver) = channel(options.backlog, options.stop.is_some());
    let following = Following {
        state,
        receiver,
        phase: Phase::Starting(
            Box::new(move |state, sink| start(state, sink).boxed()),
            sink,
        ),
        id: None,
        stop: options.stop,
        last: None,
    };
    stream::unfold(following, |mut following| async move {
        let update = following.next().await?;
        Some((update, following))
    })
}

/// What a [`subscription`] is told apart by, and what its stream is opened
/// with: the app's state, which is the same for all, and `params`.
pub struct Source<P> {
    pub state: Arc<AppState>,
    pub params: P,
}

impl<P: Hash> Hash for Source<P> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.params.hash(hasher);
    }
}

/// A stream as a subscription: it runs while `subscription()` returns it
/// with the same `params`, and is dropped (so stopped) when it no longer
/// does. `open` can't capture anything; what it needs is in `params`.
///
/// For views only (logs, stats, watches). A job is started once, with
/// `Task::run`: as a subscription it would start again each time the page
/// showing it came back.
///
/// ```ignore
/// streams::subscription(&state, (host_id, container_id), |source| {
///     let (host_id, container_id) = source.params.clone();
///     shapes::stats(source.state.clone(), host_id, container_id)
/// })
/// .map(Message::Stats)
/// ```
pub fn subscription<P, S>(
    state: &Arc<AppState>,
    params: P,
    open: fn(&Source<P>) -> S,
) -> Subscription<S::Item>
where
    P: Hash + 'static,
    S: Stream + Send + 'static,
    S::Item: 'static,
{
    let source = Source {
        state: state.clone(),
        params,
    };
    Subscription::run_with(source, open)
}
