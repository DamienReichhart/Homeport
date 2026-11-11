//! How something is read and how long it stays good: what React Query's
//! `queryOptions` said (`queryKey`, `queryFn`, `staleTime`, `retry`,
//! `refetchInterval`, `gcTime`).

use std::{any::Any, fmt, future::Future, marker::PhantomData, sync::Arc, time::Duration};

use iced::futures::{future::BoxFuture, FutureExt};

use homeport_core::error::AppResult;

use super::Key;
use crate::app::AppState;

/// What a read gave, whatever its type; [`Query`] knows which.
pub(super) type Value = Arc<dyn Any + Send + Sync>;

/// Reads a resource from the app's state.
pub(super) type Loader =
    Arc<dyn Fn(Arc<AppState>) -> BoxFuture<'static, AppResult<Value>> + Send + Sync>;

/// How long a read stays good and what is done when it fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Policy {
    /// How long after it was read a page opening it reads it again; `None`
    /// for never, until it is invalidated.
    pub stale_after: Option<Duration>,
    /// How many times a failed read is tried again before it is `Failed`.
    pub retries: u32,
    /// Read again this often while a page shows it and the window is
    /// visible; `None` for a resource events or watches keep fresh.
    pub poll: Option<Duration>,
    /// How long it is kept once no page shows it.
    pub keep: Duration,
}

impl Default for Policy {
    /// The React app's defaults: good for 10 s, one retry, no polling
    /// (Engine data is refreshed by pushed events), kept 5 min.
    fn default() -> Self {
        Self {
            stale_after: Some(Duration::from_secs(10)),
            retries: 1,
            poll: None,
            keep: Duration::from_secs(300),
        }
    }
}

/// A resource of type `T`: where it is filed, how it is read, how long it
/// stays good.
pub struct Query<T> {
    pub(super) key: Key,
    pub(super) policy: Policy,
    pub(super) loader: Loader,
    data: PhantomData<fn() -> T>,
}

impl<T: Send + Sync + 'static> Query<T> {
    /// The resource filed under `key`, read by `load`: a use case of `api`.
    pub fn new<F, Fut>(key: Key, load: F) -> Self
    where
        F: Fn(Arc<AppState>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = AppResult<T>> + Send + 'static,
    {
        Self {
            key,
            policy: Policy::default(),
            loader: Arc::new(move |state| {
                load(state)
                    .map(|result| result.map(|data| Arc::new(data) as Value))
                    .boxed()
            }),
            data: PhantomData,
        }
    }

    /// Good for `time` after it was read (`staleTime`).
    #[must_use]
    pub fn stale_after(mut self, time: Duration) -> Self {
        self.policy.stale_after = Some(time);
        self
    }

    /// Read again whenever a page opens it (`staleTime: 0`).
    #[must_use]
    pub fn always_stale(self) -> Self {
        self.stale_after(Duration::ZERO)
    }

    /// Good until it is invalidated (`staleTime: Infinity`).
    #[must_use]
    pub fn never_stale(mut self) -> Self {
        self.policy.stale_after = None;
        self
    }

    /// A failed read is tried again `times` times (`retry`).
    #[must_use]
    pub fn retries(mut self, times: u32) -> Self {
        self.policy.retries = times;
        self
    }

    /// A failed read is `Failed` at once (`retry: false`): the failure is
    /// an answer (a 403, a registry that says no).
    #[must_use]
    pub fn no_retry(self) -> Self {
        self.retries(0)
    }

    /// Read again every `interval` while shown (`refetchInterval`).
    #[must_use]
    pub fn poll(mut self, interval: Duration) -> Self {
        self.policy.poll = Some(interval);
        self
    }

    /// Kept `time` once no page shows it (`gcTime`).
    #[must_use]
    pub fn keep(mut self, time: Duration) -> Self {
        self.policy.keep = time;
        self
    }

    pub fn key(&self) -> &Key {
        &self.key
    }

    pub fn policy(&self) -> &Policy {
        &self.policy
    }
}

impl<T> fmt::Debug for Query<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Query")
            .field("key", &self.key)
            .field("policy", &self.policy)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_query_starts_with_the_defaults_and_takes_its_own() {
        let query = Query::new(Key::new(["updates"]), |_| async { Ok(1_u8) });
        assert_eq!(query.policy(), &Policy::default());
        let query = query
            .never_stale()
            .no_retry()
            .poll(Duration::from_secs(30))
            .keep(Duration::ZERO);
        assert_eq!(
            query.policy(),
            &Policy {
                stale_after: None,
                retries: 0,
                poll: Some(Duration::from_secs(30)),
                keep: Duration::ZERO,
            }
        );
        assert_eq!(query.key(), &Key::new(["updates"]));
    }
}
