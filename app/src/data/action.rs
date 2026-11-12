//! Something the user asked the app to do (start a container, save a
//! Stack): what React Query's `useMutation` held. A feature keeps an
//! [`Action`] in its state, starts it from `update` and gives its end back
//! to it; its view reads whether it is pending (the button waits) and why
//! it failed (the error widget, or a toast).
//!
//! ```ignore
//! // update: the user pressed Start.
//! Message::Start(id) => self.starting
//!     .start(state, move |state| async move {
//!         containers_act(&state, host_id, id, ContainerAction::Start).await
//!     })
//!     .map(Message::Started),
//! // update: it ended; on success the Host's containers are read again.
//! Message::Started(finished) => data
//!     .settle(state, &mut self.starting, finished, [keys::host::containers(&host_id)])
//!     .map(Message::Data),
//! ```
//!
//! An optimistic change shows the result before the answer:
//! [`Data::optimistic`] changes the cached value and gives what undoes it,
//! which [`Data::settle_optimistic`] uses if the action fails.

use std::{future::Future, sync::Arc};

use iced::Task;

use homeport_core::error::{AppError, AppResult};

use super::{query::Value, store::batch, Data, Key, Message};
use crate::app::AppState;

/// Where an action stands.
#[derive(Debug, Clone, PartialEq)]
pub enum Status<T> {
    /// Not asked for, or reset.
    Idle,
    Pending,
    Done(T),
    Failed(AppError),
}

/// One thing the user can ask for, and how its last run went.
#[derive(Debug, Clone, PartialEq)]
pub struct Action<T = ()> {
    /// Tells the latest run from one it replaced.
    run: u64,
    status: Status<T>,
}

/// The end of a run, to give to [`Action::finish`] or [`Data::settle`].
#[derive(Debug, Clone)]
pub struct Finished<T> {
    run: u64,
    result: AppResult<T>,
}

impl<T> Default for Action<T> {
    fn default() -> Self {
        Self {
            run: 0,
            status: Status::Idle,
        }
    }
}

impl<T: Send + 'static> Action<T> {
    /// Runs `work`, a use case of `api`, and is pending until its end comes
    /// back. Started again while pending, only the latest run counts.
    pub fn start<F, Fut>(&mut self, state: &Arc<AppState>, work: F) -> Task<Finished<T>>
    where
        F: FnOnce(Arc<AppState>) -> Fut,
        Fut: Future<Output = AppResult<T>> + Send + 'static,
    {
        self.run += 1;
        self.status = Status::Pending;
        let run = self.run;
        Task::perform(work(state.clone()), move |result| Finished { run, result })
    }

    /// Takes the end of a run. Returns whether it was the latest run and it
    /// succeeded; the end of a run that was replaced or reset is ignored.
    pub fn finish(&mut self, finished: Finished<T>) -> bool {
        if finished.run != self.run || !self.is_pending() {
            return false;
        }
        match finished.result {
            Ok(done) => {
                self.status = Status::Done(done);
                true
            }
            Err(error) => {
                self.status = Status::Failed(error);
                false
            }
        }
    }
}

#[cfg(test)]
impl<T> Action<T> {
    /// The end of the run that is on its way, as if its use case had
    /// answered `result`: for the tests of a page, which has no Engine.
    pub fn ending_with(&self, result: AppResult<T>) -> Finished<T> {
        Finished {
            run: self.run,
            result,
        }
    }
}

impl<T> Action<T> {
    pub fn status(&self) -> &Status<T> {
        &self.status
    }

    pub fn is_pending(&self) -> bool {
        matches!(self.status, Status::Pending)
    }

    /// Why the last run failed.
    pub fn error(&self) -> Option<&AppError> {
        match &self.status {
            Status::Failed(error) => Some(error),
            _ => None,
        }
    }

    /// What the last run gave.
    pub fn done(&self) -> Option<&T> {
        match &self.status {
            Status::Done(done) => Some(done),
            _ => None,
        }
    }

    /// Forgets the last run: its dialog closed, or its error was dismissed.
    /// A run still pending is ignored when it ends.
    pub fn reset(&mut self) {
        self.status = Status::Idle;
    }
}

/// What undoes an optimistic change.
pub struct Rollback {
    key: Key,
    before: Value,
    /// What the change put there: only this is taken back.
    changed: Value,
}

impl Data {
    /// Takes the end of `action`'s run and, when it succeeded, has
    /// everything in `invalidates` read again.
    pub fn settle<T: Send + 'static>(
        &mut self,
        state: &Arc<AppState>,
        action: &mut Action<T>,
        finished: Finished<T>,
        invalidates: impl IntoIterator<Item = Key>,
    ) -> Task<Message> {
        if !action.finish(finished) {
            return Task::none();
        }
        batch(
            invalidates
                .into_iter()
                .map(|scope| self.invalidate(state, &scope)),
        )
    }

    /// Changes the value of type `T` filed under `key` to what the action
    /// about to run should make it, and gives what puts it back. `None`
    /// when nothing is cached there, or `change` leaves it.
    pub fn optimistic<T: Send + Sync + 'static>(
        &mut self,
        key: &Key,
        change: impl FnOnce(&T) -> Option<T>,
    ) -> Option<Rollback> {
        let before = self.cache.value(key)?;
        if !self.change(key, change) {
            return None;
        }
        Some(Rollback {
            key: key.clone(),
            before,
            changed: self.cache.value(key)?,
        })
    }

    /// [`Data::settle`] for an action that changed the cache before it ran:
    /// a failure puts back what was there. Either way what the change
    /// touched is read again, to show what the backend really holds.
    pub fn settle_optimistic<T: Send + 'static>(
        &mut self,
        state: &Arc<AppState>,
        action: &mut Action<T>,
        finished: Finished<T>,
        rollback: Option<Rollback>,
        invalidates: impl IntoIterator<Item = Key>,
    ) -> Task<Message> {
        let run = finished.run;
        let succeeded = action.finish(finished);
        if run != action.run {
            // A newer run made its own change; leave it.
            return Task::none();
        }
        let touched = rollback.as_ref().map(|rollback| rollback.key.clone());
        if let (false, Some(rollback)) = (succeeded, rollback) {
            self.cache
                .restore(&rollback.key, &rollback.changed, rollback.before);
        }
        let scopes = invalidates
            .into_iter()
            .filter(|_| succeeded)
            .chain(touched)
            .collect::<Vec<_>>();
        batch(scopes.iter().map(|scope| self.invalidate(state, scope)))
    }
}
