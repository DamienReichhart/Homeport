//! Which palette the window shows: the Settings value, and the OS's
//! preference when the user follows it.
//!
//! iced 0.14 follows the OS by itself: an application whose `.theme(..)`
//! returns `None` draws `Theme::default(preference)` and redraws when the
//! preference changes, on Windows and macOS through winit and on Linux
//! through the desktop portal (iced's `linux-theme-detection` feature, on in
//! the workspace). So [`Mode::theme`] is all the window needs. Parts that
//! must know the resolved theme themselves (the terminal, a chart) keep the
//! last value of [`system_changes`] and ask [`Mode::resolve`].

use iced::{theme, Subscription, Task};

use super::Theme;

/// The Settings value of the theme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Mode {
    /// Follow the OS, live.
    #[default]
    System,
    Light,
    Dark,
}

impl Mode {
    /// What `iced::application(..).theme(..)` returns: `None` lets iced
    /// follow the OS.
    pub fn theme(self) -> Option<Theme> {
        match self {
            Self::System => None,
            Self::Light => Some(Theme::Light),
            Self::Dark => Some(Theme::Dark),
        }
    }

    /// The theme shown when the OS prefers `system`; light when the OS has
    /// no preference, as `prefers-color-scheme` reads in the React app.
    pub fn resolve(self, system: theme::Mode) -> Theme {
        self.theme()
            .unwrap_or_else(|| <Theme as theme::Base>::default(system))
    }
}

impl From<homeport_settings::Theme> for Mode {
    fn from(theme: homeport_settings::Theme) -> Self {
        match theme {
            homeport_settings::Theme::System => Self::System,
            homeport_settings::Theme::Light => Self::Light,
            homeport_settings::Theme::Dark => Self::Dark,
        }
    }
}

/// The OS's preference now.
pub fn system() -> Task<theme::Mode> {
    iced::system::theme()
}

/// The OS's preference, each time it changes while the app runs.
pub fn system_changes() -> Subscription<theme::Mode> {
    iced::system::theme_changes()
}

#[cfg(test)]
mod tests {
    use iced::theme;

    use super::{Mode, Theme};

    #[test]
    fn the_setting_wins_and_system_follows_the_os() {
        assert_eq!(Mode::Light.theme(), Some(Theme::Light));
        assert_eq!(Mode::Dark.theme(), Some(Theme::Dark));
        assert_eq!(Mode::System.theme(), None);
        assert_eq!(Mode::System.resolve(theme::Mode::Dark), Theme::Dark);
        assert_eq!(Mode::System.resolve(theme::Mode::Light), Theme::Light);
        assert_eq!(Mode::System.resolve(theme::Mode::None), Theme::Light);
        assert_eq!(Mode::Light.resolve(theme::Mode::Dark), Theme::Light);
    }

    #[test]
    fn the_settings_value_maps_one_to_one() {
        use homeport_settings::Theme as Setting;
        assert_eq!(Mode::from(Setting::System), Mode::System);
        assert_eq!(Mode::from(Setting::Light), Mode::Light);
        assert_eq!(Mode::from(Setting::Dark), Mode::Dark);
        assert_eq!(Mode::from(Setting::default()), Mode::default());
    }
}
