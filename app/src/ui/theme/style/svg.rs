//! The colour of an icon. An SVG left alone keeps its own colours (the
//! mark's ring stays red); `theme::icon` draws in the text colour, and these
//! are for an icon that sits in text of another colour.

use iced::widget::svg::{Catalog, Status, Style, StyleFn};

use super::{Theme, Tone};

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|_, _| Style::default())
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn foreground(theme: &Theme, _status: Status) -> Style {
    Style {
        color: Some(theme.tokens().foreground),
    }
}

/// The icon of the sidebar entry whose page shows (`text-sidebar-primary`).
pub fn sidebar_primary(theme: &Theme, _status: Status) -> Style {
    Style {
        color: Some(theme.tokens().sidebar.primary),
    }
}

/// `text-primary`: the icon of a notice.
pub fn primary(theme: &Theme, _status: Status) -> Style {
    Style {
        color: Some(theme.tokens().primary),
    }
}

/// `text-muted-foreground`: a select's chevron, an icon beside a hint.
pub fn muted(theme: &Theme, _status: Status) -> Style {
    Style {
        color: Some(theme.tokens().muted_foreground),
    }
}

/// On the primary button, the checked checkbox, the default badge.
pub fn on_primary(theme: &Theme, _status: Status) -> Style {
    Style {
        color: Some(theme.tokens().primary_foreground),
    }
}

/// On a tooltip, which inverts the page.
pub fn on_tooltip(theme: &Theme, _status: Status) -> Style {
    Style {
        color: Some(theme.tokens().background),
    }
}

pub fn destructive(theme: &Theme, _status: Status) -> Style {
    Style {
        color: Some(theme.tokens().destructive),
    }
}

pub fn warning(theme: &Theme, _status: Status) -> Style {
    Style {
        color: Some(theme.tokens().warning),
    }
}

/// The colour of a status: `.style(theme::svg::tone(Tone::Danger))`.
pub fn tone(tone: Tone) -> impl Fn(&Theme, Status) -> Style {
    move |theme, _| Style {
        color: Some(tone.colour(theme.tokens())),
    }
}
