//! What an action did, told to the shell itself: the toast that says it,
//! the failure, what is out of date. React's `useMutation` ran `onSuccess`
//! and `onError` whether or not the page that started the action was still
//! there; a page's own message is dropped once the page is left, so what
//! must outlive the page goes this way and not through `update`.
//!
//! A page starts an action with [`Context::act`]: the work (use cases of
//! `api`), what to report of its answer (an [`Outcome`], made where the
//! answer is, on the runtime) and the message the page gets if it is still
//! open (to close its dialog, to stop its button waiting). The shell
//! reports every outcome that has come before it handles a message.
//!
//! [`Context::act`]: super::Context::act

use std::{cell::RefCell, fmt};

use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use crate::{
    data::{Data, Key},
    ui::widgets::feedback::{toast::Said, Surfaced},
};

/// A value an action answered, filed in the data layer as a read's would.
pub(in crate::ui::shell) type Put = Box<dyn FnOnce(&mut Data) + Send>;

/// What an action did, for the shell to report.
#[derive(Default)]
pub struct Outcome {
    /// What it did, for a toast (`toast.success`).
    pub(in crate::ui::shell) said: Option<Said>,
    /// Why it failed, for one toast that leads to its Details, under a
    /// title when the message alone wouldn't say what failed.
    pub(in crate::ui::shell) failed: Option<(Option<Said>, Surfaced)>,
    /// What it made out of date: read again.
    pub(in crate::ui::shell) invalidates: Vec<Key>,
    pub(in crate::ui::shell) puts: Vec<(Key, Put)>,
}

impl fmt::Debug for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Outcome")
            .field("said", &self.said)
            .field("failed", &self.failed)
            .field("invalidates", &self.invalidates)
            .finish_non_exhaustive()
    }
}

impl Outcome {
    /// Nothing to say: the page shows the answer itself.
    pub fn quiet() -> Self {
        Self::default()
    }

    /// Says what was done in a toast. `say` is asked each time the toast is
    /// drawn, so it reads in the language of the moment.
    #[must_use]
    pub fn saying(mut self, say: impl Fn() -> String + Send + Sync + 'static) -> Self {
        self.said = Some(Said::new(say));
        self
    }

    /// Says the failure in a toast, which leads to its Details.
    #[must_use]
    pub fn failing(mut self, surfaced: Surfaced) -> Self {
        self.failed = Some((None, surfaced));
        self
    }

    /// Everything under `scope` is read again.
    #[must_use]
    pub fn invalidating(mut self, scope: Key) -> Self {
        self.invalidates.push(scope);
        self
    }

    /// Files `value` under `key` as if it had just been read
    /// (`setQueryData`).
    #[must_use]
    pub fn putting<T: Send + Sync + 'static>(mut self, key: Key, value: T) -> Self {
        let filed = key.clone();
        self.puts
            .push((filed, Box::new(move |data| data.set(&key, value))));
        self
    }
}

/// Where the outcomes of the actions on their way arrive.
pub struct Outcomes {
    sender: UnboundedSender<Outcome>,
    receiver: RefCell<UnboundedReceiver<Outcome>>,
}

impl Default for Outcomes {
    fn default() -> Self {
        let (sender, receiver) = unbounded_channel();
        Self {
            sender,
            receiver: RefCell::new(receiver),
        }
    }
}

impl Outcomes {
    pub(in crate::ui::shell) fn sender(&self) -> UnboundedSender<Outcome> {
        self.sender.clone()
    }

    /// The outcomes that came since the last call, oldest first.
    pub(in crate::ui::shell) fn take(&self) -> Vec<Outcome> {
        let mut receiver = self.receiver.borrow_mut();
        std::iter::from_fn(|| receiver.try_recv().ok()).collect()
    }
}
