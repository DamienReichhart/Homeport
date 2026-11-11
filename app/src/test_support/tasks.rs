//! Running what an `update` returns: a [`Task`] is a promise of messages,
//! and a test wants them, to feed back into `update` and assert on the state.

use iced::{futures::StreamExt, Task};
use iced_runtime::{task, Action};

/// Runs `task` to its end and returns the messages it produced, in order.
/// What it asks of the window (focus, clipboard, resize…) is dropped: no
/// window is there to do it.
pub async fn messages<Message>(task: Task<Message>) -> Vec<Message> {
    let Some(mut actions) = task::into_stream(task) else {
        return vec![];
    };
    let mut messages = vec![];
    while let Some(action) = actions.next().await {
        if let Action::Output(message) = action {
            messages.push(message);
        }
    }
    messages
}

/// Whether `task` does nothing at all, as [`Task::none`].
pub fn is_none<Message>(task: Task<Message>) -> bool {
    task::into_stream(task).is_none()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn a_task_yields_its_messages_in_order() {
        let task = Task::batch([Task::done(1), Task::perform(async { 2 }, |n| n)]);
        let mut produced = messages(task).await;
        produced.sort_unstable();
        assert_eq!(produced, [1, 2]);
        assert_eq!(messages(Task::done(1).chain(Task::done(2))).await, [1, 2]);
    }

    #[tokio::test]
    async fn no_task_yields_nothing() {
        assert!(is_none(Task::<u8>::none()));
        assert!(!is_none(Task::done(1)));
        assert!(messages(Task::<u8>::none()).await.is_empty());
    }
}
