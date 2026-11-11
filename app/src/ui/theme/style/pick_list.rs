//! `SelectTrigger`: a field with a chevron. The list it opens is
//! `theme::menu::popover`.

use iced::widget::pick_list::{Catalog, Status, Style, StyleFn};

use super::{alpha, field, hairline, radius, Theme};

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> <Self as Catalog>::Class<'a> {
        Box::new(select)
    }

    fn style(&self, class: &<Self as Catalog>::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn select(theme: &Theme, status: Status) -> Style {
    let tokens = theme.tokens();
    let hovered = matches!(
        status,
        Status::Hovered | Status::Opened { is_hovered: true }
    );
    // `dark:hover:bg-input/50`; on light, hovering changes nothing.
    let background = if theme.is_dark() && hovered {
        alpha(tokens.input, 0.5)
    } else {
        field(theme)
    };
    let border = if matches!(status, Status::Opened { .. }) {
        tokens.ring
    } else {
        tokens.input
    };
    Style {
        text_color: tokens.foreground,
        placeholder_color: tokens.muted_foreground,
        handle_color: tokens.muted_foreground,
        background: background.into(),
        border: hairline(border, radius::LG),
    }
}
