//! What a dialog stands on: a scrim over the page, the dialog in the middle,
//! and the keyboard to itself, as Radix's `Dialog` and `AlertDialog`:
//!
//! - Tab and Shift+Tab stay inside (the focus trap);
//! - the first thing that takes focus gets it when the dialog opens: the
//!   field of a form, Cancel in a confirmation;
//! - Escape asks to close, and so does a press on the scrim when the dialog
//!   says so (a confirmation doesn't);
//! - nothing under it hears a key or a press.
//!
//! What opened the dialog keeps its focus underneath, so focus is back
//! where it was once the dialog is gone. [`over`] always builds the same
//! shape, a stack with the page first, so the page keeps its state (what is
//! scrolled, what has focus) when a dialog comes and goes.

use iced::{
    advanced::{
        layout, mouse, overlay, renderer,
        widget::{tree, Operation, Tree},
        Clipboard, Layout, Renderer as _, Shell, Widget,
    },
    keyboard::{self, key::Named, Key},
    widget::Stack,
    Event, Length, Point, Rectangle, Size, Vector,
};

use crate::ui::{
    theme::{Element, Theme},
    widgets::focus::{self, Move},
};

/// `sm:max-w-sm`: a dialog's width unless it asks for another.
pub const WIDTH: f32 = 384.0;
/// `sm:max-w-2xl`: a dialog that shows machine text.
pub const WIDTH_WIDE: f32 = 672.0;

/// `max-w-[calc(100%-2rem)]`: kept free around a dialog in a small window.
const MARGIN: f32 = 16.0;

/// A dialog over the page.
pub struct Modal<'a, Message> {
    dialog: Element<'a, Message>,
    on_close: Option<Message>,
    width: f32,
    close_on_scrim: bool,
    key: u64,
}

/// `dialog` as a modal dialog. Stack it over the page with [`over`].
pub fn modal<'a, Message>(dialog: impl Into<Element<'a, Message>>) -> Modal<'a, Message> {
    Modal {
        dialog: dialog.into(),
        on_close: None,
        width: WIDTH,
        close_on_scrim: false,
        key: 0,
    }
}

/// `page` with `modal` over it while there is one.
pub fn over<'a, Message: Clone + 'a>(
    page: impl Into<Element<'a, Message>>,
    modal: Option<Modal<'a, Message>>,
) -> Element<'a, Message> {
    // The same stack with or without a dialog: iced keeps the state of a
    // widget only while the tree around it keeps its shape.
    let layers = std::iter::once(page.into()).chain(modal.map(Into::into));
    Stack::with_children(layers)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

impl<'a, Message: 'a> Modal<'a, Message> {
    /// The same dialog for a page whose messages wrap this one's.
    pub fn map<Outer: 'a>(self, wrap: impl Fn(Message) -> Outer + 'a) -> Modal<'a, Outer> {
        Modal {
            on_close: self.on_close.map(&wrap),
            dialog: self.dialog.map(wrap),
            width: self.width,
            close_on_scrim: self.close_on_scrim,
            key: self.key,
        }
    }

    /// Says which dialog this is. When another takes its place without the
    /// page showing in between (a confirmation, then the Details of what it
    /// failed with), the new one is given focus as one that just opened.
    #[must_use]
    pub fn key(mut self, key: &str) -> Self {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        key.hash(&mut hasher);
        self.key = hasher.finish();
        self
    }

    /// Sent on Escape. Leave it out while the dialog must stay (it is busy).
    #[must_use]
    pub fn on_close(mut self, message: Message) -> Self {
        self.on_close = Some(message);
        self
    }

    #[must_use]
    pub fn on_close_maybe(mut self, message: Option<Message>) -> Self {
        self.on_close = message;
        self
    }

    /// The most the dialog may be wide: [`WIDTH`] unless told.
    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// A press on the scrim closes it too: a `Dialog`, not an `AlertDialog`.
    #[must_use]
    pub fn close_on_scrim(mut self) -> Self {
        self.close_on_scrim = true;
        self
    }
}

#[derive(Debug, Default)]
struct State {
    /// Whether the dialog was given focus already.
    settled: bool,
    /// The dialog that was: see [`Modal::key`].
    key: u64,
}

impl<Message: Clone> Widget<Message, Theme, iced::Renderer> for Modal<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.dialog)]
    }

    fn diff(&self, tree: &mut Tree) {
        let state = tree.state.downcast_mut::<State>();
        if state.key != self.key {
            // Another dialog in the same place: nothing of the last one's
            // state is its own, and it has yet to be given focus.
            state.key = self.key;
            state.settled = false;
            tree.children = self.children();
        }
        tree.diff_children(std::slice::from_ref(&self.dialog));
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fill)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let window = limits.max();
        let room = Size::new(
            (window.width - 2.0 * MARGIN).clamp(0.0, self.width),
            (window.height - 2.0 * MARGIN).max(0.0),
        );
        // As wide as it may be: a dialog doesn't shrink around a short title.
        let limits = layout::Limits::new(Size::new(room.width, 0.0), room);
        let dialog = self
            .dialog
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, &limits);
        let size = dialog.size();
        let at = Point::new(
            ((window.width - size.width) / 2.0).round(),
            ((window.height - size.height) / 2.0).round(),
        );
        layout::Node::with_children(window, vec![dialog.move_to(at)])
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        if let Some(dialog) = layout.children().next() {
            self.dialog
                .as_widget_mut()
                .operate(&mut tree.children[0], dialog, renderer, operation);
        }
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
        let Some(dialog) = layout.children().next() else {
            return;
        };
        focus::modality::note(event);
        let content = self.dialog.as_widget_mut();
        let inner = &mut tree.children[0];
        let state = tree.state.downcast_mut::<State>();
        state.key = self.key;
        if !state.settled {
            state.settled = true;
            if focus::find(content, inner, dialog, renderer)
                .focused
                .is_none()
            {
                focus::move_within(content, inner, dialog, renderer, Move::First, true);
                shell.request_redraw();
            }
        }
        if let Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) = event {
            match key {
                Key::Named(Named::Escape) => {
                    if let Some(message) = &self.on_close {
                        shell.publish(message.clone());
                    }
                    shell.capture_event();
                    return;
                }
                Key::Named(Named::Tab) => {
                    let to = if modifiers.shift() {
                        Move::Previous
                    } else {
                        Move::Next
                    };
                    focus::move_within(content, inner, dialog, renderer, to, true);
                    shell.capture_event();
                    shell.request_redraw();
                    return;
                }
                _ => {}
            }
        }
        content.update(
            inner, event, dialog, cursor, renderer, clipboard, shell, viewport,
        );
        let outside = matches!(event, Event::Mouse(mouse::Event::ButtonPressed(_)))
            && !cursor.is_over(dialog.bounds());
        if let (true, true, false, Some(message)) = (
            outside,
            self.close_on_scrim,
            shell.is_event_captured(),
            &self.on_close,
        ) {
            shell.publish(message.clone());
        }
        // The page underneath hears nothing while the dialog is up.
        if matches!(
            event,
            Event::Keyboard(_) | Event::Mouse(_) | Event::Touch(_)
        ) {
            shell.capture_event();
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
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                ..renderer::Quad::default()
            },
            theme.tokens().scrim,
        );
        if let Some(dialog) = layout.children().next() {
            self.dialog.as_widget().draw(
                &tree.children[0],
                renderer,
                theme,
                style,
                dialog,
                cursor,
                viewport,
            );
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        let inside = layout.children().next().map(|dialog| {
            self.dialog.as_widget().mouse_interaction(
                &tree.children[0],
                dialog,
                cursor,
                viewport,
                renderer,
            )
        });
        // Never `None`: the pointer is on the dialog's layer, not the page's.
        match inside {
            Some(mouse::Interaction::None) | None => mouse::Interaction::Idle,
            Some(other) => other,
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &iced::Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, iced::Renderer>> {
        self.dialog.as_widget_mut().overlay(
            &mut tree.children[0],
            layout.children().next()?,
            renderer,
            viewport,
            translation,
        )
    }
}

impl<'a, Message: Clone + 'a> From<Modal<'a, Message>> for Element<'a, Message> {
    fn from(modal: Modal<'a, Message>) -> Self {
        Element::new(modal)
    }
}
