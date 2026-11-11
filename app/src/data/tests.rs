//! [`Data`] end to end, without a window: fake loaders that count their
//! calls, tasks run by hand, and tokio's clock paused so waits (retries,
//! the settle time of invalidations) pass at once and to the millisecond.

use std::{
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use iced::Task;

use homeport_core::error::{AppError, AppResult, ErrorCode};

use super::{action::Status, keys::host, Action, Data, Key, Message, Query, Resource, SETTLE};
use crate::{
    app::{testing, AppState},
    test_support::tasks,
};

/// Runs `task` and everything that follows from it, until nothing is left
/// to do.
pub(super) async fn drive(data: &mut Data, state: &Arc<AppState>, task: Task<Message>) {
    let mut task = task;
    loop {
        let messages = tasks::messages(task).await;
        if messages.is_empty() {
            return;
        }
        task = Task::batch(
            messages
                .into_iter()
                .map(|message| data.update(state, message)),
        );
    }
}

/// A test app's state, shared as the window shares it, and what must
/// outlive the test: its folder and the receiving half of its events.
pub(super) fn state() -> (Arc<AppState>, impl Sized) {
    let testing::TestApp { state, events, dir } = testing::app();
    (Arc::new(state), (events, dir))
}

pub(super) fn down() -> AppError {
    AppError::new(
        ErrorCode::HostNotConnected,
        homeport_hosts::messages::NOT_CONNECTED_UNKNOWN,
    )
}

/// A loader's side of a test: how often it was called, and what it answers.
#[derive(Clone, Default)]
pub(super) struct Source {
    pub calls: Arc<AtomicUsize>,
    /// Fails while set.
    pub broken: Arc<AtomicBool>,
}

impl Source {
    pub fn calls(&self) -> usize {
        self.calls.load(Ordering::SeqCst)
    }

    /// The names under `key`: `["web-1"]` on the first read, `["web-2"]` on
    /// the second…
    pub fn query(&self, key: Key) -> Query<Vec<String>> {
        let source = self.clone();
        Query::new(key, move |_| {
            let source = source.clone();
            async move { source.answer() }
        })
    }

    fn answer(&self) -> AppResult<Vec<String>> {
        let call = self.calls.fetch_add(1, Ordering::SeqCst) + 1;
        if self.broken.load(Ordering::SeqCst) {
            return Err(down());
        }
        Ok(vec![format!("web-{call}")])
    }
}

pub(super) fn names(data: &Data, key: &Key) -> Resource<Vec<String>> {
    data.read::<Vec<String>>(key).map(|list| (*list).clone())
}

fn list(names: &[&str]) -> Vec<String> {
    names.iter().map(|&name| name.to_owned()).collect()
}

#[tokio::test(start_paused = true)]
async fn a_watched_resource_is_read_once_and_shown() {
    let (state, _app) = state();
    let mut data = Data::default();
    let source = Source::default();
    let key = host::containers("h1");
    assert_eq!(names(&data, &key), Resource::Idle);

    let first = data.watch(&state, &source.query(key.clone()));
    // Another page opens the same list before it arrived.
    let second = data.watch(&state, &source.query(key.clone()));
    assert!(tasks::is_none(second));
    assert_eq!(names(&data, &key), Resource::Loading);
    assert_eq!(data.in_flight(), 1);

    drive(&mut data, &state, first).await;
    assert_eq!(names(&data, &key), Resource::Loaded(list(&["web-1"])));
    assert_eq!(source.calls(), 1);
    assert_eq!(data.in_flight(), 0);
}

#[tokio::test(start_paused = true)]
async fn a_page_coming_back_sees_the_old_data_while_it_is_read_again() {
    let (state, _app) = state();
    let mut data = Data::default();
    let source = Source::default();
    let key = host::containers("h1");
    let task = data.watch(&state, &source.query(key.clone()));
    drive(&mut data, &state, task).await;
    data.unwatch(&key);

    // Back within 10 s: nothing is read.
    tokio::time::advance(Duration::from_secs(9)).await;
    assert!(tasks::is_none(
        data.watch(&state, &source.query(key.clone()))
    ));
    data.unwatch(&key);

    tokio::time::advance(Duration::from_secs(1)).await;
    let task = data.watch(&state, &source.query(key.clone()));
    assert_eq!(names(&data, &key), Resource::Refreshing(list(&["web-1"])));
    drive(&mut data, &state, task).await;
    assert_eq!(names(&data, &key), Resource::Loaded(list(&["web-2"])));
}

#[tokio::test(start_paused = true)]
async fn a_failed_read_is_tried_again_after_a_wait() {
    let (state, _app) = state();
    let mut data = Data::default();
    let source = Source::default();
    source.broken.store(true, Ordering::SeqCst);
    let key = host::containers("h1");
    let started = tokio::time::Instant::now();

    let task = data.watch(&state, &source.query(key.clone()).retries(2));
    drive(&mut data, &state, task).await;
    // Three tries, 1 s then 2 s apart.
    assert_eq!(source.calls(), 3);
    assert_eq!(started.elapsed(), Duration::from_secs(3));
    assert_eq!(names(&data, &key), Resource::Failed(down(), None));
    assert_eq!(data.in_flight(), 0);

    // It answers again: the page's refresh clears the failure.
    source.broken.store(false, Ordering::SeqCst);
    let task = data.invalidate(&state, &key);
    drive(&mut data, &state, task).await;
    assert_eq!(names(&data, &key), Resource::Loaded(list(&["web-4"])));
}

#[tokio::test(start_paused = true)]
async fn one_invalidation_refreshes_everything_a_host_has() {
    let (state, _app) = state();
    let mut data = Data::default();
    let (containers, images, other) = (Source::default(), Source::default(), Source::default());
    let task = Task::batch([
        data.watch(&state, &containers.query(host::containers("h1"))),
        data.watch(&state, &images.query(host::images("h1"))),
        data.watch(&state, &other.query(host::containers("h2"))),
    ]);
    drive(&mut data, &state, task).await;

    let task = data.invalidate(&state, &host::scope("h1"));
    drive(&mut data, &state, task).await;
    assert_eq!(
        (containers.calls(), images.calls(), other.calls()),
        (2, 2, 1)
    );
}

#[tokio::test(start_paused = true)]
async fn a_burst_of_invalidations_reads_each_list_once() {
    let (state, _app) = state();
    let mut data = Data::default();
    let source = Source::default();
    let key = host::containers("h1");
    let task = data.watch(&state, &source.query(key.clone()));
    drive(&mut data, &state, task).await;

    // Twenty events, 10 ms apart: each one puts the flush off.
    let mut flushes = vec![];
    for _ in 0..20 {
        flushes.push(data.invalidate_later([key.clone(), host::stacks("h1")]));
        tokio::time::advance(Duration::from_millis(10)).await;
    }
    assert!(tasks::is_none(data.invalidate_later([])));
    let started = tokio::time::Instant::now();
    drive(&mut data, &state, Task::batch(flushes)).await;
    assert_eq!(source.calls(), 2);
    // The last one waited its whole settle time.
    assert_eq!(started.elapsed(), SETTLE - Duration::from_millis(10));
}

/// Set once the read it is moved into is dropped.
struct Dropped(Arc<AtomicBool>);

impl Drop for Dropped {
    fn drop(&mut self) {
        self.0.store(true, Ordering::SeqCst);
    }
}

/// A resource whose read never ends; `dropped` tells when it was aborted.
fn endless(key: Key, dropped: &Arc<AtomicBool>) -> Query<Vec<String>> {
    let dropped = dropped.clone();
    Query::new(key, move |_| {
        let guard = Dropped(dropped.clone());
        async move {
            let _guard = guard;
            std::future::pending::<AppResult<Vec<String>>>().await
        }
    })
}

/// Starts `task` as the runtime would, far enough for its read to begin.
async fn begun(task: Task<Message>) -> tokio::task::JoinHandle<Vec<Message>> {
    let run = tokio::spawn(tasks::messages(task));
    tokio::task::yield_now().await;
    run
}

/// Whether the read `run` was aborted: dropped, and without a message.
async fn aborted(run: tokio::task::JoinHandle<Vec<Message>>, dropped: &AtomicBool) -> bool {
    let messages = run.await.unwrap_or_default();
    dropped.load(Ordering::SeqCst) && messages.is_empty()
}

#[tokio::test(start_paused = true)]
async fn closing_the_page_aborts_its_read() {
    let (state, _app) = state();
    let mut data = Data::default();
    let dropped = Arc::new(AtomicBool::new(false));
    let key = host::containers("h1");
    let run = begun(data.watch(&state, &endless(key.clone(), &dropped))).await;
    assert_eq!(data.in_flight(), 1);
    assert!(!dropped.load(Ordering::SeqCst));

    data.unwatch(&key);
    assert_eq!(data.in_flight(), 0);
    assert!(aborted(run, &dropped).await);
    assert_eq!(names(&data, &key), Resource::Idle);
}

#[tokio::test(start_paused = true)]
async fn changing_selection_aborts_every_read_of_the_host() {
    let (state, _app) = state();
    let mut data = Data::default();
    let (first, second) = (
        Arc::new(AtomicBool::new(false)),
        Arc::new(AtomicBool::new(false)),
    );
    let containers = begun(data.watch(&state, &endless(host::containers("h1"), &first))).await;
    let images = begun(data.watch(&state, &endless(host::images("h1"), &second))).await;
    let other = Source::default();
    let kept = data.watch(&state, &other.query(host::containers("h2")));

    data.release(&host::scope("h1"));
    assert_eq!(data.in_flight(), 1);
    assert!(aborted(containers, &first).await);
    assert!(aborted(images, &second).await);
    drive(&mut data, &state, kept).await;
    assert_eq!(
        names(&data, &host::containers("h2")),
        Resource::Loaded(list(&["web-1"]))
    );
}

#[tokio::test(start_paused = true)]
async fn polling_reads_again_at_the_interval_and_stops_when_hidden() {
    let (state, _app) = state();
    let mut data = Data::default();
    let source = Source::default();
    let key = Key::new(["port-forwards", "list"]);
    let query = || source.query(key.clone()).poll(Duration::from_secs(2));
    let task = data.watch(&state, &query());
    drive(&mut data, &state, task).await;
    assert_eq!(source.calls(), 1);

    // A tick a second later is too early; the next one is due.
    tokio::time::advance(Duration::from_secs(1)).await;
    let task = data.update(&state, Message::Minimised(false));
    drive(&mut data, &state, task).await;
    assert_eq!(source.calls(), 1);
    tokio::time::advance(Duration::from_secs(1)).await;
    let task = data.update(&state, Message::Minimised(false));
    drive(&mut data, &state, task).await;
    assert_eq!(source.calls(), 2);

    // Minimised, or hidden in the tray: nothing is read however late.
    tokio::time::advance(Duration::from_secs(60)).await;
    let task = data.update(&state, Message::Minimised(true));
    assert!(tasks::is_none(task));
    data.set_hidden(true);
    assert!(tasks::is_none(data.update(&state, Message::Tick)));
    assert_eq!(source.calls(), 2);

    // Shown again: the first tick catches up.
    data.set_hidden(false);
    let task = data.update(&state, Message::Minimised(false));
    drive(&mut data, &state, task).await;
    assert_eq!(source.calls(), 3);
}

#[tokio::test(start_paused = true)]
async fn an_action_is_pending_then_done_and_refreshes_what_it_changed() {
    let (state, _app) = state();
    let mut data = Data::default();
    let source = Source::default();
    let key = host::containers("h1");
    let task = data.watch(&state, &source.query(key.clone()));
    drive(&mut data, &state, task).await;

    let mut start: Action<u8> = Action::default();
    assert_eq!(start.status(), &Status::Idle);
    let run = start.start(&state, |_| async { Ok(7) });
    assert!(start.is_pending());
    let finished = tasks::messages(run).await.remove(0);
    let task = data.settle(&state, &mut start, finished, [host::scope("h1")]);
    assert_eq!(start.done(), Some(&7));
    drive(&mut data, &state, task).await;
    assert_eq!(source.calls(), 2);
}

#[tokio::test(start_paused = true)]
async fn a_failed_action_keeps_its_error_and_refreshes_nothing() {
    let (state, _app) = state();
    let mut data = Data::default();
    let mut stop: Action = Action::default();
    let run = stop.start(&state, |_| async { Err(down()) });
    let finished = tasks::messages(run).await.remove(0);
    let task = data.settle(&state, &mut stop, finished, [host::scope("h1")]);
    assert!(tasks::is_none(task));
    assert_eq!(stop.error(), Some(&down()));
    assert!(!stop.is_pending());

    // Its error dismissed, the action is as new.
    stop.reset();
    assert_eq!(stop.status(), &Status::Idle);
}

#[tokio::test(start_paused = true)]
async fn only_the_latest_run_of_an_action_counts() {
    let (state, _app) = state();
    let mut rename: Action<&str> = Action::default();
    let first = rename.start(&state, |_| async { Ok("first") });
    let second = rename.start(&state, |_| async { Ok("second") });
    let first = tasks::messages(first).await.remove(0);
    assert!(!rename.finish(first));
    assert!(rename.is_pending());
    let second = tasks::messages(second).await.remove(0);
    assert!(rename.finish(second.clone()));
    assert_eq!(rename.done(), Some(&"second"));
    // An end that comes twice, or after a reset, changes nothing.
    assert!(!rename.finish(second));
}

#[tokio::test(start_paused = true)]
async fn an_optimistic_change_is_undone_when_the_action_fails() {
    let (state, _app) = state();
    let mut data = Data::default();
    let key = host::containers("h1");
    // Nothing cached: nothing to change, nothing to undo.
    assert!(data
        .optimistic(&key, |names: &Vec<String>| Some(names.clone()))
        .is_none());
    data.set(&key, list(&["web", "db"]));

    let mut remove: Action = Action::default();
    let rollback = data.optimistic(&key, |names: &Vec<String>| {
        Some(names.iter().filter(|n| *n != "db").cloned().collect())
    });
    assert_eq!(names(&data, &key), Resource::Loaded(list(&["web"])));
    let run = remove.start(&state, |_| async { Err(down()) });
    let finished = tasks::messages(run).await.remove(0);
    let task = data.settle_optimistic(&state, &mut remove, finished, rollback, []);
    drive(&mut data, &state, task).await;
    assert_eq!(names(&data, &key), Resource::Loaded(list(&["web", "db"])));
    assert_eq!(remove.error(), Some(&down()));
}
