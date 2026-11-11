//! The kit's button: iced's, which also takes keyboard focus. Tab reaches
//! it, Enter and Space press it, and it shows a ring while it has focus.
//! Every button, menu item, tab and option of the kit is one.

use std::rc::Rc;

use iced::{
    advanced::{
        layout, mouse, overlay, renderer,
        widget::{operation::Focusable, tree, Id, Operation, Tree},
        Clipboard, Layout, Renderer as _, Shell, Widget,
    },
    keyboard::{self, key::Named, Key, Modifiers},
    widget::button::{self, Button},
    Border, Color, Event, Length, Padding, Rectangle, Size, Vector,
};

use crate::ui::theme::{self, color::alpha, Element, Theme};

type Style = Rc<dyn Fn(&Theme, button::Status) -> button::Style>;
type OnKey<'a, Message> = Box<dyn Fn(&Key, Modifiers) -> Option<Message> + 'a>;

/// How far outside the button its focus ring is drawn, and how thick.
const RING: f32 = 2.0;

/// A button that takes keyboard focus.
pub struct Pressable<'a, Message> {
    button: Button<'a, Message, Theme, iced::Renderer>,
    id: Option<Id>,
    on_press: Option<Message>,
    before: Option<Message>,
    on_key: Option<OnKey<'a, Message>>,
    style: Style,
}

/// A [`Pressable`] around `content`: the primary button until
/// [`Pressable::style`] says otherwise, and disabled until
/// [`Pressable::on_press`] gives it something to do.
pub fn pressable<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
) -> Pressable<'a, Message> {
    Pressable {
        button: Button::new(content),
        id: None,
        on_press: None,
        before: None,
        on_key: None,
        style: Rc::new(theme::button::primary),
    }
}

impl<'a, Message: Clone + 'a> Pressable<'a, Message> {
    #[must_use]
    pub fn on_press(self, message: Message) -> Self {
        self.on_press_maybe(Some(message))
    }

    /// Disabled when `None`: it takes no focus and no press.
    #[must_use]
    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.button = self.button.on_press_maybe(message.clone());
        self.on_press = message;
        self
    }

    /// Sent before the press's own message: a menu item closes its menu,
    /// then does what it is for.
    #[must_use]
    pub fn before(mut self, message: Message) -> Self {
        self.before = Some(message);
        self
    }

    /// Other keys while it has focus: an arrow that opens a submenu, Delete
    /// on a row. The key is captured when the handler returns a message.
    #[must_use]
    pub fn on_key(mut self, handler: impl Fn(&Key, Modifiers) -> Option<Message> + 'a) -> Self {
        self.on_key = Some(Box::new(handler));
        self
    }

    /// One of `theme::button`'s variants.
    #[must_use]
    pub fn style(
        mut self,
        style: impl Fn(&Theme, button::Status) -> button::Style + 'static,
    ) -> Self {
        let style: Style = Rc::new(style);
        let drawn = style.clone();
        self.button = self.button.style(move |theme, status| drawn(theme, status));
        self.style = style;
        self
    }

    /// Names it, so focus can be sent to it (`iced::widget::operation::focus`).
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    #[must_use]
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.button = self.button.padding(padding);
        self
    }

    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.button = self.button.width(width);
        self
    }

    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.button = self.button.height(height);
        self
    }

    fn inner(&self) -> &dyn Widget<Message, Theme, iced::Renderer> {
        &self.button
    }

    fn inner_mut(&mut self) -> &mut dyn Widget<Message, Theme, iced::Renderer> {
        &mut self.button
    }

    fn press(&self, shell: &mut Shell<'_, Message>) {
        if let Some(message) = &self.on_press {
            if let Some(before) = &self.before {
                shell.publish(before.clone());
            }
            shell.publish(message.clone());
        }
    }
}

#[derive(Debug, Default)]
struct State {
    focused: bool,
}

impl Focusable for State {
    fn is_focused(&self) -> bool {
        self.focused
    }

    fn focus(&mut self) {
        self.focused = true;
    }

    fn unfocus(&mut self) {
        self.focused = false;
    }
}

impl<'a, Message: Clone + 'a> Widget<Message, Theme, iced::Renderer> for Pressable<'a, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(self.inner())]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[self.inner()]);
    }

    fn size(&self) -> Size<Length> {
        self.inner().size()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.inner_mut()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        if self.on_press.is_some() {
            let state = tree.state.downcast_mut::<State>();
            operation.focusable(self.id.as_ref(), layout.bounds(), state);
        }
        self.inner_mut()
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
        let enabled = self.on_press.is_some();
        let was = tree.state.downcast_ref::<State>().focused;
        if !enabled && was {
            tree.state.downcast_mut::<State>().focused = false;
        }
        // A press with the pointer is the button's own; here it only
        // decides who has focus, as a click does everywhere.
        if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event {
            let over = cursor.is_over(layout.bounds());
            tree.state.downcast_mut::<State>().focused = enabled && over;
        }
        if let (true, Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. })) =
            (enabled && was, event)
        {
            let pressed = matches!(key, Key::Named(Named::Enter | Named::Space))
                && !modifiers.command()
                && !modifiers.alt();
            if pressed {
                self.press(shell);
                shell.capture_event();
                return;
            }
            if let Some(message) = self.on_key.as_ref().and_then(|on| on(key, *modifiers)) {
                shell.publish(message);
                shell.capture_event();
                return;
            }
        }
        if tree.state.downcast_ref::<State>().focused != was {
            shell.request_redraw();
        }
        // The button sends the press itself; what goes before it goes first.
        let released = matches!(
            event,
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
        );
        let mut own = vec![];
        let mut inner = Shell::new(&mut own);
        self.inner_mut().update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            &mut inner,
            viewport,
        );
        if let (true, false, Some(before)) = (released, inner.is_empty(), &self.before) {
            shell.publish(before.clone());
        }
        shell.merge(inner, |message| message);
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
        self.inner().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
        if tree.state.downcast_ref::<State>().focused {
            // `focus-visible:ring-[3px] ring-ring/50`, drawn outside so it
            // shows around a button of any colour.
            let corners = (self.style)(theme, button::Status::Active).border.radius;
            renderer.fill_quad(
                renderer::Quad {
                    bounds: layout.bounds().expand(RING),
                    border: Border {
                        color: alpha(theme.tokens().ring, 0.5),
                        width: RING,
                        radius: corners,
                    },
                    ..renderer::Quad::default()
                },
                Color::TRANSPARENT,
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
        self.inner()
            .mouse_interaction(&tree.children[0], layout, cursor, viewport, renderer)
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &iced::Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, iced::Renderer>> {
        Widget::overlay(
            &mut self.button,
            &mut tree.children[0],
            layout,
            renderer,
            viewport,
            translation,
        )
    }
}

impl<'a, Message: Clone + 'a> From<Pressable<'a, Message>> for Element<'a, Message> {
    fn from(pressable: Pressable<'a, Message>) -> Self {
        Element::new(pressable)
    }
}
