//! What a message changes on the Containers page.

use iced::Task;

use homeport_engine::containers::{Container, ContainerAction};

use super::{view::columns, Message, State};
use crate::{
    data::{keys, resources},
    ui::{
        domain::containers::{actions, Bulk, Target},
        shell::{
            registry::{self, Capability},
            router::Route,
            status, Context,
        },
        widgets::{
            feedback::{error_message::Event, Surfaced},
            form, table,
        },
    },
};

/// What `/` gives focus to.
pub(super) const SEARCH: &str = "containers-search";

/// The Host's id: the page shows behind the Host's gate.
fn host_id<'a>(ctx: &Context<'a>) -> &'a str {
    ctx.subject.host_id().unwrap_or_default()
}

pub fn open(ctx: &Context<'_>) -> (State, Task<Message>) {
    let host = host_id(ctx);
    ctx.watch(resources::containers(host));
    ctx.watch(resources::freshness(host));
    // Podman says which pod a container is in.
    if status::engine::supports(ctx.host(), Capability::Podman) {
        ctx.watch(resources::pods(host));
    }
    let mut state = State::new(host, ctx.location.query.as_deref());
    state.derive(ctx);
    (state, Task::none())
}

pub fn refresh(state: &mut State, ctx: &Context<'_>) -> Task<Message> {
    state.derive(ctx);
    Task::none()
}

pub fn request(state: &mut State, action: &str, ctx: &Context<'_>) -> Task<Message> {
    match action {
        registry::SEARCH => form::focus_search(SEARCH),
        "run" => update(state, Message::Run, ctx),
        _ => Task::none(),
    }
}

/// The selected containers that are still in the list.
fn selected(state: &State, list: &[Container], ctx: &Context<'_>) -> Vec<Target> {
    let name = ctx.host().map_or("", |view| view.host.name.as_str());
    let selection = state.table.selection();
    list.iter()
        .filter(|container| selection.contains(&container.id))
        .map(|container| Target::of(&state.host_id, name, container))
        .collect()
}

pub fn update(state: &mut State, message: Message, ctx: &Context<'_>) -> Task<Message> {
    match message {
        Message::Table(message) => {
            let containers = state.containers(ctx);
            let list = containers.data().map_or(&[][..], |list| list.as_slice());
            let (table, look) = state.split(ctx);
            let rows = look.rows(list);
            if let Some(table::Event::Opened(id)) = table.update(message, &rows, &columns(look)) {
                ctx.open(Route::Container {
                    id,
                    tab: Some("logs".to_owned()),
                });
            }
        }
        Message::Search(query) => state.query = query,
        Message::OnlyRunning(only) => state.only_running = only,
        Message::Actions(message) => {
            return state.actions.update(message, ctx).map(Message::Actions);
        }
        Message::Bulk(bulk) => {
            let containers = state.containers(ctx);
            let list = containers.data().map_or(&[][..], |list| list.as_slice());
            let targets = selected(state, list, ctx);
            let act = |action| actions::Message::Act(action, targets.clone());
            let message = match bulk {
                Bulk::Start => act(ContainerAction::Start),
                Bulk::Stop => act(ContainerAction::Stop),
                Bulk::Restart => act(ContainerAction::Restart),
                Bulk::Remove => actions::Message::AskRemove {
                    targets,
                    by_host: false,
                },
            };
            return state.actions.update(message, ctx).map(Message::Actions);
        }
        Message::Check(message) => {
            let host = state.host_id.clone();
            return state.check.update(&host, message, ctx).map(Message::Check);
        }
        Message::Usage((host, usage)) => {
            // A report of the Host the page was on before is not this one's.
            if host == state.host_id {
                state.usage = usage;
            }
        }
        Message::Run => state.run_asked = true,
        Message::CloseRun => state.run_asked = false,
        Message::OpenPort(port) => {
            if let Some(view) = ctx.host() {
                let address = status::host::address(&view.host.connection);
                ctx.open_url(format!("http://{address}:{port}"));
            }
        }
        Message::Go(route) => ctx.open(route),
        Message::Failure(Event::Retry) => ctx.invalidate(keys::host::containers(&state.host_id)),
        Message::Failure(Event::Details) => {
            if let Some(error) = state.containers(ctx).error() {
                ctx.details(Surfaced::new(error.clone(), "containers_list"));
            }
        }
        Message::Failure(Event::Copy(command)) => ctx.copy(command),
    }
    Task::none()
}
