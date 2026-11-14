//! The terminal's search bar, over its grid while the search is open: the
//! log viewer's field, count and toggles (`log::toolbar`), then the steps,
//! a copy of the selection, and the button that closes it. Enter goes to
//! the next match, Shift + Enter to the one before, Escape closes the bar
//! and gives the keyboard back to the grid.

use iced::{
    keyboard::{key::Named, Key},
    widget::{container, text},
};

use super::state::{Message, State};
use crate::{
    i18n::{self, format},
    t,
    ui::{
        theme::{
            self,
            metrics::{space, text as size},
            Element, Icon,
        },
        widgets::{
            basic::flow,
            focus::key_area,
            form::{input, input_group, Addon, Toggle},
            line,
            log::toolbar::{action, toggle, CASE, REGEX},
            overlay::{tooltip, Side},
        },
    },
};

/// `w-56`: the search field.
const SEARCH_WIDTH: f32 = 224.0;

/// How many matches there are and which one the search is on, or why
/// the query can't be read.
fn found<'a, M: 'a>(state: &State) -> Element<'a, M> {
    let label = |says: String| text(says).size(size::XS);
    let search = state.search();
    if let Some(error) = &search.error {
        return label(i18n::errors::message(error))
            .style(theme::text::destructive)
            .into();
    }
    if search.query.text.is_empty() {
        return label(String::new()).into();
    }
    let (current, total, capped) = search.counted();
    // Counts of matches: far below what an `f64` holds whole.
    #[allow(clippy::cast_precision_loss)]
    let number = |count: usize| format::number(count as f64);
    let says = if total == 0 {
        t!("ui.log.noMatches")
    } else {
        let count = if capped {
            t!("ui.terminal.atLeast", count = number(total).as_str())
        } else {
            number(total)
        };
        t!(
            "ui.log.matches",
            current = number(current).as_str(),
            count = count.as_str()
        )
    };
    label(says).style(theme::text::muted).into()
}

pub(super) fn toolbar<'a, M: Clone + 'a>(
    state: &'a State,
    before: Vec<Element<'a, M>>,
    on: impl Fn(Message) -> M + Clone + 'a,
) -> Element<'a, M> {
    let mut items = before;
    let pushed = items.len();
    let search = state.search();
    let query = &search.query;
    let typed = on.clone();
    let field = input(&t!("ui.terminal.find"), &query.text)
        .id(state.search_id())
        .on_input(move |text| typed(Message::Query(text)));
    let field = if search.error.is_some() {
        field.style(theme::text_input::invalid)
    } else {
        field
    };
    let field =
        container(input_group(field, Some(Addon::icon(Icon::Search)), None)).width(SEARCH_WIDTH);
    let stepped = on.clone();
    let field = key_area(field, |_, _| None).on_captured(move |key, modifiers| match key {
        Key::Named(Named::Enter) if modifiers.shift() => Some(stepped(Message::Previous)),
        Key::Named(Named::Enter) => Some(stepped(Message::Next)),
        _ => None,
    });
    let closing = on.clone();
    let field = field.before(move |key, _| match key {
        Key::Named(Named::Escape) => Some(closing(Message::CloseSearch)),
        _ => None,
    });
    let has_matches = !search.matches.is_empty();
    items.push(
        line(vec![
            field.into(),
            found(state),
            tooltip(
                toggle(
                    Toggle::text(CASE),
                    query.case,
                    on(Message::Case(!query.case)),
                ),
                t!("ui.log.matchCase"),
                Side::Top,
            )
            .into(),
            tooltip(
                toggle(
                    Toggle::text(REGEX),
                    query.regex,
                    on(Message::Regex(!query.regex)),
                ),
                t!("ui.log.regex"),
                Side::Top,
            )
            .into(),
            action(
                Icon::ChevronUp,
                t!("containers.logs.previous"),
                has_matches.then(|| on(Message::Previous)),
            ),
            action(
                Icon::ChevronDown,
                t!("containers.logs.next"),
                has_matches.then(|| on(Message::Next)),
            ),
            action(
                Icon::Copy,
                t!("ui.copy"),
                state.has_selection().then(|| on(Message::Copy)),
            ),
            action(
                Icon::X,
                t!("ui.terminal.closeSearch"),
                Some(on(Message::CloseSearch)),
            ),
        ])
        .spacing(space::S1)
        .into(),
    );
    flow(items).spacing(space::S2).push_from(pushed).into()
}
