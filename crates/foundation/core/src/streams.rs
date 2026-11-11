//! Long-lived streams (logs, stats, exec, pulls) run as tasks the frontend can
//! cancel by id, e.g. when the view showing them unmounts.

use std::{
    collections::HashMap,
    future::Future,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc, Mutex,
    },
};

use tokio::task::AbortHandle;

pub type StreamId = u32;

#[derive(Default)]
pub struct Streams {
    next: AtomicU32,
    tasks: Arc<Mutex<HashMap<StreamId, AbortHandle>>>,
}

impl Streams {
    /// Runs `task` until it finishes or [`Streams::cancel`] is called with the returned id.
    pub fn spawn<F>(&self, task: F) -> StreamId
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let id = self.next.fetch_add(1, Ordering::Relaxed) + 1;
        let tasks = self.tasks.clone();
        // Hold the lock across spawn so a task that ends at once can't be
        // removed before it is inserted.
        let mut guard = self.tasks.lock().expect("streams lock poisoned");
        let handle = tokio::spawn(async move {
            task.await;
            tasks.lock().expect("streams lock poisoned").remove(&id);
        });
        guard.insert(id, handle.abort_handle());
        id
    }

    pub fn cancel(&self, id: StreamId) {
        if let Some(handle) = self
            .tasks
            .lock()
            .expect("streams lock poisoned")
            .remove(&id)
        {
            handle.abort();
        }
    }

    /// Stops every stream, as the app quits: none outlives its window.
    pub fn cancel_all(&self) {
        let tasks = std::mem::take(&mut *self.tasks.lock().expect("streams lock poisoned"));
        for handle in tasks.into_values() {
            handle.abort();
        }
    }

    /// How many streams are running.
    pub fn running(&self) -> usize {
        self.tasks.lock().expect("streams lock poisoned").len()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[tokio::test]
    async fn cancelled_streams_stop() {
        let streams = Streams::default();
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        let id = streams.spawn(async move {
            loop {
                if tx.send(()).is_err() {
                    return;
                }
                tokio::time::sleep(Duration::from_millis(5)).await;
            }
        });
        rx.recv().await.unwrap();
        streams.cancel(id);
        tokio::time::sleep(Duration::from_millis(20)).await;
        while rx.try_recv().is_ok() {}
        assert!(rx.recv().await.is_none(), "task kept running after cancel");
        assert_eq!(streams.running(), 0);
    }

    #[tokio::test]
    async fn quitting_stops_every_stream() {
        let streams = Streams::default();
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<()>();
        for _ in 0..3 {
            let tx = tx.clone();
            streams.spawn(async move {
                let _held = tx;
                std::future::pending::<()>().await;
            });
        }
        drop(tx);
        assert_eq!(streams.running(), 3);
        streams.cancel_all();
        assert_eq!(streams.running(), 0);
        // Each task was dropped, with what it held.
        assert!(rx.recv().await.is_none());
    }

    #[tokio::test]
    async fn finished_streams_are_forgotten() {
        let streams = Streams::default();
        let a = streams.spawn(async {});
        let b = streams.spawn(async {});
        assert_ne!(a, b);
        tokio::time::sleep(Duration::from_millis(20)).await;
        assert_eq!(streams.running(), 0);
    }
}
