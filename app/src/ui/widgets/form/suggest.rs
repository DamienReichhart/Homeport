//! A field of free text with suggestions: the browser's `<input list>`
//! over a `<datalist>`, which React's Run dialog used for the image, the
//! capabilities and the log driver. What is typed stays what it is; the
//! list under the field only offers values to take.
//!
//! - The list shows the suggestions that hold what is typed, whatever the
//!   case (Chromium's and WebKit's datalist filter), but the one already
//!   typed whole; it opens as the text changes, or on the Down arrow, and
//!   shows every suggestion over an empty field.
//! - The keyboard stays in the field: Down and Up highlight a suggestion
//!   (round at the ends), Enter takes the highlighted one, Escape closes
//!   the list (and only the list: a dialog around stays), Tab and Shift +
//!   Tab close it and leave. Enter with nothing highlighted is the field's
//!   own (the form's submit).
//! - A press on a suggestion takes it; a press elsewhere closes the list.
//! - In Arabic the list hangs from the field's right edge.
//!
//! ```ignore
//! suggest(&self.image_list, &labels, &self.image)
//!     .placeholder("nginx:latest")
//!     .view()
//!     .map(Message::Image)
//! // in update:
//! match self.image_list.update(message, &self.image, &labels) {
//!     (task, Some(suggest::Event::Input(text))) => self.image = text,
//!     (task, Some(suggest::Event::Submit)) => .., // the form's Enter
//!     (task, None) => ..,
//! }
//! ```

use iced::{
    keyboard::{key::Named, Key},
    widget::{container, scrollable, text},
    Font, Length, Task,
};

use super::input::input;
use crate::{
    i18n,
    ui::{
        theme::{
            self,
            metrics::{space, text as size},
            Element,
        },
        widgets::{
            basic::fit,
            filling,
            focus::{self, key_area, pressable},
            overlay::{anchored, Align, Placement, Side},
        },
    },
};

/// `max-h`: the list scrolls past this.
const LIST_HEIGHT: f32 = 240.0;
/// A suggestion's height.
const ROW: f32 = 28.0;

/// Whether the list is open, and which suggestion the arrows are on.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct State {
    open: bool,
    /// Among those that match.
    active: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    Typed(String),
    /// Down on a closed list.
    Open,
    /// The highlight moves by this many suggestions.
    Step(isize),
    /// This suggestion was pressed.
    Pick(String),
    /// Escape, or a press outside.
    Close,
    /// Enter in the field.
    Submit,
    /// Tab (`true` with Shift): the list closes and focus leaves.
    Leave(bool),
}

/// What the field asks of the page.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    /// The field holds this now: typed, or a suggestion taken.
    Input(String),
    /// Enter with no suggestion highlighted: the form's own Enter.
    Submit,
}

/// The suggestions that hold `value`, whatever the case, but `value`
/// itself, by index; every one for an empty field.
pub fn matching(suggestions: &[String], value: &str) -> Vec<usize> {
    let wanted = value.trim().to_lowercase();
    suggestions
        .iter()
        .enumerate()
        .filter(|(_, s)| s.as_str() != value.trim() && s.to_lowercase().contains(&wanted))
        .map(|(index, _)| index)
        .collect()
}

impl State {
    pub fn is_open(&self) -> bool {
        self.open
    }

    fn close(&mut self) {
        self.open = false;
        self.active = None;
    }

    /// Applies `message` to a field that holds `value` and offers
    /// `suggestions`; returns what to run (focus that leaves) and what the
    /// page is asked.
    pub fn update<T: Send + 'static>(
        &mut self,
        message: Message,
        value: &str,
        suggestions: &[String],
    ) -> (Task<T>, Option<Event>) {
        let shown = matching(suggestions, value);
        let event = match message {
            Message::Typed(typed) => {
                self.open = !matching(suggestions, &typed).is_empty();
                self.active = None;
                Some(Event::Input(typed))
            }
            Message::Open => {
                self.open = !shown.is_empty();
                None
            }
            Message::Step(by) => {
                if !self.open {
                    self.open = !shown.is_empty();
                    return (Task::none(), None);
                }
                if !shown.is_empty() {
                    let count = isize::try_from(shown.len()).unwrap_or(isize::MAX);
                    let next = match self.active {
                        None if by > 0 => 0,
                        None => count - 1,
                        Some(at) => (isize::try_from(at).unwrap_or(0) + by).rem_euclid(count),
                    };
                    self.active = usize::try_from(next).ok();
                }
                None
            }
            Message::Pick(picked) => {
                self.close();
                Some(Event::Input(picked))
            }
            Message::Close => {
                self.close();
                None
            }
            Message::Submit => {
                let chosen = self
                    .active
                    .filter(|_| self.open)
                    .and_then(|at| shown.get(at))
                    .map(|&index| suggestions[index].clone());
                self.close();
                Some(chosen.map_or(Event::Submit, Event::Input))
            }
            Message::Leave(back) => {
                self.close();
                return (focus::traverse(back), None);
            }
        };
        (Task::none(), event)
    }
}

/// A field with suggestions, being described.
pub struct Suggest<'a> {
    state: &'a State,
    suggestions: std::borrow::Cow<'a, [String]>,
    value: &'a str,
    placeholder: String,
    font: Option<Font>,
    invalid: bool,
}

/// The field holding `value`, offering `suggestions` (in their order).
pub fn suggest<'a>(state: &'a State, suggestions: &'a [String], value: &'a str) -> Suggest<'a> {
    Suggest {
        state,
        suggestions: std::borrow::Cow::Borrowed(suggestions),
        value,
        placeholder: String::new(),
        font: None,
        invalid: false,
    }
}

/// The same over suggestions the view builds from what it reads (the
/// Host's images).
pub fn suggest_owned<'a>(
    state: &'a State,
    suggestions: Vec<String>,
    value: &'a str,
) -> Suggest<'a> {
    Suggest {
        suggestions: std::borrow::Cow::Owned(suggestions),
        ..suggest(state, &[], value)
    }
}

impl<'a> Suggest<'a> {
    #[must_use]
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// The field's face: the monospaced one for machine text.
    #[must_use]
    pub fn font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }

    /// The value is refused: the field's invalid style.
    #[must_use]
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    pub fn view(self) -> Element<'a, Message> {
        let mut field = input(&self.placeholder, self.value)
            .on_input(Message::Typed)
            .on_submit(Message::Submit);
        if let Some(font) = self.font {
            field = field.font(font);
        }
        if self.invalid {
            field = field.style(theme::text_input::invalid);
        }
        let open = self.state.open;
        // The arrows, Escape and Tab are the list's before the field's.
        let field = key_area(field, |_, _| None).before(move |key, modifiers| match key {
            Key::Named(Named::ArrowDown) => Some(Message::Step(1)),
            Key::Named(Named::ArrowUp) if open => Some(Message::Step(-1)),
            Key::Named(Named::Escape) if open => Some(Message::Close),
            Key::Named(Named::Tab) if open => Some(Message::Leave(modifiers.shift())),
            _ => None,
        });
        let shown = matching(&self.suggestions, self.value);
        let list = (open && !shown.is_empty()).then(|| self.list(&shown));
        anchored(
            field,
            list,
            Placement::new(Side::Bottom, Align::start(i18n::direction())),
        )
        .match_width()
        // The keyboard stays in the field.
        .focus_on_open(None)
        .on_dismiss(Message::Close)
        .into()
    }

    fn list(&self, shown: &[usize]) -> Element<'a, Message> {
        let rows: Vec<Element<'a, Message>> = shown
            .iter()
            .enumerate()
            .map(|(at, &index)| {
                let label = self.suggestions[index].clone();
                let mut words = text(label.clone()).size(size::SM);
                if let Some(font) = self.font {
                    words = words.font(font);
                }
                let active = self.state.active == Some(at);
                pressable(container(filling(words)).center_y(Length::Fill))
                    .width(Length::Fill)
                    .height(ROW)
                    .padding([0.0, space::S1_5])
                    .on_press(Message::Pick(label))
                    .style(theme::button::part::item(active))
                    .into()
            })
            .collect();
        let rows = fit(rows).padding(space::S1);
        fit(vec![scrollable(rows).into()])
            .max_item_height(LIST_HEIGHT)
            .style(theme::container::popover)
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tags() -> Vec<String> {
        ["nginx:1.27", "nginx:latest", "postgres:16", "Redis:7"]
            .map(str::to_owned)
            .to_vec()
    }

    fn step(state: &mut State, message: Message, value: &str) -> Option<Event> {
        state.update::<()>(message, value, &tags()).1
    }

    #[test]
    fn what_is_offered_holds_what_is_typed_whatever_the_case_but_itself() {
        let tags = tags();
        assert_eq!(matching(&tags, ""), [0, 1, 2, 3]);
        assert_eq!(matching(&tags, "NGINX"), [0, 1]);
        assert_eq!(matching(&tags, "redis"), [3]);
        assert_eq!(matching(&tags, "nginx:latest"), Vec::<usize>::new());
        assert_eq!(matching(&tags, "mysql"), Vec::<usize>::new());
    }

    #[test]
    fn typing_opens_the_list_and_free_text_stays_what_it_is() {
        let mut state = State::default();
        let typed = step(&mut state, Message::Typed("ngi".to_owned()), "");
        assert_eq!(typed, Some(Event::Input("ngi".to_owned())));
        assert!(state.is_open());
        // Nothing matches: the list closes, the text is the field's.
        let typed = step(&mut state, Message::Typed("my-image".to_owned()), "ngi");
        assert_eq!(typed, Some(Event::Input("my-image".to_owned())));
        assert!(!state.is_open());
        // Enter with nothing highlighted is the form's.
        assert_eq!(
            step(&mut state, Message::Submit, "my-image"),
            Some(Event::Submit)
        );
    }

    #[test]
    fn the_arrows_go_round_and_enter_takes_the_highlighted_one() {
        let mut state = State::default();
        // Down on a closed list opens it, on nothing yet.
        assert_eq!(step(&mut state, Message::Step(1), "nginx"), None);
        assert!(state.is_open());
        step(&mut state, Message::Step(1), "nginx");
        step(&mut state, Message::Step(1), "nginx");
        step(&mut state, Message::Step(1), "nginx");
        // Round: the first again.
        let taken = step(&mut state, Message::Submit, "nginx");
        assert_eq!(taken, Some(Event::Input("nginx:1.27".to_owned())));
        assert!(!state.is_open());
        // Up from nothing is the last.
        step(&mut state, Message::Open, "nginx");
        step(&mut state, Message::Step(-1), "nginx");
        let taken = step(&mut state, Message::Submit, "nginx");
        assert_eq!(taken, Some(Event::Input("nginx:latest".to_owned())));
    }

    #[test]
    fn escape_a_press_and_tab_close_the_list() {
        let mut state = State::default();
        step(&mut state, Message::Open, "");
        assert_eq!(step(&mut state, Message::Close, ""), None);
        assert!(!state.is_open());
        step(&mut state, Message::Open, "");
        let picked = step(&mut state, Message::Pick("postgres:16".to_owned()), "");
        assert_eq!(picked, Some(Event::Input("postgres:16".to_owned())));
        assert!(!state.is_open());
        step(&mut state, Message::Open, "");
        let (task, event) = state.update::<()>(Message::Leave(false), "", &tags());
        assert!(event.is_none() && !state.is_open());
        assert!(!crate::test_support::tasks::is_none(task), "focus leaves");
    }
}
