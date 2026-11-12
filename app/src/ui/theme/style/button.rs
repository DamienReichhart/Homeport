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
#[derive(Clone, Copy)]
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
/// hover; iced's button style has no underline, so under the pointer the
/// text eases off instead (`primary` at 80 %, the primary button's hover).
pub fn link(theme: &Theme, status: Status) -> Style {
    let look = |opacity| Look {
        background: None,
        text: alpha(theme.tokens().primary, opacity),
        border: Color::TRANSPARENT,
    };
    build(status, look(1.0), look(0.8))
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

/// What the kit builds out of buttons: the items of a menu and of a list,
/// tabs, and the toggles of a toggle group. Each is a function of whether
/// the item is the chosen one, so a view passes `item(selected)`.
pub mod part {
    use super::{alpha, build, radius, Color, Look, Status, Style, Theme};

    fn rounded(mut style: Style, corners: f32) -> Style {
        style.border.radius = corners.into();
        style
    }

    /// `DropdownMenuItem`, `SelectItem`, `CommandItem`: accent under the
    /// pointer; `active` is the item the keyboard is on in a filtered list.
    pub fn item(active: bool) -> impl Fn(&Theme, Status) -> Style {
        move |theme, status| {
            let tokens = theme.tokens();
            let look = |on: bool| Look {
                background: on.then_some(tokens.accent),
                text: if on {
                    tokens.accent_foreground
                } else {
                    tokens.popover_foreground
                },
                border: Color::TRANSPARENT,
            };
            rounded(build(status, look(active), look(true)), radius::MD)
        }
    }

    /// An entry of the sidebar: the accent of the sidebar when it is the
    /// page that shows, a fainter one under the pointer.
    pub fn nav(current: bool) -> impl Fn(&Theme, Status) -> Style {
        move |theme, status| {
            let sidebar = &theme.tokens().sidebar;
            let idle = Look {
                background: current.then_some(sidebar.accent),
                text: if current {
                    sidebar.accent_foreground
                } else {
                    alpha(sidebar.foreground, 0.8)
                },
                border: Color::TRANSPARENT,
            };
            let hover = Look {
                background: Some(if current {
                    sidebar.accent
                } else {
                    alpha(sidebar.accent, 0.6)
                }),
                text: sidebar.accent_foreground,
                border: Color::TRANSPARENT,
            };
            rounded(build(status, idle, hover), radius::MD)
        }
    }

    /// `variant="destructive"` of a menu item: red text, a red tint under
    /// the pointer.
    pub fn destructive_item(theme: &Theme, status: Status) -> Style {
        let tokens = theme.tokens();
        let tint = if theme.is_dark() { 0.2 } else { 0.1 };
        let look = |on: bool| Look {
            background: on.then_some(alpha(tokens.destructive, tint)),
            text: tokens.destructive,
            border: Color::TRANSPARENT,
        };
        rounded(build(status, look(false), look(true)), radius::MD)
    }

    /// `TabsTrigger` in a `TabsList` (`bg-muted`): the chosen tab is a chip
    /// in the page's colour, the others are text at 60 %.
    pub fn tab(selected: bool) -> impl Fn(&Theme, Status) -> Style {
        move |theme, status| {
            let tokens = theme.tokens();
            let chosen = Look {
                background: Some(if theme.is_dark() {
                    alpha(tokens.input, 0.3)
                } else {
                    tokens.background
                }),
                text: tokens.foreground,
                border: if theme.is_dark() {
                    tokens.input
                } else {
                    Color::TRANSPARENT
                },
            };
            let other = |opacity| Look {
                background: None,
                text: alpha(tokens.foreground, opacity),
                border: Color::TRANSPARENT,
            };
            let style = if selected {
                build(status, chosen, chosen)
            } else {
                build(status, other(0.6), other(1.0))
            };
            rounded(style, radius::MD)
        }
    }

    /// `TabsTrigger` of the `line` variant: text only; the line under the
    /// chosen tab is drawn by the tabs.
    pub fn line_tab(selected: bool) -> impl Fn(&Theme, Status) -> Style {
        move |theme, status| {
            let tokens = theme.tokens();
            let look = |opacity| Look {
                background: None,
                text: alpha(tokens.foreground, opacity),
                border: Color::TRANSPARENT,
            };
            let idle = if selected { 1.0 } else { 0.6 };
            rounded(build(status, look(idle), look(1.0)), radius::MD)
        }
    }

    /// `SelectTrigger`: a button that looks like the field it stands in
    /// for; `invalid` turns its border Signal red.
    pub fn field(invalid: bool) -> impl Fn(&Theme, Status) -> Style {
        move |theme, status| {
            let tokens = theme.tokens();
            let border = if invalid {
                tokens.destructive
            } else {
                tokens.input
            };
            let look = |fill: f32| Look {
                background: theme.is_dark().then_some(alpha(tokens.input, fill)),
                text: tokens.foreground,
                border,
            };
            build(status, look(0.3), look(0.5))
        }
    }

    /// `Toggle` (`variant="outline"` when `outlined`): `bg-muted` while on
    /// and under the pointer.
    pub fn toggle(on: bool, outlined: bool) -> impl Fn(&Theme, Status) -> Style {
        move |theme, status| {
            let tokens = theme.tokens();
            let look = |filled: bool| Look {
                background: filled.then_some(tokens.muted),
                text: tokens.foreground,
                border: if outlined {
                    tokens.input
                } else {
                    Color::TRANSPARENT
                },
            };
            build(status, look(on), look(true))
        }
    }
}

/// The button that confirms a destructive action in a dialog: the one red
/// slab of the app (`bg-destructive text-white hover:bg-destructive/90`).
pub fn destructive_solid(theme: &Theme, status: Status) -> Style {
    let tokens = theme.tokens();
    let look = |opacity| Look {
        background: Some(alpha(tokens.destructive, opacity)),
        text: tokens.on_destructive,
        border: Color::TRANSPARENT,
    };
    build(status, look(1.0), look(0.9))
}
