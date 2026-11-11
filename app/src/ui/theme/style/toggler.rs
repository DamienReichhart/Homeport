//! `Switch`: a 32 by 18.4 px pill, primary when on, with a 16 px thumb.

use iced::{
    widget::toggler::{Catalog, Status, Style, StyleFn},
    Color,
};

use super::{alpha, Theme, DISABLED};
use crate::ui::theme::metrics::control;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(switch)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn switch(theme: &Theme, status: Status) -> Style {
    let tokens = theme.tokens();
    let (is_toggled, opacity) = match status {
        Status::Active { is_toggled } | Status::Hovered { is_toggled } => (is_toggled, 1.0),
        Status::Disabled { is_toggled } => (is_toggled, DISABLED),
    };
    let track = match (is_toggled, theme.is_dark()) {
        (true, _) => tokens.primary,
        (false, false) => tokens.input,
        (false, true) => alpha(tokens.input, 0.8),
    };
    // On dark the thumb contrasts with the track it sits on.
    let thumb = match (theme.is_dark(), is_toggled) {
        (false, _) => tokens.background,
        (true, true) => tokens.primary_foreground,
        (true, false) => tokens.foreground,
    };
    Style {
        background: alpha(track, opacity).into(),
        background_border_width: 0.0,
        background_border_color: Color::TRANSPARENT,
        foreground: alpha(thumb, opacity).into(),
        foreground_border_width: 0.0,
        foreground_border_color: Color::TRANSPARENT,
        text_color: (opacity < 1.0).then(|| alpha(tokens.foreground, opacity)),
        // `None` is a pill.
        border_radius: None,
        // The 16 px thumb in the 18.4 px track.
        padding_ratio: (control::SWITCH_HEIGHT - 16.0) / 2.0 / control::SWITCH_HEIGHT,
    }
}
