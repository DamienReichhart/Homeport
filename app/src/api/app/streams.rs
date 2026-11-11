//! The streams every `*_subscribe` use case starts.

use homeport_core::streams::StreamId;

use crate::app::AppState;

/// Stops a stream started by any `*_subscribe` use case.
pub fn stream_cancel(state: &AppState, id: StreamId) {
    state.streams.cancel(id);
}

#[cfg(test)]
mod tests {
    use std::sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    };

    use crate::app::testing;

    /// Set once the stream's task is dropped, finished or not.
    struct Dropped(Arc<AtomicBool>);

    impl Drop for Dropped {
        fn drop(&mut self) {
            self.0.store(true, Ordering::SeqCst);
        }
    }

    #[tokio::test]
    async fn cancelling_stops_the_stream() {
        let app = testing::app();
        let stopped = Arc::new(AtomicBool::new(false));
        let guard = Dropped(stopped.clone());
        let id = app.state.streams.spawn(async move {
            let _guard = guard;
            std::future::pending::<()>().await;
        });
        tokio::task::yield_now().await;
        assert!(!stopped.load(Ordering::SeqCst));

        super::stream_cancel(&app.state, id);
        for _ in 0..100 {
            if stopped.load(Ordering::SeqCst) {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
        assert!(stopped.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn cancelling_an_unknown_stream_does_nothing() {
        let app = testing::app();
        super::stream_cancel(&app.state, 4242);
    }
}
