//! `SearchInput`: a page's search field. The magnifier where its text
//! starts; once something is typed, a button at its end that empties
//! it. `/` gives it focus ([`focus_search`]); Escape empties it, and
//! leaves it when it is empty already.
//!
//! What is typed is the page's at once, as in React: a list in memory
//! is filtered as the letters come. A search that costs (a document of
//! megabytes, a request to an Engine) waits for a pause in the typing
//! with [`Debounce`].

use std::{rc::Rc, time::Duration};

use iced::{
    advanced::widget::Id,
    keyboard::{key::Named, Key},
    widget::{container, operation},
    Task,
};

use super::input::{input, input_group, Addon};
use crate::{
    t,
    ui::{
        theme::{
            metrics::{space, ButtonSize},
            Element, Icon,
        },
        widgets::{
            basic::button::{self, Variant},
            focus::key_area,
            overlay::{tooltip, Side},
        },
    },
};

/// `w-64`: how wide a page's search field is.
pub const SEARCH_WIDTH: f32 = 256.0;

/// How long the typing must pause before a search that costs is run.
pub const PAUSE: Duration = Duration::from_millis(250);

/// The search field: `id` is what `/` gives focus to, `on_input` gets
/// what is typed, and the empty text when the field is emptied.
pub fn search<'a, Message: Clone + 'a>(
    id: impl Into<Id>,
    placeholder: &str,
    value: &str,
    on_input: impl Fn(String) -> Message + 'a,
) -> Element<'a, Message> {
    let on_input = Rc::new(on_input);
    let typed = on_input.clone();
    let field = input(placeholder, value)
        .id(id)
        .on_input(move |text| typed(text));
    let filled = !value.is_empty();
    let clear = filled.then(|| {
        let clear = button::icon(Icon::X)
            .variant(Variant::Ghost)
            .size(ButtonSize::Xs)
            // Focus stays in the field: the next letter starts a new search.
            .keep_focus()
            .on_press(on_input(String::new()));
        Addon::new(
            tooltip(clear, t!("ui.search.clear"), Side::Top),
            ButtonSize::Xs.height() - space::S2,
        )
    });
    let group = input_group(field, Some(Addon::icon(Icon::Search)), clear);
    // Escape empties a field that holds something and stays in it; the
    // field itself lets go of focus when it is empty.
    let field = key_area(group, |_, _| None).before(move |key, _| {
        (filled && *key == Key::Named(Named::Escape)).then(|| on_input(String::new()))
    });
    container(field).width(SEARCH_WIDTH).into()
}

/// Gives focus to the search field `id` and selects what it holds, so
/// typing replaces it: what `/` does on a page with a list.
pub fn focus_search<Message: Send + 'static>(id: impl Into<Id> + Clone) -> Task<Message> {
    operation::focus(id.clone()).chain(operation::select_all(id))
}

/// What a search field holds, and whether the typing has paused: for a
/// search that costs too much to run at each letter.
///
/// ```ignore
/// Message::Typed(text) => self.search.input(text).map(Message::Paused),
/// Message::Paused(ticket) => {
///     if let Some(query) = self.search.settled(ticket) { /* search for it */ }
/// }
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Debounce {
    text: String,
    /// Counts the changes: the ticket of the last one is the only one
    /// that still stands when its pause ends.
    changes: u64,
}

impl Debounce {
    /// What the field shows: every letter, at once.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// The field changed. The task gives this change's ticket once the
    /// pause is over; at once when the field was emptied, which shows
    /// everything again and has nothing to wait for.
    pub fn input(&mut self, text: String) -> Task<u64> {
        self.text = text;
        self.changes += 1;
        let ticket = self.changes;
        if self.text.is_empty() {
            return Task::done(ticket);
        }
        Task::perform(tokio::time::sleep(PAUSE), move |()| ticket)
    }

    /// The text to search for, if nothing was typed since `ticket` was
    /// given.
    pub fn settled(&self, ticket: u64) -> Option<&str> {
        (ticket == self.changes).then_some(self.text.as_str())
    }
}

#[cfg(test)]
mod tests {
    use iced::keyboard::key::Named;

    use super::*;
    use crate::test_support::{snapshot::in_language, stage::Stage, tasks::messages};

    fn field(value: &str) -> Element<'_, String> {
        search("search-under-test", "Search", value, |text| text)
    }

    #[tokio::test(start_paused = true)]
    async fn a_search_that_costs_waits_for_the_typing_to_pause() {
        let mut search = Debounce::default();
        let first = search.input("we".to_owned());
        let second = search.input("web".to_owned());
        // The field shows every letter at once.
        assert_eq!(search.text(), "web");
        let started = tokio::time::Instant::now();
        let first = messages(first).await[0];
        assert_eq!(started.elapsed(), PAUSE);
        assert_eq!(search.settled(first), None, "typing went on");
        assert_eq!(search.settled(messages(second).await[0]), Some("web"));
        // Emptied: nothing to wait for.
        let started = tokio::time::Instant::now();
        let emptied = messages(search.input(String::new())).await[0];
        assert_eq!(started.elapsed(), std::time::Duration::ZERO);
        assert_eq!(search.settled(emptied), Some(""));
    }

    #[test]
    fn typing_reaches_the_page_and_the_button_at_the_end_empties_the_field() {
        let _english = in_language("en");
        let mut stage = Stage::new((400.0, 80.0));
        let typed = stage.frame(field(""), |frame| {
            frame.tab();
            frame.typewrite("w");
        });
        assert_eq!(typed, ["w"]);
        // The button is there once there is something to empty: at the
        // field's end, inside it.
        let emptied = stage.frame(field("web"), |frame| {
            frame.point_at((SEARCH_WIDTH - 14.0, 16.0));
            frame.press();
            assert!(frame.focused().is_some(), "focus stays in the field");
        });
        assert_eq!(emptied, [""]);
        // And not on an empty field: a press there is the field's.
        let nothing = stage.frame(field(""), |frame| {
            frame.point_at((SEARCH_WIDTH - 14.0, 16.0));
            frame.press();
        });
        assert!(nothing.is_empty());
    }

    #[test]
    fn escape_empties_the_field_and_leaves_one_that_is_empty() {
        let _english = in_language("en");
        let mut stage = Stage::new((400.0, 80.0));
        let emptied = stage.frame(field("web"), |frame| {
            frame.tab();
            frame.tap_key(Named::Escape);
            assert!(frame.focused().is_some(), "it stays in the field");
        });
        assert_eq!(emptied, [""]);
        let nothing = stage.frame(field(""), |frame| {
            assert!(frame.focused().is_some());
            frame.tap_key(Named::Escape);
            assert!(frame.focused().is_none(), "an empty field lets go");
        });
        assert!(nothing.is_empty());
        // Escape elsewhere on the page is not the field's.
        let mut stage = Stage::new((400.0, 80.0));
        let nothing = stage.frame(field("web"), |frame| frame.tap_key(Named::Escape));
        assert!(nothing.is_empty());
    }
}
