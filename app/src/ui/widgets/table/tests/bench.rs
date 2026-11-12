//! A page with one table on a window that lasts: the same widget tree
//! from one view to the next, as the real window keeps it.

use iced::{
    advanced::mouse,
    keyboard::{self, Key, Modifiers},
    widget::container,
    Event, Length, Point, Rectangle,
};

use super::Page;
use crate::{
    test_support::{
        snapshot::{in_language, InLanguage},
        stage::{Frame, Stage},
    },
    ui::{
        theme,
        widgets::{
            gallery::table::{columns, grouped, samples, source, view, Sample, Show},
            table::{Event as Outcome, Message, Mode, Rows, State, HEADER_HEIGHT},
        },
    },
};

pub(super) const WINDOW: (f32, f32) = (1100.0, 720.0);
pub(super) const NARROW: (f32, f32) = (420.0, 420.0);
/// Where the table starts: the page's padding, then the selection bar's
/// room and the gap under it.
pub(super) const TOP: f32 = 24.0 + 28.0 + 8.0;

/// A page with one table, on a window that lasts.
pub(super) struct Bench {
    pub(super) stage: Stage,
    pub(super) state: State<u32>,
    pub(super) rows: Vec<Sample>,
    pub(super) grouped: bool,
    pub(super) show: Show,
    /// The page's padding on each side of the table.
    pub(super) side: f32,
    _language: InLanguage,
}

impl Bench {
    pub(super) fn new(rows: u32) -> Self {
        Self::in_the_language("en", rows)
    }

    pub(super) fn in_the_language(tag: &str, rows: u32) -> Self {
        let mut bench = Self::fresh(tag, rows, WINDOW);
        // The first view tells the table how much room its rows have.
        bench.frame(|_| {});
        bench
    }

    /// A window that has drawn nothing yet.
    pub(super) fn fresh(tag: &str, rows: u32, window: (f32, f32)) -> Self {
        let language = in_language(tag);
        Self {
            stage: Stage::new(window),
            state: State::new().selecting(Mode::Multi),
            rows: samples(rows),
            grouped: false,
            show: Show::Rows,
            side: 24.0,
            _language: language,
        }
    }

    /// A window too narrow for the columns.
    pub(super) fn narrow(rows: u32) -> Self {
        let mut bench = Self::fresh("en", rows, NARROW);
        bench.frame(|_| {});
        bench
    }

    /// Draws the page, lets `act` use it, and gives the table what that
    /// produced. Returns what the table made of it.
    pub(super) fn frame(
        &mut self,
        act: impl FnOnce(&mut Frame<'_, '_, Page>),
    ) -> Vec<Outcome<u32>> {
        fn read(rows: &[Sample], nested: bool) -> Rows<'_, Sample, u32> {
            if nested {
                grouped(rows)
            } else {
                source(rows)
            }
        }
        let nested = self.grouped;
        let page = container(view(&self.state, read(&self.rows, nested), self.show))
            .padding([24.0, self.side])
            .width(Length::Fill)
            .height(Length::Fill)
            .style(theme::container::page);
        let messages = self.stage.frame(page, act);
        messages
            .into_iter()
            .filter_map(|message| match message {
                Page::Table(message) => {
                    self.state
                        .update(message, &read(&self.rows, nested), &columns())
                }
                _ => None,
            })
            .collect()
    }

    /// A message that comes from elsewhere than the window.
    pub(super) fn tell(&mut self, message: Message<u32>) -> Option<Outcome<u32>> {
        let rows = if self.grouped {
            grouped(&self.rows)
        } else {
            source(&self.rows)
        };
        self.state.update(message, &rows, &columns())
    }

    /// One key, and what the table made of it.
    pub(super) fn key(&mut self, key: impl Into<Key>, modifiers: Modifiers) -> Vec<Outcome<u32>> {
        let key = key.into();
        self.frame(|frame| frame.send(press_of(key, modifiers)))
    }

    pub(super) fn selected(&self) -> Vec<u32> {
        let mut keys: Vec<u32> = self.state.selection().keys().copied().collect();
        keys.sort_unstable();
        keys
    }

    /// The row whose menu is open.
    pub(super) fn menu(&self) -> Option<u32> {
        let derived = self.state.derived.borrow();
        derived.menu.as_ref().map(|(key, ..)| *key)
    }

    /// Tab until the rows have focus.
    pub(super) fn tab_to_rows(&mut self) {
        self.frame(|frame| {
            for _ in 0..12 {
                frame.tab();
                if has_rows_focused(frame) {
                    return;
                }
            }
            panic!("Tab never reached the rows");
        });
    }
}

pub(super) fn press_of(key: Key, modifiers: Modifiers) -> [Event; 2] {
    let pressed = keyboard::Event::KeyPressed {
        key: key.clone(),
        modified_key: key.clone(),
        physical_key: keyboard::key::Physical::Unidentified(
            keyboard::key::NativeCode::Unidentified,
        ),
        location: keyboard::Location::Standard,
        modifiers,
        text: None,
        repeat: false,
    };
    let released = keyboard::Event::KeyReleased {
        key: key.clone(),
        modified_key: key,
        physical_key: keyboard::key::Physical::Unidentified(
            keyboard::key::NativeCode::Unidentified,
        ),
        location: keyboard::Location::Standard,
        modifiers,
    };
    [Event::Keyboard(pressed), Event::Keyboard(released)]
}

/// The rows are the one thing that takes focus and is taller than a row.
pub(super) fn has_rows_focused(frame: &mut Frame<'_, '_, Page>) -> bool {
    frame
        .focused()
        .is_some_and(|bounds| bounds.height > 2.0 * HEADER_HEIGHT)
}

pub(super) fn row_of(frame: &mut Frame<'_, '_, Page>, name: &str) -> Rectangle {
    frame
        .find(name)
        .unwrap_or_else(|| panic!("the row {name} is not in view"))
}

pub(super) const NONE: Modifiers = Modifiers::empty();

/// Presses at `from`, moves to `to` and lets go.
pub(super) fn drag(frame: &mut Frame<'_, '_, Page>, from: Point, to: Point) {
    frame.point_at(from);
    frame.send([Event::Mouse(mouse::Event::ButtonPressed(
        mouse::Button::Left,
    ))]);
    frame.point_at(to);
    frame.send([Event::Mouse(mouse::Event::ButtonReleased(
        mouse::Button::Left,
    ))]);
}

/// Holds or lets go of modifiers, as the window tells of them.
pub(super) fn hold(frame: &mut Frame<'_, '_, Page>, modifiers: Modifiers) {
    frame.send([Event::Keyboard(keyboard::Event::ModifiersChanged(
        modifiers,
    ))]);
}
