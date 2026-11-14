//! Whether the container started again since its log stream ended, told
//! by when it started (`State.StartedAt` of its inspect document), which a
//! restart changes and nothing else does: a status (`Up 3 hours`, then
//! `Up 4 hours`, then `(healthy)`) changes all the time.
//!
//! - When a stream opens, the container's start is read: the one it
//!   follows.
//! - When it ends, the start is read once more: a restart that went from
//!   running to running before the end was seen shows there.
//! - Then it is read again only when the container's entry in the list
//!   changes, and at most once every [`READ_EVERY`]: an entry that changes
//!   within that time is read once the time is over.
//! - A container seen not running since the end, then running, started
//!   again, whatever its start reads.
//! - A stream that failed is never opened again by itself: its failure
//!   stays, with its Retry, and nothing more is read.

use std::time::{Duration, Instant};

use homeport_engine::containers::Container;
use iced::Task;

use super::{state::Tab, Message, State};
use crate::{api::engine::containers::containers_inspect, ui::shell::Context};

/// How often, at most, the container's start is read again after the end.
pub(super) const READ_EVERY: Duration = Duration::from_secs(2);

/// What the page knows of the start of the container its log stream
/// follows.
#[derive(Debug, Default)]
pub(super) struct Restarts {
    /// The stream this is about.
    generation: Option<u64>,
    /// When the container had started as that stream opened, once read.
    started: Option<String>,
    /// Since the stream ended, if it did.
    ended: Option<Ended>,
}

#[derive(Debug)]
struct Ended {
    /// The container was seen not running since.
    stopped_since: bool,
    /// Its entry in the list when its start was last asked for.
    entry: Option<Container>,
    /// When its start was last asked for, or will be.
    asked_at: Instant,
    /// A read waits for [`READ_EVERY`] to pass since the last.
    waiting: bool,
}

/// Reads when the container started, after `wait`, for the stream of
/// `generation`. A failure reads as nothing known.
fn read(state: &State, generation: u64, wait: Duration, ctx: &Context<'_>) -> Task<Message> {
    let app = ctx.state.clone();
    let (host, id) = (state.host_id.clone(), state.found.clone());
    let id = id.unwrap_or_else(|| state.id.clone());
    Task::perform(
        async move {
            if !wait.is_zero() {
                tokio::time::sleep(wait).await;
            }
            let document = containers_inspect(&app, host, id).await.ok()?;
            let started = document.pointer("/State/StartedAt")?.as_str()?;
            Some(started.to_owned())
        },
        move |started| Message::Started(generation, started),
    )
}

/// Looks at the log stream and the container as they are now: reads the
/// start of a stream that opened, and opens the logs again once the
/// container is seen to have started again since they ended.
pub(super) fn follow(state: &mut State, ctx: &Context<'_>) -> Task<Message> {
    if state.tab != Tab::Logs {
        return Task::none();
    }
    let generation = state.logs.generation();
    let mut opened = Task::none();
    if state.restarts.generation != Some(generation) {
        state.restarts = Restarts {
            generation: Some(generation),
            ..Restarts::default()
        };
        opened = read(state, generation, Duration::ZERO, ctx);
    }
    if !state.logs.has_ended() || state.logs.failure.is_some() {
        return opened;
    }
    let container = state.container(ctx);
    let running = container.as_ref().is_some_and(|c| c.state == "running");
    let Some(end) = &mut state.restarts.ended else {
        state.restarts.ended = Some(Ended {
            stopped_since: !running,
            entry: container,
            asked_at: Instant::now(),
            waiting: false,
        });
        return Task::batch([opened, read(state, generation, Duration::ZERO, ctx)]);
    };
    if !running {
        end.stopped_since = true;
        return opened;
    }
    if end.stopped_since {
        return again(state, ctx);
    }
    if container == end.entry || end.waiting {
        end.entry = container;
        return opened;
    }
    end.entry = container;
    let wait = READ_EVERY.saturating_sub(end.asked_at.elapsed());
    end.asked_at = Instant::now() + wait;
    end.waiting = !wait.is_zero();
    Task::batch([opened, read(state, generation, wait, ctx)])
}

/// The start read for the stream of `generation`: the one it follows if
/// none was read yet; once it ended, another one means a restart.
pub(super) fn answered(
    state: &mut State,
    generation: u64,
    started: Option<String>,
    ctx: &Context<'_>,
) -> Task<Message> {
    let restarts = &mut state.restarts;
    if restarts.generation != Some(generation) {
        return Task::none();
    }
    if let Some(end) = &mut restarts.ended {
        end.waiting = false;
    }
    let Some(started) = started else {
        return Task::none();
    };
    match &restarts.started {
        None => restarts.started = Some(started),
        Some(before) if *before != started && restarts.ended.is_some() => {
            let open =
                state.tab == Tab::Logs && state.logs.has_ended() && state.logs.failure.is_none();
            if open {
                return again(state, ctx);
            }
        }
        Some(_) => {}
    }
    Task::none()
}

/// Opens the logs anew, on an empty viewer, and reads the start of the
/// stream that opens.
fn again(state: &mut State, ctx: &Context<'_>) -> Task<Message> {
    state.logs.restart(jiff::Timestamp::now().as_second());
    follow(state, ctx)
}
