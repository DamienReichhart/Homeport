//! Fields: `Input`, `InputGroup` (something before or after the text, in
//! the field), the password field with its reveal button, and `Textarea`,
//! which grows with what is typed.

use iced::{
    alignment::Vertical,
    widget::{container, text_editor, text_input, Stack, TextEditor, TextInput},
    Length, Padding,
};

use crate::{
    i18n, t,
    ui::{
        theme::{
            self,
            metrics::{control, space, text as size, ButtonSize},
            Element, Icon, Theme,
        },
        widgets::{
            basic::button::{self, Variant},
            end,
            overlay::{tooltip, Side},
            start,
        },
    },
};

/// `max-h`: a textarea grows to this, then scrolls.
pub const TEXTAREA_MAX: f32 = 240.0;

/// `Input`: a one-line field of the React measures. Without `on_input` it
/// is disabled; `.style(theme::text_input::invalid)` marks a wrong value.
pub fn input<'a, Message: Clone + 'a>(
    placeholder: &str,
    value: &str,
) -> TextInput<'a, Message, Theme> {
    text_input(placeholder, value)
        .size(size::SM)
        .padding([space::S1, control::PADDING_X])
        .align_x(start())
}

/// Something that sits in a field, before or after its text, and how wide
/// it is: the field keeps that much free for it.
pub struct Addon<'a, Message> {
    content: Element<'a, Message>,
    width: f32,
}

impl<'a, Message> Addon<'a, Message> {
    pub fn new(content: impl Into<Element<'a, Message>>, width: f32) -> Self {
        Self {
            content: content.into(),
            width,
        }
    }

    /// An icon: the search field's magnifier.
    pub fn icon(glyph: Icon) -> Self
    where
        Message: 'a,
    {
        Self::new(
            theme::icon(glyph).style(theme::svg::muted),
            theme::metrics::icon::MD,
        )
    }
}

/// `InputGroup`: `field` with `before` where its text starts and `after`
/// where it ends, inside its border. The field stays the field: its border
/// shows focus and a wrong value as any other.
pub fn input_group<'a, Message: Clone + 'a>(
    field: TextInput<'a, Message, Theme>,
    before: Option<Addon<'a, Message>>,
    after: Option<Addon<'a, Message>>,
) -> Element<'a, Message> {
    let room = |addon: &Option<Addon<'a, Message>>| {
        addon
            .as_ref()
            .map_or(control::PADDING_X, |addon| addon.width + 2.0 * space::S2)
    };
    let (left, right) = i18n::direction().sides(room(&before), room(&after));
    let field = field.padding(Padding {
        top: space::S1,
        bottom: space::S1,
        left,
        right,
    });
    let mut layers: Vec<Element<'a, Message>> = vec![field.into()];
    let mut place = |addon: Option<Addon<'a, Message>>, side| {
        layers.extend(addon.map(|addon| {
            container(addon.content)
                .padding([0.0, space::S2])
                .width(Length::Fill)
                .height(control::HEIGHT)
                .align_x(side)
                .align_y(Vertical::Center)
                .into()
        }));
    };
    place(before, start());
    place(after, end());
    Stack::with_children(layers).width(Length::Fill).into()
}

/// A password field: dots, and a button at its end that shows what was
/// typed (and hides it again).
pub fn password<'a, Message: Clone + 'a>(
    placeholder: &str,
    value: &str,
    revealed: bool,
    on_input: impl Fn(String) -> Message + 'a,
    on_reveal: Message,
) -> Element<'a, Message> {
    let field = input(placeholder, value)
        .secure(!revealed)
        .on_input(on_input);
    let (glyph, says) = if revealed {
        (Icon::EyeOff, t!("ui.password.hide"))
    } else {
        (Icon::Eye, t!("ui.password.show"))
    };
    let reveal = button::icon(glyph)
        .variant(Variant::Ghost)
        .size(ButtonSize::Xs)
        .on_press(on_reveal);
    let reveal = tooltip(reveal, says, Side::Top);
    // The button sits closer to the edge than an addon's text would.
    let after = Addon::new(reveal, ButtonSize::Xs.height() - space::S2);
    input_group(field, None, Some(after))
}

/// `Textarea`: a field of several lines, as tall as what is typed in it
/// between its least height and [`TEXTAREA_MAX`], where it starts to scroll.
pub fn textarea<'a, Message: Clone + 'a>(
    content: &'a text_editor::Content,
    on_action: impl Fn(text_editor::Action) -> Message + 'a,
) -> TextEditor<'a, iced::advanced::text::highlighter::PlainText, Message, Theme> {
    text_editor(content)
        .on_action(on_action)
        .size(size::SM)
        .padding([space::S2, control::PADDING_X])
        .height(Length::Shrink)
        .min_height(control::TEXTAREA_MIN)
        .max_height(TEXTAREA_MAX)
}
