//! The React `Button`, built: a label, an icon or both, in one of the six
//! variants and four sizes, on a [`Pressable`] so the keyboard reaches it.
//!
//! ```ignore
//! button::text(t!("common.save")).on_press(Message::Save)
//! button::labelled(Icon::RotateCw, t!("common.retry"))
//!     .variant(Variant::Outline)
//!     .size(ButtonSize::Sm)
//!     .on_press(Message::Retry)
//! button::icon(Icon::X).variant(Variant::Ghost).on_press(Message::Close)
//! ```

use iced::{
    widget::{button as iced_button, container, svg, text as label_text, Row},
    Length,
};

use crate::ui::{
    theme::{self, font, icon as glyph_of, metrics::ButtonSize, Element, Icon, Theme},
    widgets::{
        focus::{pressable, Pressable},
        line,
    },
};

/// The React `Button`'s `variant`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Variant {
    /// The one action a page or a dialog is for.
    #[default]
    Primary,
    Secondary,
    Outline,
    Ghost,
    Destructive,
    Link,
}

type ButtonStyle = fn(&Theme, iced_button::Status) -> iced_button::Style;
type IconStyle = fn(&Theme, svg::Status) -> svg::Style;

impl Variant {
    pub fn style(self) -> ButtonStyle {
        match self {
            Self::Primary => theme::button::primary,
            Self::Secondary => theme::button::secondary,
            Self::Outline => theme::button::outline,
            Self::Ghost => theme::button::ghost,
            Self::Destructive => theme::button::destructive,
            Self::Link => theme::button::link,
        }
    }

    /// An icon takes no colour from the text around it, so each variant
    /// names the one its label has.
    pub fn icon(self) -> IconStyle {
        match self {
            Self::Primary => theme::svg::on_primary,
            Self::Destructive => theme::svg::destructive,
            Self::Secondary | Self::Outline | Self::Ghost | Self::Link => theme::svg::foreground,
        }
    }
}

/// A button being described; it becomes an element by itself.
pub struct Button<'a, Message> {
    glyph: Option<Icon>,
    label: Option<String>,
    variant: Variant,
    size: ButtonSize,
    width: Option<Length>,
    on_press: Option<Message>,
    before: Option<Message>,
    id: Option<iced::advanced::widget::Id>,
    _lifetime: std::marker::PhantomData<&'a ()>,
}

fn new<'a, Message>(glyph: Option<Icon>, label: Option<String>) -> Button<'a, Message> {
    Button {
        glyph,
        label,
        variant: Variant::default(),
        size: ButtonSize::default(),
        width: None,
        on_press: None,
        before: None,
        id: None,
        _lifetime: std::marker::PhantomData,
    }
}

/// A button that says `label`.
pub fn text<'a, Message>(label: impl Into<String>) -> Button<'a, Message> {
    new(None, Some(label.into()))
}

/// A square button that shows `glyph` alone.
pub fn icon<'a, Message>(glyph: Icon) -> Button<'a, Message> {
    new(Some(glyph), None)
}

/// A button with `glyph` before `label`.
pub fn labelled<'a, Message>(glyph: Icon, label: impl Into<String>) -> Button<'a, Message> {
    new(Some(glyph), Some(label.into()))
}

impl<'a, Message: Clone + 'a> Button<'a, Message> {
    #[must_use]
    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    #[must_use]
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    #[must_use]
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Disabled when `None`.
    #[must_use]
    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
        self
    }

    /// Sent before the press's own message (see [`Pressable::before`]).
    #[must_use]
    pub fn before(mut self, message: Message) -> Self {
        self.before = Some(message);
        self
    }

    #[must_use]
    pub fn id(mut self, id: impl Into<iced::advanced::widget::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// The button, to which more can be added (`on_key`).
    pub fn build(self) -> Pressable<'a, Message> {
        let size = self.size;
        let enabled = self.on_press.is_some();
        let glyph = self.glyph.map(|glyph| {
            let style = self.variant.icon();
            glyph_of(glyph)
                .width(size.icon())
                .height(size.icon())
                .style(move |theme, status| {
                    let mut style = style(theme, status);
                    if !enabled {
                        // `disabled:opacity-50`, as the label's.
                        style.color = style.color.map(|c| theme::color::alpha(c, 0.5));
                    }
                    style
                })
                .into()
        });
        let label = self.label.map(|label| {
            label_text(label)
                .size(size.text())
                .font(font::SANS_MEDIUM)
                .into()
        });
        let square = label.is_none();
        let parts: Vec<Element<'a, Message>> = glyph.into_iter().chain(label).collect();
        let content: Row<'a, Message, Theme> = line(parts).spacing(size.gap());
        let content =
            container(content)
                .center_y(Length::Fill)
                .center_x(if square || self.width.is_some() {
                    Length::Fill
                } else {
                    Length::Shrink
                });
        let button = pressable(content)
            .height(size.height())
            .padding([0.0, if square { 0.0 } else { size.padding_x() }])
            .on_press_maybe(self.on_press);
        let button = match (square, self.width) {
            (_, Some(width)) => button.width(width),
            (true, None) => button.width(size.height()),
            (false, None) => button,
        };
        let button = match self.before {
            Some(before) => button.before(before),
            None => button,
        };
        let button = match self.id {
            Some(id) => button.id(id),
            None => button,
        };
        let style = self.variant.style();
        match size {
            ButtonSize::Default | ButtonSize::Lg => button.style(style),
            ButtonSize::Xs | ButtonSize::Sm => button.style(theme::button::small(style)),
        }
    }
}

impl<'a, Message: Clone + 'a> From<Button<'a, Message>> for Element<'a, Message> {
    fn from(button: Button<'a, Message>) -> Self {
        button.build().into()
    }
}
