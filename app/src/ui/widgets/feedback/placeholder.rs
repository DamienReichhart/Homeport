//! What stands where content isn't: `Skeleton` while it loads, `EmptyState`
//! when there is none, and `StatusDot`, the shape of a status.

use iced::{
    alignment::Horizontal,
    widget::{container, svg, text, Column, Space},
    Border, Length,
};

use crate::ui::theme::{
    self, font, icon,
    metrics::{radius, space, text as size},
    Element, Icon, Theme, Tone,
};

/// `Skeleton`: a quiet block of `width` by `height` where something is
/// still loading. The React one pulses; this one is still, as the React
/// one is for who asks the OS for reduced motion.
pub fn skeleton<'a, Message: 'a>(
    width: impl Into<Length>,
    height: impl Into<Length>,
) -> Element<'a, Message> {
    container(Space::new())
        .width(width)
        .height(height)
        .style(theme::container::skeleton)
        .into()
}

/// `max-w-md`: an empty state's description wraps past this.
const DESCRIPTION_WIDTH: f32 = 448.0;

/// `EmptyState`, being described.
pub struct EmptyState<'a, Message> {
    media: Option<Element<'a, Message>>,
    title: String,
    description: Option<String>,
    action: Option<Element<'a, Message>>,
}

/// What a page shows when it has nothing to list: `title`, and why or what
/// to do under it.
pub fn empty_state<'a, Message>(title: impl Into<String>) -> EmptyState<'a, Message> {
    EmptyState {
        media: None,
        title: title.into(),
        description: None,
        action: None,
    }
}

impl<'a, Message: 'a> EmptyState<'a, Message> {
    /// The icon over the title.
    #[must_use]
    pub fn icon(self, glyph: Icon) -> Self {
        self.media(icon(glyph).width(32).height(32).style(theme::svg::muted))
    }

    /// Shown instead of an icon: the logo on a welcome screen.
    #[must_use]
    pub fn media(mut self, media: impl Into<Element<'a, Message>>) -> Self {
        self.media = Some(media.into());
        self
    }

    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// The button that fills the page: "Add a Host".
    #[must_use]
    pub fn action(mut self, action: impl Into<Element<'a, Message>>) -> Self {
        self.action = Some(action.into());
        self
    }

    pub fn view(self) -> Element<'a, Message> {
        let mut parts = Column::new()
            .spacing(space::S2)
            .align_x(Horizontal::Center)
            .width(Length::Fill);
        if let Some(media) = self.media {
            parts = parts.push(media);
        }
        parts = parts.push(
            text(self.title)
                .size(size::BASE)
                .font(font::SANS_MEDIUM)
                .align_x(Horizontal::Center),
        );
        if let Some(description) = self.description {
            parts = parts.push(
                container(
                    text(description)
                        .size(size::SM)
                        .line_height(size::LEADING_SM)
                        .style(theme::text::muted)
                        .align_x(Horizontal::Center),
                )
                .max_width(DESCRIPTION_WIDTH),
            );
        }
        if let Some(action) = self.action {
            parts = parts.push(container(action).padding(iced::Padding {
                top: space::S2,
                ..iced::Padding::ZERO
            }));
        }
        // The React one is edged with dashes; iced draws no dashed border,
        // so the edge is the same hairline, whole.
        container(parts)
            .padding(space::S8 + space::S2)
            .width(Length::Fill)
            .style(theme::container::bordered)
            .into()
    }
}

impl<'a, Message: 'a> From<EmptyState<'a, Message>> for Element<'a, Message> {
    fn from(empty: EmptyState<'a, Message>) -> Self {
        empty.view()
    }
}

/// The dot's box: `size-2`.
const DOT: f32 = 8.0;

/// The warning's diamond: a 7 px square on its corner, in the dot's box.
const DIAMOND: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><rect x="1.5" y="1.5" width="7" height="7" rx="1" transform="rotate(45 5 5)" fill="currentColor"/></svg>"#;

/// `StatusDot`: a status is a shape as well as a colour, so it reads in
/// greyscale and for colour-blind eyes (docs/BRAND.md).
///
/// ```text
/// success  ● ink disc          danger   ● Signal red disc
/// muted    ○ grey ring         warning  ◆ amber diamond
/// info     ○ ink ring
/// ```
///
/// The React dot pulses while connecting; this one is still, and the ring
/// says it.
pub fn status_dot<'a, Message: 'a>(tone: Tone) -> Element<'a, Message> {
    let ring = matches!(tone, Tone::Muted | Tone::Info);
    if tone == Tone::Warning {
        return iced::widget::svg(svg::Handle::from_memory(DIAMOND))
            .width(DOT + 2.0)
            .height(DOT + 2.0)
            .style(theme::svg::tone(tone))
            .into();
    }
    container(Space::new())
        .width(DOT)
        .height(DOT)
        .style(move |theme: &Theme| {
            let colour = tone.colour(theme.tokens());
            container::Style {
                background: (!ring).then_some(colour.into()),
                border: Border {
                    color: colour,
                    width: if ring { 1.5 } else { 0.0 },
                    radius: radius::FULL.into(),
                },
                ..container::Style::default()
            }
        })
        .into()
}
