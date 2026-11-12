//! The bars over every page: no keychain (`KeychainWarning`), a newer
//! version (`UpdateNotice`).

use iced::{
    widget::{container, rich_text, rule, span, text},
    Length,
};

use crate::{
    api::app::Capabilities,
    i18n::Tag,
    rich, t,
    ui::{
        shell::page::Update,
        theme::{
            self, font, icon,
            metrics::{space, text as size, ButtonSize},
            Element, Icon,
        },
        widgets::{
            basic::button::{self, Variant},
            filling, line,
        },
    },
};

fn bar<'a, Message: 'a>(
    parts: Vec<Element<'a, Message>>,
    style: fn(&theme::Theme) -> container::Style,
) -> Element<'a, Message> {
    iced::widget::column![
        container(line(parts).spacing(space::S2).width(Length::Fill))
            .padding([space::S2, space::S6])
            .width(Length::Fill)
            .style(style),
        rule::horizontal(1),
    ]
    .into()
}

/// Says what can't be stored when the machine has no OS keychain.
pub fn keychain<'a, Message: 'a>(capabilities: &Capabilities) -> Option<Element<'a, Message>> {
    if capabilities.keychain {
        return None;
    }
    let says = if capabilities.keychain_entry_missing {
        t!("app.keychainEntryMissing")
    } else {
        t!("app.keychainMissing")
    };
    Some(bar(
        vec![
            icon(Icon::TriangleAlert).into(),
            filling(text(says).size(size::SM)).into(),
        ],
        theme::container::banner_destructive,
    ))
}

/// A bar while a newer version is available, until dismissed.
pub fn update<'a, Message: Clone + 'a>(
    update: &Update,
    on_dismiss: Message,
) -> Element<'a, Message> {
    let spans = if update.via_package_manager {
        rich!(
            "updates.notice.availablePackageManager",
            version = update.version.as_str()
        )
    } else {
        rich!(
            "updates.notice.available",
            version = update.version.as_str()
        )
    };
    let says = rich_text::<(), Message, theme::Theme, iced::Renderer>(
        spans
            .into_iter()
            .map(|part| {
                let strong = part.tag == Some(Tag::Strong);
                span(part.text).font_maybe(strong.then(font::medium))
            })
            .collect::<Vec<_>>(),
    )
    .size(size::SM);
    bar(
        vec![
            icon(Icon::CircleArrowUp).style(theme::svg::primary).into(),
            container(says).width(Length::Fill).into(),
            button::icon(Icon::X)
                .variant(Variant::Ghost)
                .size(ButtonSize::Sm)
                .on_press(on_dismiss)
                .into(),
        ],
        theme::container::banner,
    )
}
