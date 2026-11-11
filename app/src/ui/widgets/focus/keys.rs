//! Keys for a part of the window: what is inside sees a key first, and one
//! it leaves alone goes to the handler. A search field's arrows move the
//! list under it this way.

use iced::{
    advanced::{
        layout, mouse, overlay, renderer,
        widget::{Operation, Tree},
        Clipboard, Layout, Shell, Widget,
    },
    keyboard::{self, Key, Modifiers},
    Event, Length, Rectangle, Size, Vector,
};

use crate::ui::theme::{Element, Theme};

type OnKey<'a, Message> = Box<dyn Fn(&Key, Modifiers) -> Option<Message> + 'a>;

/// `content`, with a handler for the keys it doesn't use.
pub struct KeyArea<'a, Message> {
    content: Element<'a, Message>,
    on_key: OnKey<'a, Message>,
    always: bool,
}

/// Sends what `on_key` returns for a key pressed while something inside
/// `content` has focus; the key is captured when it returns a message.
pub fn key_area<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    on_key: impl Fn(&Key, Modifiers) -> Option<Message> + 'a,
) -> KeyArea<'a, Message> {
    KeyArea {
        content: content.into(),
        on_key: Box::new(on_key),
        always: false,
    }
}

impl<Message> KeyArea<'_, Message> {
    /// Also when nothing inside has focus: the keys of a whole page.
    #[must_use]
    pub fn always(mut self) -> Self {
        self.always = true;
        self
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for KeyArea<'_, Message> {
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
        let Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) = event else {
            return;
        };
        if shell.is_event_captured() {
            return;
        }
        let inside = self.always
            || super::find(
                self.content.as_widget_mut(),
                &mut tree.children[0],
                layout,
                renderer,
            )
            .focused
            .is_some();
        if let (true, Some(message)) = (inside, (self.on_key)(key, *modifiers)) {
            shell.publish(message);
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

impl<'a, Message: 'a> From<KeyArea<'a, Message>> for Element<'a, Message> {
    fn from(area: KeyArea<'a, Message>) -> Self {
        Element::new(area)
    }
}
