//! The terminal: what xterm.js was to every terminal of the React app (a
//! container's, a containerd task's, an Incus Instance's exec and console,
//! a Pod's, a Nomad task's). The emulation is `alacritty_terminal`'s: its
//! grid, scrollback, parser, selection and search. The rest is the kit's:
//! the drawing, the keys, the pointer, the search bar, the theme.
//!
//! The page keeps a [`State`] per terminal, feeds it what the session
//! printed and hands it the terminal's [`Message`]s; it sends the session
//! what comes out as [`Event`]s. `data::streams::session` is the session
//! side, the same for the five backends:
//!
//! ```ignore
//! // state: terminal::State, made once: terminal::State::new()
//!
//! // in view:
//! terminal::Terminal::new(&self.terminal, Message::Terminal)
//!     .failed(self.failure.as_ref().map(|f| error_message(f, true, Message::Failure)))
//!     .view()
//!
//! // in update, what the session printed (a batch a frame at most):
//! Message::Session(Update::Batch { items, .. }) => {
//!     let output = session::output(items);
//!     let events = self.terminal.feed(&output.text);
//!     if let Some(exit) = output.exit { self.terminal.notice(&exit_text(exit)) }
//!     return self.told(events, ctx);
//! }
//! // and what the terminal says, in order:
//! Message::Terminal(message) => {
//!     let (task, events) = self.terminal.update(message);
//!     return Task::batch([task.map(Message::Terminal), self.told(events, ctx)]);
//! }
//!
//! // what the session's writer ended with: a failure is shown, once.
//! Message::Written(result) => {
//!     self.session.written(result);
//!     self.failure = self.session.failure().map(|e| Surfaced::new(e.clone(), "exec_input"));
//! }
//!
//! fn told(&mut self, events: Vec<terminal::Event>, ctx: &Context<'_>) -> Task<Message> {
//!     for event in events {
//!         match event {
//!             // Let go of quietly once the session ended.
//!             terminal::Event::Input(bytes) => self.session.input(bytes),
//!             terminal::Event::Resize { cols, rows } => self.session.resize(cols, rows),
//!             terminal::Event::Copy(text) => ctx.copy(text),
//!             terminal::Event::Open(url) => ctx.open_url(url),
//!             terminal::Event::Title(_) | terminal::Event::Bell => {}
//!         }
//!     }
//!     Task::none()
//! }
//! ```
//!
//! - **Output** is read a batch at a time, and a batch comes at most once
//!   a frame (`data::streams`), so a program that prints 50 MB is read in
//!   sixty slices a second and drawn once a frame. A batch of more than a
//!   megabyte (a window that fell behind) is read a megabyte a frame
//!   ([`state::READ_AT_ONCE`]), so `update` never stalls on it, and no
//!   more than [`state::UNREAD_MAX`] waits: the oldest is dropped past it.
//! - **The grid** is drawn row by row; a row that did not change is not
//!   looked at again, and text is shaped once for as long as it shows
//!   (`pane/rows.rs`). Box drawing and blocks are drawn as shapes.
//! - **Fit to its container**: it takes the room it is given, makes as
//!   many cells of it as fit and tells the page the new size
//!   ([`Event::Resize`]).
//! - **Keys** are encoded as xterm does; the terminal's own keys and those
//!   it leaves to the app are listed in `keys.rs`. The pointer selects,
//!   or is reported to a program that asked (`mouse.rs`).
//! - **Searching** is the log viewer's search bar, over the scrollback and
//!   the screen (`search.rs`, `toolbar.rs`).

mod backlog;
mod colors;
mod emulator;
pub mod keys;
mod message;
pub mod mouse;
mod pane;
pub mod search;
pub mod state;
mod toolbar;

use std::rc::Rc;

use iced::{
    widget::{container, Space},
    Length, Padding,
};

pub use self::{
    emulator::{MAX_CELLS, MIN_CELLS},
    pane::row_height,
    state::{
        Click, Due, Event, Message, Scroll, State, Viewport, DEFAULT_FONT_SIZE, DEFAULT_SCROLLBACK,
        MAX_FONT_SIZE, MIN_FONT_SIZE,
    },
};
use crate::{
    i18n,
    ui::{
        theme::{metrics::space, Element},
        widgets::{basic::shortcut::is_mac, stack},
    },
};

/// A terminal being described; [`Terminal::view`] makes it an element.
pub struct Terminal<'a, Message> {
    state: &'a State,
    on_event: Rc<dyn Fn(self::Message) -> Message + 'a>,
    toolbar: Vec<Element<'a, Message>>,
    failed: Option<Element<'a, Message>>,
    height: Length,
}

impl<'a, Message: Clone + 'a> Terminal<'a, Message> {
    /// A terminal of `state`, whose messages become the page's through
    /// `on_event`.
    pub fn new(state: &'a State, on_event: impl Fn(self::Message) -> Message + 'a) -> Self {
        Self {
            state,
            on_event: Rc::new(on_event),
            toolbar: vec![],
            failed: None,
            height: Length::Fill,
        }
    }

    /// Controls shown over the grid, before the search when it is open.
    #[must_use]
    pub fn toolbar(mut self, controls: Vec<Element<'a, Message>>) -> Self {
        self.toolbar = controls;
        self
    }

    /// The session broke or could not start: an `error_message`, shown
    /// over the grid.
    #[must_use]
    pub fn failed(mut self, failure: Option<Element<'a, Message>>) -> Self {
        self.failed = failure;
        self
    }

    /// How tall it is; all the height the page gives unless told.
    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    pub fn view(self) -> Element<'a, Message> {
        let state = self.state;
        let on = self.on_event;
        let send = on.clone();
        // The bar and the failure have their places whether they show or
        // not: the grid keeps its place in the tree, and its focus.
        let gap = |shown: bool| Padding {
            bottom: if shown { space::S2 } else { 0.0 },
            ..Padding::ZERO
        };
        let shown = state.search().open || !self.toolbar.is_empty();
        let bar: Element<'a, Message> = if state.search().open {
            toolbar::toolbar(state, self.toolbar, move |message| send(message))
        } else if self.toolbar.is_empty() {
            Space::new().into()
        } else {
            crate::ui::widgets::basic::flow(self.toolbar)
                .spacing(space::S2)
                .into()
        };
        let bar = container(bar).padding(gap(shown));
        let failed = match self.failed {
            Some(failure) => container(failure).padding(gap(true)),
            None => container(Space::new()),
        };
        let send = on.clone();
        let grid = pane::Pane {
            state,
            on_event: Box::new(move |message| send(message)),
            rtl: i18n::direction().is_rtl(),
            mac: is_mac(),
        };
        stack(vec![bar.into(), failed.into(), grid.into()])
            .height(self.height)
            .into()
    }
}

#[cfg(test)]
mod tests;
