//! What a table makes of what the user did. The keyboard and the ranges
//! go through the rows in the order they show, so `update` is given the
//! same rows and columns as the view and works that order out again when
//! the rows changed since.

use std::hash::Hash;

use iced::keyboard::Modifiers;

use super::{
    column::Column,
    derived::{comparison, Entry, Place, Rows, Target},
    selection::{self, Mode},
    state::{Event, Message, Motion, State},
    window,
};
use crate::ui::widgets::overlay::menu;

impl<K: Hash + Eq + Clone> State<K> {
    /// Works out what shows again if the rows, the sort or the marks
    /// changed since the last view: a key that was queued behind a list
    /// read again goes through the list as it is now.
    fn refresh<T, M>(&self, rows: &Rows<'_, T, K>, columns: &[Column<'_, T, M>]) {
        let compare = comparison(&self.sort, columns);
        self.rederive_if_stale(
            rows,
            compare
                .as_ref()
                .map(|compare| compare as &super::derived::Compare<'_, T>),
        );
    }

    fn scroll_to(&mut self, offset: f32) {
        let count = self.derived.get_mut().len();
        self.offset
            .set(window::clamp(offset, count, self.row_height, self.viewport));
        // Scrolled on purpose: the next view says which line is on top.
        self.derived.get_mut().top = None;
    }

    fn locate<T>(&self, place: &Place<K>, rows: &Rows<'_, T, K>) -> Option<usize> {
        self.find(&self.derived.borrow(), rows, place)
    }

    fn target_at<T>(&self, index: usize, rows: &Rows<'_, T, K>) -> Option<Target<K>> {
        Self::target(&self.derived.borrow(), rows, index)
    }

    fn key_at<T>(&self, index: usize, rows: &Rows<'_, T, K>) -> Option<K> {
        match self.target_at(index, rows)? {
            Target::Row(key) => Some(key),
            Target::Group(_) => None,
        }
    }

    fn changed(&mut self) -> Option<Event<K>> {
        self.touch();
        Some(Event::Selected)
    }

    /// Selects from the anchor to the line `to`, both included.
    fn select_range<T>(&mut self, to: usize, rows: &Rows<'_, T, K>) -> Option<Event<K>> {
        let anchor = self.derived.get_mut().anchor.clone();
        let from = anchor
            .and_then(|(key, at)| self.locate(&(Target::Row(key), at), rows))
            .unwrap_or(to);
        if self.derived.get_mut().anchor.is_none() {
            let start = self.key_at(to, rows).map(|key| (key, to));
            self.derived.get_mut().anchor = start;
        }
        let keys: Vec<K> = selection::range(from, to)
            .filter_map(|index| self.key_at(index, rows))
            .collect();
        self.derived.get_mut().selection.replace(keys);
        self.changed()
    }

    fn set_collapsed(&mut self, name: String, collapsed: bool) {
        if collapsed {
            self.collapsed.insert(name);
        } else {
            self.collapsed.remove(&name);
        }
        self.touch();
    }

    fn toggle_group(&mut self, name: String) {
        let collapsed = !self.collapsed.contains(&name);
        self.set_collapsed(name, collapsed);
    }

    /// Puts the keyboard on the line `index` and brings it into view.
    fn focus<T>(&mut self, index: usize, rows: &Rows<'_, T, K>) -> Option<Target<K>> {
        let target = self.target_at(index, rows)?;
        self.derived.get_mut().focused = Some((target.clone(), index));
        let offset = window::reveal(index, self.offset.get(), self.viewport, self.row_height);
        self.scroll_to(offset);
        Some(target)
    }

    fn moved<T>(&mut self, to: Motion, extend: bool, rows: &Rows<'_, T, K>) -> Option<Event<K>> {
        let last = self.derived.get_mut().len().checked_sub(1)?;
        let was = self.derived.get_mut().focused.clone();
        let from = was.as_ref().and_then(|place| self.locate(place, rows));
        let page = window::page(self.viewport, self.row_height);
        let index = match (to, from) {
            (Motion::Home, _) | (Motion::Down | Motion::PageDown, None) => 0,
            (Motion::End, _) | (Motion::Up | Motion::PageUp, None) => last,
            (Motion::Up, Some(at)) => at.saturating_sub(1),
            (Motion::Down, Some(at)) => (at + 1).min(last),
            (Motion::PageUp, Some(at)) => at.saturating_sub(page),
            (Motion::PageDown, Some(at)) => (at + page).min(last),
        };
        let reached = self.focus(index, rows)?;
        let extends = extend && self.mode == Mode::Multi;
        let derived = self.derived.get_mut();
        if extends && derived.anchor.is_none() {
            // The range starts where the keyboard was.
            derived.anchor = match (was, from, &reached) {
                (Some((Target::Row(key), _)), Some(at), _) => Some((key, at)),
                (_, _, Target::Row(key)) => Some((key.clone(), index)),
                _ => None,
            };
        }
        if extends {
            return self.select_range(index, rows);
        }
        derived.anchor = match reached {
            Target::Row(key) => Some((key, index)),
            Target::Group(_) => None,
        };
        None
    }

    fn pressed<T>(
        &mut self,
        key: K,
        index: usize,
        modifiers: Modifiers,
        rows: &Rows<'_, T, K>,
    ) -> Option<Event<K>> {
        let mode = self.mode;
        let derived = self.derived.get_mut();
        derived.focused = Some((Target::Row(key.clone()), index));
        match mode {
            Mode::Multi if modifiers.shift() => self.select_range(index, rows),
            Mode::Multi | Mode::Single if modifiers.command() => {
                let selected = derived.selection.contains(&key);
                if mode == Mode::Single {
                    derived.selection.clear();
                }
                derived.selection.set(key.clone(), !selected);
                derived.anchor = Some((key, index));
                self.changed()
            }
            _ if self.opens_on_press => Some(Event::Opened(key)),
            Mode::None => None,
            Mode::Single | Mode::Multi => {
                derived.selection.only(key.clone());
                derived.anchor = Some((key, index));
                self.changed()
            }
        }
    }

    /// Goes to the next row whose text starts with what was typed.
    fn typed<T, M>(&mut self, typed: &str, rows: &Rows<'_, T, K>, columns: &[Column<'_, T, M>]) {
        let Some(text) = columns.iter().find_map(|column| column.typed.as_ref()) else {
            return;
        };
        let typed = typed.to_lowercase();
        let count = self.derived.get_mut().len();
        let was = self.derived.get_mut().focused.clone();
        let from = was.and_then(|place| self.locate(&place, rows));
        // A first letter goes on from the row after; more letters narrow
        // down from the row they are on.
        let start = from.map_or(0, |at| at + usize::from(typed.chars().count() == 1));
        let found = (0..count)
            .map(|step| (start + step) % count)
            .find(|&index| {
                let Some(Entry::Row(row)) = self.derived.borrow().entry(index) else {
                    return false;
                };
                text(&rows.rows[row as usize])
                    .to_lowercase()
                    .starts_with(&typed)
            });
        if let Some(index) = found {
            if let Some(Target::Row(key)) = self.focus(index, rows) {
                self.derived.get_mut().anchor = Some((key, index));
            }
        }
    }

    /// The keys of the rows of the group `name`, collapsed or not.
    fn keys_of<T>(&self, name: &str, rows: &Rows<'_, T, K>) -> Vec<K> {
        let derived = self.derived.borrow();
        derived
            .groups
            .iter()
            .filter(|group| group.name == name)
            .flat_map(|group| &group.rows)
            .map(|&row| (rows.key)(&rows.rows[row as usize]))
            .collect()
    }

    /// Takes in what the user did. `rows` and `columns` are the ones the
    /// view was given: the keyboard goes through the rows that show, in
    /// the order the columns sort them.
    #[allow(clippy::too_many_lines)]
    pub fn update<T, M>(
        &mut self,
        message: Message<K>,
        rows: &Rows<'_, T, K>,
        columns: &[Column<'_, T, M>],
    ) -> Option<Event<K>> {
        match message {
            Message::Scrolled(offset) => {
                self.scroll_to(offset);
                None
            }
            Message::ScrolledAcross(offset) => {
                self.across = offset.max(0.0);
                None
            }
            Message::Viewport(height) => {
                self.viewport = height.max(0.0);
                let offset = self.offset.get();
                self.scroll_to(offset);
                None
            }
            Message::Sort {
                column,
                descending_first,
                additive,
            } => {
                self.sort
                    .toggle(column, descending_first, additive, self.always_sorted);
                // Another order: the view stays where it is scrolled.
                self.derived.get_mut().top = None;
                Some(Event::Arranged)
            }
            Message::Resize { column, width } => {
                match self.widths.iter_mut().find(|(id, _)| *id == column) {
                    Some(kept) => kept.1 = width,
                    None => self.widths.push((column, width)),
                }
                None
            }
            Message::Resized => Some(Event::Arranged),
            Message::ResetWidth(column) => {
                self.widths.retain(|(id, _)| *id != column);
                Some(Event::Arranged)
            }
            Message::Hide { column, hidden } => {
                self.set_hidden(column, hidden);
                Some(Event::Arranged)
            }
            Message::Pressed {
                key,
                index,
                modifiers,
            } => {
                self.refresh(rows, columns);
                // The row may have moved since it was drawn.
                let index = self.locate(&(Target::Row(key.clone()), index), rows)?;
                self.pressed(key, index, modifiers, rows)
            }
            Message::Opened(opened) => (!self.opens_on_press).then_some(Event::Opened(opened)),
            Message::Toggle { key, index } => {
                let mode = self.mode;
                let derived = self.derived.get_mut();
                if mode == Mode::Single && !derived.selection.contains(&key) {
                    derived.selection.clear();
                }
                derived.selection.toggle(key.clone());
                derived.anchor = Some((key, index));
                self.changed()
            }
            Message::ToggleAll(on) => {
                self.refresh(rows, columns);
                // The rows that show, and no other: what the filter hides
                // stays as it was.
                let shown: Vec<K> = {
                    let derived = self.derived.borrow();
                    let key = |&row: &u32| (rows.key)(&rows.rows[row as usize]);
                    derived.order.iter().map(key).collect()
                };
                let selection = &mut self.derived.get_mut().selection;
                if on && self.mode == Mode::Multi {
                    selection.extend(shown);
                } else {
                    selection.remove(shown);
                }
                self.changed()
            }
            Message::GroupPressed { name, index } => {
                self.derived.get_mut().focused = Some((Target::Group(name.clone()), index));
                self.toggle_group(name);
                None
            }
            Message::Collapse { name, collapsed } => {
                self.set_collapsed(name, collapsed);
                None
            }
            Message::SelectGroup { name, selected } => {
                self.refresh(rows, columns);
                let keys = self.keys_of(&name, rows);
                let selection = &mut self.derived.get_mut().selection;
                if selected {
                    selection.extend(keys);
                } else {
                    selection.remove(keys);
                }
                self.changed()
            }
            Message::Move { to, extend } => {
                self.refresh(rows, columns);
                self.moved(to, extend, rows)
            }
            Message::ToggleFocused | Message::OpenFocused => {
                self.refresh(rows, columns);
                let toggles = matches!(message, Message::ToggleFocused);
                let focused = self.derived.get_mut().focused.clone()?;
                let index = self.locate(&focused, rows)?;
                match focused.0 {
                    Target::Group(name) => {
                        self.toggle_group(name);
                        None
                    }
                    Target::Row(key) if toggles => {
                        if self.mode == Mode::None {
                            return None;
                        }
                        self.update(Message::Toggle { key, index }, rows, columns)
                    }
                    Target::Row(key) => Some(Event::Opened(key)),
                }
            }
            Message::SelectAll => {
                if self.mode != Mode::Multi {
                    return None;
                }
                self.update(Message::ToggleAll(true), rows, columns)
            }
            Message::Clear => {
                let selection = &mut self.derived.get_mut().selection;
                if selection.is_empty() {
                    return None;
                }
                selection.clear();
                self.changed()
            }
            Message::Typed(typed) => {
                self.refresh(rows, columns);
                self.typed(&typed, rows, columns);
                None
            }
            Message::MenuFor(place) => {
                self.refresh(rows, columns);
                let place = match place {
                    Some((key, index)) => (Target::Row(key), index),
                    None => self.derived.get_mut().focused.clone()?,
                };
                let index = self.locate(&place, rows)?;
                // The menu hangs from its row: the row comes into view.
                let Target::Row(key) = self.focus(index, rows)? else {
                    return None;
                };
                let mut menu = menu::State::default();
                menu.update(menu::Event::Toggle);
                self.derived.get_mut().menu = Some((key, index, menu));
                None
            }
            Message::Menu(of, event) => {
                let derived = self.derived.get_mut();
                let (at, mut menu) = match derived.menu.take() {
                    Some((open, at, menu)) if open == of => (at, menu),
                    _ => (0, menu::State::default()),
                };
                menu.update(event);
                if menu.is_open() {
                    derived.menu = Some((of, at, menu));
                }
                None
            }
        }
    }
}
