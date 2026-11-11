//! Keyboard focus. iced 0.14 gives focus to fields only and moves it
//! nowhere by itself, so the kit does both: [`pressable`] is the button that
//! takes focus (Enter and Space press it), [`roving`] moves focus between
//! the items of a group with the arrow keys, Home, End and by typing, as
//! Radix does, and [`traverse`] is what Tab does in the window.
//!
//! Dialogs and popovers keep Tab inside themselves (`overlay`), with the
//! same [`move_within`].

pub mod keys;
pub mod pressable;
pub mod roving;

use iced::{
    advanced::{
        widget::{operation::Focusable, Id, Operation, Tree},
        Layout, Widget,
    },
    widget::operation,
    Rectangle, Task,
};

use crate::ui::theme::Theme;

pub use self::{
    keys::{key_area, KeyArea},
    pressable::{pressable, Pressable},
    roving::{roving, Axis, Roving},
};

/// Where focus goes among the focusable widgets of a part of the window,
/// counted in the order they are laid out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    Next,
    Previous,
    First,
    Last,
    Index(usize),
}

/// How many widgets of a part of the window take focus, and which has it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Found {
    pub total: usize,
    pub focused: Option<usize>,
}

impl Found {
    /// The index `to` lands on, if any: `wrap` goes round at the ends
    /// (a dialog, a menu), otherwise focus stays at the end (a toolbar).
    pub fn target(self, to: Move, wrap: bool) -> Option<usize> {
        if self.total == 0 {
            return None;
        }
        let last = self.total - 1;
        let edge = |round: usize, stay: usize| if wrap { round } else { stay };
        Some(match (to, self.focused) {
            (Move::First, _) | (Move::Next, None) => 0,
            (Move::Last, _) | (Move::Previous, None) => last,
            (Move::Index(index), _) => index.min(last),
            (Move::Next, Some(at)) if at >= last => edge(0, last),
            (Move::Next, Some(at)) => at + 1,
            (Move::Previous, Some(0)) => edge(last, 0),
            (Move::Previous, Some(at)) => at - 1,
        })
    }
}

struct Locate(Found);

impl Operation for Locate {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        operate(self);
    }

    fn focusable(&mut self, _id: Option<&Id>, _bounds: Rectangle, state: &mut dyn Focusable) {
        if state.is_focused() {
            self.0.focused = Some(self.0.total);
        }
        self.0.total += 1;
    }
}

struct Give {
    to: Option<usize>,
    at: usize,
}

impl Operation for Give {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        operate(self);
    }

    fn focusable(&mut self, _id: Option<&Id>, _bounds: Rectangle, state: &mut dyn Focusable) {
        if Some(self.at) == self.to {
            state.focus();
        } else {
            state.unfocus();
        }
        self.at += 1;
    }
}

/// What takes focus under `widget`, and which of them has it.
pub fn find<Message>(
    widget: &mut dyn Widget<Message, Theme, iced::Renderer>,
    tree: &mut Tree,
    layout: Layout<'_>,
    renderer: &iced::Renderer,
) -> Found {
    let mut locate = Locate(Found::default());
    widget.operate(tree, layout, renderer, &mut locate);
    locate.0
}

/// Moves focus among what takes it under `widget`, and returns the index
/// that has it now. Nothing there takes focus: `None`.
pub fn move_within<Message>(
    widget: &mut dyn Widget<Message, Theme, iced::Renderer>,
    tree: &mut Tree,
    layout: Layout<'_>,
    renderer: &iced::Renderer,
    to: Move,
    wrap: bool,
) -> Option<usize> {
    let target = find(widget, tree, layout, renderer).target(to, wrap)?;
    let mut give = Give {
        to: Some(target),
        at: 0,
    };
    widget.operate(tree, layout, renderer, &mut give);
    Some(target)
}

/// Takes focus away from everything under `widget`.
pub fn clear_within<Message>(
    widget: &mut dyn Widget<Message, Theme, iced::Renderer>,
    tree: &mut Tree,
    layout: Layout<'_>,
    renderer: &iced::Renderer,
) {
    widget.operate(tree, layout, renderer, &mut Give { to: None, at: 0 });
}

/// What Tab (and Shift+Tab, `back`) does in the window: focus goes to the
/// next widget that takes it. The window sends this for a Tab nothing
/// captured; a dialog or a popover that is open captures it and keeps focus
/// inside.
pub fn traverse<Message: Send + 'static>(back: bool) -> Task<Message> {
    if back {
        operation::focus_previous()
    } else {
        operation::focus_next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focus_moves_in_order_and_wraps_only_when_asked() {
        let at = |focused| Found { total: 3, focused };
        assert_eq!(at(None).target(Move::Next, true), Some(0));
        assert_eq!(at(None).target(Move::Previous, true), Some(2));
        assert_eq!(at(Some(0)).target(Move::Next, false), Some(1));
        assert_eq!(at(Some(2)).target(Move::Next, true), Some(0));
        assert_eq!(at(Some(2)).target(Move::Next, false), Some(2));
        assert_eq!(at(Some(0)).target(Move::Previous, true), Some(2));
        assert_eq!(at(Some(0)).target(Move::Previous, false), Some(0));
        assert_eq!(at(Some(1)).target(Move::First, false), Some(0));
        assert_eq!(at(Some(1)).target(Move::Last, false), Some(2));
        assert_eq!(at(Some(1)).target(Move::Index(9), false), Some(2));
        assert_eq!(Found::default().target(Move::Next, true), None);
    }
}
