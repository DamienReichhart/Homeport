//! What a container's page draws: its header, its tabs, and the tab that
//! shows.

mod logs;
mod stats;

use iced::{widget::text, Length};

use homeport_engine::containers::Container;

use super::{
    state::{gone, Tab},
    Message, State,
};
use crate::{
    i18n::Direction,
    t,
    ui::{
        domain::{
            containers::{model, Actions},
            freshness,
        },
        shell::{layout::page_header_of, Context},
        theme::{
            self, font,
            metrics::{space, text as size, ButtonSize},
            Element, Icon,
        },
        widgets::{
            basic::{
                badge,
                button::{self, Variant},
                flow, in_reading_order, technical, truncated, BadgeVariant,
            },
            feedback::{empty_state, error_message, spinner, status_dot},
            form::{tabs, TabsVariant},
            line,
            overlay::{menu, tooltip, Modal, Side},
            stack,
        },
    },
};

/// What React wrote between two facts on one line (`· Up 3 hours`): a
/// sign, of no language.
const MIDDLE_DOT: char = '\u{b7}';

/// `fact` after the dot that parts it from what stands before it on its
/// line, in the order the language reads.
pub(super) fn after_dot(fact: &str) -> String {
    in_reading_order(&format!("{MIDDLE_DOT} {fact}"))
}

/// Text that takes the room its row leaves and is cut where that ends:
/// what stands after it keeps its place.
fn fill<'a>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    iced::widget::container(content)
        .width(Length::Fill)
        .align_x(crate::ui::widgets::start())
        .into()
}

/// The arrow that leaves the page, pointing where the language comes
/// from (`rtl:-scale-x-100`).
fn back<'a>(ctx: &Context<'a>) -> Element<'a, Message> {
    let glyph = match ctx.direction {
        Direction::Ltr => Icon::ArrowLeft,
        Direction::Rtl => Icon::ArrowRight,
    };
    let button = button::icon(glyph)
        .variant(Variant::Ghost)
        .size(ButtonSize::Sm)
        .on_press(Message::Back);
    tooltip(button, t!("common.back"), Side::Bottom).into()
}

/// The header: the way back, the container's state, its name and its
/// Stack; under them its image, whether a newer one was published, and
/// its status; at the end, what can be done to it.
fn header<'a>(state: &'a State, container: &Container, ctx: &Context<'a>) -> Element<'a, Message> {
    let name = truncated(technical(&container.name))
        .size(size::XL)
        .font(font::semibold())
        .with_tip();
    let mut title: Vec<Element<'a, Message>> = vec![
        back(ctx),
        status_dot(model::tone(&container.state, container.health.as_deref())),
        fill(name),
    ];
    if let Some(stack) = &container.stack {
        title.push(badge(technical(stack), BadgeVariant::Outline));
    }
    let image = truncated(technical(&container.image))
        .size(size::SM)
        .style(theme::text::muted)
        .with_tip();
    let mut about: Vec<Element<'a, Message>> = vec![image.into()];
    let checked = ctx
        .data
        .read::<Vec<homeport_engine::images::freshness::FreshnessResult>>(
            &crate::data::keys::host::freshness(&state.host_id),
        );
    let index = freshness::index(checked.data().map(|list| list.as_slice()));
    if index.of_container(container) == homeport_engine::images::freshness::Freshness::Newer {
        about.push(freshness::newer_mark(freshness::newer_label(&[
            &container.image
        ])));
    }
    let status = if container.status.is_empty() {
        &container.state
    } else {
        &container.status
    };
    about.push(
        text(after_dot(&technical(status)))
            .size(size::SM)
            .style(theme::text::muted)
            .into(),
    );
    let target = state.target(container, ctx);
    let mut actions: Vec<Element<'a, Message>> = state
        .actions
        .header(&target)
        .into_iter()
        .map(|button| button.map(Message::Actions))
        .collect();
    let more = button::icon(Icon::EllipsisVertical)
        .variant(Variant::Outline)
        .on_press(Message::More(menu::Event::Toggle));
    let items = Actions::more(&target)
        .into_iter()
        .map(|item| item.map(&Message::Actions))
        .collect();
    actions.push(menu::dropdown(
        &state.more,
        tooltip(more, t!("common.moreActions"), Side::Bottom),
        items,
        Message::More,
    ));
    page_header_of(
        line(title).spacing(space::S2).width(Length::Fill),
        // What lacks room goes on a second line, each part whole.
        Some(flow(about).spacing(space::S1_5).into()),
        actions,
    )
}

/// What a tab shows of the container.
fn body<'a>(state: &'a State, container: &Container, _ctx: &Context<'a>) -> Element<'a, Message> {
    let running = container.state == "running";
    let window = state.window;
    match state.tab {
        Tab::Logs => logs::view(&state.logs, window.map(|size| size.height)).map(Message::Logs),
        Tab::Stats => {
            stats::view(&state.stats, running, window.map(|size| size.width)).map(Message::Stats)
        }
        // The tabs that are not ported yet say so, as the stub of a page.
        _ => empty_state(state.tab.label())
            .description(t!("ui.table.empty"))
            .view(),
    }
}

pub fn view<'a>(state: &'a State, ctx: &Context<'a>) -> Element<'a, Message> {
    let containers = state.containers(ctx);
    let list = containers.data().map_or(&[][..], |list| list.as_slice());
    let Some(container) = state.find(list) else {
        // `isPending ? <Spinner />`; a list that could not be read is
        // said as such, where React said the container was gone.
        if containers.is_pending() {
            return spinner();
        }
        if let (Some(error), None) = (containers.error(), containers.data()) {
            return error_message(error, true, Message::Failure);
        }
        let back = button::text(t!("containers.detail.backToList"))
            .variant(Variant::Outline)
            .on_press(Message::BackToList);
        return stack(vec![
            error_message(&gone(), false, Message::Failure),
            back.into(),
        ])
        .spacing(space::S3)
        .into();
    };
    let shown = Tab::shown(container);
    let chosen = shown.iter().position(|tab| *tab == state.tab);
    let labels = shown.iter().map(|tab| tab.label()).collect();
    let pick = move |index: usize| Message::Tab(shown[index]);
    stack(vec![
        header(state, container, ctx),
        tabs(
            labels,
            chosen.unwrap_or(usize::MAX),
            TabsVariant::Default,
            pick,
        ),
        body(state, container, ctx),
    ])
    .spacing(space::S4)
    .into()
}

/// The confirmation or the Rename dialog the header opened.
pub fn dialog<'a>(state: &'a State, _ctx: &Context<'a>) -> Option<Modal<'a, Message>> {
    state
        .actions
        .dialog()
        .map(|modal| modal.map(Message::Actions))
}
