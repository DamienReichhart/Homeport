//! The cache itself, with no I/O and no clock of its own: it is told what
//! happens and when, and answers with what to do ([`Effect`]). [`Data`]
//! (`super::store`) turns those into tasks; the tests read them directly.
//!
//! What it guarantees:
//!
//! - **Stale-while-revalidate.** A page that opens a resource read long ago
//!   is shown it at once, while it is read again.
//! - **One read at a time.** Two pages watching a key share its read; a
//!   resource invalidated while it is being read is read once more after.
//! - **Retries.** A failed read is tried again with a growing wait before
//!   the resource is `Failed`, with what was read before kept.
//! - **Nothing read for nobody.** When the last page showing a resource
//!   leaves, its read is aborted and its answer ignored; the data is kept a
//!   while for the page coming back.
//!
//! [`Data`]: super::Data

use std::{collections::BTreeMap, sync::Arc, time::Duration};

use tokio::time::Instant;

use homeport_core::error::AppError;

use super::{
    query::{Loader, Policy, Value},
    Key, Query, Resource,
};

#[cfg(test)]
mod tests;

/// The longest wait between two tries of a read.
const MAX_BACKOFF: Duration = Duration::from_secs(30);

/// How long after try number `attempt` (from 0) failed the next one starts:
/// 1 s, 2 s, 4 s… up to 30 s, as React Query waited.
pub(super) fn backoff(attempt: u32) -> Duration {
    Duration::from_secs(1)
        .saturating_mul(2_u32.saturating_pow(attempt))
        .min(MAX_BACKOFF)
}

/// What the cache asks for.
pub(super) enum Effect {
    /// Read `key` with `loader` once `after` has passed, and give the
    /// answer to [`Cache::settle`] with `flight`.
    Load {
        key: Key,
        flight: u64,
        after: Duration,
        loader: Loader,
    },
    /// Stop the read `flight`: nobody waits for it.
    Abort { flight: u64 },
}

/// A read on its way.
struct Flight {
    id: u64,
    /// How many tries failed before this one.
    attempt: u32,
    /// The resource was invalidated since this read started: its answer
    /// may already be out of date, so it is read again after.
    again: bool,
}

struct Entry {
    value: Option<Value>,
    error: Option<AppError>,
    /// When `value` was read.
    loaded_at: Option<Instant>,
    /// When the last read ended, well or not: what polling counts from.
    settled_at: Option<Instant>,
    /// Told out of date, whatever its age.
    invalidated: bool,
    flight: Option<Flight>,
    /// How many pages show it.
    watchers: u32,
    /// When the last page showing it left.
    left_at: Option<Instant>,
    policy: Policy,
    /// `None` for a value only ever set by hand.
    loader: Option<Loader>,
}

impl Entry {
    fn new(now: Instant) -> Self {
        Self {
            value: None,
            error: None,
            loaded_at: None,
            settled_at: None,
            invalidated: false,
            flight: None,
            watchers: 0,
            left_at: Some(now),
            policy: Policy::default(),
            loader: None,
        }
    }

    fn is_stale(&self, now: Instant) -> bool {
        let Some(loaded_at) = self.loaded_at else {
            return true;
        };
        self.invalidated
            || self.error.is_some()
            || self
                .policy
                .stale_after
                .is_some_and(|time| now.saturating_duration_since(loaded_at) >= time)
    }
}

#[derive(Default)]
pub(super) struct Cache {
    entries: BTreeMap<Key, Entry>,
    flights: u64,
}

impl Cache {
    /// A page starts showing `query`'s resource: it is read if it never
    /// was, or again if it is out of date.
    pub fn watch<T>(&mut self, query: &Query<T>, now: Instant) -> Vec<Effect> {
        self.sweep(now);
        let entry = self
            .entries
            .entry(query.key.clone())
            .or_insert_with(|| Entry::new(now));
        entry.watchers += 1;
        entry.left_at = None;
        entry.policy = query.policy;
        entry.loader = Some(query.loader.clone());
        if entry.flight.is_some() || !entry.is_stale(now) {
            return vec![];
        }
        self.start(&query.key).into_iter().collect()
    }

    /// A page stops showing `key`. When it was the last one, the read on
    /// its way is aborted.
    pub fn unwatch(&mut self, key: &Key, now: Instant) -> Vec<Effect> {
        let mut effects = vec![];
        if let Some(entry) = self.entries.get_mut(key) {
            entry.watchers = entry.watchers.saturating_sub(1);
            if entry.watchers == 0 {
                entry.left_at.get_or_insert(now);
                effects.extend(abort(entry));
            }
        }
        self.sweep(now);
        effects
    }

    /// Every page stops showing what is in `scope`: the selection moved to
    /// another Host or Cluster.
    pub fn release(&mut self, scope: &Key, now: Instant) -> Vec<Effect> {
        let mut effects = vec![];
        for entry in self.in_scope(scope) {
            entry.watchers = 0;
            entry.left_at.get_or_insert(now);
            effects.extend(abort(entry));
        }
        self.sweep(now);
        effects
    }

    /// Everything in `scope` is out of date: what a page shows is read
    /// again now, the rest when a page next opens it.
    pub fn invalidate(&mut self, scope: &Key) -> Vec<Effect> {
        let mut restart = vec![];
        for (key, entry) in self.entries.iter_mut().filter(|(key, _)| key.is_in(scope)) {
            entry.invalidated = true;
            match &mut entry.flight {
                Some(flight) => flight.again = true,
                None if entry.watchers > 0 => restart.push(key.clone()),
                None => {}
            }
        }
        restart.iter().filter_map(|key| self.start(key)).collect()
    }

    /// Everything in `scope` can't be read any more, for `error`: its Host
    /// or Cluster went down. What was read is kept, and reads on their way
    /// are aborted: they would only wait to fail.
    pub fn fail(&mut self, scope: &Key, error: &AppError, now: Instant) -> Vec<Effect> {
        let mut effects = vec![];
        for entry in self.in_scope(scope) {
            effects.extend(abort(entry));
            entry.error = Some(error.clone());
            entry.settled_at = Some(now);
        }
        effects
    }

    /// The read `flight` of `key` ended with `result`.
    pub fn settle(
        &mut self,
        key: &Key,
        flight: u64,
        result: Result<Value, AppError>,
        now: Instant,
    ) -> Vec<Effect> {
        let Some(entry) = self.entries.get_mut(key) else {
            return vec![];
        };
        let Some(current) = entry.flight.take_if(|current| current.id == flight) else {
            // Aborted, or replaced by a newer read: nobody waits for it.
            return vec![];
        };
        entry.settled_at = Some(now);
        match result {
            Ok(value) => {
                entry.value = Some(value);
                entry.error = None;
                entry.loaded_at = Some(now);
                entry.invalidated = current.again;
            }
            Err(_) if current.again => {}
            Err(_) if current.attempt < entry.policy.retries => {
                let Some(loader) = entry.loader.clone() else {
                    return vec![];
                };
                entry.flight = Some(Flight {
                    attempt: current.attempt + 1,
                    ..current
                });
                return vec![Effect::Load {
                    key: key.clone(),
                    flight,
                    after: backoff(current.attempt),
                    loader,
                }];
            }
            Err(error) => entry.error = Some(error),
        }
        if current.again && entry.watchers > 0 {
            return self.start(key).into_iter().collect();
        }
        vec![]
    }

    /// Reads again what a page shows and whose polling interval has passed,
    /// and forgets what no page has shown for long.
    pub fn poll(&mut self, now: Instant) -> Vec<Effect> {
        self.sweep(now);
        let due: Vec<Key> = self
            .entries
            .iter()
            .filter(|(_, entry)| entry.watchers > 0 && entry.flight.is_none())
            .filter(|(_, entry)| {
                let Some(interval) = entry.policy.poll else {
                    return false;
                };
                entry
                    .settled_at
                    .is_none_or(|at| now.saturating_duration_since(at) >= interval)
            })
            .map(|(key, _)| key.clone())
            .collect();
        due.iter().filter_map(|key| self.start(key)).collect()
    }

    /// Whether a page shows something that is polled.
    pub fn polls(&self) -> bool {
        self.entries
            .values()
            .any(|entry| entry.watchers > 0 && entry.policy.poll.is_some())
    }

    /// Files `value` under `key` as if it had just been read: what a use
    /// case returned (the saved settings) needs no second read.
    pub fn set<T: Send + Sync + 'static>(&mut self, key: &Key, value: T, now: Instant) {
        let entry = self
            .entries
            .entry(key.clone())
            .or_insert_with(|| Entry::new(now));
        entry.value = Some(Arc::new(value));
        entry.error = None;
        entry.loaded_at = Some(now);
        entry.invalidated = false;
    }

    /// Changes what is filed under `key`, if something of type `T` is:
    /// `change` gives the new value, or `None` to leave it. Returns whether
    /// it changed. Its age stays what it was.
    pub fn update<T: Send + Sync + 'static>(
        &mut self,
        key: &Key,
        change: impl FnOnce(&T) -> Option<T>,
    ) -> bool {
        let Some(entry) = self.entries.get_mut(key) else {
            return false;
        };
        let changed = entry
            .value
            .as_ref()
            .and_then(|value| value.downcast_ref::<T>())
            .and_then(change);
        match changed {
            Some(value) => {
                entry.value = Some(Arc::new(value));
                true
            }
            None => false,
        }
    }

    /// What is known of the resource filed under `key`.
    pub fn read<T: Send + Sync + 'static>(&self, key: &Key) -> Resource<Arc<T>> {
        let Some(entry) = self.entries.get(key) else {
            return Resource::Idle;
        };
        let data = entry
            .value
            .clone()
            .and_then(|value| value.downcast::<T>().ok());
        match (&entry.flight, &entry.error, data) {
            (Some(_), _, Some(data)) => Resource::Refreshing(data),
            (Some(_), _, None) => Resource::Loading,
            (None, Some(error), data) => Resource::Failed(error.clone(), data),
            (None, None, Some(data)) => Resource::Loaded(data),
            (None, None, None) => Resource::Idle,
        }
    }

    /// The value filed under `key`, whatever its type.
    pub fn value(&self, key: &Key) -> Option<Value> {
        self.entries.get(key)?.value.clone()
    }

    /// Puts back under `key` a value [`Cache::value`] gave.
    pub fn restore(&mut self, key: &Key, value: Value) {
        if let Some(entry) = self.entries.get_mut(key) {
            entry.value = Some(value);
        }
    }

    /// Whether something is filed under `key`, shown or not.
    pub fn holds(&self, key: &Key) -> bool {
        self.entries.contains_key(key)
    }

    /// The keys in `scope` that hold something or are being read.
    pub fn keys_in(&self, scope: &Key) -> Vec<Key> {
        self.entries
            .keys()
            .filter(|key| key.is_in(scope))
            .cloned()
            .collect()
    }

    fn in_scope<'a>(&'a mut self, scope: &'a Key) -> impl Iterator<Item = &'a mut Entry> {
        self.entries
            .iter_mut()
            .filter(move |(key, _)| key.is_in(scope))
            .map(|(_, entry)| entry)
    }

    /// Starts reading `key`, unless it has no loader.
    fn start(&mut self, key: &Key) -> Option<Effect> {
        let entry = self.entries.get_mut(key)?;
        let loader = entry.loader.clone()?;
        self.flights += 1;
        entry.flight = Some(Flight {
            id: self.flights,
            attempt: 0,
            again: false,
        });
        Some(Effect::Load {
            key: key.clone(),
            flight: self.flights,
            after: Duration::ZERO,
            loader,
        })
    }

    /// Forgets what no page has shown for as long as its policy keeps it.
    fn sweep(&mut self, now: Instant) {
        self.entries.retain(|_, entry| {
            entry
                .left_at
                .is_none_or(|at| now.saturating_duration_since(at) < entry.policy.keep)
        });
    }
}

/// Stops the read of `entry`, if one is on its way.
fn abort(entry: &mut Entry) -> Option<Effect> {
    let flight = entry.flight.take()?;
    // What it would have brought is still missing.
    entry.invalidated |= flight.again;
    Some(Effect::Abort { flight: flight.id })
}
