//! Keyboard focus. iced 0.14 gives focus to fields only and moves it
//! nowhere by itself, so the kit does both: [`pressable`] is the button that
//! takes focus (Enter and Space press it), [`roving`] moves focus between
//! the items of a group with the arrow keys, Home, End and by typing, as
//! Radix does, and [`traverse`] is what Tab does in the window.
//!
//! Dialogs and popovers keep Tab inside themselves (`overlay`), with the
//! same [`move_within`].

pub mod keys;
pub mod modality;
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

/// Where each widget that takes focus is, and its name, in tree order.
struct Spots(Vec<(Option<Id>, Rectangle)>);

impl Operation for Spots {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        operate(self);
    }

    fn focusable(&mut self, id: Option<&Id>, bounds: Rectangle, _state: &mut dyn Focusable) {
        self.0.push((id.cloned(), bounds));
    }
}

/// What takes focus first under `widget` (a dialog that opens): the one
/// named `first` when there is one, else the first in the order the
/// language reads: of the topmost row, the one at its start (the right
/// in Arabic). Tree order would give a mirrored row's end, as `line`
/// lays a right-to-left row out from the left.
pub fn first_index<Message>(
    widget: &mut dyn Widget<Message, Theme, iced::Renderer>,
    tree: &mut Tree,
    layout: Layout<'_>,
    renderer: &iced::Renderer,
    first: Option<&Id>,
    rtl: bool,
) -> Option<usize> {
    let mut spots = Spots(vec![]);
    widget.operate(tree, layout, renderer, &mut spots);
    let spots = spots.0;
    if let Some(at) =
        first.and_then(|first| spots.iter().position(|(id, _)| id.as_ref() == Some(first)))
    {
        return Some(at);
    }
    reading_first(spots.iter().map(|(_, bounds)| *bounds), rtl)
}

/// Of `bounds`, the first in reading order: the topmost row (those that
/// start above the middle of the topmost), then its start.
pub fn reading_first(bounds: impl Iterator<Item = Rectangle> + Clone, rtl: bool) -> Option<usize> {
    let top = bounds.clone().min_by(|a, b| a.y.total_cmp(&b.y))?;
    let row = top.y + top.height / 2.0;
    let edge = |b: &Rectangle| if rtl { -(b.x + b.width) } else { b.x };
    bounds
        .enumerate()
        .filter(|(_, b)| b.y < row)
        .min_by(|(_, a), (_, b)| edge(a).total_cmp(&edge(b)))
        .map(|(at, _)| at)
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

/// Looks for a button that shows its focus ring.
pub struct Shown(pub bool);

impl Operation for Shown {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        operate(self);
    }

    fn custom(&mut self, _id: Option<&Id>, _bounds: Rectangle, state: &mut dyn std::any::Any) {
        if let Some(state) = state.downcast_ref::<pressable::State>() {
            self.0 |= state.is_focus_visible();
        }
    }
}

/// Whether a button under `widget` has focus the keyboard gave it: what a
/// tooltip shows for.
pub fn is_visible_within<Message>(
    widget: &mut dyn Widget<Message, Theme, iced::Renderer>,
    tree: &mut Tree,
    layout: Layout<'_>,
    renderer: &iced::Renderer,
) -> bool {
    let mut shown = Shown(false);
    widget.operate(tree, layout, renderer, &mut shown);
    shown.0
}

/// Looks for a text field that has focus: a field reports itself both as
/// something that takes focus and as a text input, at the same place.
#[derive(Default)]
struct Typing {
    focused: Option<Rectangle>,
    fields: Vec<Rectangle>,
}

impl Operation for Typing {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        operate(self);
    }

    fn focusable(&mut self, _id: Option<&Id>, bounds: Rectangle, state: &mut dyn Focusable) {
        if state.is_focused() {
            self.focused = Some(bounds);
        }
    }

    fn text_input(
        &mut self,
        _id: Option<&Id>,
        bounds: Rectangle,
        _state: &mut dyn iced::advanced::widget::operation::TextInput,
    ) {
        self.fields.push(bounds);
    }
}

/// Whether a text field under `widget` has focus: what is typed belongs to
/// it, and so do the keys it leaves alone (an arrow with Alt).
pub fn is_typing_within<Message>(
    widget: &mut dyn Widget<Message, Theme, iced::Renderer>,
    tree: &mut Tree,
    layout: Layout<'_>,
    renderer: &iced::Renderer,
) -> bool {
    let mut typing = Typing::default();
    widget.operate(tree, layout, renderer, &mut typing);
    typing
        .focused
        .is_some_and(|focused| typing.fields.contains(&focused))
}

/// What a widget that needs every key (the terminal) reports of itself
/// with `Operation::custom`: whether it has focus. The shell's shortcuts
/// keep out of it while it does.
pub struct EveryKey(pub bool);

struct KeysKept(bool);

impl Operation for KeysKept {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        operate(self);
    }

    fn custom(&mut self, _id: Option<&Id>, _bounds: Rectangle, state: &mut dyn std::any::Any) {
        if let Some(EveryKey(focused)) = state.downcast_ref::<EveryKey>() {
            self.0 |= *focused;
        }
    }
}

/// Whether a widget under `widget` that needs every key has focus: a key
/// it used is the program's in it, and no shortcut's.
pub fn is_every_key_within<Message>(
    widget: &mut dyn Widget<Message, Theme, iced::Renderer>,
    tree: &mut Tree,
    layout: Layout<'_>,
    renderer: &iced::Renderer,
) -> bool {
    let mut kept = KeysKept(false);
    widget.operate(tree, layout, renderer, &mut kept);
    kept.0
}

struct Cover(bool);

impl Operation for Cover {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        operate(self);
    }

    fn custom(&mut self, _id: Option<&Id>, _bounds: Rectangle, state: &mut dyn std::any::Any) {
        if let Some(state) = state.downcast_mut::<pressable::State>() {
            state.cover(self.0);
        }
    }
}

/// Hides (or shows again) the focus ring of the buttons under `widget`:
/// while the panel one of them opened has the keyboard, the ring is in the
/// panel, and one ring shows at a time.
pub fn cover_within<Message>(
    widget: &mut dyn Widget<Message, Theme, iced::Renderer>,
    tree: &mut Tree,
    layout: Layout<'_>,
    renderer: &iced::Renderer,
    covered: bool,
) {
    widget.operate(tree, layout, renderer, &mut Cover(covered));
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

/// Tab and Shift+Tab (`true`) that nothing captured, for the window to
/// answer with [`traverse`]: a dialog or an open panel captures its own.
pub fn tab_presses() -> iced::Subscription<bool> {
    use iced::keyboard::{self, key::Named, Key};
    iced::event::listen_with(|event, status, _window| match (event, status) {
        (
            iced::Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(Named::Tab),
                modifiers,
                ..
            }),
            iced::event::Status::Ignored,
        ) => Some(modifiers.shift()),
        _ => None,
    })
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
