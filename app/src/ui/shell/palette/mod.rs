//! The command palette, opened with Mod+K from any page
//! (`src/app/palette`): go to a page, switch Host, run a command. Its rows
//! and their ranking are [`items`]; this is what is open, typed and
//! highlighted.

pub mod items;
#[cfg(test)]
mod tests;
pub mod view;

use iced::{
    widget::{operation, scrollable, Id},
    Task,
};

use self::items::{sections, Act, Item};

/// The height of a row, of a heading and of the list (`max-h-72`).
pub const ROW: f32 = 32.0;
pub const HEADING: f32 = 24.0;
pub const LIST_HEIGHT: f32 = 288.0;

/// The palette: closed, or open on what is typed.
#[derive(Debug, Clone)]
pub struct State {
    open: bool,
    query: String,
    /// The ids of the items run in this session, most recent first.
    recent: Vec<String>,
    /// The row the keyboard is on, among those listed.
    active: usize,
    list: Id,
}

impl Default for State {
    fn default() -> Self {
        Self {
            open: false,
            query: String::new(),
            recent: vec![],
            active: 0,
            list: Id::unique(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    /// Mod+K: opens it, or closes it.
    Toggle,
    Close,
    Query(String),
    /// The highlight moves by this many rows.
    Step(isize),
    /// Enter: the highlighted row.
    Submit,
    /// This row, among those listed, was pressed.
    Choose(usize),
}

impl State {
    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn active(&self) -> usize {
        self.active
    }

    pub fn recent(&self) -> &[String] {
        &self.recent
    }

    fn close(&mut self) {
        self.open = false;
        self.query.clear();
        self.active = 0;
    }

    /// The rows listed for what is typed, in the order they show.
    pub fn listed<'a>(&self, items: &'a [Item]) -> Vec<&'a Item> {
        sections(items, &self.query, &self.recent)
            .into_iter()
            .flat_map(|section| section.items)
            .collect()
    }

    /// Applies `message` given the palette's `items`, and returns what the
    /// row it chose does, if it chose one: the palette is then closed and
    /// the row remembered.
    pub fn update(&mut self, message: Message, items: &[Item]) -> Option<Act> {
        let listed = self.listed(items);
        let chosen = match message {
            Message::Toggle if self.open => {
                self.close();
                return None;
            }
            Message::Toggle => {
                self.open = true;
                return None;
            }
            Message::Close => {
                self.close();
                return None;
            }
            Message::Query(query) => {
                self.query = query;
                self.active = 0;
                return None;
            }
            Message::Step(by) => {
                // It stops at the ends, as the React one.
                let last = listed.len().saturating_sub(1);
                self.active = self.active.saturating_add_signed(by).min(last);
                return None;
            }
            Message::Submit => listed.get(self.active),
            Message::Choose(index) => listed.get(index),
        }?;
        let act = chosen.act.clone();
        self.recent = items::remember(&self.recent, &chosen.id);
        self.close();
        Some(act)
    }

    /// Scrolls the list so the highlighted row shows: run it after a
    /// [`Message::Step`].
    pub fn reveal<T: Send + 'static>(&self, items: &[Item]) -> Task<T> {
        let mut top = 0.0;
        let mut at = 0;
        'sections: for section in sections(items, &self.query, &self.recent) {
            top += HEADING;
            for _ in section.items {
                if at == self.active {
                    break 'sections;
                }
                top += ROW;
                at += 1;
            }
        }
        let y = (top - (LIST_HEIGHT - ROW) / 2.0).max(0.0);
        operation::scroll_to(
            self.list.clone(),
            scrollable::AbsoluteOffset {
                x: None,
                y: Some(y),
            },
        )
    }
}
