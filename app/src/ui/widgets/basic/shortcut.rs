//! A shortcut as the keymap writes it, `Mod+K`, `Alt+ArrowUp`, `?`, and
//! the keys to show for it on this computer: `Mod` is ⌘ on macOS and
//! Ctrl elsewhere (`src/shared/hooks/shortcuts/keys.ts`). The shell
//! matches key presses against the same reading (`shell::shortcuts`).

use crate::ui::theme::Element;

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

/// A key as shortcuts compare it: a letter in lower case.
pub fn normal(key: &str) -> String {
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

/// `Keys`: the keys of the shortcut `spec`, as this computer names them.
pub fn shortcut<'a, Message: 'a>(spec: &str) -> Element<'a, Message> {
    super::keys(&format(spec, is_mac()))
}
