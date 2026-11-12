//! `DataTable`: a table over any rows, however many. Only the rows in view
//! are built, so a list of a hundred thousand scrolls as one of ten.
//!
//! The page keeps a [`State`] per table and hands it the table's
//! [`Message`]s; the rows and the columns are data, built for the view
//! and for `update` alike:
//!
//! ```ignore
//! // state: table::State<String>, made once:
//! //   table::State::new().selecting(Mode::Multi).sorted(Sort::by("name", Order::Ascending))
//! fn columns<'a>() -> Vec<Column<'a, Image, Message>> {
//!     vec![
//!         Column::new("name", t!("images.column.name"), |image: &Image| {
//!             table::text(image.name.clone()).into()
//!         })
//!         .width(Width::Fill(2))
//!         .sort_by(|a, b| sort::natural(&a.name, &b.name))
//!         .typed(|image| image.name.clone()),
//!         Column::new("size", t!("images.column.size"), |image: &Image| {
//!             table::text(format::bytes(image.size)).into()
//!         })
//!         .align(Align::End)
//!         .sort_by(|a, b| a.size.cmp(&b.size))
//!         .descending_first(),
//!     ]
//! }
//! fn rows<'r>(&'r self, images: &'r [Image]) -> Rows<'r, Image, String> {
//!     Rows::new(images, |image| image.id.clone())
//!         .filter(|image| table::matches(&self.query, &[&image.name]))
//! }
//!
//! // in view:
//! Table::new(&self.table, columns(), self.rows(&images), Message::Table)
//!     .row_menu(|image| vec![menu::Item::action(t!("common.remove"), Message::Remove(image.id.clone()))])
//!     .view()
//!
//! // in update:
//! Message::Table(message) => match self.table.update(message, &self.rows(&images), &columns()) {
//!     Some(table::Event::Opened(id)) => ctx.open(Route::Image { id }),
//!     Some(table::Event::Selected | table::Event::Arranged) | None => {}
//! }
//! ```
//!
//! - **Rows are known by their key.** The selection, the row the keyboard
//!   is on, the open menu and the scroll position follow the keys when the
//!   list is read again, sorted or filtered, and let go of a row that left
//!   it.
//! - **Sorting.** A header with [`Column::sort_by`] is a button (Tab
//!   reaches it, Enter and Space press it): ascending, descending, unsorted,
//!   as TanStack's; with Shift it sorts after the columns that already do.
//!   The sort is stable; [`sort::natural`] compares text as TanStack does.
//! - **Selecting** ([`Mode`]): a press selects a row, Ctrl or Cmd adds or
//!   removes one, Shift selects from the last one to this one; the
//!   checkbox over the rows selects all those that show, and shows a mark
//!   when only some are. [`State::opening_on_press`] makes a plain press
//!   open the row, as the React table's `onRowClick`; [`State::mark`]
//!   marks one row and scrolls to it, as its `selectedId`.
//! - **Groups** ([`Rows::group_by`]): rows nested under a header that
//!   closes and opens, as the containers of a Stack.
//! - **The keyboard.** The rows are one Tab stop; the keys are listed in
//!   `grid/input.rs`. The ring shows on the line the keyboard is on, and
//!   only for the keyboard.
//! - **Columns** are resized by dragging the end of their header, within
//!   [`Column::between`] and what the others leave (twice on the edge
//!   gives the column its own width back), hidden with
//!   [`column_items`], and scroll sideways when they don't fit.
//!   [`State::sort`], [`State::widths`] and [`State::hidden`] are what a
//!   page keeps to find the table as it was left ([`Event::Arranged`]
//!   says when they change).
//! - **While there are no rows**: [`Table::loading`] shows skeletons,
//!   [`Table::failed`] a failure (`feedback::error_message`), and an empty
//!   list [`Table::empty`] (`feedback::empty_state`) or "Nothing here."
//!
//! The table is as tall as its rows, up to [`Table::max_height`] (a page
//! is in a scroll area, which gives it all the height it asks for), or
//! takes the height it is given with [`Table::fill`].

mod build;
pub mod column;
mod derived;
mod grid;
pub mod selection;
pub mod sort;
pub mod state;
mod update;
pub mod window;

#[cfg(test)]
mod tests;

use std::{hash::Hash, rc::Rc};

use iced::widget::text::Wrapping;

use self::state::Message as Action;
pub use self::{
    column::{Align, Column, ColumnId, Width},
    derived::Rows,
    selection::{Mode, Selection},
    sort::{Order, Sort},
    state::{Event, Message, Motion, State, HEADER_HEIGHT, ROW_HEIGHT},
};
use crate::ui::{
    theme::{metrics::text as size, Element, Theme},
    widgets::overlay::menu,
};

/// How tall the rows get when the page gives all the height they ask for.
const MAX_HEIGHT: f32 = 600.0;

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

/// The items of a menu of the columns: each one checked while it shows,
/// and hidden or shown again by choosing it. A page puts them in a
/// `menu::dropdown` beside its search.
pub fn column_items<T, K, Message>(
    state: &State<K>,
    columns: &[Column<'_, T, Message>],
    on_event: impl Fn(self::Message<K>) -> Message,
) -> Vec<menu::Item<Message>>
where
    K: Hash + Eq + Clone,
{
    columns
        .iter()
        .filter(|column| column.is_hideable())
        .map(|column| {
            let hidden = state.is_hidden(column.id());
            let toggle = on_event(Action::Hide {
                column: column.id(),
                hidden: !hidden,
            });
            menu::Item::action(column.header(), toggle).checked(!hidden)
        })
        .collect()
}

/// What the header of a group shows.
pub struct GroupHeader<'a, Message> {
    /// Beside the chevron: the group's name, and what there is to say.
    pub label: Element<'a, Message>,
    /// Under the columns of these names: the totals of the group's rows.
    pub cells: Vec<(ColumnId, Element<'a, Message>)>,
}

type RowMenu<'s, T, Message> = Box<dyn Fn(&T) -> Vec<menu::Item<Message>> + 's>;
type GroupLook<'a, 's, T, Message> = Box<dyn Fn(&str, &[&T]) -> GroupHeader<'a, Message> + 's>;

/// A table being described; [`Table::view`] makes it an element.
pub struct Table<'a, 's, T, K, Message> {
    state: &'s State<K>,
    columns: Vec<Column<'a, T, Message>>,
    rows: Rows<'s, T, K>,
    on_event: Rc<dyn Fn(Action<K>) -> Message + 'a>,
    menu: Option<RowMenu<'s, T, Message>>,
    group_header: Option<GroupLook<'a, 's, T, Message>>,
    loading: bool,
    empty: Option<Element<'a, Message>>,
    empty_label: Option<String>,
    failed: Option<Element<'a, Message>>,
    bulk: Option<Option<Element<'a, Message>>>,
    fill: bool,
    max_height: f32,
}

impl<'a, 's, T, K, Message> Table<'a, 's, T, K, Message>
where
    K: Hash + Eq + Clone + 'a,
    Message: Clone + 'a,
{
    /// A table of `rows`, whose messages become the page's through
    /// `on_event`. The cells are built here and now: nothing of the rows
    /// is kept.
    pub fn new(
        state: &'s State<K>,
        columns: Vec<Column<'a, T, Message>>,
        rows: Rows<'s, T, K>,
        on_event: impl Fn(Action<K>) -> Message + 'a,
    ) -> Self {
        Self {
            state,
            columns,
            rows,
            on_event: Rc::new(on_event),
            menu: None,
            group_header: None,
            loading: false,
            empty: None,
            empty_label: None,
            failed: None,
            bulk: None,
            fill: false,
            max_height: MAX_HEIGHT,
        }
    }

    /// A menu at the end of every row: its button, the secondary button
    /// of the pointer on the row, the menu key or Shift+F10 on the focused
    /// row. The items are asked for the row whose menu is open.
    #[must_use]
    pub fn row_menu(mut self, items: impl Fn(&T) -> Vec<menu::Item<Message>> + 's) -> Self {
        self.menu = Some(Box::new(items));
        self
    }

    /// What the header of a group shows, from its name and its rows (the
    /// name alone unless told): a Stack's status and its totals.
    #[must_use]
    pub fn group_header(
        mut self,
        header: impl Fn(&str, &[&T]) -> GroupHeader<'a, Message> + 's,
    ) -> Self {
        self.group_header = Some(Box::new(header));
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

    /// What an empty list says in place of "Nothing here.": the React
    /// table's `empty` when it was a line of text.
    #[must_use]
    pub fn empty_label(mut self, label: impl Into<String>) -> Self {
        self.empty_label = Some(label.into());
        self
    }

    /// The list could not be read: an `error_message`, in place of the rows.
    #[must_use]
    pub fn failed(mut self, failure: Option<Element<'a, Message>>) -> Self {
        self.failed = failure;
        self
    }

    /// A line over the table for what can be done to the selected rows
    /// (how many there are, and the buttons), shown while the page gives
    /// one. Its room is kept when it is empty, so the rows don't move
    /// under the pointer as the first one is selected.
    #[must_use]
    pub fn selection_bar(mut self, bar: Option<Element<'a, Message>>) -> Self {
        self.bulk = Some(bar);
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
}
