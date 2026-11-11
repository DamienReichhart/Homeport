//! How Homeport looks (docs/BRAND.md): the tokens of the React app's
//! stylesheet, the fonts, the icons, and a style for every widget, so a page
//! names a style and never a colour.
//!
//! # Why a theme type of its own
//!
//! iced offers two ways. `iced::Theme::custom` keeps the built-in theme and
//! gives it six colours, from which iced *generates* the rest; every widget
//! then draws in iced's own idea of a button or a field unless each call site
//! passes a style function, and a style function only learns "light or dark"
//! by asking the generated palette. Homeport's palette has some sixty named
//! tokens and no accent colour, which six colours cannot say, and a forgotten
//! `.style(..)` would show iced's blue.
//!
//! So [`Theme`] is its own type and implements every widget's `Catalog`
//! (`style/`): a widget left alone is already Homeport's default (a `button`
//! is the primary button, a `container` is transparent), the variants are
//! plain functions (`.style(theme::button::ghost)`), and each reads
//! [`Theme::tokens`]. Nothing can draw outside the palette by omission. The
//! cost is that code names the theme: use [`Element`] from here instead of
//! `iced::Element`, and a third-party widget must be generic over its theme
//! (those worth using are).
//!
//! ```ignore
//! use crate::ui::theme::{self, icon, Element, Icon};
//!
//! button(label).style(theme::button::destructive)
//! container(page).style(theme::container::card)
//! icon(Icon::Trash)
//! ```

pub mod color;
pub mod font;
pub mod icon;
pub mod metrics;
pub mod mode;
pub mod scale;
mod style;
#[cfg(test)]
mod tests;
pub mod tokens;

use iced::theme::{self, Base, Palette};

pub use self::{
    icon::{icon, Icon},
    mode::Mode,
    scale::Scale,
    style::{
        button, checkbox, container, menu, pane_grid, pick_list, progress_bar, radio, rule,
        scrollable, slider, svg, table, text, text_editor, text_input, toggler,
    },
    tokens::{Tokens, Tone},
};

/// The app's theme: one of the two palettes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Theme {
    #[default]
    Light,
    Dark,
}

/// An element drawn with [`Theme`]: what every `view` of the app returns.
pub type Element<'a, Message> = iced::Element<'a, Message, Theme>;

impl Theme {
    pub const ALL: [Self; 2] = [Self::Light, Self::Dark];

    /// Every colour of this theme.
    pub const fn tokens(self) -> &'static Tokens {
        match self {
            Self::Light => &tokens::LIGHT,
            Self::Dark => &tokens::DARK,
        }
    }

    pub const fn is_dark(self) -> bool {
        matches!(self, Self::Dark)
    }
}

impl Base for Theme {
    /// What iced shows while the app names no theme ([`Mode::System`]): the
    /// OS's preference, light when it has none.
    fn default(preference: theme::Mode) -> Self {
        match preference {
            theme::Mode::Dark => Self::Dark,
            theme::Mode::Light | theme::Mode::None => Self::Light,
        }
    }

    fn mode(&self) -> theme::Mode {
        match self {
            Self::Light => theme::Mode::Light,
            Self::Dark => theme::Mode::Dark,
        }
    }

    fn base(&self) -> theme::Style {
        let tokens = self.tokens();
        theme::Style {
            background_color: tokens.background,
            text_color: tokens.foreground,
        }
    }

    /// Only iced's own debug overlay reads this.
    fn palette(&self) -> Option<Palette> {
        let tokens = self.tokens();
        Some(Palette {
            background: tokens.background,
            text: tokens.foreground,
            primary: tokens.primary,
            success: tokens.success,
            warning: tokens.warning,
            danger: tokens.destructive,
        })
    }

    fn name(&self) -> &str {
        match self {
            Self::Light => "Homeport Light",
            Self::Dark => "Homeport Dark",
        }
    }
}
