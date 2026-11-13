//! The app's window, for a dialog to open over.

use iced::{
    window::{self, Window},
    Task,
};

/// Runs `start` with the app's window, on the window's own thread, and
/// gives what it made. `None` when there is no window to run it with: it
/// closed meanwhile, or nothing answers for it. The task always ends
/// with one answer, so whoever asked is never left waiting.
pub(super) fn with_window<T>(
    start: impl FnOnce(&dyn Window) -> T + Send + 'static,
) -> Task<Option<T>>
where
    T: Send + 'static,
{
    let mut start = Some(start);
    window::oldest()
        .then(move |id| match (id, start.take()) {
            (Some(id), Some(start)) => window::run(id, start),
            _ => Task::none(),
        })
        // A window that went away answers nothing: that is told as `None`.
        .collect()
        .map(|mut made| made.pop())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::tasks::messages;

    #[tokio::test]
    async fn with_no_window_to_answer_the_task_still_ends_with_an_answer() {
        // Nothing answers for a window here, as when it has just closed.
        let asked = with_window(|_| "started");
        assert_eq!(messages(asked).await, [None]);
    }
}
