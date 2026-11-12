//! The table's tests: what it draws (`drawn`), what its state does with
//! the messages it is sent (`state`), what it does under the keyboard and
//! the pointer on a window that lasts (`keyboard`, `pointer`), how much
//! work a view is (`counts`) and how long it takes (`perf`).

mod bench;
mod columns;
mod counts;
mod drawn;
mod keyboard;
mod perf;
mod pointer;
mod state;

use iced::{keyboard::Modifiers, widget::container, Length};

use super::{derived::Entry, state::Motion, Event, Message, Mode, Order, Rows, Sort, State, Table};
use crate::ui::{
    theme::{self, Element, Theme},
    widgets::gallery::{
        self,
        table::{columns, samples, source, view, Sample, Show},
    },
};

type Page = gallery::Message;

const SIZE: (f32, f32) = (860.0, 420.0);

fn on_page(part: Element<'_, Page>) -> Element<'_, Page> {
    container(part)
        .padding(24)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(theme::container::page)
        .into()
}

fn multi() -> State<u32> {
    State::new().selecting(Mode::Multi)
}

/// Sends `message` as a page does: with the rows and the columns.
fn tell(state: &mut State<u32>, message: Message<u32>, rows: &[Sample]) -> Option<Event<u32>> {
    state.update(message, &source(rows), &columns())
}

/// Sorted by memory, the largest first, with three rows selected.
fn arranged(rows: &[Sample]) -> State<u32> {
    let mut state = multi().sorted(Sort::by("memory", Order::Descending));
    for id in [3, 4, 9] {
        tell(&mut state, Message::Toggle { key: id, index: 0 }, rows);
    }
    state
}

fn press(index: usize, key: u32, modifiers: Modifiers) -> Message<u32> {
    Message::Pressed {
        key,
        index,
        modifiers,
    }
}

fn go(to: Motion) -> Message<u32> {
    Message::Move { to, extend: false }
}

fn selected(state: &State<u32>) -> Vec<u32> {
    let mut keys: Vec<u32> = state.selection().keys().copied().collect();
    keys.sort_unstable();
    keys
}

/// Draws the table once so its state knows the rows, as a page does
/// before any message can come.
fn seen(state: &State<u32>, rows: &[Sample]) {
    drop(view(state, source(rows), Show::Rows));
}

fn seen_as(state: &State<u32>, rows: Rows<'_, Sample, u32>) {
    drop(Table::new(state, columns(), rows, Page::Table).view());
}

/// The lines of the body, as they would be read: a row by its name, the
/// header of a group in brackets.
fn lines(state: &State<u32>, rows: &[Sample]) -> Vec<String> {
    let derived = state.derived.borrow();
    (0..derived.len())
        .map(|index| match derived.entry(index) {
            Some(Entry::Row(row)) => rows[row as usize].name.clone(),
            Some(Entry::Group(group)) => format!("[{}]", derived.groups[group as usize].name),
            None => unreachable!(),
        })
        .collect()
}

fn menu_of(state: &State<u32>) -> Option<u32> {
    state.derived.borrow().menu.as_ref().map(|(key, ..)| *key)
}
