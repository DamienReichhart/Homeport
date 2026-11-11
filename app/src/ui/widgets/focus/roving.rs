//! Focus that roves inside a group, as Radix's menus, tabs and toggle
//! groups do: the arrow keys go to the next and previous item, Home and End
//! to the first and last, and typing goes to the item that starts with what
//! was typed.

use std::time::{Duration, Instant};

use iced::{
    advanced::{
        layout, mouse, overlay, renderer,
        widget::{tree, Operation, Tree},
        Clipboard, Layout, Shell, Widget,
    },
    keyboard::{self, key::Named, Key},
    Event, Length, Rectangle, Size, Vector,
};

use super::Move;
use crate::{
    i18n,
    ui::theme::{Element, Theme},
};

/// Typing pauses this long before it starts a new word (Radix's second).
const TYPEAHEAD: Duration = Duration::from_secs(1);

/// The way a group runs, which says which arrows move in it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    /// A menu, a list: Up and Down.
    Vertical,
    /// Tabs, a toggle group: Left and Right, swapped in a right-to-left
    /// language.
    Horizontal,
}

type OnMove<'a, Message> = Box<dyn Fn(usize) -> Message + 'a>;

/// A group whose items take focus in turn with the arrow keys.
pub struct Roving<'a, Message> {
    content: Element<'a, Message>,
    axis: Axis,
    wrap: bool,
    labels: Vec<String>,
    on_move: Option<OnMove<'a, Message>>,
}

/// `content`, whose focusable items the arrows of `axis` go through.
pub fn roving<'a, Message>(
    axis: Axis,
    content: impl Into<Element<'a, Message>>,
) -> Roving<'a, Message> {
    Roving {
        content: content.into(),
        axis,
        wrap: true,
        labels: vec![],
        on_move: None,
    }
}

impl<'a, Message> Roving<'a, Message> {
    /// Whether the last item leads back to the first. It does unless told.
    #[must_use]
    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    /// The text of each focusable item, in order, for typing to reach it.
    #[must_use]
    pub fn labels(mut self, labels: Vec<String>) -> Self {
        self.labels = labels;
        self
    }

    /// Sent with the index of the item focus moved to: tabs select the tab
    /// that takes focus.
    #[must_use]
    pub fn on_move(mut self, on_move: impl Fn(usize) -> Message + 'a) -> Self {
        self.on_move = Some(Box::new(on_move));
        self
    }

    fn step(&self, key: &Key) -> Option<Move> {
        let Key::Named(named) = key else {
            return None;
        };
        let rtl = i18n::direction().is_rtl();
        match (self.axis, named) {
            (Axis::Vertical, Named::ArrowDown) => Some(Move::Next),
            (Axis::Vertical, Named::ArrowUp) => Some(Move::Previous),
            (Axis::Horizontal, Named::ArrowRight) => {
                Some(if rtl { Move::Previous } else { Move::Next })
            }
            (Axis::Horizontal, Named::ArrowLeft) => {
                Some(if rtl { Move::Next } else { Move::Previous })
            }
            (_, Named::Home) => Some(Move::First),
            (_, Named::End) => Some(Move::Last),
            _ => None,
        }
    }
}

/// What was typed so far, and when its last letter was.
#[derive(Debug, Default)]
struct State {
    typed: String,
    at: Option<Instant>,
}

/// The item typing `typed` leads to from `current`: the next one that
/// starts with it, case aside. One letter typed again goes on to the next
/// match; a longer word may stay where it is.
pub fn typeahead(labels: &[String], typed: &str, current: usize) -> Option<usize> {
    let wanted = typed.to_lowercase();
    if wanted.is_empty() || labels.is_empty() {
        return None;
    }
    let repeated = wanted.chars().all(|c| Some(c) == wanted.chars().next());
    let (wanted, skip) = if repeated {
        (wanted.chars().take(1).collect::<String>(), 1)
    } else {
        (wanted, 0)
    };
    (0..labels.len())
        .map(|offset| (current + skip + offset) % labels.len())
        .find(|&index| labels[index].to_lowercase().starts_with(&wanted))
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Roving<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
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

    fn size_hint(&self) -> Size<Length> {
        self.content.as_widget().size_hint()
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
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
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
        let Event::Keyboard(keyboard::Event::KeyPressed {
            key,
            modifiers,
            text,
            ..
        }) = event
        else {
            return;
        };
        if shell.is_event_captured() || modifiers.command() || modifiers.alt() {
            return;
        }
        let found = super::find(
            self.content.as_widget_mut(),
            &mut tree.children[0],
            layout,
            renderer,
        );
        let Some(current) = found.focused else {
            return;
        };
        let to = self.step(key).or_else(|| {
            let letter = text.as_ref().filter(|t| !t.chars().any(char::is_control))?;
            let state = tree.state.downcast_mut::<State>();
            let now = Instant::now();
            if state.at.is_none_or(|at| now.duration_since(at) > TYPEAHEAD) {
                state.typed.clear();
            }
            state.typed.push_str(letter);
            state.at = Some(now);
            typeahead(&self.labels, &state.typed, current).map(Move::Index)
        });
        let Some(to) = to else {
            return;
        };
        let landed = super::move_within(
            self.content.as_widget_mut(),
            &mut tree.children[0],
            layout,
            renderer,
            to,
            self.wrap,
        );
        shell.capture_event();
        shell.request_redraw();
        if let (Some(index), Some(on_move)) = (landed, &self.on_move) {
            if index != current {
                shell.publish(on_move(index));
            }
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
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

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &iced::Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, iced::Renderer>> {
        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            layout,
            renderer,
            viewport,
            translation,
        )
    }
}

impl<'a, Message: 'a> From<Roving<'a, Message>> for Element<'a, Message> {
    fn from(roving: Roving<'a, Message>) -> Self {
        Element::new(roving)
    }
}

#[cfg(test)]
mod tests {
    use super::typeahead;

    #[test]
    fn typing_goes_to_the_next_item_that_starts_with_it() {
        let labels: Vec<String> = ["Start", "Stop", "Restart", "Remove"]
            .map(str::to_owned)
            .to_vec();
        assert_eq!(typeahead(&labels, "r", 0), Some(2));
        // The same letter again goes on, and round.
        assert_eq!(typeahead(&labels, "rr", 2), Some(3));
        assert_eq!(typeahead(&labels, "s", 1), Some(0));
        // A word narrows, and may stay on the item it is on.
        assert_eq!(typeahead(&labels, "sto", 1), Some(1));
        assert_eq!(typeahead(&labels, "REM", 0), Some(3));
        assert_eq!(typeahead(&labels, "x", 0), None);
        assert_eq!(typeahead(&[], "s", 0), None);
    }
}
