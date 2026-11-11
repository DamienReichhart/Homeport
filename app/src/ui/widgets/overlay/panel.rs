//! The panel of an [`Anchored`](super::anchored::Anchored) itself: placed
//! beside its anchor, with the keyboard and, when modal, the pointer to
//! itself.

use std::cell::Cell;

use iced::{
    advanced::{
        layout, mouse, overlay, renderer,
        widget::{Operation, Tree},
        Clipboard, Layout, Shell,
    },
    keyboard::{self, key::Named, Key},
    Event, Rectangle, Size, Vector,
};

use super::{
    anchored::State,
    position::{place, Placement, Side},
};
use crate::ui::{
    theme::{Element, Theme},
    widgets::focus::{self, Move},
};

thread_local! {
    /// The side the panel being asked for its own overlays ended up on,
    /// when it sits beside its anchor: a submenu of a submenu goes on the
    /// same way, not back over the menu it came from.
    static FLOW: Cell<Option<Side>> = const { Cell::new(None) };
}

/// The side the panel around what is being drawn went, if it is beside its
/// anchor.
pub(super) fn flow() -> Option<Side> {
    FLOW.get()
}

/// The floating panel itself.
pub(super) struct Panel<'a, 'b, Message> {
    pub(super) content: &'b mut Element<'a, Message>,
    pub(super) tree: &'b mut Tree,
    pub(super) state: &'b mut State,
    pub(super) anchor: Rectangle,
    pub(super) placement: Placement,
    pub(super) on_dismiss: Option<&'b Message>,
    pub(super) on_escape: Option<&'b Message>,
    pub(super) modal: bool,
    pub(super) match_width: bool,
    pub(super) take_focus: Option<usize>,
    pub(super) interactive: bool,
    pub(super) viewport: Rectangle,
}

impl<Message: Clone> Panel<'_, '_, Message> {
    fn dismiss(&self, shell: &mut Shell<'_, Message>) {
        if let Some(message) = self.on_dismiss {
            shell.publish(message.clone());
        }
    }
}

impl<Message: Clone> overlay::Overlay<Message, Theme, iced::Renderer> for Panel<'_, '_, Message> {
    fn layout(&mut self, renderer: &iced::Renderer, bounds: Size) -> layout::Node {
        let margin = self.placement.margin * 2.0;
        let max = Size::new(
            (bounds.width - margin).max(0.0),
            (bounds.height - margin).max(0.0),
        );
        let content = self.content.as_widget_mut();
        let limits = if self.match_width {
            // Once to learn how wide the panel wants to be, then at that
            // width or its anchor's, whichever is wider. A panel whose root
            // passes the least width on (`basic::fit`) fills it.
            let measure = layout::Limits::with_compression(Size::ZERO, max, Size::new(true, false));
            let wanted = content.layout(self.tree, renderer, &measure).size().width;
            let width = wanted.max(self.anchor.width).min(max.width);
            layout::Limits::new(Size::new(width, 0.0), Size::new(width, max.height))
        } else {
            layout::Limits::new(Size::ZERO, max)
        };
        let panel = content.layout(self.tree, renderer, &limits);
        let (at, _) = place(self.anchor, panel.size(), bounds, self.placement);
        layout::Node::with_children(panel.size(), vec![panel]).move_to(at)
    }

    fn update(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) {
        if !self.interactive {
            return;
        }
        let Some(panel) = layout.children().next() else {
            return;
        };
        let content = self.content.as_widget_mut();
        if !self.state.settled {
            self.state.settled = true;
            let found = focus::find(content, self.tree, panel, renderer);
            if let (Some(index), None) = (self.take_focus, found.focused) {
                focus::move_within(
                    content,
                    self.tree,
                    panel,
                    renderer,
                    Move::Index(index),
                    true,
                );
                shell.request_redraw();
            }
        }
        let pressed = match event {
            Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) => {
                Some((key, modifiers))
            }
            _ => None,
        };
        match pressed {
            Some((Key::Named(Named::Escape), _)) => {
                if let Some(message) = self.on_escape {
                    shell.publish(message.clone());
                }
                shell.capture_event();
                return;
            }
            Some((Key::Named(Named::Tab), modifiers)) => {
                let to = if modifiers.shift() {
                    Move::Previous
                } else {
                    Move::Next
                };
                focus::move_within(content, self.tree, panel, renderer, to, true);
                shell.capture_event();
                shell.request_redraw();
                return;
            }
            _ => {}
        }
        content.update(
            self.tree,
            event,
            panel,
            cursor,
            renderer,
            clipboard,
            shell,
            &self.viewport,
        );
        if pressed.is_some() {
            // The panel has the keyboard: nothing under it types along.
            shell.capture_event();
        }
        if let Event::Mouse(mouse::Event::ButtonPressed(_)) = event {
            if cursor.is_over(panel.bounds()) {
                shell.capture_event();
            } else if !cursor.is_over(self.anchor) && !shell.is_event_captured() {
                self.dismiss(shell);
                if self.modal {
                    // The press closed the panel; it presses nothing else.
                    shell.capture_event();
                }
            }
        }
    }

    fn draw(
        &self,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        if let Some(panel) = layout.children().next() {
            self.content.as_widget().draw(
                self.tree,
                renderer,
                theme,
                style,
                panel,
                cursor,
                &Rectangle::with_size(Size::INFINITE),
            );
        }
    }

    fn operate(
        &mut self,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        if let Some(panel) = layout.children().next() {
            self.content
                .as_widget_mut()
                .operate(self.tree, panel, renderer, operation);
        }
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        let Some(panel) = layout.children().next() else {
            return mouse::Interaction::None;
        };
        if !self.interactive {
            return mouse::Interaction::None;
        }
        if !cursor.is_over(panel.bounds()) {
            // A modal panel has the pointer wherever it is, but for the
            // anchor, which closes it: nothing under it shows a hover.
            return if self.modal && !cursor.is_over(self.anchor) {
                mouse::Interaction::Idle
            } else {
                mouse::Interaction::None
            };
        }
        // Never `None` over the panel: that is how iced knows the pointer
        // is on it and not on the page underneath.
        match self.content.as_widget().mouse_interaction(
            self.tree,
            panel,
            cursor,
            &self.viewport,
            renderer,
        ) {
            mouse::Interaction::None => mouse::Interaction::Idle,
            other => other,
        }
    }

    fn overlay<'c>(
        &'c mut self,
        layout: Layout<'c>,
        renderer: &iced::Renderer,
    ) -> Option<overlay::Element<'c, Message, Theme, iced::Renderer>> {
        let panel = layout.children().next()?;
        // The side this panel is on, for the panels it opens in turn.
        let side = self.placement.side_of(self.anchor, panel.bounds());
        let outer = FLOW.replace(side);
        let overlay = self.content.as_widget_mut().overlay(
            self.tree,
            panel,
            renderer,
            &self.viewport,
            Vector::ZERO,
        );
        FLOW.set(outer);
        overlay
    }
}
