//! The help sheet listing every active shortcut, opened with `?`
//! (`ShortcutsHelp.tsx`).

use iced::{widget::text, Length};

use super::{
    keymap::{grouped, Binding},
    keys,
};
use crate::{
    t,
    ui::{
        theme::{
            self, font,
            metrics::{space, text as size},
            Element,
        },
        widgets::{
            basic, filling, line,
            overlay::{dialog, modal, Modal},
            stack,
        },
    },
};

/// `sm:max-w-lg`: the sheet's width.
const WIDTH: f32 = 512.0;

/// The sheet over the page; Escape and a press outside send `on_close`.
pub fn sheet<'a, Message: Clone + 'a>(
    map: &[Binding],
    mac: bool,
    on_close: Message,
) -> Modal<'a, Message> {
    let mut sections: Vec<Element<'a, Message>> = vec![];
    for (group, list) in grouped(map) {
        let heading = text(group.label().to_uppercase())
            .size(size::XS)
            .font(font::medium())
            .style(theme::text::muted);
        let mut rows: Vec<Element<'a, Message>> = vec![heading.into()];
        for binding in list {
            rows.push(
                line(vec![
                    filling(text(binding.says.text()).size(size::SM)).into(),
                    basic::keys(&keys::format(&binding.keys, mac)),
                ])
                .spacing(space::S4)
                .width(Length::Fill)
                .into(),
            );
        }
        sections.push(stack(rows).spacing(space::S1).into());
    }
    let sheet = dialog(t!("app.shortcuts.title"))
        .description(t!("app.shortcuts.description"))
        .push(basic::scroll_area(stack(sections).spacing(space::S4)));
    modal(sheet)
        .key("shortcuts")
        .width(WIDTH)
        .close_on_scrim()
        .on_close(on_close)
}
