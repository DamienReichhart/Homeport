//! `KeyValue`: a list of labelled values in two columns, the labels as
//! wide as the longest of them and the values in what is left, wrapped
//! where they must be (anywhere in a word: a digest has no space).

use iced::{
    advanced::text::{self as core_text, Paragraph as _},
    alignment::Vertical,
    widget::{
        container,
        text::{self, Wrapping},
        Row, Space,
    },
    Length, Pixels, Size,
};

use crate::{
    i18n,
    ui::{
        theme::{
            self, font,
            metrics::{space, text as size},
            Element,
        },
        widgets::{stack, start},
    },
};

/// What stands for a value there is none of, as the tables write it:
/// a mark of no language.
pub const NONE: &str = "—";

/// The most of the list's width the labels take: `max-content` in
/// React, which a label longer than the list would have overflowed.
const LABELS: f32 = 240.0;

/// One labelled value.
pub struct Item<'a, Message> {
    label: String,
    value: Option<Element<'a, Message>>,
}

impl<'a, Message: 'a> Item<'a, Message> {
    /// `label` and what it is: any element (a badge, a link, a button).
    pub fn new(label: impl Into<String>, value: impl Into<Element<'a, Message>>) -> Self {
        Self {
            label: label.into(),
            value: Some(value.into()),
        }
    }

    /// `label` and a text, wrapped where the list ends.
    pub fn text(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self::new(label, said(value.into(), font::DEFAULT))
    }

    /// The same in the monospaced face: an id, a path, a digest.
    pub fn mono(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self::new(
            label,
            said(
                crate::ui::widgets::basic::technical(&value.into()),
                font::MONO,
            ),
        )
    }

    /// `label` and a text if there is one; a dash if not.
    pub fn maybe(label: impl Into<String>, value: Option<String>) -> Self {
        match value {
            Some(value) => Self::text(label, value),
            None => Self {
                label: label.into(),
                value: None,
            },
        }
    }
}

fn said<'a, Message: 'a>(value: String, font: iced::Font) -> Element<'a, Message> {
    text::Text::new(value)
        .size(size::SM)
        .line_height(size::LEADING_SM)
        .font(font)
        // `break-all`: a value with no space still wraps.
        .wrapping(Wrapping::WordOrGlyph)
        .into()
}

type Raw = <iced::Renderer as core_text::Renderer>::Paragraph;

/// How wide `label` is drawn.
fn measure(label: &str) -> f32 {
    Raw::with_text(core_text::Text {
        content: label,
        bounds: Size::INFINITE,
        size: Pixels(size::SM),
        line_height: core_text::LineHeight::default(),
        font: font::DEFAULT,
        align_x: core_text::Alignment::Default,
        align_y: Vertical::Top,
        // As the label itself is shaped, so the two agree.
        shaping: core_text::Shaping::default(),
        wrapping: core_text::Wrapping::None,
    })
    .min_bounds()
    .width
}

/// `items`, one under the other: each label where text starts, in the
/// muted colour, and its value after it.
pub fn key_value<'a, Message: 'a>(items: Vec<Item<'a, Message>>) -> Element<'a, Message> {
    let widest = items
        .iter()
        .map(|item| measure(&item.label))
        .fold(0.0, f32::max);
    // A pixel to spare: a label as wide as its room to the last
    // fraction would wrap.
    let labels = (widest.ceil() + 1.0).min(LABELS);
    let rows = items.into_iter().map(|item| {
        let label = text::Text::new(item.label)
            .size(size::SM)
            .line_height(size::LEADING_SM)
            .style(theme::text::muted);
        let value = item
            .value
            .unwrap_or_else(|| said(NONE.to_owned(), font::DEFAULT));
        let cells: Vec<Element<'a, Message>> = vec![
            container(label).width(labels).align_x(start()).into(),
            Space::new().width(space::S6).into(),
            container(value).width(Length::Fill).align_x(start()).into(),
        ];
        // The label stays at the top of a value of several lines.
        Row::from_vec(i18n::direction().ordered(cells))
            .align_y(Vertical::Top)
            .width(Length::Fill)
            .into()
    });
    stack(rows.collect()).spacing(space::S1_5).into()
}
