//! `DialogContent`: a title, what it says, a body and a footer of buttons,
//! laid out as the React dialogs are. Give the result to [`super::modal`].

use iced::{
    widget::{container, rule, text, Column},
    Length,
};

use crate::ui::{
    theme::{
        self, font,
        metrics::{space, text as size},
        Element,
    },
    widgets::{end, line, stack},
};

/// A dialog being described.
pub struct Dialog<'a, Message> {
    title: String,
    destructive: bool,
    body: Vec<Element<'a, Message>>,
    footer: Vec<Element<'a, Message>>,
}

/// A dialog titled `title`.
pub fn dialog<'a, Message>(title: impl Into<String>) -> Dialog<'a, Message> {
    Dialog {
        title: title.into(),
        destructive: false,
        body: vec![],
        footer: vec![],
    }
}

impl<'a, Message: 'a> Dialog<'a, Message> {
    /// The title in red: what it confirms destroys data.
    #[must_use]
    pub fn destructive(mut self, destructive: bool) -> Self {
        self.destructive = destructive;
        self
    }

    /// `DialogDescription`: what will happen, in the quieter colour.
    #[must_use]
    pub fn description(self, description: impl Into<String>) -> Self {
        self.push(
            text(description.into())
                .size(size::SM)
                .line_height(size::LEADING_SM)
                .style(theme::text::muted)
                .width(Length::Fill)
                .align_x(crate::ui::widgets::start()),
        )
    }

    /// One more part of the body, under what is there.
    #[must_use]
    pub fn push(mut self, part: impl Into<Element<'a, Message>>) -> Self {
        self.body.push(part.into());
        self
    }

    /// The buttons, written start to end: Cancel first, the action last.
    #[must_use]
    pub fn footer(mut self, buttons: Vec<Element<'a, Message>>) -> Self {
        self.footer = buttons;
        self
    }

    pub fn view(self) -> Element<'a, Message> {
        let title = text(self.title)
            .size(size::BASE)
            .font(font::SANS_MEDIUM)
            .width(Length::Fill)
            .align_x(crate::ui::widgets::start());
        let title = if self.destructive {
            title.style(theme::text::destructive)
        } else {
            title
        };
        let mut parts: Vec<Element<'a, Message>> = vec![title.into()];
        parts.extend(self.body);
        let body = container(stack(parts).spacing(space::S4))
            .padding(space::S4)
            .width(Length::Fill);
        let mut whole = Column::new().push(body);
        if !self.footer.is_empty() {
            let buttons = container(line(self.footer).spacing(space::S2))
                .padding(space::S4)
                .width(Length::Fill)
                .align_x(end())
                .style(theme::container::dialog_footer);
            whole = whole.push(rule::horizontal(1)).push(buttons);
        }
        container(whole)
            .width(Length::Fill)
            .style(theme::container::dialog)
            .into()
    }
}

impl<'a, Message: 'a> From<Dialog<'a, Message>> for Element<'a, Message> {
    fn from(dialog: Dialog<'a, Message>) -> Self {
        dialog.view()
    }
}
