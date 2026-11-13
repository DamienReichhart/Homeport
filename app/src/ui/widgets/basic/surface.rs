//! `Badge`, `Card`, `Separator`, `Progress` and `ScrollArea`: iced has the
//! widget for each, and these give it the measures of the React one.

use iced::{
    widget::{
        container, progress_bar, rule, scrollable, text, Column, Container, ProgressBar, Scrollable,
    },
    Length,
};

use crate::i18n;
use crate::ui::{
    theme::{
        self, font,
        metrics::{control, space, text as size},
        Element, Theme, Tone,
    },
    widgets::{paragraph, stack},
};

/// The React `Badge`'s `variant`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum BadgeVariant {
    #[default]
    Primary,
    Secondary,
    Destructive,
    Outline,
    Ghost,
    Link,
    /// An outline whose text says it went well (`variant="outline"
    /// className="text-success"`): a check that passed.
    Passed,
    /// An outline in the destructive colour, border and text
    /// (`border-destructive text-destructive`): a check that failed.
    Failed,
}

/// `Keys`: a shortcut's keys, each in its `<kbd>`, in the order they are
/// pressed (`["Ctrl", "K"]`). Keys read left to right in every language.
pub fn keys<'a, Message: 'a>(keys: &[String]) -> Element<'a, Message> {
    let caps = keys.iter().map(|key| {
        container(text(key.clone()).size(11).font(font::MONO))
            .padding([0.0, space::S1])
            .center_x(iced::Length::Shrink)
            .center_y(16)
            .style(theme::container::kbd)
            .into()
    });
    iced::widget::Row::with_children(caps)
        .spacing(space::S0_5)
        .align_y(iced::alignment::Vertical::Center)
        .into()
}

/// `Badge`: a pill that names a state or a count.
pub fn badge<'a, Message: 'a>(
    label: impl Into<String>,
    variant: BadgeVariant,
) -> Element<'a, Message> {
    let pill = container(text(label.into()).size(size::XS).font(font::medium()))
        .padding([0.0, space::S2])
        .center_y(control::BADGE);
    match variant {
        BadgeVariant::Primary => pill.style(theme::container::badge::primary),
        BadgeVariant::Secondary => pill.style(theme::container::badge::secondary),
        BadgeVariant::Destructive => pill.style(theme::container::badge::destructive),
        BadgeVariant::Outline => pill.style(theme::container::badge::outline),
        // A badge that isn't pressed: the ghost at rest.
        BadgeVariant::Ghost => pill.style(theme::container::badge::ghost),
        BadgeVariant::Link => pill.style(theme::container::badge::link),
        BadgeVariant::Passed => pill.style(theme::container::badge::passed),
        BadgeVariant::Failed => pill.style(theme::container::badge::failed),
    }
    .into()
}

/// `Card`, being described.
pub struct Card<'a, Message> {
    title: Option<String>,
    aside: Option<String>,
    description: Option<String>,
    body: Vec<Element<'a, Message>>,
    footer: Option<Element<'a, Message>>,
}

/// `Card`: a raised surface around `body`.
pub fn card<'a, Message>(body: impl Into<Element<'a, Message>>) -> Card<'a, Message> {
    Card {
        title: None,
        aside: None,
        description: None,
        body: vec![body.into()],
        footer: None,
    }
}

impl<'a, Message: 'a> Card<'a, Message> {
    /// `CardTitle`.
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// What is said beside the title, muted: the figure of the moment
    /// after a chart's name (`CPU · 12.5%`). It goes on a line of its own
    /// when the title leaves it no room.
    #[must_use]
    pub fn aside(mut self, aside: impl Into<String>) -> Self {
        self.aside = Some(aside.into());
        self
    }

    /// `CardDescription`, under the title.
    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// `CardFooter`: under a hairline, on the quieter fill.
    #[must_use]
    pub fn footer(mut self, footer: impl Into<Element<'a, Message>>) -> Self {
        self.footer = Some(footer.into());
        self
    }

    pub fn view(self) -> Container<'a, Message, Theme> {
        let mut parts: Vec<Element<'a, Message>> = vec![];
        let mut header: Vec<Element<'a, Message>> = vec![];
        let title = self.title.map(|title| -> Element<'a, Message> {
            paragraph(title)
                .size(size::BASE)
                .font(font::medium())
                .into()
        });
        let aside = self.aside.map(|aside| -> Element<'a, Message> {
            paragraph(aside)
                .size(size::BASE)
                .style(theme::text::muted)
                .into()
        });
        header.extend(match (title, aside) {
            (Some(title), Some(aside)) => {
                Some(super::flow(vec![title, aside]).spacing(space::S1_5).into())
            }
            (title, aside) => title.or(aside),
        });
        header.extend(
            self.description
                .map(|words| paragraph(words).style(theme::text::muted).into()),
        );
        if !header.is_empty() {
            parts.push(stack(header).spacing(space::S1).into());
        }
        parts.extend(self.body);
        let mut whole = Column::new().push(
            container(stack(parts).spacing(space::S4))
                .padding(space::S4)
                .width(Length::Fill),
        );
        if let Some(footer) = self.footer {
            whole = whole.push(rule::horizontal(1)).push(
                container(footer)
                    .padding(space::S4)
                    .width(Length::Fill)
                    .align_x(crate::ui::widgets::start())
                    .style(theme::container::dialog_footer),
            );
        }
        container(whole)
            .width(Length::Fill)
            .style(theme::container::card)
    }
}

impl<'a, Message: 'a> From<Card<'a, Message>> for Element<'a, Message> {
    fn from(card: Card<'a, Message>) -> Self {
        card.view().into()
    }
}

/// `Separator`: a hairline across.
pub fn separator<'a, Message: 'a>() -> Element<'a, Message> {
    rule::horizontal(1).into()
}

/// `Separator orientation="vertical"`: a hairline down, as tall as its row.
pub fn separator_vertical<'a, Message: 'a>() -> Element<'a, Message> {
    rule::vertical(1).into()
}

/// `Progress`: how far something is, from 0 to 100. `tone` colours it when
/// it means more than progress: amber for nearly full, red for full.
pub fn progress<'a>(percent: f32, tone: Option<Tone>) -> ProgressBar<'a, Theme> {
    let style: fn(&Theme) -> progress_bar::Style = match tone {
        Some(Tone::Warning) => theme::progress_bar::warning,
        Some(Tone::Danger) => theme::progress_bar::destructive,
        _ => theme::progress_bar::progress,
    };
    let percent = percent.clamp(0.0, 100.0);
    let (filled, rtl) = filled_from_the_left(percent, i18n::direction());
    progress_bar(0.0..=100.0, filled)
        .girth(control::PROGRESS)
        .style(move |theme| {
            let mut style = style(theme);
            if rtl {
                std::mem::swap(&mut style.background, &mut style.bar);
            }
            style
        })
}

/// iced's bar fills from the left. Right to left, progress starts on the
/// right: the bar is drawn as what is left to do, in the track's colour,
/// over a track in the bar's colour. Returns what to fill from the left and
/// whether the two colours swap.
fn filled_from_the_left(percent: f32, direction: i18n::Direction) -> (f32, bool) {
    if direction.is_rtl() {
        (100.0 - percent, true)
    } else {
        (percent, false)
    }
}

/// `ScrollArea`: `content` behind the theme's thin scrollbar, shown only
/// when there is something to scroll. iced draws the bar on the right in
/// every language; right to left that is where text starts, so the bar is
/// then given room of its own instead of lying over the text.
pub fn scroll_area<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
) -> Scrollable<'a, Message, Theme> {
    use iced::widget::scrollable::{Direction, Scrollbar};
    let bar = Scrollbar::new();
    let bar = match bar_room(i18n::direction()) {
        Some(room) => bar.spacing(room),
        None => bar,
    };
    scrollable(content)
        .direction(Direction::Vertical(bar))
        .width(Length::Fill)
}

/// The room kept between the text and the scrollbar, where the bar would
/// otherwise lie over the text.
fn bar_room(direction: i18n::Direction) -> Option<f32> {
    direction.is_rtl().then_some(space::S2)
}

/// `Collapsible`: a line that shows or hides what is under it. Enter and
/// Space toggle it, as any button; `on_toggle` gets the state it asks for.
pub fn collapsible<'a, Message: Clone + 'a>(
    title: impl Into<String>,
    open: bool,
    on_toggle: impl Fn(bool) -> Message,
    content: impl FnOnce() -> Element<'a, Message>,
) -> Element<'a, Message> {
    use crate::ui::{
        theme::{icon, Icon},
        widgets::{focus::pressable, line},
    };
    let chevron = match (open, crate::i18n::direction().is_rtl()) {
        (true, _) => Icon::ChevronDown,
        (false, false) => Icon::ChevronRight,
        (false, true) => Icon::ChevronLeft,
    };
    let trigger = pressable(
        line(vec![
            icon(chevron).style(theme::svg::muted).into(),
            text(title.into())
                .size(size::SM)
                .font(font::medium())
                .into(),
        ])
        .spacing(space::S1_5),
    )
    .padding([space::S1, space::S1_5])
    .on_press(on_toggle(!open))
    .style(theme::button::small(theme::button::ghost));
    let mut parts: Vec<Element<'a, Message>> = vec![trigger.into()];
    if open {
        parts.push(content());
    }
    stack(parts).spacing(space::S2).into()
}

#[cfg(test)]
mod tests {
    use super::{bar_room, filled_from_the_left};
    use crate::i18n::Direction;

    #[test]
    fn progress_starts_where_text_starts() {
        assert_eq!(filled_from_the_left(30.0, Direction::Ltr), (30.0, false));
        // Right to left, the 70 % still to do are drawn from the left in
        // the track's colour: the 30 % done show on the right.
        assert_eq!(filled_from_the_left(30.0, Direction::Rtl), (70.0, true));
    }

    #[test]
    fn the_scrollbar_keeps_off_text_that_starts_on_its_side() {
        assert_eq!(bar_room(Direction::Ltr), None);
        assert!(bar_room(Direction::Rtl).is_some());
    }
}
