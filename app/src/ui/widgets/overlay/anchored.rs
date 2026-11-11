//! A panel that floats beside what it belongs to: the one overlay under
//! every popover, menu, select list and tooltip. It is drawn over the whole
//! window (nothing clips it), placed by [`super::position::place`], and
//! behaves as Radix's do:
//!
//! - open (the page says when): Escape and a press outside ask to dismiss
//!   it, Tab stays inside, the first thing that takes focus gets it, and
//!   the keys go no further than the panel;
//! - on hover (a tooltip): it shows after a delay under the pointer, at
//!   once when what it describes takes keyboard focus, and takes no input.
//!
//! What opened it keeps its focus underneath, so closing the panel gives
//! focus back without anyone asking.

use std::time::{Duration, Instant};

use iced::{
    advanced::{
        layout, mouse, overlay, renderer,
        widget::{tree, Operation, Tree},
        Clipboard, Layout, Shell, Widget,
    },
    keyboard::{self, key::Named, Key},
    window, Event, Length, Rectangle, Size, Vector,
};

use super::position::{place, Placement};
use crate::ui::{
    theme::{Element, Theme},
    widgets::focus::{self, Move},
};

/// When the panel shows.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Show {
    /// While the page gives one.
    Open,
    /// While the pointer is over the anchor (after `delay`) or the anchor
    /// has keyboard focus.
    Hover { delay: Duration },
}

/// An anchor and the panel that floats beside it.
pub struct Anchored<'a, Message> {
    anchor: Element<'a, Message>,
    panel: Option<Element<'a, Message>>,
    placement: Placement,
    show: Show,
    on_dismiss: Option<Message>,
    match_width: bool,
    take_focus: bool,
}

/// `anchor`, with `panel` floating beside it while there is one. Give it
/// [`Anchored::on_dismiss`], or nothing but the page closes it.
pub fn anchored<'a, Message>(
    anchor: impl Into<Element<'a, Message>>,
    panel: Option<Element<'a, Message>>,
    placement: Placement,
) -> Anchored<'a, Message> {
    Anchored {
        anchor: anchor.into(),
        panel,
        placement,
        show: Show::Open,
        on_dismiss: None,
        match_width: false,
        take_focus: true,
    }
}

/// `anchor`, with `panel` shown while the pointer rests on it or it has
/// keyboard focus.
pub fn hovered<'a, Message>(
    anchor: impl Into<Element<'a, Message>>,
    panel: impl Into<Element<'a, Message>>,
    placement: Placement,
    delay: Duration,
) -> Anchored<'a, Message> {
    Anchored {
        show: Show::Hover { delay },
        ..anchored(anchor, Some(panel.into()), placement)
    }
}

impl<Message> Anchored<'_, Message> {
    /// Sent on Escape and on a press outside the panel and its anchor.
    #[must_use]
    pub fn on_dismiss(mut self, message: Message) -> Self {
        self.on_dismiss = Some(message);
        self
    }

    /// The panel is at least as wide as its anchor: a select's list.
    #[must_use]
    pub fn match_width(mut self) -> Self {
        self.match_width = true;
        self
    }

    /// Whether opening gives focus to the first thing in the panel that
    /// takes it. It does unless told.
    #[must_use]
    pub fn take_focus(mut self, take: bool) -> Self {
        self.take_focus = take;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum Hover {
    #[default]
    Idle,
    Waiting {
        since: Instant,
    },
    Shown,
}

#[derive(Debug, Default)]
struct State {
    hover: Hover,
    /// Whether the open panel was given focus already.
    settled: bool,
}

impl<Message: Clone> Widget<Message, Theme, iced::Renderer> for Anchored<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn children(&self) -> Vec<Tree> {
        let mut children = vec![Tree::new(&self.anchor)];
        children.extend(self.panel.as_ref().map(Tree::new));
        children
    }

    fn diff(&self, tree: &mut Tree) {
        match &self.panel {
            Some(panel) => tree.diff_children(&[&self.anchor, panel]),
            None => {
                tree.diff_children(std::slice::from_ref(&self.anchor));
                tree.state.downcast_mut::<State>().settled = false;
            }
        }
    }

    fn size(&self) -> Size<Length> {
        self.anchor.as_widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.anchor.as_widget().size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.anchor
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
        self.anchor
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
        self.anchor.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
        let Show::Hover { delay } = self.show else {
            return;
        };
        let focused = focus::find(
            self.anchor.as_widget_mut(),
            &mut tree.children[0],
            layout,
            renderer,
        )
        .focused
        .is_some();
        let wanted = focused || cursor.is_over(layout.bounds());
        let state = tree.state.downcast_mut::<State>();
        let now = Instant::now();
        let next = match (state.hover, wanted) {
            (Hover::Idle, true) if focused || delay.is_zero() => Hover::Shown,
            (Hover::Idle, true) => Hover::Waiting { since: now },
            (Hover::Waiting { since }, true) if focused || since.elapsed() >= delay => Hover::Shown,
            (_, false) => Hover::Idle,
            (same, true) => same,
        };
        if let Hover::Waiting { since } = next {
            shell.request_redraw_at(since + delay);
        }
        if next != state.hover {
            let appears = (next == Hover::Shown) != (state.hover == Hover::Shown);
            state.hover = next;
            if appears {
                shell.invalidate_layout();
                if !matches!(event, Event::Window(window::Event::RedrawRequested(_))) {
                    shell.request_redraw();
                }
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
        self.anchor.as_widget().draw(
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
        self.anchor.as_widget().mouse_interaction(
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
        let state = tree.state.downcast_mut::<State>();
        let mut children = tree.children.iter_mut();
        let own = self.anchor.as_widget_mut().overlay(
            children.next()?,
            layout,
            renderer,
            viewport,
            translation,
        );
        let interactive = self.show == Show::Open;
        let shown = interactive || state.hover == Hover::Shown;
        let panel = match (&mut self.panel, children.next()) {
            (Some(content), Some(tree)) if shown => Some(overlay::Element::new(Box::new(Panel {
                content,
                tree,
                state,
                anchor: layout.bounds() + translation,
                placement: self.placement,
                on_dismiss: self.on_dismiss.as_ref(),
                match_width: self.match_width,
                take_focus: self.take_focus,
                interactive,
                viewport: *viewport,
            }))),
            _ => None,
        };
        match (own, panel) {
            (None, None) => None,
            (own, panel) => Some(
                overlay::Group::with_children(own.into_iter().chain(panel).collect()).overlay(),
            ),
        }
    }
}

impl<'a, Message: Clone + 'a> From<Anchored<'a, Message>> for Element<'a, Message> {
    fn from(anchored: Anchored<'a, Message>) -> Self {
        Element::new(anchored)
    }
}

/// The floating panel itself.
struct Panel<'a, 'b, Message> {
    content: &'b mut Element<'a, Message>,
    tree: &'b mut Tree,
    state: &'b mut State,
    anchor: Rectangle,
    placement: Placement,
    on_dismiss: Option<&'b Message>,
    match_width: bool,
    take_focus: bool,
    interactive: bool,
    viewport: Rectangle,
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
        let min = if self.match_width {
            Size::new(self.anchor.width.min(max.width), 0.0)
        } else {
            Size::ZERO
        };
        let panel = self.content.as_widget_mut().layout(
            self.tree,
            renderer,
            &layout::Limits::new(min, max),
        );
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
            if self.take_focus && found.focused.is_none() {
                focus::move_within(content, self.tree, panel, renderer, Move::First, true);
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
                self.dismiss(shell);
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
                // Not captured: what was pressed outside gets its press.
                self.dismiss(shell);
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
        if !self.interactive || !cursor.is_over(panel.bounds()) {
            return mouse::Interaction::None;
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
        self.content.as_widget_mut().overlay(
            self.tree,
            layout.children().next()?,
            renderer,
            &self.viewport,
            Vector::ZERO,
        )
    }
}
