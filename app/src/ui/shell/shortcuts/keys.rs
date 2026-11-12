//! Keyboard shortcuts as written in the keymap, `Mod+K`, `Alt+ArrowUp`,
//! `?`: parsing, matching key presses, and the keys to show. `Mod` is ⌘ on
//! macOS and Ctrl elsewhere (`src/shared/hooks/shortcuts/keys.ts`).

use iced::keyboard::{Key, Modifiers};

/// A shortcut, parsed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Combo {
    /// ⌘ on macOS, Ctrl elsewhere.
    pub mod_key: bool,
    pub alt: bool,
    pub shift: bool,
    /// The key's name, lower case for letters.
    pub key: String,
}

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

fn normal(key: &str) -> String {
    if key.chars().count() == 1 {
        key.to_lowercase()
    } else {
        key.to_owned()
    }
}

pub fn parse(spec: &str) -> Combo {
    // `Mod++` would be ambiguous; no shortcut uses `+` as its key.
    let mut parts: Vec<&str> = spec.split('+').collect();
    let key = parts.pop().unwrap_or_default();
    let has = |name: &str| parts.iter().any(|part| part.eq_ignore_ascii_case(name));
    Combo {
        mod_key: has("mod"),
        alt: has("alt"),
        shift: has("shift"),
        key: normal(key),
    }
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

/// The keys to show for `spec`, in order: `["⌘", "K"]` or `["Ctrl", "K"]`.
pub fn format(spec: &str, mac: bool) -> Vec<String> {
    let combo = parse(spec);
    let pick = |on_mac: &str, other: &str| if mac { on_mac } else { other }.to_owned();
    let mut keys = vec![];
    if combo.mod_key {
        keys.push(pick("⌘", "Ctrl"));
    }
    if combo.alt {
        keys.push(pick("⌥", "Alt"));
    }
    if combo.shift {
        keys.push(pick("⇧", "Shift"));
    }
    keys.push(match combo.key.as_str() {
        "ArrowUp" => "↑".to_owned(),
        "ArrowDown" => "↓".to_owned(),
        "ArrowLeft" => "←".to_owned(),
        "ArrowRight" => "→".to_owned(),
        "Enter" => pick("↵", "Enter"),
        "Escape" => "Esc".to_owned(),
        key => key.to_uppercase(),
    });
    keys
}

/// Whether this is a Mac, where Mod is ⌘.
pub const fn is_mac() -> bool {
    cfg!(target_os = "macos")
}
