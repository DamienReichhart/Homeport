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
//! where it was once the dialog is gone.

use iced::{
    advanced::{
        layout, mouse, overlay, renderer,
        widget::{tree, Operation, Tree},
        Clipboard, Layout, Renderer as _, Shell, Widget,
    },
    keyboard::{self, key::Named, Key},
    widget::Stack,
    Color, Event, Length, Point, Rectangle, Size, Vector,
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

/// The page behind is dimmed. The React app dims by a tenth and blurs; iced
/// has no blur, so it dims a little more to set the dialog apart.
const SCRIM: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.25);

/// A dialog over the page.
pub struct Modal<'a, Message> {
    dialog: Element<'a, Message>,
    on_close: Option<Message>,
    width: f32,
    close_on_scrim: bool,
}

/// `dialog` as a modal dialog. Stack it over the page with [`over`].
pub fn modal<'a, Message>(dialog: impl Into<Element<'a, Message>>) -> Modal<'a, Message> {
    Modal {
        dialog: dialog.into(),
        on_close: None,
        width: WIDTH,
        close_on_scrim: false,
    }
}

/// `page` with `modal` over it while there is one.
pub fn over<'a, Message: Clone + 'a>(
    page: impl Into<Element<'a, Message>>,
    modal: Option<Modal<'a, Message>>,
) -> Element<'a, Message> {
    let page = page.into();
    match modal {
        Some(modal) => Stack::with_children([page, modal.into()])
            .width(Length::Fill)
            .height(Length::Fill)
            .into(),
        None => page,
    }
}

impl<Message> Modal<'_, Message> {
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
        let content = self.dialog.as_widget_mut();
        let inner = &mut tree.children[0];
        let state = tree.state.downcast_mut::<State>();
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
            SCRIM,
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
