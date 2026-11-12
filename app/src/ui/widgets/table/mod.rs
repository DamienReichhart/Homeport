//! `DataTable`: a table over any rows, however many. Only the rows in view
//! are built, so a list of a hundred thousand scrolls as one of ten.
//!
//! The page keeps a [`State`] per table and hands it the table's
//! [`Message`]s; the columns are data, built in `view`:
//!
//! ```ignore
//! // state: table::State<String>, made once:
//! //   table::State::new().selecting(Mode::Multi).sorted(Sort::by("name", Order::Ascending))
//! let columns = vec![
//!     Column::new("name", t!("images.column.name"), |image: &Image| {
//!         table::text(image.name.clone()).into()
//!     })
//!     .width(Width::Fill(2))
//!     .sort_by(|a, b| a.name.cmp(&b.name)),
//!     Column::new("size", t!("images.column.size"), |image: &Image| {
//!         table::text(format::bytes(image.size)).into()
//!     })
//!     .width(Width::Fixed(96.0))
//!     .align(Align::End)
//!     .sort_by(|a, b| a.size.cmp(&b.size))
//!     .descending_first(),
//! ];
//! Table::new(&self.table, columns, &images, |image| image.id.clone(), Message::Table)
//!     .filter(|image| table::matches(&self.query, &[&image.name]))
//!     .row_menu(|image| vec![menu::Item::action(t!("common.remove"), Message::Remove(image.id.clone()))])
//!     .view()
//!
//! // in update:
//! Message::Table(message) => match self.table.update(message, &images, |image| image.id.clone()) {
//!     Some(table::Event::Opened(id)) => ctx.open(Route::Image { id }),
//!     Some(table::Event::Selected | table::Event::Arranged) | None => {}
//! }
//! ```
//!
//! - **Rows are known by their key.** The selection, the row the keyboard
//!   is on, the open menu and the scroll position follow the keys when the
//!   list is read again, sorted or filtered.
//! - **Sorting.** A header with [`Column::sort_by`] is a button (Tab
//!   reaches it, Enter and Space press it): ascending, descending, unsorted,
//!   as TanStack's; with Shift it sorts after the columns that already do.
//!   The sort is stable.
//! - **Selecting** ([`Mode`]): a press selects a row, Ctrl or Cmd adds or
//!   removes one, Shift selects from the last one to this one; the
//!   checkbox over the rows selects all, and shows a mark when only some
//!   are. [`State::opening_on_press`] makes a plain press open the row,
//!   as the React table's `onRowClick`.
//! - **The keyboard.** The rows are one Tab stop; the keys are listed in
//!   `grid/input.rs`. The ring shows on the row the keyboard is on, and
//!   only for the keyboard.
//! - **Columns** are resized by dragging the end of their header, within
//!   [`Column::between`]. [`State::sort`] and [`State::widths`] are what a
//!   page keeps to find the table as it was left ([`Event::Arranged`]
//!   says when they change).
//! - **While there are no rows**: [`Table::loading`] shows skeletons,
//!   [`Table::failed`] a failure (`feedback::error_message`), and an empty
//!   list [`Table::empty`] (`feedback::empty_state`) or "Nothing here."
//!
//! The table is as tall as its rows, up to [`Table::max_height`] (a page
//! is in a scroll area, which gives it all the height it asks for), or
//! takes the height it is given with [`Table::fill`].

pub mod column;
mod derived;
mod grid;
pub mod selection;
pub mod sort;
pub mod state;
pub mod window;

#[cfg(test)]
mod perf;
#[cfg(test)]
mod stage_tests;
#[cfg(test)]
mod tests;

use std::{
    cmp::Ordering,
    hash::{DefaultHasher, Hash, Hasher},
    rc::Rc,
};

use iced::{
    widget::{container, text::Wrapping, Space},
    Length,
};

use self::{
    column::Track,
    derived::Reading,
    grid::{Grid, Row},
    selection::Extent,
    state::Message as Action,
};
pub use self::{
    column::{Align, Column, ColumnId, Width},
    selection::{Mode, Selection},
    sort::{Order, Sort},
    state::{Event, Message, Motion, State, HEADER_HEIGHT, ROW_HEIGHT},
};
use crate::{
    i18n, t,
    ui::{
        theme::{
            self, font, icon,
            metrics::{control, icon as glyph, space, text as size},
            Element, Icon, Theme,
        },
        widgets::{
            feedback::skeleton,
            focus::pressable,
            form::checkbox::{checkbox, Check},
            line,
            overlay::menu,
            stack,
        },
    },
};

/// How many rows are built past each end of what shows.
const OVERSCAN: usize = 4;
/// `w-8`: the column of checkboxes.
const SELECT_WIDTH: f32 = 32.0;
/// The column of the rows' menus: an icon button and its margins.
const MENU_WIDTH: f32 = 44.0;
/// How tall the rows get when the page gives all the height they ask for.
const MAX_HEIGHT: f32 = 600.0;
/// `h-24`: the room of "Nothing here."
const EMPTY_HEIGHT: f32 = 96.0;
/// How many rows of skeletons stand for a list that is being read.
const SKELETONS: usize = 6;

/// A line of a cell: `text-sm`, never wrapped (`whitespace-nowrap`), cut
/// where its column ends.
pub fn text<'a>(content: impl Into<String>) -> iced::widget::Text<'a, Theme> {
    iced::widget::text(content.into())
        .size(size::SM)
        .wrapping(Wrapping::None)
}

/// Whether one of `fields` holds `query`, whatever the case: what the
/// React table's `filter` did with every column that had a value.
pub fn matches(query: &str, fields: &[&str]) -> bool {
    let query = query.trim().to_lowercase();
    query.is_empty()
        || fields
            .iter()
            .any(|field| field.to_lowercase().contains(&query))
}

type Key<'s, T, K> = Box<dyn Fn(&T) -> K + 's>;
type Filter<'s, T> = Box<dyn Fn(&T) -> bool + 's>;
type RowMenu<'s, T, Message> = Box<dyn Fn(&T) -> Vec<menu::Item<Message>> + 's>;

/// A table being described; [`Table::view`] makes it an element.
pub struct Table<'a, 's, T, K, Message> {
    state: &'s State<K>,
    columns: Vec<Column<'a, T, Message>>,
    rows: &'s [T],
    key: Key<'s, T, K>,
    on_event: Rc<dyn Fn(Action<K>) -> Message + 'a>,
    filter: Option<Filter<'s, T>>,
    menu: Option<RowMenu<'s, T, Message>>,
    revision: Option<u64>,
    loading: bool,
    empty: Option<Element<'a, Message>>,
    failed: Option<Element<'a, Message>>,
    bulk: Option<Element<'a, Message>>,
    fill: bool,
    max_height: f32,
}

impl<'a, 's, T, K, Message> Table<'a, 's, T, K, Message>
where
    K: Hash + Eq + Clone + 'a,
    Message: Clone + 'a,
{
    /// A table of `rows`, each known by its `key`, whose messages become
    /// the page's through `on_event`. The cells are built here and now:
    /// nothing of `rows` is kept.
    pub fn new(
        state: &'s State<K>,
        columns: Vec<Column<'a, T, Message>>,
        rows: &'s [T],
        key: impl Fn(&T) -> K + 's,
        on_event: impl Fn(Action<K>) -> Message + 'a,
    ) -> Self {
        Self {
            state,
            columns,
            rows,
            key: Box::new(key),
            on_event: Rc::new(on_event),
            filter: None,
            menu: None,
            revision: None,
            loading: false,
            empty: None,
            failed: None,
            bulk: None,
            fill: false,
            max_height: MAX_HEIGHT,
        }
    }

    /// Shows only the rows `passes` lets through: the search of a page.
    #[must_use]
    pub fn filter(mut self, passes: impl Fn(&T) -> bool + 's) -> Self {
        self.filter = Some(Box::new(passes));
        self
    }

    /// A menu at the end of every row: its button, the secondary button
    /// of the pointer on the row, the menu key or Shift+F10 on the focused
    /// row. The items are asked for the row whose menu is open.
    #[must_use]
    pub fn row_menu(mut self, items: impl Fn(&T) -> Vec<menu::Item<Message>> + 's) -> Self {
        self.menu = Some(Box::new(items));
        self
    }

    /// A number the page changes whenever the rows, or what the filter
    /// looks for, change. With it, a view that changes nothing sorts and
    /// filters nothing; without it, every view checks the order against
    /// the rows, which is a pass over a sorted or filtered list.
    #[must_use]
    pub fn revision(mut self, revision: u64) -> Self {
        self.revision = Some(revision);
        self
    }

    /// The list is being read for the first time: skeletons stand for it.
    #[must_use]
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// What shows when there is no row: an `empty_state`. "Nothing here."
    /// unless told.
    #[must_use]
    pub fn empty(mut self, empty: impl Into<Element<'a, Message>>) -> Self {
        self.empty = Some(empty.into());
        self
    }

    /// The list could not be read: an `error_message`, in place of the rows.
    #[must_use]
    pub fn failed(mut self, failure: Option<Element<'a, Message>>) -> Self {
        self.failed = failure;
        self
    }

    /// What can be done to the selected rows, shown over the table while
    /// the page gives it: how many are selected, and the buttons.
    #[must_use]
    pub fn selection_bar(mut self, bar: Option<Element<'a, Message>>) -> Self {
        self.bulk = bar;
        self
    }

    /// Takes all the height the page gives, however few the rows: for a
    /// table that is the whole of a pane.
    #[must_use]
    pub fn fill(mut self) -> Self {
        self.fill = true;
        self
    }

    /// The tallest the rows get before they scroll.
    #[must_use]
    pub fn max_height(mut self, height: f32) -> Self {
        self.max_height = height.max(0.0);
        self
    }

    /// What stands for the rows when there is none to show.
    fn stand_in(&mut self, shown: usize) -> Option<Element<'a, Message>> {
        if let Some(failure) = self.failed.take() {
            return Some(
                container(failure)
                    .padding(space::S4)
                    .width(Length::Fill)
                    .into(),
            );
        }
        if shown > 0 {
            return None;
        }
        if self.loading {
            let bar = || skeleton(Length::Fill, glyph::MD);
            let lines = (0..SKELETONS)
                .map(|_| {
                    let bars = self.columns.iter().map(|_| bar()).collect();
                    container(line(bars).spacing(space::S6).width(Length::Fill))
                        .padding([0.0, column::CELL_PADDING])
                        .center_y(self.state.row_height)
                        .into()
                })
                .collect();
            return Some(stack(lines).into());
        }
        Some(match self.empty.take() {
            Some(empty) => container(empty)
                .center_x(Length::Fill)
                .padding(space::S4)
                .into(),
            None => container(text(t!("ui.table.empty")).style(theme::text::muted))
                .center_x(Length::Fill)
                .center_y(EMPTY_HEIGHT)
                .into(),
        })
    }

    /// The header of a column: a button when it sorts, with the arrow of
    /// the way it does.
    fn header(&self, column: &Column<'a, T, Message>) -> (Element<'a, Message>, Option<Message>) {
        let label = text(column.header.clone()).font(font::medium());
        if column.compare.is_none() {
            return (label.into(), None);
        }
        let arrow = self.state.sort.of(column.id).map(|order| {
            icon(match order {
                Order::Ascending => Icon::ArrowUp,
                Order::Descending => Icon::ArrowDown,
            })
            .width(glyph::XS)
            .height(glyph::XS)
            .into()
        });
        let sort = |additive| {
            (self.on_event)(Action::Sort {
                column: column.id,
                descending_first: column.descending_first,
                additive,
            })
        };
        let parts = std::iter::once(label.into()).chain(arrow).collect();
        let button = pressable(line(parts).spacing(space::S1))
            .padding(0.0)
            .style(theme::button::part::bare)
            .on_press(sort(false));
        (button.into(), Some(sort(true)))
    }

    /// The table, built for the rows that are in view.
    pub fn view(mut self) -> Element<'a, Message> {
        let state = self.state;
        let multi = state.mode == Mode::Multi;
        let sorting: Vec<_> = state
            .sort
            .keys()
            .iter()
            .filter_map(|(id, order)| {
                let column = self.columns.iter().find(|column| column.id == *id)?;
                Some((column.compare.as_deref()?, *order))
            })
            .collect();
        let compare = |a: &T, b: &T| {
            sorting
                .iter()
                .fold(Ordering::Equal, |so_far, (compare, order)| {
                    so_far.then_with(|| order.apply(compare(a, b)))
                })
        };
        let derived = state.derive(
            self.rows,
            &Reading {
                key: &*self.key,
                filter: self.filter.as_deref(),
                compare: (!sorting.is_empty()).then_some(&compare),
                revision: self.revision,
            },
        );
        let count = derived.order.len();
        let stand_in = self.stand_in(count);
        let shown = if stand_in.is_some() { 0 } else { count };
        let view = window::window(
            state.offset(),
            state.viewport,
            state.row_height,
            shown,
            OVERSCAN,
        );

        let mut tracks = vec![];
        let mut ids = vec![];
        let mut children: Vec<Element<'a, Message>> = vec![];
        let mut additive = vec![];
        if multi {
            let extent = Selection::<K>::extent(derived.selected, count);
            let check = match extent {
                Extent::None => Check::Off,
                Extent::Some => Check::Mixed,
                Extent::All => Check::On,
            };
            tracks.push(Track::exactly(SELECT_WIDTH));
            ids.push(None);
            additive.push(None);
            children.push(
                checkbox(check)
                    .on_press_maybe(
                        (count > 0)
                            .then(|| (self.on_event)(Action::ToggleAll(extent != Extent::All))),
                    )
                    .into(),
            );
        }
        for column in &self.columns {
            let (header, with_shift) = self.header(column);
            tracks.push(column.track(state.width_of(column.id)));
            ids.push(Some(column.id));
            additive.push(with_shift);
            children.push(header);
        }
        if self.menu.is_some() {
            tracks.push(Track::exactly(MENU_WIDTH));
            ids.push(None);
            additive.push(None);
            children.push(Space::new().into());
        }

        let mut rows = Vec::with_capacity(view.len());
        let mut focused = None;
        for index in view.first..view.end {
            let row = &self.rows[derived.order[index] as usize];
            let key = (self.key)(row);
            let selected = state.selection.contains(&key);
            if state.focused() == Some(&key) {
                focused = Some(index);
            }
            if multi {
                let toggle = (self.on_event)(Action::Toggle {
                    key: key.clone(),
                    index,
                });
                let check = if selected { Check::On } else { Check::Off };
                children.push(checkbox(check).tab_stop(false).on_press(toggle).into());
            }
            children.extend(self.columns.iter().map(|column| (column.cell)(row)));
            if let Some(items) = &self.menu {
                children.push(self.menu_of(&key, state.menu_of(&key), || items(row)));
            }
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            rows.push(Row {
                id: hasher.finish(),
                key,
                selected,
            });
        }
        let any_selected = !state.selection.is_empty();
        drop(derived);

        let placeholder = stand_in.is_some();
        children.extend(stand_in);
        let on_event = self.on_event.clone();
        let grid = Grid {
            tracks,
            ids,
            children,
            additive,
            rows,
            first: view.first,
            count: shown,
            offset: state.offset(),
            row_height: state.row_height,
            viewport: state.viewport,
            placeholder,
            focused,
            menu_open: state.menu.is_some(),
            has_menu: self.menu.is_some(),
            any_selected,
            pointer: state.opens_on_press,
            fill: self.fill,
            max_height: self.max_height,
            rtl: i18n::direction().is_rtl(),
            on_event: Box::new(move |message| on_event(message)),
        };
        match self.bulk {
            Some(bar) => stack(vec![bar, grid.into()]).spacing(space::S2).into(),
            None => grid.into(),
        }
    }

    /// The button at the end of a row and the menu under it.
    fn menu_of(
        &self,
        key: &K,
        open: Option<&menu::State>,
        items: impl FnOnce() -> Vec<menu::Item<Message>>,
    ) -> Element<'a, Message> {
        let on_event = self.on_event.clone();
        let of = key.clone();
        let to_menu = move |event| on_event(Action::Menu(of.clone(), event));
        let trigger = pressable(
            container(icon(Icon::Ellipsis))
                .center_x(Length::Fill)
                .center_y(Length::Fill),
        )
        .padding(0.0)
        .width(control::HEIGHT_SM)
        .height(control::HEIGHT_SM)
        .style(theme::button::small(theme::button::ghost))
        // The rows are one stop: the menu key opens the focused row's.
        .tab_stop(false)
        .on_press(to_menu(menu::Event::Toggle));
        let closed = menu::State::default();
        match open {
            Some(state) => menu::dropdown(state, trigger, items(), to_menu),
            None => menu::dropdown(&closed, trigger, vec![], to_menu),
        }
    }
}
