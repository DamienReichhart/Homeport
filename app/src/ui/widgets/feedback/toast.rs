//! Toasts: what happened, said in a corner of the window for a few
//! seconds (the React app's Sonner). The newest is nearest the corner, three
//! show at once, each goes by itself, and resting the pointer on them stops
//! the clocks. A failure's toast stays longer and has a Details button.
//!
//! The window keeps one [`Toasts`], adds to it, gives it the messages of
//! [`Toasts::view`] and runs [`Toasts::subscription`]. A toast keeps what
//! it was told, not its text: it is said again in the language of the
//! moment every time it is drawn.

use std::{fmt, sync::Arc, time::Duration};

use iced::{
    alignment::Vertical,
    widget::{container, mouse_area, svg, Column},
    Length, Subscription,
};

use super::surfaced::Surfaced;
use crate::{
    i18n, t,
    ui::{
        theme::{
            self, font, icon,
            metrics::{space, ButtonSize},
            Element, Icon, Theme, Tokens,
        },
        widgets::{
            basic::button::{self, Variant},
            end, line, paragraph, stack,
        },
    },
};

/// How long a toast stays (Sonner's four seconds).
pub const LIFETIME: Duration = Duration::from_secs(4);
/// How long a failure's toast stays: there is more to read, and a button.
pub const ERROR_LIFETIME: Duration = Duration::from_secs(10);
/// How many show at once; the others wait their turn underneath.
pub const VISIBLE: usize = 3;
/// How often the clocks move.
const TICK: Duration = Duration::from_millis(100);
/// Sonner's width.
const WIDTH: f32 = 356.0;

/// What a toast reports, which is its colour and its icon.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Success,
    Info,
    Warning,
    Error,
}

impl Kind {
    fn glyph(self) -> Icon {
        match self {
            Self::Success => Icon::CircleCheck,
            Self::Info => Icon::Info,
            Self::Warning => Icon::TriangleAlert,
            Self::Error => Icon::OctagonX,
        }
    }

    fn colour(self) -> fn(&Tokens) -> iced::Color {
        match self {
            Self::Success => |tokens| tokens.success,
            Self::Info => |tokens| tokens.info,
            Self::Warning => |tokens| tokens.warning,
            Self::Error => |tokens| tokens.destructive,
        }
    }
}

/// Text said when it is drawn: `|| t!("containers.started")`.
#[derive(Clone)]
pub struct Said(Arc<dyn Fn() -> String + Send + Sync>);

impl Said {
    pub fn new(say: impl Fn() -> String + Send + Sync + 'static) -> Self {
        Self(Arc::new(say))
    }

    fn now(&self) -> String {
        (self.0)()
    }
}

impl fmt::Debug for Said {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Said({:?})", self.now())
    }
}

#[derive(Debug, Clone)]
enum Body {
    Said {
        title: Said,
        description: Option<Said>,
    },
    /// A failure, with the title that names what failed when the message
    /// alone wouldn't say.
    Failure {
        title: Option<Said>,
        surfaced: Box<Surfaced>,
    },
}

/// One toast.
#[derive(Debug, Clone)]
pub struct Toast {
    pub id: u64,
    pub kind: Kind,
    body: Body,
    /// How long it still stays.
    left: Duration,
}

impl Toast {
    /// The title and the line under it, in the app's language.
    pub fn text(&self) -> (String, Option<String>) {
        match &self.body {
            Body::Said { title, description } => (title.now(), description.as_ref().map(Said::now)),
            Body::Failure {
                title: Some(title),
                surfaced,
            } => (title.now(), Some(i18n::errors::message(&surfaced.error))),
            Body::Failure {
                title: None,
                surfaced,
            } => (
                i18n::errors::message(&surfaced.error),
                surfaced
                    .error
                    .hint
                    .as_ref()
                    .map(|hint| i18n::errors::hint(hint))
                    .filter(|advice| !advice.is_empty()),
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    Dismiss(u64),
    /// The pointer came over the toasts, or left them.
    Hovered(bool),
    Tick,
    /// The Details button of a failure's toast.
    Details(u64),
}

/// The toasts on screen and those waiting, oldest first.
#[derive(Debug, Default)]
pub struct Toasts {
    list: Vec<Toast>,
    next: u64,
    hovered: bool,
}

impl Toasts {
    fn add(&mut self, kind: Kind, body: Body, left: Duration) -> u64 {
        let id = self.next;
        self.next += 1;
        self.list.push(Toast {
            id,
            kind,
            body,
            left,
        });
        id
    }

    /// A toast of `kind` that says `title`.
    pub fn say(&mut self, kind: Kind, title: impl Fn() -> String + Send + Sync + 'static) -> u64 {
        let body = Body::Said {
            title: Said::new(title),
            description: None,
        };
        self.add(kind, body, LIFETIME)
    }

    /// The same with a line under the title.
    pub fn say_more(
        &mut self,
        kind: Kind,
        title: impl Fn() -> String + Send + Sync + 'static,
        description: impl Fn() -> String + Send + Sync + 'static,
    ) -> u64 {
        let body = Body::Said {
            title: Said::new(title),
            description: Some(Said::new(description)),
        };
        self.add(kind, body, LIFETIME)
    }

    /// `toastError(error)`: the failure's message, its hint's advice, and a
    /// Details button.
    pub fn failure(&mut self, surfaced: Surfaced) -> u64 {
        let body = Body::Failure {
            title: None,
            surfaced: Box::new(surfaced),
        };
        self.add(Kind::Error, body, ERROR_LIFETIME)
    }

    /// `toastError(error, title)`: `title` names what failed, the message
    /// goes under it.
    pub fn failure_of(
        &mut self,
        title: impl Fn() -> String + Send + Sync + 'static,
        surfaced: Surfaced,
    ) -> u64 {
        let body = Body::Failure {
            title: Some(Said::new(title)),
            surfaced: Box::new(surfaced),
        };
        self.add(Kind::Error, body, ERROR_LIFETIME)
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    /// Those on screen, oldest first: the last is nearest the corner.
    pub fn visible(&self) -> &[Toast] {
        &self.list[self.list.len().saturating_sub(VISIBLE)..]
    }

    /// Moves the clocks of the toasts on screen by `by`, unless the pointer
    /// rests on them, and drops those whose time is up. One that waits
    /// underneath keeps its whole time for when it shows.
    pub fn advance(&mut self, by: Duration) {
        if self.hovered {
            return;
        }
        let first = self.list.len().saturating_sub(VISIBLE);
        for toast in &mut self.list[first..] {
            toast.left = toast.left.saturating_sub(by);
        }
        self.list.retain(|toast| !toast.left.is_zero());
    }

    /// Returns the failure whose Details were asked for, for the window to
    /// open the dialog; the toast goes, as its job is done.
    pub fn update(&mut self, message: Message) -> Option<Surfaced> {
        match message {
            Message::Tick => self.advance(TICK),
            Message::Hovered(hovered) => self.hovered = hovered,
            Message::Dismiss(id) => self.remove(id),
            Message::Details(id) => {
                let surfaced = self
                    .list
                    .iter()
                    .find(|t| t.id == id)
                    .and_then(|toast| match &toast.body {
                        Body::Failure { surfaced, .. } => Some((**surfaced).clone()),
                        Body::Said { .. } => None,
                    });
                self.remove(id);
                return surfaced;
            }
        }
        None
    }

    fn remove(&mut self, id: u64) {
        self.list.retain(|toast| toast.id != id);
        if self.list.is_empty() {
            // Nothing is left for the pointer to leave.
            self.hovered = false;
        }
    }

    /// The clock, which only runs while there is a toast.
    pub fn subscription(&self) -> Subscription<Message> {
        if self.list.is_empty() {
            Subscription::none()
        } else {
            iced::time::every(TICK).map(|_| Message::Tick)
        }
    }

    /// The toasts, to lay over the page with [`over`]; `None` without any.
    pub fn view(&self) -> Option<Element<'_, Message>> {
        if self.list.is_empty() {
            return None;
        }
        let toasts = self.visible().iter().map(toast);
        let column = Column::with_children(toasts)
            .spacing(space::S2)
            .width(WIDTH);
        Some(
            mouse_area(column)
                .on_enter(Message::Hovered(true))
                .on_exit(Message::Hovered(false))
                .into(),
        )
    }
}

/// `page` with `toasts` in its bottom corner, on the side text ends.
pub fn over<'a, Message: 'a>(
    page: impl Into<Element<'a, Message>>,
    toasts: Option<Element<'a, Message>>,
) -> Element<'a, Message> {
    let page = page.into();
    let Some(toasts) = toasts else {
        return page;
    };
    let corner = container(toasts)
        .padding(space::S4)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(end())
        .align_y(Vertical::Bottom);
    iced::widget::Stack::with_children([page, corner.into()])
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn tinted(colour: fn(&Tokens) -> iced::Color) -> impl Fn(&Theme, svg::Status) -> svg::Style {
    move |theme, _| svg::Style {
        color: Some(colour(theme.tokens())),
    }
}

fn toast(toast: &Toast) -> Element<'_, Message> {
    let colour = toast.kind.colour();
    let (title, description) = toast.text();
    let mut words: Vec<Element<'_, Message>> =
        vec![paragraph(title).font(font::SANS_MEDIUM).into()];
    words.extend(description.map(|description| paragraph(description).into()));
    let mut parts: Vec<Element<'_, Message>> = vec![
        container(icon(toast.kind.glyph()).style(tinted(colour)))
            .padding([space::S0_5, 0.0])
            .into(),
        stack(words).spacing(space::S0_5).into(),
    ];
    if matches!(toast.body, Body::Failure { .. }) {
        parts.push(
            button::text(t!("common.details"))
                .variant(Variant::Outline)
                .size(ButtonSize::Xs)
                .on_press(Message::Details(toast.id))
                .into(),
        );
    }
    parts.push(
        button::icon(Icon::X)
            .variant(Variant::Ghost)
            .size(ButtonSize::Xs)
            .on_press(Message::Dismiss(toast.id))
            .into(),
    );
    container(line(parts).spacing(space::S2).align_y(Vertical::Top))
        .padding(space::S3)
        .width(Length::Fill)
        .style(theme::container::toast(colour))
        .into()
}

#[cfg(test)]
mod tests {
    use homeport_core::{
        error::{AppError, ErrorCode},
        messages as msg,
    };

    use super::*;

    fn failure() -> Surfaced {
        let error = AppError::new(ErrorCode::Io, msg::IO_FAILED).with_hint(msg::UNREADABLE_DATA);
        Surfaced::at(error, Some("containers_act"), jiff::Timestamp::UNIX_EPOCH)
    }

    fn ids(toasts: &Toasts) -> Vec<u64> {
        toasts.visible().iter().map(|toast| toast.id).collect()
    }

    #[test]
    fn the_newest_three_show_and_the_others_wait_their_turn() {
        let mut toasts = Toasts::default();
        for n in 0..5 {
            toasts.say(Kind::Info, move || format!("toast {n}"));
        }
        assert_eq!(ids(&toasts), [2, 3, 4]);
        // Those on screen go together; the two that waited then show, with
        // their whole time ahead of them.
        toasts.advance(LIFETIME);
        assert_eq!(ids(&toasts), [0, 1]);
        toasts.advance(LIFETIME - TICK);
        assert_eq!(ids(&toasts), [0, 1]);
        toasts.advance(TICK);
        assert!(toasts.is_empty());
    }

    #[test]
    fn a_failure_stays_longer_than_the_rest() {
        let mut toasts = Toasts::default();
        let failed = toasts.failure(failure());
        toasts.say(Kind::Success, || "done".to_owned());
        toasts.advance(LIFETIME);
        assert_eq!(ids(&toasts), [failed]);
        toasts.advance(ERROR_LIFETIME - LIFETIME - TICK);
        assert_eq!(ids(&toasts), [failed]);
        toasts.update(Message::Tick);
        assert!(toasts.is_empty());
    }

    #[test]
    fn the_pointer_resting_on_them_stops_the_clocks() {
        let mut toasts = Toasts::default();
        let id = toasts.say(Kind::Success, || "done".to_owned());
        toasts.update(Message::Hovered(true));
        toasts.advance(LIFETIME * 3);
        assert_eq!(ids(&toasts), [id]);
        toasts.update(Message::Hovered(false));
        toasts.advance(LIFETIME);
        assert!(toasts.is_empty());
        // Dismissing the last one under the pointer leaves no stopped clock.
        toasts.say(Kind::Info, || "one".to_owned());
        toasts.update(Message::Hovered(true));
        toasts.update(Message::Dismiss(1));
        toasts.say(Kind::Info, || "two".to_owned());
        toasts.advance(LIFETIME);
        assert!(toasts.is_empty());
    }

    #[test]
    fn details_hands_the_failure_over_and_the_toast_goes() {
        let mut toasts = Toasts::default();
        let said = toasts.say(Kind::Info, || "one".to_owned());
        let failed = toasts.failure_of(|| "Could not start web".to_owned(), failure());
        assert_eq!(toasts.update(Message::Details(said)), None);
        assert_eq!(toasts.update(Message::Details(failed)), Some(failure()));
        assert!(toasts.is_empty());
    }

    #[test]
    fn a_failure_says_its_message_and_its_hint_or_what_failed_and_its_message() {
        let _english = crate::test_support::snapshot::in_language("en");
        let mut toasts = Toasts::default();
        toasts.failure(failure());
        toasts.failure_of(|| "Could not start web".to_owned(), failure());
        let error = failure().error;
        let hint = error.hint.as_ref().map(|hint| i18n::errors::hint(hint));
        let message = i18n::errors::message(&error);
        assert_eq!(toasts.visible()[0].text(), (message.clone(), hint));
        assert_eq!(
            toasts.visible()[1].text(),
            ("Could not start web".to_owned(), Some(message))
        );
    }
}
