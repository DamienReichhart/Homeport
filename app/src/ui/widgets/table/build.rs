//! The table built for a view: the header, the lines that are in view
//! and what stands for the rows when there is none.

use std::hash::{DefaultHasher, Hash, Hasher};

use iced::{
    widget::{container, Space},
    Length, Padding,
};

use super::{
    column::{Column, Track},
    derived::{comparison, Compare, Derived, Entry, Group},
    grid::{Focus, Grid, Row, What},
    selection::{Extent, Mode, Selection},
    sort::Order,
    state::Message as Action,
    text, window, GroupHeader, Table,
};
use crate::{
    i18n, t,
    ui::{
        theme::{
            self, font, icon,
            metrics::{control, icon as glyph, space},
            Element, Icon,
        },
        widgets::{
            basic::truncated,
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
/// `h-24`: the room of "Nothing here."
const EMPTY_HEIGHT: f32 = 96.0;
/// How many rows of skeletons stand for a list that is being read.
const SKELETONS: usize = 6;
/// How far the rows of a group start after its header's chevron.
const NESTED: f32 = 24.0;

fn hashed(value: impl Hash) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

/// Where the keyboard is, for a view of the lines `view`: on a line that
/// shows, on one that doesn't (the wheel took the view elsewhere), or on
/// none yet.
pub(super) fn focus_in(focused: Option<usize>, view: window::Window) -> Focus {
    match focused {
        None => Focus::None,
        Some(at) if view.contains(at) => Focus::At(at),
        Some(_) => Focus::Away,
    }
}

fn check(extent: Extent) -> Check {
    match extent {
        Extent::None => Check::Off,
        Extent::Some => Check::Mixed,
        Extent::All => Check::On,
    }
}

/// What a line's cells are built with: which columns show, and whether
/// the checkboxes' and the menus' do.
struct Shape<'c, 'a, T, Message> {
    columns: Vec<&'c Column<'a, T, Message>>,
    selects: bool,
    menus: bool,
}

impl<T, Message> Shape<'_, '_, T, Message> {
    /// The place of the first column of the page among all of them.
    fn first(&self) -> usize {
        usize::from(self.selects)
    }
}

impl<'a, T, K, Message> Table<'a, '_, T, K, Message>
where
    K: Hash + Eq + Clone + 'a,
    Message: Clone + 'a,
{
    /// What stands for the rows when there is none to show.
    fn stand_in(
        failed: Option<Element<'a, Message>>,
        empty: Option<Element<'a, Message>>,
        label: Option<String>,
        loading: bool,
        shown: usize,
    ) -> Option<Element<'a, Message>> {
        if let Some(failure) = failed {
            return Some(
                container(failure)
                    .padding(space::S4)
                    .width(Length::Fill)
                    .into(),
            );
        }
        if shown > 0 || loading {
            return None;
        }
        Some(match empty {
            Some(empty) => container(empty)
                .center_x(Length::Fill)
                .padding(space::S4)
                .into(),
            None => {
                let label = label.unwrap_or_else(|| t!("ui.table.empty"));
                container(text(label).style(theme::text::muted))
                    .center_x(Length::Fill)
                    .center_y(EMPTY_HEIGHT)
                    .into()
            }
        })
    }

    /// The header of a column: a button when it sorts, with the arrow of
    /// the way it does.
    fn header(&self, column: &Column<'a, T, Message>) -> (Element<'a, Message>, Option<Message>) {
        if column.compare.is_none() {
            // Cut where its column ends, with an ellipsis at the end the
            // language reads to, and whole under the pointer.
            let label = truncated(column.header.clone())
                .font(font::medium())
                .with_tip();
            return (label.into(), None);
        }
        let label = text(column.header.clone()).font(font::medium());
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

    /// The cells of a row, and the line they make.
    fn row(
        &self,
        shape: &Shape<'_, 'a, T, Message>,
        derived: &Derived<K>,
        row: &T,
        index: usize,
        cells: &mut Vec<Element<'a, Message>>,
    ) -> Row<K> {
        let key = (self.rows.key)(row);
        let selected = derived.selection.contains(&key);
        if shape.selects {
            let toggle = (self.on_event)(Action::Toggle {
                key: key.clone(),
                index,
            });
            let check = if selected { Check::On } else { Check::Off };
            cells.push(checkbox(check).tab_stop(false).on_press(toggle).into());
        }
        let nested = derived.entries.is_some() && index < derived.alone_from;
        for (at, column) in shape.columns.iter().enumerate() {
            let cell = (column.cell)(row);
            cells.push(if nested && at == 0 {
                // Under its group's name, past the chevron.
                let (left, right) = i18n::direction().pick((NESTED, 0.0), (0.0, NESTED));
                container(cell)
                    .padding(Padding::ZERO.left(left).right(right))
                    .into()
            } else {
                cell
            });
        }
        if let Some(items) = &self.menu {
            let open = derived
                .menu
                .as_ref()
                .filter(|(of, ..)| *of == key)
                .map(|(.., state)| state);
            cells.push(self.menu_of(&key, open, || items(row)));
        }
        Row {
            id: hashed(&key),
            selected: selected || self.state.marked.as_ref() == Some(&key),
            what: What::Row(key),
            span: None,
        }
    }

    /// The header of a group: its checkbox, its chevron and its name over
    /// the first columns, and what the page shows under the others.
    fn group(
        &self,
        shape: &Shape<'_, 'a, T, Message>,
        group: &Group,
        index: usize,
        cells: &mut Vec<Element<'a, Message>>,
    ) -> Row<K> {
        let name = group.name.clone();
        if shape.selects {
            let extent = Selection::<K>::extent(group.selected, group.rows.len());
            let select = (self.on_event)(Action::SelectGroup {
                name: name.clone(),
                selected: extent != Extent::All,
            });
            cells.push(
                checkbox(check(extent))
                    .tab_stop(false)
                    .on_press(select)
                    .into(),
            );
        }
        let header = match &self.group_header {
            Some(look) => {
                let members: Vec<&T> = group
                    .rows
                    .iter()
                    .map(|&row| &self.rows.rows[row as usize])
                    .collect();
                look(&name, &members)
            }
            None => GroupHeader {
                label: text(name.clone()).font(font::medium()).into(),
                cells: vec![],
            },
        };
        let chevron = if group.collapsed {
            i18n::direction().pick(Icon::ChevronRight, Icon::ChevronLeft)
        } else {
            Icon::ChevronDown
        };
        let press = (self.on_event)(Action::GroupPressed {
            name: name.clone(),
            index,
        });
        let toggle = pressable(icon(chevron))
            .padding(0.0)
            .style(theme::button::part::bare)
            // The rows are one stop: Space, Enter and the arrows across
            // close and open the group the keyboard is on.
            .tab_stop(false)
            .on_press(press);
        let label = line(vec![toggle.into(), header.label]).spacing(space::S2);
        let mut under = header.cells;
        // The name runs up to the first column the page fills.
        let mut until = shape.columns.len();
        if !shape.columns.is_empty() {
            cells.push(label.into());
        }
        for (at, column) in shape.columns.iter().enumerate().skip(1) {
            match under.iter().position(|(id, _)| *id == column.id) {
                Some(found) => {
                    until = until.min(at);
                    cells.push(under.swap_remove(found).1);
                }
                None => cells.push(Space::new().into()),
            }
        }
        if shape.menus {
            cells.push(Space::new().into());
        }
        Row {
            id: hashed(("group", &name)),
            selected: false,
            what: What::Group {
                name,
                collapsed: group.collapsed,
            },
            span: (!shape.columns.is_empty()).then(|| (shape.first(), shape.first() + until)),
        }
    }

    /// A line of skeletons, a bar under each column of the page.
    fn skeleton(
        shape: &Shape<'_, 'a, T, Message>,
        at: usize,
        cells: &mut Vec<Element<'a, Message>>,
    ) -> Row<K> {
        if shape.selects {
            cells.push(Space::new().into());
        }
        cells.extend(
            shape
                .columns
                .iter()
                .map(|_| skeleton(Length::Fill, glyph::MD)),
        );
        if shape.menus {
            cells.push(Space::new().into());
        }
        Row {
            id: hashed(("skeleton", at)),
            selected: false,
            what: What::Skeleton,
            span: None,
        }
    }

    /// The table, built for the lines that are in view.
    pub fn view(mut self) -> Element<'a, Message> {
        let state = self.state;
        let (failed, empty, label) = (
            self.failed.take(),
            self.empty.take(),
            self.empty_label.take(),
        );
        let compare = comparison(&state.sort, &self.columns);
        let derived = state.derive(
            &self.rows,
            compare.as_ref().map(|compare| compare as &Compare<'_, T>),
        );
        let count = derived.len();
        let stand_in = Self::stand_in(failed, empty, label, self.loading, count);
        let skeletons = stand_in.is_none() && count == 0;
        let shown = match (&stand_in, skeletons) {
            (Some(_), _) => 0,
            (None, true) => SKELETONS,
            (None, false) => count,
        };
        let view = window::window(
            state.offset(),
            state.viewport,
            state.row_height,
            shown,
            OVERSCAN,
        );
        let shape = Shape {
            columns: self
                .columns
                .iter()
                .filter(|column| !(column.hideable && state.is_hidden(column.id)))
                .collect(),
            selects: state.mode == Mode::Multi,
            menus: self.menu.is_some(),
        };

        let mut tracks = vec![];
        let mut ids = vec![];
        let mut keys = vec![];
        let mut children: Vec<Element<'a, Message>> = vec![];
        let mut additive = vec![];
        if shape.selects {
            let extent = Selection::<K>::extent(derived.selected, derived.order.len());
            tracks.push(Track::exactly(SELECT_WIDTH));
            ids.push(None);
            keys.push(hashed("the checkboxes"));
            additive.push(None);
            let all = (!derived.order.is_empty())
                .then(|| (self.on_event)(Action::ToggleAll(extent != Extent::All)));
            children.push(checkbox(check(extent)).on_press_maybe(all).into());
        }
        for column in &shape.columns {
            let (header, with_shift) = self.header(column);
            tracks.push(column.track(state.width_of(column.id)));
            ids.push(Some(column.id));
            keys.push(hashed(("column", column.id)));
            additive.push(with_shift);
            children.push(header);
        }
        if shape.menus {
            tracks.push(Track::exactly(MENU_WIDTH));
            ids.push(None);
            keys.push(hashed("the menus"));
            additive.push(None);
            children.push(Space::new().into());
        }

        let mut rows = Vec::with_capacity(view.len());
        for index in view.first..view.end {
            rows.push(match derived.entry(index) {
                _ if skeletons => Self::skeleton(&shape, index, &mut children),
                Some(Entry::Row(row)) => self.row(
                    &shape,
                    &derived,
                    &self.rows.rows[row as usize],
                    index,
                    &mut children,
                ),
                Some(Entry::Group(group)) => self.group(
                    &shape,
                    &derived.groups[group as usize],
                    index,
                    &mut children,
                ),
                None => break,
            });
        }
        let focused = focus_in(derived.focused.as_ref().map(|(_, at)| *at), view);
        // The menu of a row that scrolled out of view is not there to
        // have the keyboard.
        let menu_open = derived
            .menu
            .as_ref()
            .is_some_and(|(_, at, _)| view.contains(*at));
        let any_selected = !derived.selection.is_empty();
        drop(derived);

        let placeholder = stand_in.is_some();
        children.extend(stand_in);
        let on_event = self.on_event.clone();
        let grid = Grid {
            tracks,
            ids,
            keys,
            children,
            additive,
            rows,
            first: view.first,
            count: shown,
            offset: state.offset(),
            across: state.across,
            row_height: state.row_height,
            viewport: state.viewport,
            placeholder,
            focused,
            menu_open,
            has_menu: shape.menus,
            any_selected,
            pointer: state.opens_on_press,
            types: self.columns.iter().any(|column| column.typed.is_some()),
            fill: self.fill,
            max_height: self.max_height,
            rtl: i18n::direction().is_rtl(),
            on_event: Box::new(move |message| on_event(message)),
        };
        match self.bulk {
            // The same tree with a bar or without: the rows keep their
            // focus when the first one is selected.
            Some(bar) => stack(vec![
                container(bar.unwrap_or_else(|| Space::new().into()))
                    .center_y(control::HEIGHT_SM)
                    .into(),
                grid.into(),
            ])
            .spacing(space::S2)
            .into(),
            None => grid.into(),
        }
    }
}
