//! The React `Button`'s variants. A button left alone is the primary one:
//! Ink on light, Paper on dark, and there is no accent colour.
//!
//! The size is layout, not style: `metrics::ButtonSize` gives a button its
//! height, padding and text size; [`small`] gives the `xs` and `sm` sizes
//! their tighter corners.

use iced::{
    widget::button::{Catalog, Status, Style, StyleFn},
    Background, Color,
};

use super::{alpha, hairline, radius, Theme, DISABLED};
use crate::ui::theme::color::mix;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(primary)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

/// A button's three colours in one state.
struct Look {
    background: Option<Color>,
    text: Color,
    border: Color,
}

/// The React button moves a pixel down when pressed and keeps its hover
/// colours; a disabled one is the idle one at half opacity.
fn build(status: Status, idle: Look, hover: Look) -> Style {
    let (look, opacity) = match status {
        Status::Active => (idle, 1.0),
        Status::Hovered | Status::Pressed => (hover, 1.0),
        Status::Disabled => (idle, DISABLED),
    };
    Style {
        background: look
            .background
            .map(|colour| Background::Color(alpha(colour, opacity))),
        text_color: alpha(look.text, opacity),
        border: hairline(alpha(look.border, opacity), radius::LG),
        ..Style::default()
    }
}

/// `variant="default"`: the one action a page is for.
pub fn primary(theme: &Theme, status: Status) -> Style {
    let tokens = theme.tokens();
    let look = |background| Look {
        background: Some(background),
        text: tokens.primary_foreground,
        border: Color::TRANSPARENT,
    };
    build(
        status,
        look(tokens.primary),
        look(alpha(tokens.primary, 0.8)),
    )
}

/// `variant="secondary"`: an action beside the primary one.
pub fn secondary(theme: &Theme, status: Status) -> Style {
    let tokens = theme.tokens();
    let look = |background| Look {
        background: Some(background),
        text: tokens.secondary_foreground,
        border: Color::TRANSPARENT,
    };
    build(
        status,
        look(tokens.secondary),
        look(mix(tokens.secondary, tokens.foreground, 0.05)),
    )
}

/// `variant="outline"`: a bordered button on the page's colour.
pub fn outline(theme: &Theme, status: Status) -> Style {
    let tokens = theme.tokens();
    let look = |background, border| Look {
        background: Some(background),
        text: tokens.foreground,
        border,
    };
    if theme.is_dark() {
        build(
            status,
            look(alpha(tokens.input, 0.3), tokens.input),
            look(alpha(tokens.input, 0.5), tokens.input),
        )
    } else {
        build(
            status,
            look(tokens.background, tokens.border),
            look(tokens.muted, tokens.border),
        )
    }
}

/// `variant="ghost"`: nothing until hovered. Toolbars, rows, icon buttons.
pub fn ghost(theme: &Theme, status: Status) -> Style {
    let tokens = theme.tokens();
    let hover = if theme.is_dark() {
        alpha(tokens.muted, 0.5)
    } else {
        tokens.muted
    };
    let look = |background| Look {
        background,
        text: tokens.foreground,
        border: Color::TRANSPARENT,
    };
    build(status, look(None), look(Some(hover)))
}

/// `variant="destructive"`: red text on a red tint, never a red slab.
pub fn destructive(theme: &Theme, status: Status) -> Style {
    let tokens = theme.tokens();
    let (idle, hover) = if theme.is_dark() {
        (0.2, 0.3)
    } else {
        (0.1, 0.2)
    };
    let look = |opacity| Look {
        background: Some(alpha(tokens.destructive, opacity)),
        text: tokens.destructive,
        border: Color::TRANSPARENT,
    };
    build(status, look(idle), look(hover))
}

/// `variant="link"`: text in the primary colour. The React one underlines on
/// hover; iced's button style has no underline, so this one stays plain.
pub fn link(theme: &Theme, status: Status) -> Style {
    let look = || Look {
        background: None,
        text: theme.tokens().primary,
        border: Color::TRANSPARENT,
    };
    build(status, look(), look())
}

/// `variant` with the corners of the `xs` and `sm` sizes:
/// `.style(theme::button::small(theme::button::ghost))`.
pub fn small(variant: fn(&Theme, Status) -> Style) -> impl Fn(&Theme, Status) -> Style {
    move |theme, status| {
        let mut style = variant(theme, status);
        style.border.radius = radius::MD.into();
        style
    }
}
