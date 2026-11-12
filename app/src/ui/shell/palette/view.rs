//! The palette as it is drawn: a search field over the rows, under their
//! headings (`CommandPalette.tsx`).

use iced::{
    keyboard::{key::Named, Key},
    widget::{container, rule, scrollable, text},
    Length,
};

use super::{
    items::{sections, Item},
    Message, State, HEADING, LIST_HEIGHT, ROW,
};
use crate::{
    t,
    ui::{
        shell::shortcuts::keys,
        theme::{
            self, font, icon,
            metrics::{space, text as size},
            Element, Icon,
        },
        widgets::{
            basic,
            feedback::status_dot,
            filling,
            focus::{key_area, pressable},
            form::{input, input_group, Addon},
            line,
            overlay::{modal, Modal},
            stack, start,
        },
    },
};

fn row<'a>(item: &Item, index: usize, active: bool, mac: bool) -> Element<'a, Message> {
    let mut parts: Vec<Element<'a, Message>> = vec![];
    parts.extend(
        item.icon
            .map(|glyph| icon(glyph).style(theme::svg::muted).into()),
    );
    parts.push(filling(text(item.label.clone()).size(size::SM)).into());
    if let Some((label, tone)) = &item.status {
        parts.push(
            line(vec![
                status_dot(*tone),
                text(label.clone())
                    .size(size::XS)
                    .style(theme::text::muted)
                    .into(),
            ])
            .spacing(space::S1_5)
            .into(),
        );
    }
    if let Some(shortcut) = &item.shortcut {
        parts.push(basic::keys(&keys::format(shortcut, mac)));
    }
    pressable(
        container(line(parts).spacing(space::S2).width(Length::Fill))
            .center_y(Length::Fill)
            .width(Length::Fill),
    )
    .width(Length::Fill)
    .height(ROW)
    .padding([0.0, space::S2])
    // The field keeps the keyboard: the arrows move the highlight.
    .tab_stop(false)
    .focus_on_press(false)
    .on_press(Message::Choose(index))
    .style(theme::button::part::item(active))
    .into()
}

/// The palette over the page while it is open.
pub fn palette<'a>(state: &'a State, items: &[Item], mac: bool) -> Option<Modal<'a, Message>> {
    if !state.is_open() {
        return None;
    }
    let mut rows: Vec<Element<'a, Message>> = vec![];
    let active = state.highlighted(state.listed(items).len());
    let mut index = 0;
    for section in sections(items, state.query(), state.recent()) {
        rows.push(
            container(
                text(section.group.label())
                    .size(size::XS)
                    .font(font::medium())
                    .style(theme::text::muted),
            )
            .padding([0.0, space::S2])
            .center_y(HEADING)
            .width(Length::Fill)
            .align_x(start())
            .into(),
        );
        for item in section.items {
            rows.push(row(item, index, index == active, mac));
            index += 1;
        }
    }
    let list: Element<'a, Message> = if index == 0 {
        container(text(t!("app.palette.empty")).size(size::SM))
            .padding([space::S6, 0.0])
            .center_x(Length::Fill)
            .into()
    } else {
        container(scrollable(container(stack(rows)).padding(space::S1)).id(state.list.clone()))
            .max_height(LIST_HEIGHT)
            .into()
    };
    let search = input(&t!("app.palette.placeholder"), state.query())
        .on_input(Message::Query)
        .on_submit(Message::Submit)
        .style(theme::text_input::bare);
    let field =
        container(input_group(search, Some(Addon::icon(Icon::Search)), None)).padding(space::S1);
    let body = container(stack(vec![field.into(), rule::horizontal(1).into(), list]))
        .style(theme::container::dialog);
    // The arrows the field leaves alone move the highlight.
    let keyed = key_area(body, |key, _| match key {
        Key::Named(Named::ArrowDown) => Some(Message::Step(1)),
        Key::Named(Named::ArrowUp) => Some(Message::Step(-1)),
        _ => None,
    });
    Some(
        modal(keyed)
            .key("palette")
            .close_on_scrim()
            .on_close(Message::Close),
    )
}
