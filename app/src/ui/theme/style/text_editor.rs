//! The multi-line field (`Textarea`) and the code editor's surface
//! (`editorTheme.ts`). The editor's syntax colours are `Tokens::syntax`,
//! for the highlighter the editor widget brings.

use iced::widget::text_editor::{Catalog, Status, Style, StyleFn};

use super::{alpha, field, hairline, radius, selection, Theme, DISABLED};

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(textarea)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

/// `Textarea`: a field with several lines.
pub fn textarea(theme: &Theme, status: Status) -> Style {
    let tokens = theme.tokens();
    let active = Style {
        background: field(theme).into(),
        border: hairline(tokens.input, radius::LG),
        placeholder: tokens.muted_foreground,
        value: tokens.foreground,
        selection: selection(tokens),
    };
    match status {
        Status::Active | Status::Hovered => active,
        Status::Focused { .. } => Style {
            border: hairline(tokens.ring, radius::LG),
            ..active
        },
        Status::Disabled => {
            let fill = if theme.is_dark() { 0.8 } else { 0.5 };
            Style {
                background: alpha(tokens.input, fill * DISABLED).into(),
                border: hairline(alpha(tokens.input, DISABLED), radius::LG),
                placeholder: alpha(active.placeholder, DISABLED),
                value: alpha(active.value, DISABLED),
                ..active
            }
        }
    }
}

/// The code editor: a card of machine text, its selection the primary
/// colour at 22 %. Focus changes nothing: the caret says it.
pub fn code(theme: &Theme, status: Status) -> Style {
    let tokens = theme.tokens();
    let opacity = if status == Status::Disabled {
        DISABLED
    } else {
        1.0
    };
    Style {
        background: tokens.card.into(),
        border: hairline(tokens.border, radius::MD),
        placeholder: alpha(tokens.muted_foreground, opacity),
        value: alpha(tokens.card_foreground, opacity),
        selection: alpha(tokens.primary, 0.22),
    }
}
