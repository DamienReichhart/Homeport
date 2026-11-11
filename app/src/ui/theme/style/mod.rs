//! A style for every widget: [`Theme`]'s `Catalog` implementations (what a
//! widget looks like left alone) and the variants as plain functions for
//! `.style(..)`. One module per widget, named after iced's.
//!
//! The values are the classes of `src/shared/ui/primitives`. Three things
//! CSS did that iced 0.14 cannot, so nobody looks for them:
//! - the focus ring is a 3 px halo *outside* the control there; a widget
//!   draws only inside its bounds, so focus shows as the border in `--ring`;
//! - buttons, checkboxes and toggles take no keyboard focus in iced 0.14 and
//!   so have no focused state to style (fields and editors do);
//! - `disabled:opacity-50` fades the whole control; here each of its colours
//!   is faded instead, which is the same over a flat surface.

pub mod button;
pub mod checkbox;
pub mod container;
pub mod pick_list;
pub mod progress_bar;
pub mod radio;
pub mod scrollable;
pub mod slider;
pub mod svg;
pub mod text_editor;
pub mod text_input;
pub mod toggler;

use iced::{border::Radius, Border, Color, Shadow, Vector};

use super::{
    color::alpha,
    metrics::radius,
    tokens::{Tokens, Tone},
    Theme,
};

/// `disabled:opacity-50`.
const DISABLED: f32 = 0.5;

/// A hairline in `colour`, with corners of `radius`.
fn hairline(colour: Color, radius: f32) -> Border {
    Border {
        color: colour,
        width: 1.0,
        radius: Radius::from(radius),
    }
}

/// The fill of a field: nothing on light, `dark:bg-input/30` on dark.
fn field(theme: &Theme) -> Color {
    if theme.is_dark() {
        alpha(theme.tokens().input, 0.3)
    } else {
        Color::TRANSPARENT
    }
}

/// `ring-1 ring-foreground/10`: the edge of a card, a popover, a menu.
fn edge(tokens: &Tokens) -> Color {
    alpha(tokens.foreground, 0.1)
}

/// Tailwind's `shadow-md`, the only shadow: what floats over the page.
fn floating(tokens: &Tokens) -> Shadow {
    Shadow {
        color: tokens.shadow,
        offset: Vector::new(0.0, 4.0),
        blur_radius: 6.0,
    }
}

/// `::selection`: the primary colour at 24 %.
fn selection(tokens: &Tokens) -> Color {
    alpha(tokens.primary, 0.24)
}

/// Text colours. Text left alone takes the colour around it.
pub mod text {
    use iced::widget::text::{Catalog, Style, StyleFn};

    use super::{Theme, Tone};

    impl Catalog for Theme {
        type Class<'a> = StyleFn<'a, Self>;

        fn default<'a>() -> Self::Class<'a> {
            Box::new(|_| Style::default())
        }

        fn style(&self, class: &Self::Class<'_>) -> Style {
            class(self)
        }
    }

    /// `text-foreground`, where the colour around is another.
    pub fn foreground(theme: &Theme) -> Style {
        Style {
            color: Some(theme.tokens().foreground),
        }
    }

    /// `text-muted-foreground`: descriptions, placeholders, units.
    pub fn muted(theme: &Theme) -> Style {
        Style {
            color: Some(theme.tokens().muted_foreground),
        }
    }

    /// `text-destructive`: an error, and nothing else.
    pub fn destructive(theme: &Theme) -> Style {
        Style {
            color: Some(theme.tokens().destructive),
        }
    }

    /// `text-warning`.
    pub fn warning(theme: &Theme) -> Style {
        Style {
            color: Some(theme.tokens().warning),
        }
    }

    /// The colour of a status: `.style(theme::text::tone(Tone::Warning))`.
    pub fn tone(tone: Tone) -> impl Fn(&Theme) -> Style {
        move |theme| Style {
            color: Some(tone.colour(theme.tokens())),
        }
    }
}

/// The separator: one hairline in `--border`.
pub mod rule {
    use iced::widget::rule::{Catalog, FillMode, Style, StyleFn};

    use super::Theme;

    impl Catalog for Theme {
        type Class<'a> = StyleFn<'a, Self>;

        fn default<'a>() -> Self::Class<'a> {
            Box::new(separator)
        }

        fn style(&self, class: &Self::Class<'_>) -> Style {
            class(self)
        }
    }

    pub fn separator(theme: &Theme) -> Style {
        Style {
            color: theme.tokens().border,
            radius: 0.0.into(),
            fill_mode: FillMode::Full,
            // A hairline between two pixels is a blur.
            snap: true,
        }
    }

    /// The sidebar's own, fainter on dark (`--sidebar-border`).
    pub fn sidebar(theme: &Theme) -> Style {
        Style {
            color: theme.tokens().sidebar.border,
            ..separator(theme)
        }
    }
}

/// The list a pick list or a combo box opens: `SelectContent`.
pub mod menu {
    use iced::widget::overlay::menu::{Catalog, Style, StyleFn};

    use super::{edge, floating, hairline, radius, Theme};

    impl Catalog for Theme {
        type Class<'a> = StyleFn<'a, Self>;

        fn default<'a>() -> <Self as Catalog>::Class<'a> {
            Box::new(popover)
        }

        fn style(&self, class: &<Self as Catalog>::Class<'_>) -> Style {
            class(self)
        }
    }

    pub fn popover(theme: &Theme) -> Style {
        let tokens = theme.tokens();
        Style {
            background: tokens.popover.into(),
            border: hairline(edge(tokens), radius::LG),
            text_color: tokens.popover_foreground,
            selected_text_color: tokens.accent_foreground,
            selected_background: tokens.accent.into(),
            shadow: floating(tokens),
        }
    }
}

/// The lines of iced's `table`: `--border`, as the rows of the React table.
pub mod table {
    use iced::widget::table::{Catalog, Style, StyleFn};

    use super::Theme;

    impl Catalog for Theme {
        type Class<'a> = StyleFn<'a, Self>;

        fn default<'a>() -> Self::Class<'a> {
            Box::new(lines)
        }

        fn style(&self, class: &Self::Class<'_>) -> Style {
            class(self)
        }
    }

    pub fn lines(theme: &Theme) -> Style {
        let border = theme.tokens().border;
        Style {
            separator_x: border.into(),
            separator_y: border.into(),
        }
    }
}

/// Split panes: the handle is a hairline that turns to the text colour.
pub mod pane_grid {
    use iced::widget::pane_grid::{Catalog, Highlight, Line, Style, StyleFn};

    use super::{alpha, hairline, radius, Theme};

    impl Catalog for Theme {
        type Class<'a> = StyleFn<'a, Self>;

        fn default<'a>() -> <Self as Catalog>::Class<'a> {
            Box::new(panes)
        }

        fn style(&self, class: &<Self as Catalog>::Class<'_>) -> Style {
            class(self)
        }
    }

    pub fn panes(theme: &Theme) -> Style {
        let tokens = theme.tokens();
        Style {
            hovered_region: Highlight {
                background: alpha(tokens.primary, 0.06).into(),
                border: hairline(tokens.ring, radius::LG),
            },
            picked_split: Line {
                color: tokens.ring,
                width: 2.0,
            },
            hovered_split: Line {
                color: tokens.input,
                width: 2.0,
            },
        }
    }
}
