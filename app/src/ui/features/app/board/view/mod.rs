//! What the Board draws.

mod columns;
mod filters;

pub(super) use self::{
    columns::{columns, rows},
    filters::{stack_labels, state_labels},
};
use homeport_engine::containers::Container;

use super::{
    model::Source,
    state::{docker_hosts, is_connected},
    Message, State,
};
use crate::{
    data::keys,
    t,
    ui::{
        domain::containers::{actions::view::selection_bar, Actions},
        shell::{layout::page_header, Context},
        theme::{metrics::space, Element, Icon},
        widgets::{
            feedback::{empty_state, spinner},
            overlay::Modal,
            stack,
            table::Table,
        },
    },
};

pub fn view<'a>(state: &'a State, ctx: &Context<'a>) -> Element<'a, Message> {
    let connected = docker_hosts(ctx).filter(|view| is_connected(view)).count();
    // Until the Hosts are known, and while every connected Host is still
    // being read for the first time.
    let pending = |source: &Source| {
        let key = keys::host::containers(&source.host_id);
        ctx.data.read::<Vec<Container>>(&key).is_pending()
    };
    let waiting = !state.sources.is_empty() && state.sources.iter().all(pending);
    if ctx.world.hosts.is_pending() || waiting {
        return spinner();
    }
    let failed = filters::notices(state, ctx);
    let (running, count) = state.counts;
    let summary = t!(
        "board.summary",
        running = running,
        count = count,
        hosts = connected
    );
    // The selection's buttons stand in the header, which has room for
    // them at any width: the rows don't move when the first is selected.
    let selected = state.table.selection().len();
    let mut parts = vec![page_header(
        t!("board.title"),
        Some(summary),
        vec![selection_bar(selected, Message::Bulk)],
    )];
    parts.extend(failed);
    if connected == 0 {
        parts.push(
            empty_state(t!("board.noHosts.title"))
                .icon(Icon::Server)
                .description(t!("board.noHosts.description"))
                .view(),
        );
    } else if state.rows.is_empty() {
        parts.push(
            empty_state(t!("board.empty.title"))
                .icon(Icon::LayoutGrid)
                .description(t!("board.empty.description"))
                .view(),
        );
    } else {
        let look = state.look();
        parts.push(filters::filters(state, ctx));
        parts.push(
            Table::new(&state.table, columns(look), rows(look), Message::Table)
                .row_menu(|row| {
                    Actions::menu(&row.target())
                        .into_iter()
                        .map(|item| item.map(&Message::Actions))
                        .collect()
                })
                .empty_label(t!("board.noMatch"))
                .view(),
        );
    }
    stack(parts).spacing(space::S4).into()
}

/// The confirmation or the Rename dialog a container's menu opened.
pub fn dialog<'a>(state: &'a State, _ctx: &Context<'a>) -> Option<Modal<'a, Message>> {
    state
        .actions
        .dialog()
        .map(|modal| modal.map(Message::Actions))
}
