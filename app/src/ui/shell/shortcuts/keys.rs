//! Keyboard shortcuts as written in the keymap, `Mod+K`, `Alt+ArrowUp`,
//! `?`: parsing, matching key presses, and the keys to show. `Mod` is ⌘ on
//! macOS and Ctrl elsewhere (`src/shared/hooks/shortcuts/keys.ts`).

use iced::keyboard::{Key, Modifiers};

pub use crate::ui::widgets::basic::shortcut::{format, is_mac, normal, parse, Combo};

/// The parts of a key press matching needs.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Press {
    /// As the browser named it: `k`, `?`, `ArrowUp`.
    pub key: String,
    pub meta: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
}

impl Press {
    pub fn of(key: &str) -> Self {
        Self {
            key: key.to_owned(),
            ..Self::default()
        }
    }

    /// The press iced reports, if it is of a key a shortcut can name.
    pub fn from_iced(key: &Key, modifiers: Modifiers) -> Option<Self> {
        let key = match key {
            Key::Character(text) => text.to_string(),
            Key::Named(named) => format!("{named:?}"),
            Key::Unidentified => return None,
        };
        Some(Self {
            key,
            meta: modifiers.logo(),
            ctrl: modifiers.control(),
            alt: modifiers.alt(),
            shift: modifiers.shift(),
        })
    }
}

/// Where a key press happened, which decides what may fire there.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    /// A terminal, which needs every key.
    Terminal,
    /// A field that types.
    Text,
    /// A dialog, a menu or a list that keeps the keyboard.
    Dialog,
    /// The page itself.
    Page,
}

/// Whether `press` is the shortcut `combo`.
pub fn matches(combo: &Combo, press: &Press, mac: bool) -> bool {
    let (mod_key, other) = if mac {
        (press.meta, press.ctrl)
    } else {
        (press.ctrl, press.meta)
    };
    if mod_key != combo.mod_key || other || press.alt != combo.alt {
        return false;
    }
    if normal(&press.key) != combo.key {
        return false;
    }
    // Letters and named keys care about Shift; `?` or `/` are typed with
    // whatever the layout needs.
    let letter = combo.key.len() == 1 && combo.key.chars().all(|c| c.is_ascii_lowercase());
    let cares = letter || combo.key.chars().count() > 1;
    !cares || press.shift == combo.shift
}

/// Whether `combo` may fire at `target`. Nothing fires in the terminal,
/// which needs every key. Combinations with Mod fire everywhere else; plain
/// keys (and Alt combinations, which editors use) only outside text fields
/// and dialogs, so typing `n` or `/` types it.
pub fn fires_at(combo: &Combo, target: Target) -> bool {
    match target {
        Target::Terminal => false,
        _ if combo.mod_key => true,
        target => target == Target::Page,
    }
}
