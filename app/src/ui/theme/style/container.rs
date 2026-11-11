//! Surfaces. A container left alone draws nothing: it is layout. The
//! surfaces are flat, with hairline borders (docs/BRAND.md, Shape); only
//! what floats over the page casts a shadow.

use iced::{
    widget::container::{Catalog, Style, StyleFn},
    Border, Color,
};

use super::{alpha, edge, floating, hairline, radius, Theme};
use crate::ui::theme::color::mix;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(transparent)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

fn surface(background: Color, text: Color) -> Style {
    Style {
        background: Some(background.into()),
        text_color: Some(text),
        ..Style::default()
    }
}

pub fn transparent(_theme: &Theme) -> Style {
    Style::default()
}

/// The window's background and text colour.
pub fn page(theme: &Theme) -> Style {
    let tokens = theme.tokens();
    surface(tokens.background, tokens.foreground)
}

/// `Card`: a raised surface with a faint edge.
pub fn card(theme: &Theme) -> Style {
    let tokens = theme.tokens();
    Style {
        border: hairline(edge(tokens), radius::XL),
        ..surface(tokens.card, tokens.card_foreground)
    }
}

/// `CardFooter`: `bg-muted/50` under the card's content.
pub fn card_footer(theme: &Theme) -> Style {
    Style {
        background: Some(alpha(theme.tokens().muted, 0.5).into()),
        ..Style::default()
    }
}

/// The sidebar's surface. Its edge towards the page is a `rule::sidebar`:
/// a container's border goes all the way round.
pub fn sidebar(theme: &Theme) -> Style {
    let sidebar = &theme.tokens().sidebar;
    surface(sidebar.background, sidebar.foreground)
}

/// The selected entry of the sidebar (`bg-sidebar-accent`).
pub fn sidebar_accent(theme: &Theme) -> Style {
    let sidebar = &theme.tokens().sidebar;
    Style {
        border: Border::default().rounded(radius::MD),
        ..surface(sidebar.accent, sidebar.accent_foreground)
    }
}

/// `PopoverContent`, a menu, a dialog: what floats over the page.
pub fn popover(theme: &Theme) -> Style {
    let tokens = theme.tokens();
    Style {
        border: hairline(edge(tokens), radius::LG),
        shadow: floating(tokens),
        ..surface(tokens.popover, tokens.popover_foreground)
    }
}

/// `DialogContent`: a popover with a card's corners; the scrim sets it
/// apart, so it casts no shadow.
pub fn dialog(theme: &Theme) -> Style {
    let tokens = theme.tokens();
    Style {
        border: hairline(edge(tokens), radius::XL),
        ..surface(tokens.popover, tokens.popover_foreground)
    }
}

/// `DialogFooter`: `bg-muted/50` under the dialog, down to its corners.
pub fn dialog_footer(theme: &Theme) -> Style {
    Style {
        border: Border {
            radius: iced::border::Radius::default().bottom(radius::XL),
            ..Border::default()
        },
        ..card_footer(theme)
    }
}

/// `Alert`: a card with a field's corners and the page's border.
pub fn alert(theme: &Theme) -> Style {
    let tokens = theme.tokens();
    Style {
        border: hairline(tokens.border, radius::LG),
        ..surface(tokens.card, tokens.card_foreground)
    }
}

/// A toast: the popover tinted with the colour of what it reports (Sonner's
/// `richColors`: a tenth of it in the fill, three tenths in the border).
pub fn toast(colour: fn(&super::Tokens) -> Color) -> impl Fn(&Theme) -> Style {
    move |theme| {
        let tokens = theme.tokens();
        let colour = colour(tokens);
        Style {
            border: hairline(mix(tokens.popover, colour, 0.3), radius::LG),
            shadow: floating(tokens),
            ..surface(mix(tokens.popover, colour, 0.1), colour)
        }
    }
}

/// `Skeleton`: where something is still loading.
pub fn skeleton(theme: &Theme) -> Style {
    Style {
        background: Some(theme.tokens().muted.into()),
        border: Border::default().rounded(radius::MD),
        ..Style::default()
    }
}

/// A box drawn by its `--border` alone, on whatever is behind.
pub fn bordered(theme: &Theme) -> Style {
    Style {
        border: hairline(theme.tokens().border, radius::LG),
        ..Style::default()
    }
}

/// `bg-muted`: a group row, a gutter, a code block.
pub fn muted(theme: &Theme) -> Style {
    let tokens = theme.tokens();
    Style {
        border: Border::default().rounded(radius::LG),
        ..surface(tokens.muted, tokens.foreground)
    }
}

/// `bg-accent`: the row under the pointer, the selected item of a list.
pub fn accent(theme: &Theme) -> Style {
    let tokens = theme.tokens();
    Style {
        border: Border::default().rounded(radius::MD),
        ..surface(tokens.accent, tokens.accent_foreground)
    }
}

/// `TooltipContent`: the page inverted, so it reads over anything. Pass it
/// to `tooltip(..).style(..)`.
pub fn tooltip(theme: &Theme) -> Style {
    let tokens = theme.tokens();
    Style {
        border: Border::default().rounded(radius::MD),
        ..surface(tokens.foreground, tokens.background)
    }
}

/// The terminal's pane: its own background, whatever the page's.
pub fn terminal(theme: &Theme) -> Style {
    let tokens = theme.tokens();
    Style {
        border: hairline(tokens.border, radius::MD),
        ..surface(tokens.terminal.background, tokens.terminal.foreground)
    }
}

/// `Badge`, by variant. A badge is a pill (`rounded-4xl`).
pub mod badge {
    use super::{alpha, hairline, surface, Border, Style, Theme};

    /// `calc(var(--radius) * 2.6)`: half a badge's height, so a pill.
    const RADIUS: f32 = 10.4;

    fn pill(style: Style) -> Style {
        Style {
            border: Border {
                radius: RADIUS.into(),
                ..style.border
            },
            ..style
        }
    }

    pub fn primary(theme: &Theme) -> Style {
        let tokens = theme.tokens();
        pill(surface(tokens.primary, tokens.primary_foreground))
    }

    pub fn secondary(theme: &Theme) -> Style {
        let tokens = theme.tokens();
        pill(surface(tokens.secondary, tokens.secondary_foreground))
    }

    pub fn destructive(theme: &Theme) -> Style {
        let tokens = theme.tokens();
        let tint = if theme.is_dark() { 0.2 } else { 0.1 };
        pill(surface(alpha(tokens.destructive, tint), tokens.destructive))
    }

    pub fn outline(theme: &Theme) -> Style {
        let tokens = theme.tokens();
        Style {
            text_color: Some(tokens.foreground),
            border: hairline(tokens.border, RADIUS),
            ..Style::default()
        }
    }

    /// `variant="ghost"`: text and nothing else at rest.
    pub fn ghost(theme: &Theme) -> Style {
        pill(Style {
            text_color: Some(theme.tokens().foreground),
            ..Style::default()
        })
    }

    /// The ghost under the pointer: `hover:bg-muted
    /// hover:text-muted-foreground`, the fill at half on dark.
    pub fn ghost_hovered(theme: &Theme) -> Style {
        let tokens = theme.tokens();
        let fill = if theme.is_dark() { 0.5 } else { 1.0 };
        pill(surface(alpha(tokens.muted, fill), tokens.muted_foreground))
    }

    /// `variant="link"`: text in the primary colour, no pill behind it.
    pub fn link(theme: &Theme) -> Style {
        pill(Style {
            text_color: Some(theme.tokens().primary),
            ..Style::default()
        })
    }
}

/// A sample of `colour`, edged so it shows on any page: the gallery's
/// swatches, and wherever a colour itself is what is shown.
pub fn swatch(colour: Color) -> impl Fn(&Theme) -> Style {
    move |theme| Style {
        background: Some(colour.into()),
        border: hairline(alpha(theme.tokens().foreground, 0.2), radius::LG),
        ..Style::default()
    }
}

/// `TableRow`: nothing at rest, `bg-muted/50` under the pointer (and while
/// what it opened is open), `bg-muted` when selected, which wins.
pub fn table_row(hovered: bool, selected: bool) -> impl Fn(&Theme) -> Style {
    move |theme| {
        let tokens = theme.tokens();
        let fill = match (selected, hovered) {
            (true, _) => Some(tokens.muted),
            (false, true) => Some(alpha(tokens.muted, 0.5)),
            (false, false) => None,
        };
        Style {
            background: fill.map(Into::into),
            text_color: Some(tokens.foreground),
            ..Style::default()
        }
    }
}
