//! The plain parts: buttons, badges, cards, separators, progress bars,
//! scroll areas, rows that wrap and text cut with an ellipsis, as `src/shared/ui/primitives` drew them.

pub mod button;
pub mod fit;
pub mod flow;
pub mod shortcut;
pub mod surface;
pub mod truncate;

pub use self::{
    fit::{fit, Fit},
    flow::{flow, Flow},
    shortcut::shortcut,
    surface::{
        badge, card, collapsible, keys, progress, scroll_area, separator, separator_vertical,
        BadgeVariant,
    },
    truncate::{truncated, Truncated},
};

/// `text` as something a machine named: a file, a path, an id, an
/// image, a key. It is laid out left to right whatever its letters and
/// whatever the language of the app, and kept apart from the text
/// around it: "تقرير.txt" keeps its extension at its end, where a
/// paragraph that takes its direction from its first letter would draw
/// "txt.تقرير". (An isolate that reads left to right, U+2066 to U+2069.)
pub fn technical(text: &str) -> String {
    format!("\u{2066}{text}\u{2069}")
}

#[cfg(test)]
mod tests {
    use super::technical;

    #[test]
    fn a_technical_string_is_isolated_left_to_right() {
        assert_eq!(technical("تقرير.txt"), "\u{2066}تقرير.txt\u{2069}");
        assert_eq!(technical(""), "\u{2066}\u{2069}");
    }
}
