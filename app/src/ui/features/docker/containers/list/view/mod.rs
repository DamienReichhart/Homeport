//! What the Containers page draws.

mod columns;

use iced::{
    widget::{toggler, Space},
    Length,
};

pub(super) use self::columns::columns;
use super::{update::SEARCH, Message, State};
use crate::{
    t,
    ui::{
        domain::containers::{actions::view::selection_bar, Actions},
        shell::{
            layout::page_header,
            registry::PAGE_ACTION,
            shortcuts::keys::{format, is_mac},
            Context,
        },
        theme::{
            metrics::{control, space, text as size},
            Element, Icon,
        },
        widgets::{
            basic::button,
            feedback::{empty_state, error_message, spinner},
            form, line,
            overlay::{tooltip_with_keys, Modal, Side},
            stack,
            table::Table,
        },
    },
};

/// "Run container", which `n` presses too.
fn run_button<'a>() -> Element<'a, Message> {
    let label = t!("containers.runContainer");
    let run = button::labelled(Icon::Plus, label.clone()).on_press(Message::Run);
    tooltip_with_keys(run, label, &format(PAGE_ACTION, is_mac()), Side::Bottom).into()
}

pub fn view<'a>(state: &'a State, ctx: &Context<'a>) -> Element<'a, Message> {
    let containers = state.containers(ctx);
    // `isPending ? <Spinner />`, then the failure and nothing else,
    // whatever was read before.
    if containers.is_pending() {
        return spinner();
    }
    if let Some(error) = containers.error() {
        return error_message(error, true, Message::Failure);
    }
    let list = containers.data().map_or(&[][..], |list| list.as_slice());
    let (running, count) = state.counts;
    let header = page_header(
        t!("containers.title"),
        Some(t!("containers.summary", running = running, count = count)),
        vec![state.check.button().map(Message::Check), run_button()],
    );
    let only_running = toggler(state.only_running)
        .label(t!("containers.onlyRunning"))
        .size(control::SWITCH_HEIGHT)
        .text_size(size::SM)
        .on_toggle(Message::OnlyRunning);
    // The selection's buttons end the line of the filters, as in React:
    // the rows don't move when the first one is selected.
    let selected = state.table.selection().len();
    let filters = line(vec![
        form::search(
            SEARCH,
            &t!("containers.search"),
            &state.query,
            Message::Search,
        ),
        only_running.into(),
        Space::new().width(Length::Fill).into(),
        selection_bar(selected, Message::Bulk),
    ])
    .spacing(space::S3)
    .width(Length::Fill)
    // As tall as its field, with the buttons or without.
    .height(control::HEIGHT);
    let body = if list.is_empty() {
        let run = button::text(t!("containers.runContainer")).on_press(Message::Run);
        empty_state(t!("containers.empty.title"))
            .icon(Icon::Box)
            .description(t!("containers.empty.description"))
            .action(run)
            .view()
    } else {
        let look = state.look(ctx);
        Table::new(&state.table, columns(look), look.rows(list), Message::Table)
            .group_header(move |group, members| columns::stack_header(look, group, members))
            .row_menu(move |container| {
                Actions::menu(&look.target(container))
                    .into_iter()
                    .map(|item| item.map(&Message::Actions))
                    .collect()
            })
            .empty_label(t!("containers.noMatch"))
            .view()
    };
    stack(vec![header, filters.into(), body])
        .spacing(space::S4)
        .into()
}

/// The confirmation or the Rename dialog a container's menu opened, or
/// the Run dialog.
pub fn dialog<'a>(state: &'a State, ctx: &Context<'a>) -> Option<Modal<'a, Message>> {
    if let Some(modal) = state.actions.dialog(ctx) {
        return Some(modal.map(Message::Actions));
    }
    let run = state.run.as_deref()?;
    Some(run.view(ctx).map(Message::RunDialog))
}
