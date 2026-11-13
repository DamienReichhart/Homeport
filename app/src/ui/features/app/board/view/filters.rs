//! The Board's filters (`BoardFilters`) and the Hosts it can't show
//! (`BoardNotices`).

use iced::widget::text;

use homeport_engine::containers::Container;
use homeport_hosts::service::HostView;

use super::super::{
    model::StateFilter,
    state::{docker_hosts, is_connected},
    update::SEARCH,
    Message, State,
};
use crate::{
    data::keys,
    t,
    ui::{
        shell::{router::Route, status, Context},
        theme::{
            self, font, icon,
            metrics::{space, text as size},
            Element, Icon,
        },
        widgets::{
            basic::{
                button::{self, Variant},
                technical,
            },
            feedback::error_message,
            focus::pressable,
            form::{self, select::select_owned},
            line,
            overlay::menu::{self, Item},
            stack,
        },
    },
};

/// The states to filter by, as the select lists them.
pub fn state_labels() -> Vec<String> {
    vec![
        t!("board.state.all"),
        t!("board.state.running"),
        t!("board.state.stopped"),
        t!("board.state.unhealthy"),
    ]
}

/// "Any Stack", then the Stacks.
pub fn stack_labels(stacks: &[String]) -> Vec<String> {
    std::iter::once(t!("board.filter.anyStack"))
        .chain(stacks.iter().cloned())
        .collect()
}

/// The search, the Hosts, the state and the Stack to show, and what
/// clears them all.
pub fn filters<'a>(state: &'a State, ctx: &Context<'a>) -> Element<'a, Message> {
    let filter = &state.filter;
    let hosts_label = match filter.hosts.len() {
        0 => t!("board.filter.allHosts"),
        count => t!("board.filter.someHosts", count = count),
    };
    let trigger = button::labelled(Icon::ChevronDown, hosts_label)
        .variant(Variant::Outline)
        .on_press(Message::HostsMenu(menu::Event::Toggle));
    let hosts = ctx
        .world
        .host_list()
        .iter()
        .map(|view| {
            let id = view.host.id.clone();
            let shown = filter.hosts.contains(&id);
            Item::action(technical(&view.host.name), Message::ToggleHost(id)).checked(shown)
        })
        .collect();
    let chosen_state = StateFilter::ALL.iter().position(|s| *s == filter.state);
    let chosen_stack = match &filter.stack {
        None => Some(0),
        Some(stack) => state
            .stacks
            .iter()
            .position(|s| s == stack)
            .map(|at| at + 1),
    };
    let mut parts: Vec<Element<'a, Message>> = vec![
        form::search(SEARCH, &t!("board.search"), &filter.query, Message::Search),
        menu::dropdown(&state.hosts_menu, trigger, hosts, Message::HostsMenu),
        select_owned(&state.state_select, state_labels(), chosen_state)
            .width(144.0)
            .view()
            .map(Message::StateSelect),
        select_owned(
            &state.stack_select,
            stack_labels(&state.stacks),
            chosen_stack,
        )
        // A Stack that is filtered by and no longer there.
        .placeholder(filter.stack.clone().unwrap_or_default())
        .width(160.0)
        .view()
        .map(Message::StackSelect),
    ];
    if filter.is_set() {
        parts.push(
            button::text(t!("board.filter.clear"))
                .variant(Variant::Ghost)
                .on_press(Message::ClearFilters)
                .into(),
        );
    }
    line(parts).spacing(space::S2).into()
}

/// One line per Host the filter shows and the Board can't: not
/// connected, or its list failed. `None` when every one is there.
pub fn notices<'a>(state: &'a State, ctx: &Context<'a>) -> Option<Element<'a, Message>> {
    let shown = |view: &&HostView| state.filter.shows_host(&view.host.id);
    let mut lines: Vec<Element<'a, Message>> = vec![];
    for view in docker_hosts(ctx).filter(shown) {
        if is_connected(view) {
            continue;
        }
        let says = t!(
            "board.skipped",
            name = technical(&view.host.name),
            status = status::host::describe(&view.status).label
        );
        let open = pressable(text(t!("board.openHost")).size(size::SM))
            .padding(0.0)
            .style(theme::button::link)
            .on_press(Message::Go(Route::Host {
                host_id: view.host.id.clone(),
            }));
        lines.push(
            line(vec![
                icon(Icon::TriangleAlert).style(theme::svg::warning).into(),
                text(says).size(size::SM).style(theme::text::muted).into(),
                open.into(),
            ])
            .spacing(space::S2)
            .into(),
        );
    }
    for view in docker_hosts(ctx)
        .filter(shown)
        .filter(|view| is_connected(view))
    {
        let id = view.host.id.clone();
        let list = ctx
            .data
            .read::<Vec<Container>>(&keys::host::containers(&id));
        let Some(error) = list.error() else {
            continue;
        };
        let title = text(t!("board.listFailed", name = technical(&view.host.name)))
            .size(size::SM)
            .font(font::medium());
        let failure = error_message(error, true, move |event| {
            Message::Failure(id.clone(), event)
        });
        lines.push(stack(vec![title.into(), failure]).spacing(space::S1).into());
    }
    (!lines.is_empty()).then(|| stack(lines).spacing(space::S2).into())
}
