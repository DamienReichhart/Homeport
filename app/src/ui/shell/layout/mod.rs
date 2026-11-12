//! The window's layout: what a page puts at its top ([`page_header`]), the
//! bars over every page ([`banners`]) and the mark ([`logo`]).

pub mod banners;
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
    let mut titles: Vec<Element<'a, Message>> = vec![text(title.into())
        .size(size::XL)
        .font(font::semibold())
        .into()];
    if let Some(description) = description {
        titles.push(
            text(description)
                .size(size::SM)
                .style(theme::text::muted)
                .into(),
        );
    }
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
