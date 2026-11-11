//! `Progress`: a 4 px pill of `--muted`, filled with the primary colour.
//! Give the widget `.girth(metrics::control::PROGRESS)`.

use iced::{
    widget::progress_bar::{Catalog, Style, StyleFn},
    Border,
};

use super::{radius, Theme};

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(progress)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

pub fn progress(theme: &Theme) -> Style {
    let tokens = theme.tokens();
    Style {
        background: tokens.muted.into(),
        bar: tokens.primary.into(),
        border: Border::default().rounded(radius::FULL),
    }
}

/// A disk or a quota close to full: the bar in amber.
pub fn warning(theme: &Theme) -> Style {
    Style {
        bar: theme.tokens().warning.into(),
        ..progress(theme)
    }
}

/// Full, or failed: the bar in Signal red.
pub fn destructive(theme: &Theme) -> Style {
    Style {
        bar: theme.tokens().destructive.into(),
        ..progress(theme)
    }
}
