//! `Alert`: a notice in place that is no failure, in a card with an icon:
//! what a form says of what it will do (a copy that shares a port, a name
//! that moves), or, destructive, what it warns of (a privileged
//! container). A failure is [`super::error_message`].

use iced::{
    alignment::Vertical,
    widget::{container, text},
    Length,
};

use crate::ui::{
    theme::{
        self, font, icon,
        metrics::{space, text as size},
        Element, Icon,
    },
    widgets::{line, paragraph, stack},
};

/// `description` in a card, under `title` when there is one; `destructive`
/// draws it in red with a warning sign (`variant="destructive"`).
pub fn alert<'a, Message: 'a>(
    title: Option<String>,
    description: String,
    destructive: bool,
) -> Element<'a, Message> {
    let mut body: Vec<Element<'a, Message>> = vec![];
    if let Some(title) = title {
        let title = text(title)
            .size(size::SM)
            .line_height(size::LEADING_SM)
            .font(font::medium());
        body.push(if destructive {
            title.style(theme::text::destructive).into()
        } else {
            title.into()
        });
    }
    let words = paragraph(description);
    body.push(if destructive {
        words.style(theme::text::destructive).into()
    } else {
        words.style(theme::text::muted).into()
    });
    let glyph = if destructive {
        icon(Icon::TriangleAlert).style(theme::svg::destructive)
    } else {
        icon(Icon::Info)
    };
    let glyph = container(glyph).padding([space::S0_5, 0.0]);
    container(
        line(vec![glyph.into(), stack(body).spacing(space::S1).into()])
            .spacing(space::S2)
            .width(Length::Fill)
            .align_y(Vertical::Top),
    )
    .padding([space::S2, space::S2_5])
    .width(Length::Fill)
    .style(theme::container::alert)
    .into()
}
