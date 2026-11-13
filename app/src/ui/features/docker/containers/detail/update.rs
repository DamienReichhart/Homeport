//! What a message changes on a container's page.

use iced::Task;

use super::{
    state::{gone, Tab},
    Message, State,
};
use crate::{
    data::{keys, resources},
    ui::{
        domain::containers::actions,
        shell::{router::Route, Context},
        widgets::feedback::{error_message::Event, Surfaced},
    },
};

pub fn open(ctx: &Context<'_>) -> (State, Task<Message>) {
    let mut state = State::new(ctx);
    ctx.watch(resources::containers(&state.host_id));
    ctx.watch(resources::freshness(&state.host_id));
    state.seen_state = state.container(ctx).map(|container| container.state);
    // The log viewer takes the height the window leaves the page.
    let size = iced::window::latest()
        .and_then(iced::window::size)
        .map(Message::Resized);
    (state, size)
}

/// The moment a stream is opened at, in Unix seconds.
fn now() -> i64 {
    jiff::Timestamp::now().as_second()
}

pub fn refresh(state: &mut State, ctx: &Context<'_>) -> Task<Message> {
    let now_state = state.container(ctx).map(|container| container.state);
    if now_state != state.seen_state {
        let started = now_state.as_deref() == Some("running");
        state.seen_state = now_state;
        // A container that starts again writes again: its stream ended
        // with it, so the logs that show are opened anew. React's stayed
        // on "end of logs" until an option of the viewer changed.
        if started && state.tab == Tab::Logs && state.logs.is_over() {
            state.logs.restart(now());
        }
    }
    Task::none()
}

pub fn request(_state: &mut State, _action: &str, _ctx: &Context<'_>) -> Task<Message> {
    Task::none()
}

/// Shows `tab`, and keeps it in the page's route: the window finds the
/// page on it again, and nothing is added to the history
/// (`navigate(.., { replace: true })`).
fn show(state: &mut State, tab: Tab, ctx: &Context<'_>) -> Task<Message> {
    if state.tab == tab {
        return Task::none();
    }
    state.tab = tab;
    ctx.keep_route(state.route(tab));
    // A tab that comes back starts over, as a React tab mounted again:
    // its stream was let go of when it was left.
    match tab {
        Tab::Logs => state.logs.restart(now()),
        Tab::Stats => state.stats.restart(),
        _ => {}
    }
    Task::none()
}

/// What the header's buttons and dialogs did. A container that was
/// removed has no page: the window goes to the list, as React's
/// `navigate('/containers')` once the removal ended.
fn acted(state: &mut State, message: actions::Message, ctx: &Context<'_>) -> Task<Message> {
    let removed = match &message {
        actions::Message::Removed(results) => results.iter().any(|(_, result)| result.is_ok()),
        _ => false,
    };
    let task = state.actions.update(message, ctx).map(Message::Actions);
    if removed {
        ctx.open(Route::Containers);
    }
    task
}

pub fn update(state: &mut State, message: Message, ctx: &Context<'_>) -> Task<Message> {
    match message {
        Message::Back if state.came_from_elsewhere => ctx.back(),
        Message::Back | Message::BackToList => ctx.open(Route::Containers),
        Message::Tab(tab) => return show(state, tab, ctx),
        Message::Actions(message) => return acted(state, message, ctx),
        Message::More(event) => state.more.update(event),
        Message::Failure(Event::Retry) => ctx.invalidate(keys::host::containers(&state.host_id)),
        Message::Failure(Event::Details) => {
            match state.containers(ctx).error() {
                Some(error) => ctx.details(Surfaced::new(error.clone(), "containers_list")),
                // The container is gone: that is what the page says.
                None => ctx.details(Surfaced::in_window(gone())),
            }
        }
        Message::Failure(Event::Copy(command)) => ctx.copy(command),
        Message::Resized(size) => state.window = Some(size),
        Message::Stats(message) => state.stats.update(message, ctx),
        Message::Logs(message) => return state.logs.update(message, ctx).map(Message::Logs),
    }
    Task::none()
}
