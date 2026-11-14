//! Toasts: what happened, said in a corner of the window for a few
//! seconds (the React app's Sonner). The newest is nearest the corner, three
//! show at once, each goes by itself, and resting the pointer on them stops
//! the clocks. A failure's toast stays longer and has a Details button.
//!
//! The window keeps one [`Toasts`], adds to it, gives it the messages of
//! [`Toasts::view`] and runs [`Toasts::subscription`]. It lays them over
//! everything, a dialog included (`toast::over(overlay::over(page, dialog),
//! toasts)`): what a dialog's action failed with is said while the dialog
//! is still up, and Sonner's toasts were above Radix's dialogs too. The
//! dialog keeps Tab to itself, so those toasts are for the pointer. A toast keeps what
//! it was told, not its text: it is said again in the language of the
//! moment every time it is drawn.
//!
//! A toast may offer one action (Sonner's `action: { label, onClick }`):
//! a button after its words that hands back what it was given
//! ([`Toasts::say_acting`], [`Pressed::Action`]) and dismisses the toast,
//! as Sonner's did. It stays as long as any toast; the pointer resting on
//! it stops the clock, and Tab reaches its button as any other.

use std::{
    any::Any,
    fmt,
    sync::Arc,
    time::{Duration, Instant},
};

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
/// How often the clocks are read. A toast's time is what passed between
/// two readings, not their number: a window that was busy or asleep loses
/// no tick.
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

    /// What it says in the language of the moment.
    pub fn now(&self) -> String {
        (self.0)()
    }
}

impl fmt::Debug for Said {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Said({:?})", self.now())
    }
}

/// What a toast's action hands back when pressed: whatever the window
/// gave it (a page to go to), which only the window knows the type of.
#[derive(Clone)]
pub struct Then(Arc<dyn Any + Send + Sync>);

impl Then {
    pub fn new<T: Send + Sync + 'static>(then: T) -> Self {
        Self(Arc::new(then))
    }

    /// What it was given, if it was a `T`.
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.0.downcast_ref()
    }
}

impl fmt::Debug for Then {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Then(..)")
    }
}

/// What a press on a toast asks of the window.
#[derive(Debug, Clone)]
pub enum Pressed {
    /// The Details of a failure.
    Details(Surfaced),
    /// The toast's action.
    Action(Then),
}

#[derive(Debug, Clone)]
enum Body {
    Said {
        title: Said,
        description: Option<Said>,
        /// The button's label and what it hands back.
        action: Option<(Said, Then)>,
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
            Body::Said {
                title, description, ..
            } => (title.now(), description.as_ref().map(Said::now)),
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
    /// The pointer came over this toast.
    Entered(u64),
    /// The pointer left this toast.
    Left(u64),
    /// The clock, read at this instant.
    Tick(Instant),
    /// The Details button of a failure's toast.
    Details(u64),
    /// The action's button of this toast.
    Act(u64),
}

/// The toasts on screen and those waiting, oldest first.
#[derive(Debug, Default)]
pub struct Toasts {
    list: Vec<Toast>,
    next: u64,
    /// The toast the pointer rests on, which stops every clock.
    hovered: Option<u64>,
    /// When the clocks were last read.
    read: Option<Instant>,
}

impl Toasts {
    fn add(&mut self, kind: Kind, body: Body, left: Duration) -> u64 {
        let id = self.next;
        self.next += 1;
        if self.list.is_empty() {
            // The clock starts with the first toast.
            self.read = None;
        }
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
            action: None,
        };
        self.add(kind, body, LIFETIME)
    }

    /// A toast of `kind` that says `title`, with a button `label` whose
    /// press hands `then` back ([`Pressed::Action`]) and dismisses it.
    pub fn say_acting<T: Send + Sync + 'static>(
        &mut self,
        kind: Kind,
        title: impl Fn() -> String + Send + Sync + 'static,
        label: impl Fn() -> String + Send + Sync + 'static,
        then: T,
    ) -> u64 {
        self.say_with(
            kind,
            Said::new(title),
            Said::new(label),
            Then(Arc::new(then)),
        )
    }

    /// The same, from what a page said and what it hands back.
    pub fn say_with(&mut self, kind: Kind, title: Said, label: Said, then: Then) -> u64 {
        let body = Body::Said {
            title,
            description: None,
            action: Some((label, then)),
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
            action: None,
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
        if self.hovered.is_some() {
            return;
        }
        let first = self.list.len().saturating_sub(VISIBLE);
        for toast in &mut self.list[first..] {
            toast.left = toast.left.saturating_sub(by);
        }
        self.list.retain(|toast| !toast.left.is_zero());
    }

    /// Returns what a press asked for: the failure whose Details the
    /// window opens, or what the toast's action hands back. The toast
    /// goes either way, as its job is done.
    pub fn update(&mut self, message: Message) -> Option<Pressed> {
        match message {
            Message::Tick(now) => {
                let passed = self
                    .read
                    .map_or(Duration::ZERO, |read| now.saturating_duration_since(read));
                self.read = Some(now);
                self.advance(passed);
            }
            Message::Entered(id) => self.hovered = Some(id),
            Message::Left(id) => {
                // Leaving one for its neighbour: the neighbour's enter may
                // have come first.
                if self.hovered == Some(id) {
                    self.hovered = None;
                }
            }
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
                return surfaced.map(Pressed::Details);
            }
            Message::Act(id) => {
                let then =
                    self.list
                        .iter()
                        .find(|t| t.id == id)
                        .and_then(|toast| match &toast.body {
                            Body::Said {
                                action: Some((_, then)),
                                ..
                            } => Some(then.clone()),
                            _ => None,
                        });
                self.remove(id);
                return then.map(Pressed::Action);
            }
        }
        None
    }

    fn remove(&mut self, id: u64) {
        self.list.retain(|toast| toast.id != id);
        if self.hovered == Some(id) {
            // It is gone from under the pointer, which will never leave
            // it: the others go on.
            self.hovered = None;
        }
    }

    /// The clock, which only runs while there is a toast.
    pub fn subscription(&self) -> Subscription<Message> {
        if self.list.is_empty() {
            Subscription::none()
        } else {
            iced::time::every(TICK).map(Message::Tick)
        }
    }

    /// The toasts, to lay over the page with [`over`]; `None` without any.
    pub fn view(&self) -> Option<Element<'_, Message>> {
        if self.list.is_empty() {
            return None;
        }
        let toasts = self.visible().iter().map(|shown| {
            mouse_area(toast(shown))
                .on_enter(Message::Entered(shown.id))
                .on_exit(Message::Left(shown.id))
                .into()
        });
        Some(
            Column::with_children(toasts)
                .spacing(space::S2)
                .width(WIDTH)
                .into(),
        )
    }
}

/// `page` with `toasts` in its bottom corner, on the side text ends.
pub fn over<'a, Message: 'a>(
    page: impl Into<Element<'a, Message>>,
    toasts: Option<Element<'a, Message>>,
) -> Element<'a, Message> {
    // The same stack with or without toasts, so the page keeps its state
    // (what is scrolled, what has focus) when the first one shows.
    let corner = toasts.map(|toasts| {
        container(toasts)
            .padding(space::S4)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(end())
            .align_y(Vertical::Bottom)
            .into()
    });
    iced::widget::Stack::with_children(std::iter::once(page.into()).chain(corner))
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
    let mut words: Vec<Element<'_, Message>> = vec![paragraph(title).font(font::medium()).into()];
    words.extend(description.map(|description| paragraph(description).into()));
    let mut parts: Vec<Element<'_, Message>> = vec![
        container(icon(toast.kind.glyph()).style(tinted(colour)))
            .padding([space::S0_5, 0.0])
            .into(),
        stack(words).spacing(space::S0_5).into(),
    ];
    if let Body::Said {
        action: Some((label, _)),
        ..
    } = &toast.body
    {
        parts.push(
            button::text(label.now())
                .size(ButtonSize::Xs)
                .on_press(Message::Act(toast.id))
                .into(),
        );
    }
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
    container(
        line(parts)
            .spacing(space::S2)
            .width(Length::Fill)
            .align_y(Vertical::Top),
    )
    .padding(space::S3)
    .width(Length::Fill)
    .style(theme::container::toast(colour))
    .into()
}

#[cfg(test)]
mod tests;
