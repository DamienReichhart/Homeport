//! Sliders. The React app has none; this is the progress bar's rail with a
//! thumb, so one added later is already in the palette.

use iced::widget::slider::{Catalog, Handle, HandleShape, Rail, Status, Style, StyleFn};

use super::{radius, Theme};
use crate::ui::theme::metrics::control;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(slider)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn slider(theme: &Theme, status: Status) -> Style {
    let tokens = theme.tokens();
    Style {
        rail: Rail {
            backgrounds: (tokens.primary.into(), tokens.muted.into()),
            width: control::PROGRESS,
            border: iced::Border::default().rounded(radius::FULL),
        },
        handle: Handle {
            shape: HandleShape::Circle { radius: 7.0 },
            background: tokens.background.into(),
            border_width: if status == Status::Dragged { 2.0 } else { 1.0 },
            border_color: tokens.primary,
        },
    }
}
