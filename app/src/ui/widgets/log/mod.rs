//! The log viewer: the lines of one or several sources as they come, in
//! a view that follows the latest, with a program's colours, a search, a
//! selection to copy and the lines to save. What a container, a Pod, a
//! task, a service and a console show of their logs.
//!
//! The page keeps a [`State`] per viewer, feeds it the lines of its
//! stream and hands it the viewer's [`Message`]s:
//!
//! ```ignore
//! // state: log::State, made once: log::State::new(settings.log_lines)
//!
//! // in view:
//! log::Viewer::new(&self.log, Message::Log)
//!     .toolbar(vec![history_select, since_select])
//!     .failed(self.failure.as_ref().map(|f| error_message(f, true, Message::Failure)))
//!     .view()
//!
//! // in update:
//! Message::Lines(Update::Batch { items, dropped }) => {
//!     let batch = shapes::log_lines(items);
//!     self.log.skipped(dropped);
//!     let read = self.log.extend(batch.lines.into_iter().map(log::Line::from));
//!     if batch.ended { self.log.end(); }
//!     self.failure = batch.error.map(|error| Surfaced::new(error, "logs_subscribe"));
//!     read.map(Message::Log)
//! }
//! Message::Log(message) => {
//!     let (task, event) = self.log.update(message);
//!     match event {
//!         Some(log::Event::Copy(text)) => ctx.copy(text),
//!         Some(log::Event::Download(lines)) => { /* the file dialog, then lines.write_to(file) */ }
//!         Some(log::Event::Timestamps(_) | log::Event::Cleared) | None => {}
//!     }
//!     task.map(Message::Log)
//! }
//! ```
//!
//! - **The lines** are kept up to a cap the page gives ([`State::new`]),
//!   the oldest dropped first. Appending looks at the new lines only, and
//!   only the rows in view are ever laid out: a million lines scroll as a
//!   hundred do (`buffer.rs`, `display.rs`).
//! - **Following.** The view stays on the latest line until the user
//!   scrolls up; then it stays where it is, whatever comes and whatever
//!   is dropped, until it is scrolled back to the end, End is pressed, or
//!   the Follow toggle or "Jump to latest" is.
//! - **Colours.** SGR sequences colour the text through the theme's
//!   terminal palette; every other escape sequence is taken out
//!   (`ansi.rs`, `palette.rs`).
//! - **Wrapping.** Lines wrap at the viewer's width as in a terminal, or
//!   scroll sideways ([`State::wrapping`]).
//! - **Searching** as it is typed: the matches are marked, counted, and
//!   stepped through; the lines are read off the window's thread
//!   (`search.rs`). [`Message::Filter`] shows only the lines that match.
//! - **Selecting** with the pointer, as text; Ctrl or Cmd + A and C.
//!   The keys are listed in `pane/input.rs`.

pub mod ansi;
pub mod buffer;
mod display;
mod index;
pub mod line;
pub mod palette;
mod pane;
pub mod search;
pub mod selection;
pub mod state;
mod toolbar;
mod update;

use iced::{
    alignment::Vertical,
    widget::{container, Space, Stack},
    Length, Padding,
};

pub use self::{
    line::{Line, Marker, Stream},
    search::Query,
    selection::{Mark, Selection},
    state::{Anchor, Download, Event, Message, Metrics, Scroll, State, DEFAULT_CAP},
    update::Outcome,
};
use crate::{
    i18n, t,
    ui::{
        theme::{
            metrics::{space, ButtonSize},
            Element, Icon,
        },
        widgets::{
            basic::button::{self, Variant},
            end, stack,
        },
    },
};

/// The room kept under the rows of a paused view: a small button and
/// four pixels over and under it.
const FOOTER: f32 = 36.0;

/// A viewer being described; [`Viewer::view`] makes it an element.
pub struct Viewer<'a, Message> {
    state: &'a State,
    on_event: std::rc::Rc<dyn Fn(self::Message) -> Message + 'a>,
    toolbar: Vec<Element<'a, Message>>,
    failed: Option<Element<'a, Message>>,
    height: Length,
}

impl<'a, Message: Clone + 'a> Viewer<'a, Message> {
    /// A viewer of `state`'s lines, whose messages become the page's
    /// through `on_event`.
    pub fn new(state: &'a State, on_event: impl Fn(self::Message) -> Message + 'a) -> Self {
        Self {
            state,
            on_event: std::rc::Rc::new(on_event),
            toolbar: vec![],
            failed: None,
            height: Length::Fill,
        }
    }

    /// Controls shown before the viewer's own: which container, how far
    /// back, stdout and stderr apart.
    #[must_use]
    pub fn toolbar(mut self, controls: Vec<Element<'a, Message>>) -> Self {
        self.toolbar = controls;
        self
    }

    /// The stream broke or could not start: an `error_message`, shown
    /// over the lines that came before.
    #[must_use]
    pub fn failed(mut self, failure: Option<Element<'a, Message>>) -> Self {
        self.failed = failure;
        self
    }

    /// How tall the lines are; all the height the page gives unless told
    /// (a page that scrolls gives none: give a height there).
    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    pub fn view(self) -> Element<'a, Message> {
        let state = self.state;
        let on = self.on_event;
        let send = on.clone();
        let controls = toolbar::toolbar(state, self.toolbar, move |message| send(message));
        let send = on.clone();
        // Paused away from the latest line, the way back shows: in room
        // of its own under the rows, where it covers no text.
        let paused = !state.follows();
        let away = paused && !state.window(state.metrics).at_end;
        let pane = pane::Pane {
            state,
            on_event: Box::new(move |message| send(message)),
            height: Length::Fill,
            rtl: i18n::direction().is_rtl(),
            footer: if away { FOOTER } else { 0.0 },
        };
        let latest: Element<'a, Message> = if away {
            button::labelled(Icon::ArrowDown, t!("ui.log.latest"))
                .variant(Variant::Secondary)
                .size(ButtonSize::Sm)
                .on_press(on(self::Message::Follow(true)))
                .into()
        } else {
            Space::new().into()
        };
        let latest = container(latest)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(end())
            .align_y(Vertical::Bottom)
            .padding([space::S1 + 1.0, space::S5]);
        let lines = Stack::with_children(vec![pane.into(), latest.into()])
            .width(Length::Fill)
            .height(Length::Fill);
        // The failure has its place whether there is one or not: the
        // rows keep theirs in the tree.
        let gap = |above: bool| Padding {
            top: if above { space::S2 } else { 0.0 },
            ..Padding::ZERO
        };
        let failed = match self.failed {
            Some(failure) => container(failure).padding(gap(true)),
            None => container(Space::new()),
        };
        stack(vec![
            controls,
            failed.into(),
            container(lines).padding(gap(true)).into(),
        ])
        .height(self.height)
        .into()
    }
}

/// How many pieces of text the viewers of this thread have shaped so far.
#[cfg(test)]
pub(crate) fn shaped() -> usize {
    pane::SHAPED.with(std::cell::Cell::get)
}

#[cfg(test)]
mod tests;
