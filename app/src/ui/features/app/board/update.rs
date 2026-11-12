//! What a message changes on the Board.

use iced::Task;

use homeport_engine::containers::ContainerAction;

use super::{
    model::Filter,
    view::{columns, rows, stack_labels, state_labels},
    Message, State,
};
use crate::{
    data::keys,
    ui::{
        domain::containers::{actions, Bulk, Target},
        shell::{registry, Context},
        widgets::{
            feedback::{error_message::Event, Surfaced},
            form, table,
        },
    },
};

/// What `/` gives focus to.
pub(super) const SEARCH: &str = "board-search";

pub fn open(ctx: &Context<'_>) -> (State, Task<Message>) {
    // The filter is in the location, so it is still there after opening
    // a container and coming back.
    let mut state = State::new(Filter::read(ctx.location));
    state.derive(ctx);
    (state, Task::none())
}

pub fn refresh(state: &mut State, ctx: &Context<'_>) -> Task<Message> {
    state.derive(ctx);
    Task::none()
}

pub fn request(_state: &mut State, action: &str, _ctx: &Context<'_>) -> Task<Message> {
    match action {
        registry::SEARCH => form::focus_search(SEARCH),
        _ => Task::none(),
    }
}

/// The filter changed: the rows are gone through again, and the location
/// keeps it.
fn filtered(state: &mut State, ctx: &Context<'_>) {
    state.touch();
    let (query, params) = state.filter.write();
    ctx.keep(query, params);
}

/// The selected containers that are still on the Board.
fn selected(state: &State) -> Vec<Target> {
    let selection = state.table.selection();
    state
        .rows
        .iter()
        .filter(|row| selection.contains(&row.key))
        .map(super::model::Row::target)
        .collect()
}

pub fn update(state: &mut State, message: Message, ctx: &Context<'_>) -> Task<Message> {
    match message {
        Message::Table(message) => {
            let (table, look) = state.split();
            let opened = table.update(message, &rows(look), &columns(look));
            if let Some(table::Event::Opened(key)) = opened {
                let target = state.rows.iter().find(|row| row.key == key);
                if let Some(target) = target.map(super::model::Row::target) {
                    // Its Host is selected, then its logs open there.
                    let open = actions::Message::Open(target, "logs");
                    return state.actions.update(open, ctx).map(Message::Actions);
                }
            }
        }
        Message::Search(query) => {
            state.filter.query = query;
            filtered(state, ctx);
        }
        Message::HostsMenu(event) => state.hosts_menu.update(event),
        Message::ToggleHost(id) => {
            match state.filter.hosts.iter().position(|host| *host == id) {
                Some(at) => {
                    state.filter.hosts.remove(at);
                }
                None => state.filter.hosts.push(id),
            }
            filtered(state, ctx);
        }
        Message::StateSelect(message) => {
            if let Some(chosen) = state.state_select.update(message, &state_labels()) {
                state.filter.state = super::model::StateFilter::ALL[chosen];
                filtered(state, ctx);
            }
        }
        Message::StackSelect(message) => {
            let labels = stack_labels(&state.stacks);
            if let Some(chosen) = state.stack_select.update(message, &labels) {
                // The first is "Any Stack".
                state.filter.stack = chosen.checked_sub(1).map(|at| state.stacks[at].clone());
                filtered(state, ctx);
            }
        }
        Message::ClearFilters => {
            state.filter = Filter::default();
            filtered(state, ctx);
        }
        Message::Actions(message) => {
            return state.actions.update(message, ctx).map(Message::Actions);
        }
        Message::Bulk(bulk) => {
            let targets = selected(state);
            let act = |action| actions::Message::Act(action, targets.clone());
            let message = match bulk {
                Bulk::Start => act(ContainerAction::Start),
                Bulk::Stop => act(ContainerAction::Stop),
                Bulk::Restart => act(ContainerAction::Restart),
                // Named Host by Host: the selection may span them.
                Bulk::Remove => actions::Message::AskRemove {
                    targets,
                    by_host: true,
                },
            };
            return state.actions.update(message, ctx).map(Message::Actions);
        }
        Message::Usage((host, usage)) => {
            state.usage.insert(host, usage);
            // The numbers are read as the rows are drawn; their order only
            // changes when the Board is sorted by one of them.
            let sort = state.table.sort();
            if sort.of("cpu").is_some() || sort.of("memory").is_some() {
                state.touch();
            }
        }
        Message::Go(route) => ctx.open(route),
        Message::Failure(host, Event::Retry) => ctx.invalidate(keys::host::containers(&host)),
        Message::Failure(host, Event::Details) => {
            let list = ctx
                .data
                .read::<Vec<homeport_engine::containers::Container>>(&keys::host::containers(
                    &host,
                ));
            if let Some(error) = list.error() {
                ctx.details(Surfaced::new(error.clone(), "containers_list"));
            }
        }
        Message::Failure(_, Event::Copy(command)) => ctx.copy(command),
    }
    Task::none()
}
