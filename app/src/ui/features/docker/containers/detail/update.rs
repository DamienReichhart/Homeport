//! What a message changes on a container's page.

use std::collections::HashSet;

use iced::Task;

use super::{
    queries,
    state::{gone, Tab},
    tabs::{files, health, inspect, processes},
    view, Message, State,
};
use crate::{
    data::{keys, resources, Key},
    ui::{
        domain::containers::actions,
        shell::{router::Route, Context},
        widgets::{
            data::file_browser,
            feedback::{error_message::Event, Surfaced},
            table::Rows,
        },
    },
};

pub fn open(ctx: &Context<'_>) -> (State, Task<Message>) {
    let mut state = State::new(ctx);
    ctx.watch(resources::containers(&state.host_id));
    ctx.watch(resources::freshness(&state.host_id));
    state.seen_state = state.container(ctx).map(|container| container.state);
    let read = needs(&mut state, ctx);
    // The log viewer takes the height the window leaves the page.
    let size = iced::window::latest()
        .and_then(iced::window::size)
        .map(Message::Resized);
    (state, Task::batch([size, read]))
}

/// Asks for what the tab that shows reads of the container, and takes
/// what was read since the last look. A tab's read starts when the tab
/// first shows, and is another one when what it depends on changes (the
/// container's state for its inspect document).
fn needs(state: &mut State, ctx: &Context<'_>) -> Task<Message> {
    let mut wanted = HashSet::new();
    let task = reads(state, &mut wanted, ctx);
    // What another tab, another folder or another state read is not
    // kept fresh any more: an Engine event no longer reads it again.
    state.unwatch_all_but(&wanted, ctx);
    task
}

/// What the tab that shows reads, watched and counted in `wanted`.
fn reads(state: &mut State, wanted: &mut HashSet<Key>, ctx: &Context<'_>) -> Task<Message> {
    let Some(container) = state.container(ctx) else {
        return Task::none();
    };
    state.found = Some(container.id.clone());
    let host = state.host_id.clone();
    match state.tab {
        Tab::Inspect => {
            state.watch(
                queries::inspect(&host, &container.id, &container.state),
                wanted,
                ctx,
            );
            let read = state.inspected(&container, ctx);
            state.inspect.take(read.data()).map(Message::Inspect)
        }
        // `enabled: running`: a stopped container's processes are not
        // asked for.
        Tab::Processes if container.state == "running" => {
            state.watch(queries::processes(&host, &container.id), wanted, ctx);
            let read = state.processes_read(&container, ctx);
            state.processes.take(read.data());
            Task::none()
        }
        Tab::Health => {
            let status = container.health.as_deref();
            state.watch(queries::health(&host, &container.id, status), wanted, ctx);
            Task::none()
        }
        // `enabled: running`.
        Tab::Files if container.state == "running" => {
            let path = state.files.path.clone();
            state.watch(queries::files(&host, &container.id, &path), wanted, ctx);
            let read = state.files_read(&container, ctx);
            state.files.take(&read);
            Task::none()
        }
        _ => Task::none(),
    }
}

/// What the Inspect tab asked the page for.
fn inspect(state: &mut State, message: inspect::Message, ctx: &Context<'_>) -> Task<Message> {
    let (task, event) = state.inspect.update(message, ctx);
    if let (Some(event), Some(container)) = (event, state.container(ctx)) {
        let key = queries::inspect_key(&state.host_id, &container.id, &container.state);
        match event {
            inspect::Event::Retry => ctx.invalidate(key),
            inspect::Event::Details => {
                if let Some(error) = state.inspected(&container, ctx).error() {
                    ctx.details(Surfaced::new(error.clone(), "containers_inspect"));
                }
            }
        }
    }
    task.map(Message::Inspect)
}

/// What the Processes tab asked for.
fn processes(state: &mut State, message: processes::Message, ctx: &Context<'_>) {
    let Some(container) = state.container(ctx) else {
        return;
    };
    let key = queries::processes_key(&state.host_id, &container.id);
    match message {
        processes::Message::Table(message) => {
            let tab = &mut state.processes;
            let columns = view::processes::columns(&tab.titles);
            let rows = Rows::new(&tab.rows, |row| row.at);
            // Rows are not opened nor selected: only widths and scroll.
            tab.table.update(message, &rows, &columns);
        }
        processes::Message::Poll | processes::Message::Failure(Event::Retry) => {
            ctx.invalidate(key);
        }
        processes::Message::Failure(Event::Details) => {
            if let Some(error) = state.processes_read(&container, ctx).error() {
                ctx.details(Surfaced::new(error.clone(), "containers_processes"));
            }
        }
        processes::Message::Failure(Event::Copy(command)) => ctx.copy(command),
    }
}

/// What the Health tab asked for.
fn health(state: &mut State, message: health::Message, ctx: &Context<'_>) {
    let Some(container) = state.container(ctx) else {
        return;
    };
    let status = container.health.as_deref();
    let key = queries::health_key(&state.host_id, &container.id, status);
    match message {
        health::Message::Fold(start) => state.health.fold(start),
        health::Message::Poll | health::Message::Failure(Event::Retry) => ctx.invalidate(key),
        health::Message::Failure(Event::Details) => {
            if let Some(error) = state.health_read(&container, ctx).error() {
                ctx.details(Surfaced::new(error.clone(), "containers_health"));
            }
        }
        health::Message::Failure(Event::Copy(command)) => ctx.copy(command),
    }
}

/// What the Files tab asked for. A transfer is about the container the
/// page shows: the page is opened anew for another one. Its messages are
/// handled whether or not the list has the container at the moment (it
/// may be read again): by its id as it was last found.
fn files(state: &mut State, message: files::Message, ctx: &Context<'_>) -> Task<Message> {
    let task = match message {
        files::Message::Progress(number, step) => {
            state.files.progress(number, step);
            Task::none()
        }
        files::Message::Ended(number) => {
            state.files.ended(number);
            Task::none()
        }
        files::Message::Cancel => {
            state.files.cancel();
            Task::none()
        }
        message => {
            let Some(id) = state.found.clone() else {
                return Task::none();
            };
            let host = state.host_id.clone();
            let key = queries::files_key(&host, &id, &state.files.path);
            match message {
                files::Message::Browser(message) => {
                    let (task, event) = state.files.browse(message, ctx);
                    match event {
                        Some(file_browser::Event::Refresh) => ctx.invalidate(key.clone()),
                        Some(file_browser::Event::Failure(Event::Details)) => {
                            if let Some(error) = &state.files.failure {
                                ctx.details(Surfaced::new(error.clone(), "files_list"));
                            }
                        }
                        Some(file_browser::Event::Failure(Event::Copy(command))) => {
                            ctx.copy(command);
                        }
                        _ => {}
                    }
                    task
                }
                files::Message::Upload => state.files.ask_upload(ctx).0,
                files::Message::SaveTo(target) => state.files.save_to(target, &host, &id, ctx),
                files::Message::UploadFrom(sources) => {
                    state.files.upload_from(sources, &host, &id, ctx)
                }
                _ => Task::none(),
            }
        }
    };
    // Another folder may show now: it is read.
    Task::batch([task.map(Message::Files), needs(state, ctx)])
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
    needs(state, ctx)
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
    needs(state, ctx)
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
        Message::Inspect(message) => return inspect(state, message, ctx),
        Message::Processes(message) => processes(state, message, ctx),
        Message::Health(message) => health(state, message, ctx),
        Message::Files(message) => return files(state, message, ctx),
        Message::Logs(message) => return state.logs.update(message, ctx).map(Message::Logs),
    }
    Task::none()
}
