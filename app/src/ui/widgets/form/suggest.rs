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
//!   Tab close it on their way. Enter with nothing highlighted is the
//!   field's own (the form's submit); with one highlighted, never.
//! - The list is what matched as the text changed, the first [`SHOWN`]: a
//!   suggestion read again meanwhile changes nothing of what Enter takes.
//! - While an input method composes, the arrows and Escape are its own.
//! - A press on a suggestion takes it; a press elsewhere closes the list.
//! - A suggestion taken puts the caret at the end of the field, as the
//!   browser does.
//! - In Arabic the list hangs from the field's right edge.
//!
//! ```ignore
//! suggest(&self.image_list, &self.image)
//!     .placeholder("nginx:latest")
//!     .view()
//!     .map(Message::Image)
//! // in update:
//! match self.image_list.update::<Message>(message, &self.image, &labels).1 {
//!     Some(suggest::Event::Input(text)) => self.image = text,
//!     Some(suggest::Event::Submit) => .., // the form's Enter
//!     None => {}
//! }
//! ```

use iced::{
    advanced::{
        layout, mouse, renderer,
        widget::{self, tree, Operation, Tree},
        Clipboard, Layout, Shell, Widget,
    },
    keyboard::{key::Named, Key},
    widget::{container, scrollable, text},
    Font, Length, Rectangle, Size, Task,
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
            focus::{key_area, pressable},
            overlay::{anchored, Align, Placement, Side},
        },
    },
};

/// `max-h`: the list scrolls past this.
const LIST_HEIGHT: f32 = 240.0;
/// A suggestion's height.
const ROW: f32 = 28.0;

/// How many suggestions the list holds at most: it scrolls, and what is
/// typed narrows it (the browser's list is as short).
pub const SHOWN: usize = 50;

/// Whether the list is open, what it shows and which the arrows are on.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct State {
    open: bool,
    /// What the list shows, worked out as the text or the list changed:
    /// the view draws it as it is, and what is highlighted is what Enter
    /// takes, even if the suggestions were read again meanwhile.
    shown: Vec<String>,
    /// Among those shown.
    active: Option<usize>,
    /// How many suggestions were taken: the field's caret goes to the end
    /// each time.
    taken: u64,
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
    /// Escape, Tab, or a press outside.
    Close,
    /// Enter in the field.
    Submit,
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

    /// What the list shows.
    pub fn shown(&self) -> &[String] {
        &self.shown
    }

    fn close(&mut self) {
        self.open = false;
        self.active = None;
    }

    /// Opens the list on what matches `value` now, the first [`SHOWN`].
    fn show(&mut self, suggestions: &[String], value: &str) {
        self.shown = matching(suggestions, value)
            .into_iter()
            .take(SHOWN)
            .map(|index| suggestions[index].clone())
            .collect();
        self.open = !self.shown.is_empty();
        self.active = None;
    }

    /// Applies `message` to a field that holds `value` and offers
    /// `suggestions`; returns what the page is asked. (The task is none:
    /// Tab moves on by itself, the list closing as it goes.)
    pub fn update<T: Send + 'static>(
        &mut self,
        message: Message,
        value: &str,
        suggestions: &[String],
    ) -> (Task<T>, Option<Event>) {
        let event = match message {
            Message::Typed(typed) => {
                self.show(suggestions, &typed);
                Some(Event::Input(typed))
            }
            Message::Open => {
                self.show(suggestions, value);
                None
            }
            Message::Step(by) => {
                if !self.open {
                    self.show(suggestions, value);
                    return (Task::none(), None);
                }
                let count = isize::try_from(self.shown.len()).unwrap_or(isize::MAX);
                if count > 0 {
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
                self.taken += 1;
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
                    .and_then(|at| self.shown.get(at).cloned());
                self.close();
                if chosen.is_some() {
                    self.taken += 1;
                }
                Some(chosen.map_or(Event::Submit, Event::Input))
            }
        };
        (Task::none(), event)
    }
}

/// A field with suggestions, being described.
pub struct Suggest<'a> {
    state: &'a State,
    value: &'a str,
    placeholder: String,
    font: Option<Font>,
    invalid: bool,
}

/// The field holding `value`; its list is what `state` worked out of the
/// suggestions as the text changed.
pub fn suggest<'a>(state: &'a State, value: &'a str) -> Suggest<'a> {
    Suggest {
        state,
        value,
        placeholder: String::new(),
        font: None,
        invalid: false,
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
        // The arrows and Escape are the list's before the field's; Tab
        // closes it on its way (the window or the dialog moves focus on).
        // An input method that composes keeps its keys (`key_area`).
        let field = AtEnd {
            content: field.into(),
            taken: self.state.taken,
        };
        let field = key_area(field, |_, _| None)
            .before(move |key, _| match key {
                Key::Named(Named::ArrowDown) => Some(Message::Step(1)),
                Key::Named(Named::ArrowUp) if open => Some(Message::Step(-1)),
                Key::Named(Named::Escape) if open => Some(Message::Close),
                _ => None,
            })
            .alongside(move |key, _| {
                (open && *key == Key::Named(Named::Tab)).then_some(Message::Close)
            });
        let shown = &self.state.shown;
        let list = (open && !shown.is_empty()).then(|| self.list(shown));
        anchored(
            field,
            list,
            Placement::new(Side::Bottom, Align::start(i18n::direction())),
        )
        .match_width()
        // The keyboard stays in the field.
        .keys_to_anchor()
        .on_dismiss(Message::Close)
        .into()
    }

    fn list(&self, shown: &[String]) -> Element<'a, Message> {
        let rows: Vec<Element<'a, Message>> = shown
            .iter()
            .enumerate()
            .map(|(at, label)| {
                let label = label.clone();
                let mut words = text(label.clone()).size(size::SM);
                if let Some(font) = self.font {
                    words = words.font(font);
                }
                let active = self.state.active == Some(at);
                pressable(container(filling(words)).center_y(Length::Fill))
                    .width(Length::Fill)
                    .height(ROW)
                    .padding([0.0, space::S1_5])
                    // The keyboard stays in the field, as in a datalist.
                    .focus_on_press(false)
                    .tab_stop(false)
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

/// The field, whose caret goes to the end when a suggestion was taken
/// since it last looked (`taken` is the count; the tree keeps the one
/// seen).
struct AtEnd<'a> {
    content: Element<'a, Message>,
    taken: u64,
}

/// Puts the caret of every field it meets at the end.
struct CaretToEnd;

impl Operation for CaretToEnd {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        operate(self);
    }

    fn text_input(
        &mut self,
        _id: Option<&widget::Id>,
        _bounds: Rectangle,
        state: &mut dyn widget::operation::TextInput,
    ) {
        state.move_cursor_to_end();
    }
}

impl Widget<Message, theme::Theme, iced::Renderer> for AtEnd<'_> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<u64>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(self.taken)
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn size(&self) -> Size<Length> {
        self.content.as_widget().size()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.content
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        self.content
            .as_widget_mut()
            .operate(&mut tree.children[0], layout, renderer, operation);
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &iced::Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let seen = tree.state.downcast_mut::<u64>();
        if *seen != self.taken {
            *seen = self.taken;
            self.content.as_widget_mut().operate(
                &mut tree.children[0],
                layout,
                renderer,
                &mut CaretToEnd,
            );
        }
        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &theme::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }
}

impl<'a> From<AtEnd<'a>> for Element<'a, Message> {
    fn from(field: AtEnd<'a>) -> Self {
        Element::new(field)
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
        assert_eq!(state.shown().len(), 4);
    }
}
