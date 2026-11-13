//! What a page is laid out with (`src/shared/ui/layout`): its header
//! ([`page_header`]) and Homeport's mark ([`logo`]). They know nothing
//! of the shell, which uses them from here as the pages do.

pub mod logo;

use iced::{widget::text, Length};

use crate::ui::{
    theme::{
        self, font,
        metrics::{space, text as size},
        Element,
    },
    widgets::{line, stack},
};

/// `PageHeader`: the page's title, what it is under it, and its actions at
/// the end of the line.
pub fn page_header<'a, Message: 'a>(
    title: impl Into<String>,
    description: Option<String>,
    actions: Vec<Element<'a, Message>>,
) -> Element<'a, Message> {
    let title = text(title.into()).size(size::XL).font(font::semibold());
    let description = description.map(|description| {
        text(description)
            .size(size::SM)
            .style(theme::text::muted)
            .into()
    });
    page_header_of(title, description, actions)
}

/// The same for a title and a description that are more than text, as
/// `PageHeader` took nodes: a page about one thing puts its Back button,
/// its status and its name in the title (a container's page). The page
/// gives the title its size (`size::XL`, semibold) and the description
/// its own (`size::SM`, muted).
pub fn page_header_of<'a, Message: 'a>(
    title: impl Into<Element<'a, Message>>,
    description: Option<Element<'a, Message>>,
    actions: Vec<Element<'a, Message>>,
) -> Element<'a, Message> {
    let mut titles: Vec<Element<'a, Message>> = vec![title.into()];
    titles.extend(description);
    let mut parts: Vec<Element<'a, Message>> =
        vec![filling_column(stack(titles).spacing(space::S0_5).into())];
    if !actions.is_empty() {
        parts.push(line(actions).spacing(space::S2).into());
    }
    line(parts)
        .spacing(space::S3)
        .width(Length::Fill)
        .align_y(iced::alignment::Vertical::Top)
        .into()
}

fn filling_column<'a, Message: 'a>(content: Element<'a, Message>) -> Element<'a, Message> {
    iced::widget::container(content).width(Length::Fill).into()
}
