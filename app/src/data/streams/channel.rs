//! Between a backend stream and the window: the backend calls
//! [`Sink::send`] for each item, from its own task and as fast as its
//! source goes; the window takes everything that waits at once, when it
//! draws next. Nothing here waits on the other side.

use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex, MutexGuard, PoisonError,
    },
};

use tokio::sync::Notify;

/// What is done with items the window has no time to take.
#[derive(Debug)]
pub struct Backlog<T> {
    /// How much may wait, in what `weight` counts.
    pub max: usize,
    /// What an item counts for: one, or the lines it carries.
    pub weight: fn(&T) -> usize,
}

// Not derived: `T` itself needn't be `Copy`.
impl<T> Clone for Backlog<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Backlog<T> {}

/// What waits for the window.
struct Queue<T> {
    items: VecDeque<T>,
    /// The weight of `items`.
    weight: usize,
    /// How much weight was dropped since the window last took items.
    dropped: u64,
    /// The backend is done: nothing more will come.
    ended: bool,
}

struct Shared<T> {
    queue: Mutex<Queue<T>>,
    /// Wakes the window's side when something waits or the stream ended.
    wake: Notify,
    /// The window's side is gone.
    gone: AtomicBool,
    /// Whether the backend is told so, to stop: not for a job.
    stops_when_gone: bool,
    backlog: Option<Backlog<T>>,
}

impl<T> Shared<T> {
    fn queue(&self) -> MutexGuard<'_, Queue<T>> {
        // A panic while holding it leaves a queue that is still a queue.
        self.queue.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

/// The backend's side: what a `*_subscribe` use case is given as its
/// callback, `move |item| sink.send(item)`. Dropping it ends the stream.
pub struct Sink<T>(Arc<Shared<T>>);

/// The window's side.
pub(super) struct Receiver<T>(Arc<Shared<T>>);

/// What the window takes at once.
pub(super) struct Taken<T> {
    pub items: Vec<T>,
    pub dropped: u64,
    /// Nothing more will come after these.
    pub ended: bool,
}

/// A queue from a backend stream to the window. With a `backlog`, the
/// oldest items are dropped once more than it allows wait; without one
/// everything waits. `stops_when_gone` makes [`Sink::send`] answer `false`
/// once the receiver is dropped.
pub(super) fn channel<T>(
    backlog: Option<Backlog<T>>,
    stops_when_gone: bool,
) -> (Sink<T>, Receiver<T>) {
    let shared = Arc::new(Shared {
        queue: Mutex::new(Queue {
            items: VecDeque::new(),
            weight: 0,
            dropped: 0,
            ended: false,
        }),
        wake: Notify::new(),
        gone: AtomicBool::new(false),
        stops_when_gone,
        backlog,
    });
    (Sink(shared.clone()), Receiver(shared))
}

impl<T> Sink<T> {
    /// Hands `item` to the window. The answer is whether to go on, as the
    /// `*_subscribe` use cases expect of their callback: `false` once the
    /// view that showed the stream is gone. A job is always told to go on,
    /// so leaving its page never aborts it.
    pub fn send(&self, item: T) -> bool {
        let shared = &self.0;
        if shared.gone.load(Ordering::Acquire) {
            return !shared.stops_when_gone;
        }
        let mut queue = shared.queue();
        let weight = shared.backlog.map_or(1, |backlog| (backlog.weight)(&item));
        queue.items.push_back(item);
        queue.weight += weight;
        if let Some(backlog) = shared.backlog {
            // The newest item always waits, however heavy.
            while queue.weight > backlog.max && queue.items.len() > 1 {
                let Some(oldest) = queue.items.pop_front() else {
                    break;
                };
                let weight = (backlog.weight)(&oldest);
                queue.weight -= weight;
                queue.dropped += weight as u64;
            }
        }
        drop(queue);
        shared.wake.notify_one();
        true
    }
}

impl<T> Drop for Sink<T> {
    fn drop(&mut self) {
        self.0.queue().ended = true;
        self.0.wake.notify_one();
    }
}

impl<T> Receiver<T> {
    /// Waits until something waits or the stream ended, then takes it all.
    pub async fn take(&self) -> Taken<T> {
        loop {
            {
                let mut queue = self.0.queue();
                if !queue.items.is_empty() || queue.ended {
                    queue.weight = 0;
                    return Taken {
                        items: std::mem::take(&mut queue.items).into(),
                        dropped: std::mem::take(&mut queue.dropped),
                        ended: queue.ended,
                    };
                }
            }
            self.0.wake.notified().await;
        }
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        self.0.gone.store(true, Ordering::Release);
        // Nobody will take what waits.
        let mut queue = self.0.queue();
        queue.items.clear();
        queue.weight = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn everything_that_waits_is_taken_at_once_in_order() {
        let (sink, receiver) = channel::<u32>(None, true);
        for n in 0..1000 {
            assert!(sink.send(n));
        }
        let taken = receiver.take().await;
        assert_eq!(taken.items, (0..1000).collect::<Vec<_>>());
        assert_eq!(taken.dropped, 0);
        assert!(!taken.ended);

        // The end comes after what was sent before it.
        sink.send(7);
        drop(sink);
        let taken = receiver.take().await;
        assert_eq!(taken.items, [7]);
        assert!(taken.ended);
        assert!(receiver.take().await.items.is_empty());
    }

    #[tokio::test]
    async fn a_backlog_drops_the_oldest_and_counts_them() {
        let lines = Backlog {
            max: 10,
            weight: |lines: &Vec<u8>| lines.len(),
        };
        let (sink, receiver) = channel(Some(lines), true);
        for n in 0..6 {
            sink.send(vec![n; 4]);
        }
        let taken = receiver.take().await;
        // Two events of four lines fit in ten; four were dropped.
        assert_eq!(taken.items, [vec![4; 4], vec![5; 4]]);
        assert_eq!(taken.dropped, 16);
        // An event heavier than the backlog still gets through, alone.
        sink.send(vec![1; 3]);
        sink.send(vec![2; 50]);
        let taken = receiver.take().await;
        assert_eq!(taken.items, [vec![2; 50]]);
        assert_eq!(taken.dropped, 3);
    }

    #[tokio::test]
    async fn a_view_that_left_stops_its_stream_and_a_job_goes_on() {
        let (view, receiver) = channel::<u8>(None, true);
        assert!(view.send(1));
        drop(receiver);
        assert!(!view.send(2));

        let (job, receiver) = channel::<u8>(None, false);
        drop(receiver);
        assert!(job.send(1));
        // Nothing piles up for nobody.
        assert!(job.0.queue().items.is_empty());
    }

    #[tokio::test]
    async fn taking_waits_for_the_first_item() {
        let (sink, receiver) = channel::<u8>(None, true);
        let waiting = tokio::spawn(async move { receiver.take().await.items });
        tokio::task::yield_now().await;
        assert!(!waiting.is_finished());
        sink.send(9);
        assert_eq!(waiting.await.unwrap(), [9]);
    }
}
