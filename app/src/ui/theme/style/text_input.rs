//! `Input`: a field is drawn by its `--input` border alone, which is why
//! that border is held to 3:1 (WCAG 1.4.11).

use iced::widget::text_input::{Catalog, Status, Style, StyleFn};

use super::{alpha, field, hairline, radius, selection, Theme, DISABLED};

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(field_style)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

/// A field: `.style(theme::text_input::field_style)` is the default.
pub fn field_style(theme: &Theme, status: Status) -> Style {
    let tokens = theme.tokens();
    let active = Style {
        background: field(theme).into(),
        border: hairline(tokens.input, radius::LG),
        icon: tokens.muted_foreground,
        placeholder: tokens.muted_foreground,
        value: tokens.foreground,
        selection: selection(tokens),
    };
    match status {
        Status::Active | Status::Hovered => active,
        Status::Focused { .. } => Style {
            border: hairline(tokens.ring, radius::LG),
            ..active
        },
        Status::Disabled => {
            // `disabled:bg-input/50` (80 % on dark), then the whole at half.
            let fill = if theme.is_dark() { 0.8 } else { 0.5 };
            Style {
                background: alpha(tokens.input, fill * DISABLED).into(),
                border: hairline(alpha(tokens.input, DISABLED), radius::LG),
                icon: alpha(active.icon, DISABLED),
                placeholder: alpha(active.placeholder, DISABLED),
                value: alpha(active.value, DISABLED),
                ..active
            }
        }
    }
}

/// `aria-invalid`: the border turns Signal red, focused or not.
pub fn invalid(theme: &Theme, status: Status) -> Style {
    let tokens = theme.tokens();
    let border = if theme.is_dark() {
        alpha(tokens.destructive, 0.5)
    } else {
        tokens.destructive
    };
    let mut style = field_style(theme, status);
    if status != Status::Disabled {
        style.border.color = border;
    }
    style
}

/// A field inside a bordered group (`InputGroup`, the palette's search): the
/// group draws the border and the focus.
pub fn bare(theme: &Theme, status: Status) -> Style {
    let mut style = field_style(theme, status);
    style.border.width = 0.0;
    style.background = iced::Color::TRANSPARENT.into();
    style
}
