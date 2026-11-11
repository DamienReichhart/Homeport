//! The cache as the window uses it: [`Data`] sits in the app's state, pages
//! ask it for resources and read them in their `view`, and what it must do
//! on the side comes back to it as a [`Message`].
//!
//! ```ignore
//! // A page opens: read the Host's containers, or show them if they're known.
//! let task = data.watch(&state, &resources::containers(&host_id));
//! // Its view: Loading, Loaded, Refreshing, Failed with the old list…
//! let containers = data.read::<Vec<Container>>(&keys::host::containers(&host_id));
//! // It closes, or the selection moves to another Host.
//! data.unwatch(&keys::host::containers(&host_id));
//! data.release(&keys::host::scope(&host_id));
//! ```

use std::{
    collections::{BTreeSet, HashMap},
    fmt,
    sync::Arc,
    time::Duration,
};

use iced::{task::Handle, window, Subscription, Task};
use tokio::time::Instant;

use homeport_core::error::AppError;

use super::{
    cache::{Cache, Effect},
    query::Value,
    Key, Query, Resource,
};
use crate::app::AppState;

/// How long invalidations asked for with [`Data::invalidate_later`] wait
/// for more: a Stack starting sends dozens of events, and each list is read
/// once for all of them.
pub const SETTLE: Duration = Duration::from_millis(250);

/// How often polled resources are looked at.
const TICK: Duration = Duration::from_secs(1);

/// What a read gave. Only [`Data`] opens it.
#[derive(Clone)]
pub struct Loaded(Value);

impl fmt::Debug for Loaded {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Loaded")
    }
}

/// What comes back to [`Data::update`].
#[derive(Debug, Clone)]
pub enum Message {
    /// A read ended.
    Fetched {
        key: Key,
        flight: u64,
        result: Result<Loaded, AppError>,
    },
    /// The invalidations waiting since the flush numbered so are due.
    Flush(u64),
    /// Time to look at what is polled.
    Tick,
    /// Whether the window is minimised, asked on each tick.
    Minimised(bool),
}

/// The live data of the app: everything pages read from Hosts and Clusters,
/// cached by [`Key`].
#[derive(Default)]
pub struct Data {
    pub(super) cache: Cache,
    /// The reads on their way; dropping one's handle aborts it.
    flights: HashMap<u64, Handle>,
    /// What [`Data::invalidate_later`] collected since the last flush.
    pending: BTreeSet<Key>,
    /// The number of the flush the pending invalidations wait for.
    flush: u64,
    /// The window is hidden in the tray: nothing is polled.
    hidden: bool,
}

impl Data {
    /// A page starts showing `query`'s resource. It is read unless a recent
    /// read is cached; a read already on its way is shared.
    pub fn watch<T>(&mut self, state: &Arc<AppState>, query: &Query<T>) -> Task<Message> {
        let effects = self.cache.watch(query, Instant::now());
        self.run(state, effects)
    }

    /// A page stops showing `key`: when it was the last, its read is
    /// aborted. Call it once for each [`Data::watch`].
    pub fn unwatch(&mut self, key: &Key) {
        let effects = self.cache.unwatch(key, Instant::now());
        self.stop(effects);
    }

    /// Every page stops showing what is in `scope`, as when the selection
    /// moves to another Host or Cluster: their reads are aborted.
    pub fn release(&mut self, scope: &Key) {
        let effects = self.cache.release(scope, Instant::now());
        self.stop(effects);
    }

    /// Everything in `scope` is out of date: what is shown is read again
    /// now, the rest when a page opens it. One call with a Host's or a
    /// Cluster's scope refreshes all it has.
    pub fn invalidate(&mut self, state: &Arc<AppState>, scope: &Key) -> Task<Message> {
        let effects = self.cache.invalidate(scope);
        self.run(state, effects)
    }

    /// [`Data::invalidate`] for each of `scopes`, once no more were asked
    /// for during [`SETTLE`]: a burst of events reads each list once.
    pub fn invalidate_later(&mut self, scopes: impl IntoIterator<Item = Key>) -> Task<Message> {
        let scopes: Vec<Key> = scopes.into_iter().collect();
        if scopes.is_empty() {
            return Task::none();
        }
        self.pending.extend(scopes);
        self.flush += 1;
        let flush = self.flush;
        Task::perform(tokio::time::sleep(SETTLE), move |()| Message::Flush(flush))
    }

    /// Everything in `scope` can't be read for `error`, as when its Host or
    /// Cluster goes down. What was read stays on screen, marked failed.
    pub fn fail(&mut self, scope: &Key, error: &AppError) {
        let effects = self.cache.fail(scope, error, Instant::now());
        self.stop(effects);
    }

    /// What is known of the resource filed under `key`, for a `view`.
    pub fn read<T: Send + Sync + 'static>(&self, key: &Key) -> Resource<Arc<T>> {
        self.cache.read(key)
    }

    /// Files `value` under `key` as if it had just been read: what a use
    /// case returned needs no second read.
    pub fn set<T: Send + Sync + 'static>(&mut self, key: &Key, value: T) {
        self.cache.set(key, value, Instant::now());
    }

    /// Changes the value of type `T` filed under `key`, if there is one:
    /// `change` gives the new one, or `None` to leave it. Returns whether
    /// it changed.
    pub fn change<T: Send + Sync + 'static>(
        &mut self,
        key: &Key,
        change: impl FnOnce(&T) -> Option<T>,
    ) -> bool {
        self.cache.update(key, change)
    }

    /// The window was hidden in the tray, or shown again: nothing is polled
    /// while nobody can see it. A minimised window is found out by itself.
    pub fn set_hidden(&mut self, hidden: bool) {
        self.hidden = hidden;
    }

    pub fn update(&mut self, state: &Arc<AppState>, message: Message) -> Task<Message> {
        match message {
            Message::Fetched {
                key,
                flight,
                result,
            } => {
                let result = result.map(|loaded| loaded.0);
                let effects = self.cache.settle(&key, flight, result, Instant::now());
                if !effects.iter().any(|effect| retries(effect, flight)) {
                    self.flights.remove(&flight);
                }
                self.run(state, effects)
            }
            Message::Flush(flush) => {
                if flush != self.flush {
                    // More were asked for since: a later flush takes them all.
                    return Task::none();
                }
                let scopes = std::mem::take(&mut self.pending);
                batch(scopes.iter().map(|scope| self.invalidate(state, scope)))
            }
            Message::Tick if self.hidden => Task::none(),
            // With no window at all nothing answers, and nothing is polled.
            Message::Tick => window::latest()
                .and_then(window::is_minimized)
                .map(|minimised| Message::Minimised(minimised.unwrap_or(false))),
            Message::Minimised(true) => Task::none(),
            Message::Minimised(false) => self.poll(state),
        }
    }

    /// Reads again what is shown and due. [`Data::update`] does it on each
    /// tick while the window is visible.
    pub fn poll(&mut self, state: &Arc<AppState>) -> Task<Message> {
        let effects = self.cache.poll(Instant::now());
        self.run(state, effects)
    }

    /// Ticks while a page shows something that is polled.
    pub fn subscription(&self) -> Subscription<Message> {
        if self.hidden || !self.cache.polls() {
            return Subscription::none();
        }
        iced::time::every(TICK).map(|_| Message::Tick)
    }

    /// How many reads are on their way.
    pub fn in_flight(&self) -> usize {
        self.flights.len()
    }

    /// Does what the cache asked for.
    fn run(&mut self, state: &Arc<AppState>, effects: Vec<Effect>) -> Task<Message> {
        let mut tasks = vec![];
        for effect in effects {
            match effect {
                Effect::Load {
                    key,
                    flight,
                    after,
                    loader,
                } => {
                    let state = state.clone();
                    let read = async move {
                        if !after.is_zero() {
                            tokio::time::sleep(after).await;
                        }
                        loader(state).await
                    };
                    let (task, handle) = Task::perform(read, move |result| Message::Fetched {
                        key,
                        flight,
                        result: result.map(Loaded),
                    })
                    .abortable();
                    self.flights.insert(flight, handle.abort_on_drop());
                    tasks.push(task);
                }
                Effect::Abort { flight } => {
                    self.flights.remove(&flight);
                }
            }
        }
        batch(tasks)
    }

    /// Does what the cache asked for when it can only be to stop reads.
    fn stop(&mut self, effects: Vec<Effect>) {
        for effect in effects {
            if let Effect::Abort { flight } = effect {
                self.flights.remove(&flight);
            }
        }
    }
}

/// `tasks` as one, and no task at all when each of them is none: a caller
/// can tell that nothing was started.
pub(super) fn batch(tasks: impl IntoIterator<Item = Task<Message>>) -> Task<Message> {
    let mut tasks: Vec<_> = tasks.into_iter().filter(|task| task.units() > 0).collect();
    match tasks.len() {
        0 => Task::none(),
        1 => tasks.remove(0),
        _ => Task::batch(tasks),
    }
}

/// Whether `effect` tries the read `flight` again.
fn retries(effect: &Effect, flight: u64) -> bool {
    matches!(effect, Effect::Load { flight: again, .. } if *again == flight)
}
