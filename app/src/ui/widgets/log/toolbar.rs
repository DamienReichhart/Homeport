//! The viewer's controls, over its lines: what the page adds (which
//! container, how far back), then how the lines show, and at the end of
//! the row the search and what is done with the lines. On a narrow viewer
//! the row goes on a second line.

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
            metrics::{space, text as size, ButtonSize},
            Element, Icon,
        },
        widgets::{
            basic::{
                button::{self, Variant},
                flow,
            },
            focus::key_area,
            form::{input, input_group, toggle_group, Addon, Toggle},
            line,
            overlay::{tooltip, Side},
        },
    },
};

/// `w-44`: the search field.
const SEARCH_WIDTH: f32 = 176.0;
/// What stands for "match case" and "regular expression" everywhere: no
/// language writes them otherwise, and each has its name under the pointer.
pub(in crate::ui::widgets) const CASE: &str = "Aa";
pub(in crate::ui::widgets) const REGEX: &str = ".*";

/// One toggle, alone: `Toggle variant="outline"`.
pub(in crate::ui::widgets) fn toggle<'a, M: Clone + 'a>(
    look: Toggle,
    on: bool,
    message: M,
) -> Element<'a, M> {
    toggle_group(vec![look], move |_| on, true, move |_| message.clone())
}

pub(in crate::ui::widgets) fn action<'a, M: Clone + 'a>(
    glyph: Icon,
    says: String,
    message: Option<M>,
) -> Element<'a, M> {
    let button = button::icon(glyph)
        .variant(Variant::Ghost)
        .size(ButtonSize::Sm)
        .on_press_maybe(message);
    tooltip(button, says, Side::Top).into()
}

/// How many matches there are and which one the search is on, or why
/// the query can't be read. Nothing while nothing is looked for.
fn found<'a, M: 'a>(state: &State) -> Element<'a, M> {
    let label = |says: String| text(says).size(size::XS);
    if let Some(error) = state.search_error() {
        return label(i18n::errors::message(error))
            .style(theme::text::destructive)
            .into();
    }
    if state.query().text.is_empty() {
        return label(String::new()).into();
    }
    let (current, total) = state.matches();
    // Counts of lines: far below what an `f64` holds whole.
    #[allow(clippy::cast_precision_loss)]
    let number = |count: u64| format::number(count as f64);
    let says = if total == 0 && !state.is_searching() {
        t!("ui.log.noMatches")
    } else {
        t!(
            "ui.log.matches",
            current = number(current.unwrap_or(0)).as_str(),
            count = number(total).as_str()
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
    let pushed = items.len() + 3;
    items.push(toggle(
        Toggle::text(t!("containers.logs.timestamps")),
        state.shows_timestamps(),
        on(Message::Timestamps(!state.shows_timestamps())),
    ));
    items.push(toggle(
        Toggle::text(t!("ui.log.wrap")),
        state.wraps(),
        on(Message::Wrap(!state.wraps())),
    ));
    items.push(toggle(
        Toggle {
            glyph: Some(Icon::ArrowDownToLine),
            label: Some(t!("containers.logs.follow")),
        },
        state.follows(),
        on(Message::Follow(!state.follows())),
    ));

    let query = state.query();
    let typed = on.clone();
    let field = input(&t!("containers.logs.find"), &query.text)
        .id(state.search_id())
        .on_input(move |text| typed(Message::Query(text)));
    let field = if state.search_error().is_some() {
        field.style(theme::text_input::invalid)
    } else {
        field
    };
    let field =
        container(input_group(field, Some(Addon::icon(Icon::Search)), None)).width(SEARCH_WIDTH);
    // Enter goes to the next match, with Shift to the one before.
    let stepped = on.clone();
    let field = key_area(field, |_, _| None).on_captured(move |key, modifiers| match key {
        Key::Named(Named::Enter) if modifiers.shift() => Some(stepped(Message::Previous)),
        Key::Named(Named::Enter) => Some(stepped(Message::Next)),
        _ => None,
    });
    let has_matches = state.matches().1 > 0;
    let mut search: Vec<Element<'a, M>> = vec![field.into()];
    search.extend([
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
        tooltip(
            toggle(
                Toggle::text(t!("ui.log.filter")),
                state.filters(),
                on(Message::Filter(!state.filters())),
            ),
            t!("ui.log.filterHint"),
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
            Icon::Eraser,
            t!("containers.logs.clear"),
            Some(on(Message::Clear)),
        ),
        action(
            Icon::Download,
            t!("containers.logs.download"),
            (!state.is_empty()).then(|| on(Message::Download)),
        ),
    ]);
    items.push(line(search).spacing(space::S1).into());
    flow(items).spacing(space::S2).push_from(pushed).into()
}
