//! Items written start to end that go on the next line when the row is
//! full (`flex flex-wrap`), in the direction the language reads: a
//! toolbar that must fit a narrow pane. iced's own wrapping row always
//! fills its lines from the left.

use iced::{
    advanced::{
        layout, mouse, overlay, renderer,
        widget::{Operation, Tree},
        Clipboard, Layout, Shell, Widget,
    },
    Event, Length, Point, Rectangle, Size, Vector,
};

use crate::{
    i18n,
    ui::theme::{Element, Theme},
};

/// Items on as many lines as they need.
pub struct Flow<'a, Message> {
    items: Vec<Element<'a, Message>>,
    spacing: f32,
    line_spacing: f32,
    pushed: Option<usize>,
    rtl: bool,
}

/// `items` start to end, wrapped onto more lines when they don't fit,
/// each centred on its line.
pub fn flow<'a, Message>(items: Vec<Element<'a, Message>>) -> Flow<'a, Message> {
    Flow {
        items,
        spacing: 0.0,
        line_spacing: 0.0,
        pushed: None,
        rtl: i18n::direction().is_rtl(),
    }
}

impl<Message> Flow<'_, Message> {
    /// Between two items of a line, and between two lines.
    #[must_use]
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self.line_spacing = spacing;
        self
    }

    /// The item at `index` and those after it on its line go to the end
    /// of the line (`ms-auto`): what the room left is put before.
    #[must_use]
    pub fn push_from(mut self, index: usize) -> Self {
        self.pushed = Some(index);
        self
    }
}

/// Where each item of the sizes `sizes` goes in `width` pixels, from the
/// left, and how tall they are together.
fn place(
    sizes: &[Size],
    width: f32,
    spacing: f32,
    line_spacing: f32,
    pushed: Option<usize>,
) -> (Vec<Point>, f32) {
    let mut points = vec![Point::ORIGIN; sizes.len()];
    let mut y = 0.0;
    let mut start = 0;
    while start < sizes.len() {
        // The items of this line: as many as fit, one at least.
        let mut end = start;
        let mut used = 0.0;
        while end < sizes.len() {
            let gap = if end == start { 0.0 } else { spacing };
            if end > start && used + gap + sizes[end].width > width {
                break;
            }
            used += gap + sizes[end].width;
            end += 1;
        }
        let height = sizes[start..end]
            .iter()
            .map(|size| size.height)
            .fold(0.0, f32::max);
        let free = (width - used).max(0.0);
        let mut x = 0.0;
        for index in start..end {
            if pushed == Some(index) {
                x += free;
            }
            points[index] = Point::new(x, y + (height - sizes[index].height) / 2.0);
            x += sizes[index].width + spacing;
        }
        y += height + line_spacing;
        start = end;
    }
    (points, (y - line_spacing).max(0.0))
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Flow<'_, Message> {
    fn children(&self) -> Vec<Tree> {
        self.items.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.items);
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Shrink)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let width = limits.max().width;
        let room = layout::Limits::new(Size::ZERO, Size::new(width, f32::INFINITY));
        let nodes: Vec<layout::Node> = self
            .items
            .iter_mut()
            .zip(&mut tree.children)
            .map(|(item, tree)| item.as_widget_mut().layout(tree, renderer, &room))
            .collect();
        let sizes: Vec<Size> = nodes.iter().map(layout::Node::size).collect();
        let (points, height) = place(&sizes, width, self.spacing, self.line_spacing, self.pushed);
        let nodes = nodes
            .into_iter()
            .zip(points)
            .map(|(node, at)| {
                let x = if self.rtl {
                    width - at.x - node.size().width
                } else {
                    at.x
                };
                node.move_to(Point::new(x.round(), at.y.round()))
            })
            .collect();
        layout::Node::with_children(Size::new(width, height), nodes)
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

impl<'a, Message: 'a> From<Flow<'a, Message>> for Element<'a, Message> {
    fn from(flow: Flow<'a, Message>) -> Self {
        Element::new(flow)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn xs(points: &[Point]) -> Vec<(f32, f32)> {
        points.iter().map(|point| (point.x, point.y)).collect()
    }

    #[test]
    fn items_fill_a_line_then_go_on_the_next() {
        let sizes = [Size::new(40.0, 20.0); 3];
        let (points, height) = place(&sizes, 100.0, 10.0, 6.0, None);
        assert_eq!(xs(&points), [(0.0, 0.0), (50.0, 0.0), (0.0, 26.0)]);
        assert_eq!(height, 46.0);
        // One line when they fit; an item too wide for a line has its own.
        let (points, height) = place(&sizes, 140.0, 10.0, 6.0, None);
        assert_eq!(xs(&points), [(0.0, 0.0), (50.0, 0.0), (100.0, 0.0)]);
        assert_eq!(height, 20.0);
        let (points, _) = place(&[Size::new(300.0, 20.0); 2], 100.0, 10.0, 6.0, None);
        assert_eq!(xs(&points), [(0.0, 0.0), (0.0, 26.0)]);
    }

    #[test]
    fn what_is_pushed_goes_to_the_end_of_its_line_and_short_items_are_centred() {
        let sizes = [
            Size::new(40.0, 20.0),
            Size::new(30.0, 10.0),
            Size::new(20.0, 20.0),
        ];
        let (points, _) = place(&sizes, 200.0, 10.0, 6.0, Some(1));
        assert_eq!(xs(&points), [(0.0, 0.0), (140.0, 5.0), (180.0, 0.0)]);
        // Wrapped, it is still at the end of the line it is on.
        let (points, _) = place(&sizes, 60.0, 10.0, 6.0, Some(1));
        assert_eq!(xs(&points), [(0.0, 0.0), (0.0, 31.0), (40.0, 26.0)]);
        assert!(place(&[], 60.0, 10.0, 6.0, None).0.is_empty());
    }
}
