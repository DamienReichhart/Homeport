//! `Checkbox`: a 16 px box with the field's border; checked, it fills with
//! the primary colour.

use iced::widget::checkbox::{Catalog, Status, Style, StyleFn};

use super::{alpha, field, hairline, radius, Theme, DISABLED};

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(checkbox)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn checkbox(theme: &Theme, status: Status) -> Style {
    let tokens = theme.tokens();
    let (is_checked, opacity) = match status {
        Status::Active { is_checked } | Status::Hovered { is_checked } => (is_checked, 1.0),
        Status::Disabled { is_checked } => (is_checked, DISABLED),
    };
    let (background, border) = if is_checked {
        (tokens.primary, tokens.primary)
    } else {
        (field(theme), tokens.input)
    };
    Style {
        background: alpha(background, opacity).into(),
        icon_color: alpha(tokens.primary_foreground, opacity),
        border: hairline(alpha(border, opacity), radius::LG),
        // The label keeps the colour around it, faded with the box.
        text_color: (opacity < 1.0).then(|| alpha(tokens.foreground, opacity)),
    }
}
