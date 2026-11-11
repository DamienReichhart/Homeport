//! A panel that floats beside what it belongs to: the one overlay under
//! every popover, menu, select list and tooltip. It is drawn over the whole
//! window (nothing clips it), placed by [`super::position::place`], and
//! behaves as Radix's do:
//!
//! - open (the page says when): Escape and a press outside ask to dismiss
//!   it, Tab stays inside, the first thing that takes focus gets it, and
//!   the keys go no further than the panel;
//! - open and [`Anchored::modal`] (a menu, a select's list): a press
//!   outside only dismisses it; what was under the pointer hears nothing;
//! - on hover (a tooltip): it shows after a delay under the pointer, at
//!   once when the keyboard gives focus to what it describes, goes when
//!   anything is pressed or Escape is, and takes no input.
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

use super::{
    panel::{flow, Panel},
    position::Placement,
};
use crate::ui::{
    theme::{Element, Theme},
    widgets::focus,
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
    on_escape: Option<Message>,
    modal: bool,
    match_width: bool,
    take_focus: Option<usize>,
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
        on_escape: None,
        modal: false,
        match_width: false,
        take_focus: Some(0),
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

    /// Sent on Escape in place of [`Anchored::on_dismiss`]'s message: a
    /// submenu closes alone on a press outside, the whole menu on Escape.
    #[must_use]
    pub fn on_escape(mut self, message: Message) -> Self {
        self.on_escape = Some(message);
        self
    }

    /// A press outside dismisses the panel and does nothing else, as with
    /// Radix's menus and selects: what was pressed doesn't hear it.
    #[must_use]
    pub fn modal(mut self) -> Self {
        self.modal = true;
        self
    }

    /// The panel is at least as wide as its anchor: a select's list.
    #[must_use]
    pub fn match_width(mut self) -> Self {
        self.match_width = true;
        self
    }

    /// What takes focus as the panel opens, counted among what can in it:
    /// the first unless told (a select starts on the chosen option), and
    /// `None` leaves focus where it is.
    #[must_use]
    pub fn focus_on_open(mut self, index: Option<usize>) -> Self {
        self.take_focus = index;
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
pub(super) struct State {
    hover: Hover,
    /// Whether the open panel was given focus already.
    pub(super) settled: bool,
    /// Whether the anchor's focus ring is hidden under the open panel.
    covering: bool,
    /// Whether the anchor had focus at the last event, and whether the
    /// keyboard gave it.
    focused: bool,
    by_keyboard: bool,
    /// A press or Escape sent the tooltip away until the pointer leaves
    /// and focus moves.
    dismissed: bool,
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
        focus::modality::note(event);
        let Show::Hover { delay } = self.show else {
            // One ring at a time: the anchor's hides while its panel is open.
            let open = self.panel.is_some();
            if open != tree.state.downcast_ref::<State>().covering {
                tree.state.downcast_mut::<State>().covering = open;
                focus::cover_within(
                    self.anchor.as_widget_mut(),
                    &mut tree.children[0],
                    layout,
                    renderer,
                    open,
                );
                shell.request_redraw();
            }
            return;
        };
        let anchor = self.anchor.as_widget_mut();
        let has_focus = focus::find(anchor, &mut tree.children[0], layout, renderer)
            .focused
            .is_some();
        let ring = focus::is_visible_within(anchor, &mut tree.children[0], layout, renderer);
        let over = cursor.is_over(layout.bounds());
        let state = tree.state.downcast_mut::<State>();
        if has_focus && !state.focused {
            // Focus just came: the tooltip is for the keyboard's.
            state.by_keyboard = ring || focus::modality::keyboard();
        }
        state.focused = has_focus;
        let escape = matches!(
            event,
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(Named::Escape),
                ..
            })
        );
        if focus::modality::is_pointer_down(event) || escape {
            state.dismissed = true;
            state.by_keyboard = false;
        } else if !over && !has_focus {
            state.dismissed = false;
        }
        let focused = has_focus && state.by_keyboard;
        let wanted = focused || (over && !state.dismissed);
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
                placement: self.placement.following(flow()),
                on_dismiss: self.on_dismiss.as_ref(),
                on_escape: self.on_escape.as_ref().or(self.on_dismiss.as_ref()),
                modal: self.modal,
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
