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
//!   The answer of a read that was on its way while the entry changed is
//!   followed by another read, for the entry as it is then.
//! - The start a stream follows is the one read as it opened; if that
//!   read failed, a start read after the end that is later than the
//!   moment the stream opened (on this computer's clock: an Engine whose
//!   clock is ahead may open the logs once more than needed) is a restart.
//! - A container seen not running since the end, then running, started
//!   again, whatever its start reads.
//! - A stream that failed is never opened again by itself: its failure
//!   stays, with its Retry, and nothing more is read.

use std::time::{Duration, Instant};

use homeport_engine::containers::Container;
use iced::Task;
use jiff::Timestamp;

use super::{state::Tab, Message, State};
use crate::{api::engine::containers::containers_inspect, ui::shell::Context};

/// How often, at most, the container's start is read again after the end.
pub(super) const READ_EVERY: Duration = Duration::from_secs(2);

/// Which read of the container's start an answer is: of the log stream of
/// `generation`, the `number`th the page asked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StartRead {
    pub generation: u64,
    pub number: u64,
}

/// What the page knows of the start of the container its log stream
/// follows.
#[derive(Debug, Default)]
pub(super) struct Restarts {
    /// The stream this is about.
    generation: Option<u64>,
    /// When that stream opened, on this computer's clock.
    opened_at: Option<Timestamp>,
    /// When the container had started as that stream opened, once read.
    started: Option<String>,
    /// How many reads were asked for.
    asked: u64,
    /// Since the stream ended, if it did.
    ended: Option<Ended>,
}

#[derive(Debug)]
struct Ended {
    /// The container was seen not running since.
    stopped_since: bool,
    /// The container's entry in the list when the last read was asked
    /// for: what its answer tells of.
    read_for: Option<Container>,
    /// When the last read was asked for, or will go.
    asked_at: Instant,
    /// The read on its way, if one is.
    on_its_way: Option<u64>,
}

impl Restarts {
    /// The last read asked for.
    pub(super) fn last_asked(&self) -> StartRead {
        StartRead {
            generation: self.generation.unwrap_or_default(),
            number: self.asked,
        }
    }
}

/// Reads when the container started, after `wait`, for the stream that
/// shows. A failure reads as nothing known.
fn read(state: &mut State, wait: Duration, ctx: &Context<'_>) -> (Task<Message>, u64) {
    state.restarts.asked += 1;
    let asked = StartRead {
        generation: state.restarts.generation.unwrap_or_default(),
        number: state.restarts.asked,
    };
    let app = ctx.state.clone();
    let (host, id) = (state.host_id.clone(), state.found.clone());
    let id = id.unwrap_or_else(|| state.id.clone());
    let task = Task::perform(
        async move {
            if !wait.is_zero() {
                tokio::time::sleep(wait).await;
            }
            let document = containers_inspect(&app, host, id).await.ok()?;
            let started = document.pointer("/State/StartedAt")?.as_str()?;
            Some(started.to_owned())
        },
        move |started| Message::Started(asked, started),
    );
    (task, asked.number)
}

/// Asks for the start once [`READ_EVERY`] passed since the last read,
/// for the container's entry `entry`.
fn read_after_end(state: &mut State, entry: Option<Container>, ctx: &Context<'_>) -> Task<Message> {
    let Some(end) = &mut state.restarts.ended else {
        return Task::none();
    };
    let wait = READ_EVERY.saturating_sub(end.asked_at.elapsed());
    end.asked_at = Instant::now() + wait;
    end.read_for = entry;
    let (task, number) = read(state, wait, ctx);
    if let Some(end) = &mut state.restarts.ended {
        end.on_its_way = Some(number);
    }
    task
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
            opened_at: Some(Timestamp::now()),
            asked: state.restarts.asked,
            ..Restarts::default()
        };
        opened = read(state, Duration::ZERO, ctx).0;
    }
    if !state.logs.has_ended() || state.logs.failure.is_some() {
        return opened;
    }
    let container = state.container(ctx);
    let running = container.as_ref().is_some_and(|c| c.state == "running");
    let Some(end) = &mut state.restarts.ended else {
        state.restarts.ended = Some(Ended {
            stopped_since: !running,
            read_for: None,
            // The first read after the end goes at once.
            asked_at: Instant::now()
                .checked_sub(READ_EVERY)
                .unwrap_or_else(Instant::now),
            on_its_way: None,
        });
        return Task::batch([opened, read_after_end(state, container, ctx)]);
    };
    if !running {
        end.stopped_since = true;
        return opened;
    }
    if end.stopped_since {
        return again(state, ctx);
    }
    // A read on its way is followed by another once it answers, if the
    // entry is not the one it was asked for.
    if container == end.read_for || end.on_its_way.is_some() {
        return opened;
    }
    Task::batch([opened, read_after_end(state, container, ctx)])
}

/// Whether `started` is later than the moment the stream opened.
fn after_open(started: &str, opened_at: Option<Timestamp>) -> bool {
    let started = started.parse::<Timestamp>().ok();
    started
        .zip(opened_at)
        .is_some_and(|(started, opened)| started > opened)
}

/// The start the read `asked` gave: the one the stream follows if none
/// was read yet; once it ended, another one means a restart.
pub(super) fn answered(
    state: &mut State,
    asked: StartRead,
    started: Option<String>,
    ctx: &Context<'_>,
) -> Task<Message> {
    let restarts = &mut state.restarts;
    if restarts.generation != Some(asked.generation) {
        return Task::none();
    }
    let ended = restarts.ended.is_some();
    if let Some(end) = &mut restarts.ended {
        if end.on_its_way == Some(asked.number) {
            end.on_its_way = None;
        }
    }
    let open = state.tab == Tab::Logs && state.logs.has_ended() && state.logs.failure.is_none();
    if let Some(started) = started {
        let restarts = &mut state.restarts;
        let restarted = match &restarts.started {
            Some(before) => *before != started,
            // The read as the stream opened failed: a start later than
            // that moment is a restart.
            None if ended => after_open(&started, restarts.opened_at),
            None => false,
        };
        if restarted && ended {
            if open {
                return again(state, ctx);
            }
        } else if restarts.started.is_none() {
            restarts.started = Some(started);
        }
    }
    if !open {
        return Task::none();
    }
    // The entry changed while this read was on its way: what it read may
    // be from before; the entry as it is now is read too.
    let container = state.container(ctx);
    match &state.restarts.ended {
        Some(end) if end.on_its_way.is_none() && container != end.read_for => {
            read_after_end(state, container, ctx)
        }
        _ => Task::none(),
    }
}

/// Opens the logs anew, on an empty viewer, and reads the start of the
/// stream that opens.
fn again(state: &mut State, ctx: &Context<'_>) -> Task<Message> {
    state.logs.restart(Timestamp::now().as_second());
    follow(state, ctx)
}
