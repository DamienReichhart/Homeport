//! Radio buttons. The React app has none (a choice is a select or a toggle
//! group); this is the checkbox's look on a circle, so one added later is
//! already in the palette.

use iced::widget::radio::{Catalog, Status, Style, StyleFn};

use super::{field, Theme};

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(radio)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn radio(theme: &Theme, status: Status) -> Style {
    let tokens = theme.tokens();
    let (Status::Active { is_selected } | Status::Hovered { is_selected }) = status;
    Style {
        background: field(theme).into(),
        dot_color: tokens.primary,
        border_width: 1.0,
        border_color: if is_selected {
            tokens.primary
        } else {
            tokens.input
        },
        text_color: None,
    }
}
