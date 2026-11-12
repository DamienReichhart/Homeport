//! `ScrollArea`: no track, and a thumb in `--border` that never changes
//! colour (nothing lifts on hover, docs/BRAND.md).

use iced::{
    widget::{
        container,
        scrollable::{AutoScroll, Catalog, Rail, Scroller, Status, Style, StyleFn},
    },
    Border,
};

use super::{edge, floating, hairline, radius, Theme};

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(area)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn area(theme: &Theme, _status: Status) -> Style {
    let tokens = theme.tokens();
    let rail = Rail {
        background: None,
        border: Border::default(),
        scroller: Scroller {
            background: tokens.border.into(),
            border: Border::default().rounded(radius::FULL),
        },
    };
    Style {
        container: container::Style::default(),
        vertical_rail: rail,
        horizontal_rail: rail,
        gap: None,
        // The marker of a middle-click scroll: a small popover.
        auto_scroll: AutoScroll {
            background: tokens.popover.into(),
            border: hairline(edge(tokens), radius::FULL),
            shadow: floating(tokens),
            icon: tokens.popover_foreground,
        },
    }
}

/// How much of the muted text's colour a table's thumb takes: the least
/// that is seen on a selected row (3:1) in both themes.
const THUMB: f32 = 0.7;

/// The thumb of a table's scrollbars. It lies on the rows, selected ones
/// included (`bg-muted`), where `--border` cannot be told from the fill:
/// the muted text's colour is seen on both.
pub fn table_thumb(theme: &Theme) -> iced::Color {
    super::alpha(theme.tokens().muted_foreground, THUMB)
}
