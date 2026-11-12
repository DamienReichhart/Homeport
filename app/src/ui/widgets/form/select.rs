//! `Select` and the combobox: a button that shows the chosen option and
//! opens the list of them, which a search field narrows.
//!
//! The keyboard, as Radix's and cmdk's:
//! - on the button: Enter, Space and the Down and Up arrows open the list;
//! - in a plain list: the arrows go through the options (round at the
//!   ends), Home and End to the first and last, typing to the option that
//!   starts with it, Enter or Space choose;
//! - in a searchable list the search field keeps focus: typing narrows,
//!   the arrows move the highlighted option, Enter chooses it;
//! - Escape closes and gives focus back to the button.
//!
//! The page keeps a [`State`] and gives it the [`Message`]s; when one of
//! them chooses, [`State::update`] returns the option's index.
//!
//! ```ignore
//! select(&self.engine, &labels, self.engine_index)
//!     .placeholder(t!("hosts.engine"))
//!     .searchable(true)
//!     .view()
//!     .map(Message::Engine)
//! ```

use iced::{
    advanced::widget::Id,
    keyboard::{key::Named, Key},
    widget::{container, operation, rule, scrollable, text},
    Length, Task,
};

use super::input::{input, input_group, Addon};
use crate::{
    i18n, t,
    ui::{
        theme::{
            self, icon,
            metrics::{control, space, text as size},
            Element, Icon,
        },
        widgets::{
            basic::fit,
            filling,
            focus::{key_area, pressable, roving, Axis},
            line,
            overlay::{anchored, Align, Placement, Side},
        },
    },
};

/// `max-h-72`: the list scrolls past this.
const LIST_HEIGHT: f32 = 288.0;
/// An option's height: `py-1` around a line of `text-sm`.
const ROW: f32 = 28.0;
/// `min-w-36`: the list is never narrower.
const MIN_WIDTH: f32 = 144.0;

/// An open or closed select: what is searched, and the option the
/// keyboard is on among those that match.
#[derive(Debug, Clone, PartialEq)]
pub struct State {
    open: bool,
    query: String,
    active: usize,
    list: Id,
}

impl Default for State {
    fn default() -> Self {
        Self {
            open: false,
            query: String::new(),
            active: 0,
            list: Id::unique(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    /// The button was pressed.
    Toggle,
    /// An arrow on the closed button.
    Open,
    Close,
    Query(String),
    /// The highlight moves by this many options.
    Step(isize),
    /// Enter in the search field: the highlighted option.
    Submit,
    /// This option (its index among all of them) was pressed.
    Choose(usize),
}

/// The options whose label contains `query`, case aside, by index.
pub fn matching(labels: &[String], query: &str) -> Vec<usize> {
    let wanted = query.trim().to_lowercase();
    labels
        .iter()
        .enumerate()
        .filter(|(_, label)| wanted.is_empty() || label.to_lowercase().contains(&wanted))
        .map(|(index, _)| index)
        .collect()
}

impl State {
    pub fn is_open(&self) -> bool {
        self.open
    }

    fn close(&mut self) {
        self.open = false;
        self.query.clear();
        self.active = 0;
    }

    /// Applies `message` given the options' `labels`, and returns the index
    /// of the option it chose, if it chose one.
    pub fn update(&mut self, message: Message, labels: &[String]) -> Option<usize> {
        let shown = matching(labels, &self.query);
        match message {
            Message::Toggle if self.open => self.close(),
            Message::Toggle | Message::Open => self.open = true,
            Message::Close => self.close(),
            Message::Query(query) => {
                self.query = query;
                self.active = 0;
            }
            Message::Step(by) => {
                if !shown.is_empty() {
                    // Round at the ends, as the arrows of a menu.
                    let count = isize::try_from(shown.len()).unwrap_or(isize::MAX);
                    let at = isize::try_from(self.active).unwrap_or(0);
                    self.active = usize::try_from((at + by).rem_euclid(count)).unwrap_or(0);
                }
            }
            Message::Submit => {
                let chosen = shown.get(self.active).copied();
                if chosen.is_some() {
                    self.close();
                }
                return chosen;
            }
            Message::Choose(index) => {
                self.close();
                return (index < labels.len()).then_some(index);
            }
        }
        None
    }

    /// Scrolls the list so the highlighted option shows: run it after a
    /// [`Message::Step`] of a list long enough to scroll.
    pub fn reveal<T: Send + 'static>(&self) -> Task<T> {
        // Rows are far fewer than 2^24.
        #[allow(clippy::cast_precision_loss)]
        let top = self.active as f32 * ROW;
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

/// A select, being described.
pub struct Select<'a> {
    state: &'a State,
    labels: std::borrow::Cow<'a, [String]>,
    selected: Option<usize>,
    placeholder: String,
    searchable: bool,
    invalid: bool,
    disabled: bool,
    width: Length,
}

/// A select over `labels`, showing the one at `selected`.
pub fn select<'a>(state: &'a State, labels: &'a [String], selected: Option<usize>) -> Select<'a> {
    with_labels(state, std::borrow::Cow::Borrowed(labels), selected)
}

/// The same over labels made for this draw: a list the view builds from
/// what it reads.
pub fn select_owned(state: &State, labels: Vec<String>, selected: Option<usize>) -> Select<'_> {
    with_labels(state, std::borrow::Cow::Owned(labels), selected)
}

fn with_labels<'a>(
    state: &'a State,
    labels: std::borrow::Cow<'a, [String]>,
    selected: Option<usize>,
) -> Select<'a> {
    Select {
        state,
        labels,
        selected,
        placeholder: String::new(),
        searchable: false,
        invalid: false,
        disabled: false,
        width: Length::Shrink,
    }
}

impl<'a> Select<'a> {
    /// Shown, in the quieter colour, while nothing is chosen.
    #[must_use]
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// A search field over the list: the combobox. For a long list.
    #[must_use]
    pub fn searchable(mut self, searchable: bool) -> Self {
        self.searchable = searchable;
        self
    }

    /// The border in Signal red: the form refused the value.
    #[must_use]
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn view(self) -> Element<'a, Message> {
        let chosen = self.selected.and_then(|index| self.labels.get(index));
        let shown = match chosen {
            Some(label) => text(label.clone()).size(size::SM),
            None => text(self.placeholder.clone())
                .size(size::SM)
                .style(theme::text::muted),
        };
        // One line, cut where the button ends: a long label doesn't make
        // the button taller.
        let shown = shown.wrapping(iced::widget::text::Wrapping::None);
        let chevron = icon(Icon::ChevronDown).style(theme::svg::muted);
        let face = line(vec![filling(shown).clip(true).into(), chevron.into()])
            .spacing(space::S1_5)
            .width(Length::Fill);
        let button = pressable(container(face).center_y(Length::Fill).width(Length::Fill))
            .width(self.width)
            .height(control::HEIGHT)
            .padding([0.0, space::S2_5])
            .on_press_maybe((!self.disabled).then_some(Message::Toggle))
            .on_key(|key, _| {
                matches!(key, Key::Named(Named::ArrowDown | Named::ArrowUp))
                    .then_some(Message::Open)
            })
            .style(theme::button::part::field(self.invalid));
        let list = self.state.open.then(|| self.list());
        // A plain list opens on the chosen option; a searchable one on its
        // search field, which comes first.
        let first = if self.searchable {
            0
        } else {
            self.selected.unwrap_or(0)
        };
        anchored(
            button,
            list,
            Placement::new(Side::Bottom, Align::start(i18n::direction())),
        )
        .match_width()
        .focus_on_open(Some(first))
        .on_dismiss(Message::Close)
        .modal()
        .into()
    }

    fn option(&self, index: usize, active: bool) -> Element<'a, Message> {
        let mut parts: Vec<Element<'a, Message>> =
            vec![filling(text(self.labels[index].clone()).size(size::SM)).into()];
        if self.selected == Some(index) {
            parts.push(icon(Icon::Check).into());
        }
        pressable(
            container(line(parts).spacing(space::S1_5).width(Length::Fill))
                .center_y(Length::Fill)
                .width(Length::Fill),
        )
        .width(Length::Fill)
        .height(ROW)
        .padding([0.0, space::S1_5])
        .on_press(Message::Choose(index))
        .style(theme::button::part::item(active))
        .into()
    }

    fn list(&self) -> Element<'a, Message> {
        let shown = if self.searchable {
            matching(&self.labels, &self.state.query)
        } else {
            (0..self.labels.len()).collect()
        };
        let options: Vec<Element<'a, Message>> = shown
            .iter()
            .enumerate()
            .map(|(at, &index)| self.option(index, self.searchable && at == self.state.active))
            .collect();
        let rows = if options.is_empty() {
            vec![container(
                text(t!("ui.select.noMatch"))
                    .size(size::SM)
                    .style(theme::text::muted),
            )
            .padding([space::S4, space::S2])
            .center_x(Length::Fill)
            .into()]
        } else {
            options
        };
        let rows = fit(rows).min_width(MIN_WIDTH).padding(space::S1);
        let list: Element<'a, Message> = if self.searchable {
            scrollable(rows).id(self.state.list.clone()).into()
        } else {
            let labels = self.labels.to_vec();
            scrollable(roving(Axis::Vertical, rows).labels(labels))
                .id(self.state.list.clone())
                .into()
        };
        let mut parts: Vec<Element<'a, Message>> = vec![];
        if self.searchable {
            let search = input(&t!("ui.search"), &self.state.query)
                .on_input(Message::Query)
                .on_submit(Message::Submit)
                .style(theme::text_input::bare);
            parts.push(input_group(search, Some(Addon::icon(Icon::Search)), None));
            parts.push(rule::horizontal(1).into());
        }
        parts.push(list);
        // The panel is a `fit`, so the width of the button reaches the
        // options and their highlight goes from edge to edge.
        let panel = fit(parts)
            .max_item_height(LIST_HEIGHT)
            .style(theme::container::popover);
        if self.searchable {
            key_area(panel, |key, _| match key {
                Key::Named(Named::ArrowDown) => Some(Message::Step(1)),
                Key::Named(Named::ArrowUp) => Some(Message::Step(-1)),
                _ => None,
            })
            .into()
        } else {
            panel.into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engines() -> Vec<String> {
        ["Docker Engine", "containerd", "Incus", "Podman"]
            .map(str::to_owned)
            .to_vec()
    }

    #[test]
    fn the_search_narrows_whatever_the_case() {
        let labels = engines();
        assert_eq!(matching(&labels, ""), [0, 1, 2, 3]);
        assert_eq!(matching(&labels, "  "), [0, 1, 2, 3]);
        assert_eq!(matching(&labels, "d"), [0, 1, 3]);
        assert_eq!(matching(&labels, "INC"), [2]);
        assert_eq!(matching(&labels, "kube"), Vec::<usize>::new());
    }

    #[test]
    fn the_arrows_go_round_the_matches_and_enter_chooses_the_highlighted_one() {
        let labels = engines();
        let mut state = State::default();
        assert_eq!(state.update(Message::Open, &labels), None);
        assert!(state.is_open());
        state.update(Message::Query("d".to_owned()), &labels);
        // Up from the first goes to the last match: Podman.
        state.update(Message::Step(-1), &labels);
        assert_eq!(state.update(Message::Submit, &labels), Some(3));
        // Choosing closes, and the next opening starts afresh.
        assert!(!state.is_open());
        state.update(Message::Toggle, &labels);
        state.update(Message::Step(1), &labels);
        state.update(Message::Step(1), &labels);
        assert_eq!(state.update(Message::Submit, &labels), Some(2));
    }

    #[test]
    fn nothing_is_chosen_when_nothing_matches() {
        let labels = engines();
        let mut state = State::default();
        state.update(Message::Open, &labels);
        state.update(Message::Query("kube".to_owned()), &labels);
        state.update(Message::Step(1), &labels);
        assert_eq!(state.update(Message::Submit, &labels), None);
        assert!(state.is_open());
        state.update(Message::Close, &labels);
        assert!(!state.is_open());
        assert_eq!(state.update(Message::Choose(1), &labels), Some(1));
        assert_eq!(state.update(Message::Choose(9), &labels), None);
    }
}
