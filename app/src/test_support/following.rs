//! A page's subscriptions, run as the window's runtime runs them: after
//! each `update` the runtime asks the page what it follows, starts what
//! is new and drops what is no longer returned. A test does the same
//! with [`Following::sync`], and reads what the streams send.

use std::{future::poll_fn, hash::Hasher as _, task::Poll, time::Duration};

use iced::{
    advanced::subscription::{into_recipes, Hasher},
    futures::{stream, stream::BoxStream, StreamExt},
    Subscription,
};

/// The streams of the subscriptions a page returned last.
pub struct Following<Message> {
    running: Vec<(u64, BoxStream<'static, Message>)>,
}

impl<Message> Default for Following<Message> {
    fn default() -> Self {
        Self { running: vec![] }
    }
}

impl<Message: 'static> Following<Message> {
    /// What the runtime does with what `subscription()` returned: a
    /// stream that is still returned goes on, one that is new starts
    /// (nothing is asked of the backend before it is polled), and one
    /// that is no longer returned is dropped, which is how a page lets
    /// go of it.
    pub fn sync(&mut self, subscription: Subscription<Message>) {
        let mut next = vec![];
        for recipe in into_recipes(subscription) {
            let mut hasher = Hasher::default();
            recipe.hash(&mut hasher);
            let id = hasher.finish();
            let running = self.running.iter().position(|(known, _)| *known == id);
            next.push(match running {
                Some(at) => self.running.swap_remove(at),
                // No window sends events: what listens to them hears none.
                None => (id, recipe.stream(stream::pending().boxed())),
            });
        }
        self.running = next;
    }

    /// How many streams run.
    pub fn count(&self) -> usize {
        self.running.len()
    }

    /// The next message of any stream; `None` when none comes in `wait`.
    pub async fn next(&mut self, wait: Duration) -> Option<Message> {
        let next = poll_fn(|context| {
            for (_, stream) in &mut self.running {
                if let Poll::Ready(Some(message)) = stream.poll_next_unpin(context) {
                    return Poll::Ready(message);
                }
            }
            Poll::Pending
        });
        tokio::time::timeout(wait, next).await.ok()
    }
}
