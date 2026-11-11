//! `Badge`, `Card`, `Separator`, `Progress` and `ScrollArea`: iced has the
//! widget for each, and these give it the measures of the React one.

use iced::{
    widget::{
        container, progress_bar, rule, scrollable, text, Column, Container, ProgressBar, Scrollable,
    },
    Length,
};

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
}

/// `Badge`: a pill that names a state or a count.
pub fn badge<'a, Message: 'a>(
    label: impl Into<String>,
    variant: BadgeVariant,
) -> Element<'a, Message> {
    let style: fn(&Theme) -> container::Style = match variant {
        BadgeVariant::Primary => theme::container::badge::primary,
        BadgeVariant::Secondary => theme::container::badge::secondary,
        BadgeVariant::Destructive => theme::container::badge::destructive,
        BadgeVariant::Outline => theme::container::badge::outline,
    };
    container(text(label.into()).size(size::XS).font(font::SANS_MEDIUM))
        .padding([0.0, space::S2])
        .center_y(control::BADGE)
        .style(style)
        .into()
}

/// `Card`, being described.
pub struct Card<'a, Message> {
    title: Option<String>,
    description: Option<String>,
    body: Vec<Element<'a, Message>>,
    footer: Option<Element<'a, Message>>,
}

/// `Card`: a raised surface around `body`.
pub fn card<'a, Message>(body: impl Into<Element<'a, Message>>) -> Card<'a, Message> {
    Card {
        title: None,
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
        header.extend(self.title.map(|title| {
            paragraph(title)
                .size(size::BASE)
                .font(font::SANS_MEDIUM)
                .into()
        }));
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
    progress_bar(0.0..=100.0, percent.clamp(0.0, 100.0))
        .girth(control::PROGRESS)
        .style(style)
}

/// `ScrollArea`: `content` behind the theme's thin scrollbar, shown only
/// when there is something to scroll.
pub fn scroll_area<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
) -> Scrollable<'a, Message, Theme> {
    scrollable(content).width(Length::Fill)
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
                .font(font::SANS_MEDIUM)
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
