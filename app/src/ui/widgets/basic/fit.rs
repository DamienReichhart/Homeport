//! A column as wide as its widest item, with every item stretched to that
//! width: the lines of a menu or of a list of options, whose highlight goes
//! from edge to edge. iced's own column is either as wide as it is allowed
//! or leaves each item its own width.

use iced::{
    advanced::{
        layout, mouse, overlay, renderer,
        widget::{Operation, Tree},
        Clipboard, Layout, Shell, Widget,
    },
    Event, Length, Point, Rectangle, Size, Vector,
};

use crate::ui::theme::{Element, Theme};

/// Items one under the other, all as wide as the widest.
pub struct Fit<'a, Message> {
    items: Vec<Element<'a, Message>>,
    min_width: f32,
}

/// `items` in a column that fits the widest. Give each `Length::Fill` as
/// its width, so it stretches to it.
pub fn fit<'a, Message>(items: Vec<Element<'a, Message>>) -> Fit<'a, Message> {
    Fit {
        items,
        min_width: 0.0,
    }
}

impl<Message> Fit<'_, Message> {
    /// Never narrower than this, however short the items.
    #[must_use]
    pub fn min_width(mut self, width: f32) -> Self {
        self.min_width = width;
        self
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Fit<'_, Message> {
    fn children(&self) -> Vec<Tree> {
        self.items.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.items);
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Shrink, Length::Shrink)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let max = limits.max();
        // Once to learn how wide each item wants to be…
        let measure = layout::Limits::with_compression(Size::ZERO, max, Size::new(true, true));
        let widest = self
            .items
            .iter_mut()
            .zip(&mut tree.children)
            .map(|(item, tree)| {
                item.as_widget_mut()
                    .layout(tree, renderer, &measure)
                    .size()
                    .width
            })
            .fold(self.min_width.max(limits.min().width), f32::max)
            .min(max.width);
        // … and once more at the width of the widest.
        let mut y = 0.0;
        let nodes = self
            .items
            .iter_mut()
            .zip(&mut tree.children)
            .map(|(item, tree)| {
                let limits = layout::Limits::with_compression(
                    Size::new(widest, 0.0),
                    Size::new(widest, f32::INFINITY),
                    Size::new(false, true),
                );
                let node = item
                    .as_widget_mut()
                    .layout(tree, renderer, &limits)
                    .move_to(Point::new(0.0, y));
                y += node.size().height;
                node
            })
            .collect();
        layout::Node::with_children(Size::new(widest, y), nodes)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        operation.container(None, layout.bounds());
        operation.traverse(&mut |operation| {
            for ((item, tree), layout) in self
                .items
                .iter_mut()
                .zip(&mut tree.children)
                .zip(layout.children())
            {
                item.as_widget_mut()
                    .operate(tree, layout, renderer, operation);
            }
        });
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
        for ((item, tree), layout) in self
            .items
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
        {
            item.as_widget_mut().update(
                tree, event, layout, cursor, renderer, clipboard, shell, viewport,
            );
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
        for ((item, tree), layout) in self.items.iter().zip(&tree.children).zip(layout.children()) {
            item.as_widget()
                .draw(tree, renderer, theme, style, layout, cursor, viewport);
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
        self.items
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
            .map(|((item, tree), layout)| {
                item.as_widget()
                    .mouse_interaction(tree, layout, cursor, viewport, renderer)
            })
            .max()
            .unwrap_or_default()
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &iced::Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, iced::Renderer>> {
        overlay::from_children(
            &mut self.items,
            tree,
            layout,
            renderer,
            viewport,
            translation,
        )
    }
}

impl<'a, Message: 'a> From<Fit<'a, Message>> for Element<'a, Message> {
    fn from(fit: Fit<'a, Message>) -> Self {
        Element::new(fit)
    }
}
