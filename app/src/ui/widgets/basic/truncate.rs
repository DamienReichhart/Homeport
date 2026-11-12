//! `truncate`: a line of text that ends with an ellipsis where its room
//! does, instead of being cut through a letter. iced 0.14 has no ellipsis
//! of its own; this shows as many whole characters as fit before one.
//!
//! The whole text stays what a test or a screen reader finds. Pair it with
//! `overlay::tooltip` where the React cell had a `title`.

use iced::{
    advanced::{
        layout, mouse, renderer,
        text::{self, paragraph, LineHeight, Paragraph, Shaping, Wrapping},
        widget::{tree, Operation, Tree},
        Layout, Widget,
    },
    alignment, Font, Length, Pixels, Rectangle, Size,
};

use crate::ui::theme::{metrics::text as size, Element, Theme};

/// How many times a text was fitted: what a frame costs, for the tests.
#[cfg(test)]
pub static FITS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// What ends a text that is cut.
const ELLIPSIS: char = '…';

type Raw = <iced::Renderer as text::Renderer>::Paragraph;
type StyleFn<'a> = Box<dyn Fn(&Theme) -> iced::widget::text::Style + 'a>;

/// A line of text, cut with an ellipsis where its room ends.
pub struct Truncated<'a> {
    content: String,
    size: f32,
    font: Option<Font>,
    style: Option<StyleFn<'a>>,
}

/// `content` on one line of `text-sm`, ending with `…` when it is longer
/// than the room it is given.
pub fn truncated<'a>(content: impl Into<String>) -> Truncated<'a> {
    Truncated {
        content: content.into(),
        size: size::SM,
        font: None,
        style: None,
    }
}

impl<'a> Truncated<'a> {
    #[must_use]
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    #[must_use]
    pub fn font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }

    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme) -> iced::widget::text::Style + 'a) -> Self {
        self.style = Some(Box::new(style));
        self
    }
}

/// What was fitted last, so a view that changes nothing shapes nothing.
#[derive(Default)]
struct State {
    paragraph: paragraph::Plain<Raw>,
    /// The text, its size, its font and the room it was fitted to.
    fitted: Option<(String, f32, Font, f32)>,
}

/// `content` as one line that never wraps, however long.
fn line(content: &str, size: f32, font: Font) -> text::Text<&str, Font> {
    text::Text {
        content,
        bounds: Size::new(f32::INFINITY, f32::INFINITY),
        size: Pixels(size),
        line_height: LineHeight::default(),
        font,
        align_x: text::Alignment::Default,
        align_y: alignment::Vertical::Top,
        // Shaped as every other text is, so a text that is cut is set
        // as the one beside it that isn't.
        shaping: Shaping::Advanced,
        wrapping: Wrapping::None,
    }
}

/// The longest start of `content` that fits in `room` with an ellipsis
/// after it, given how wide a text is. `content` itself when it fits.
///
/// Measuring is shaping, which is what a cell costs: the cut is first
/// guessed from how wide the whole text is, then moved a character at a
/// time until it is the longest that fits, a measure or two for most.
pub fn fit(content: &str, room: f32, width: impl Fn(&str) -> f32) -> String {
    let whole = width(content);
    if whole <= room {
        return content.to_owned();
    }
    let ends: Vec<usize> = content.char_indices().map(|(at, _)| at).collect();
    let cut = |count: usize| {
        let mut shown = content[..ends.get(count).copied().unwrap_or(content.len())]
            .trim_end()
            .to_owned();
        shown.push(ELLIPSIS);
        shown
    };
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    let guess = ((ends.len() as f32 * room / whole).floor().max(0.0) as usize).min(ends.len() - 1);
    let mut count = guess;
    if width(&cut(count)) <= room {
        // It fits: as many more as still do.
        while count + 1 < ends.len() && width(&cut(count + 1)) <= room {
            count += 1;
        }
    } else {
        // Too long: fewer, until it fits or nothing is left but the sign.
        while count > 0 {
            count -= 1;
            if width(&cut(count)) <= room {
                break;
            }
        }
    }
    cut(count)
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Truncated<'_> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
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
        use text::Renderer as _;
        let state = tree.state.downcast_mut::<State>();
        let font = self.font.unwrap_or_else(|| renderer.default_font());
        let room = limits.max().width;
        let wanted = (self.content.clone(), self.size, font, room);
        if state.fitted.as_ref() != Some(&wanted) {
            #[cfg(test)]
            FITS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let size = self.size;
            // The whole text first, as any text is laid out: most fit,
            // and cost no more than a text that is never cut.
            let _ = state.paragraph.update(line(&self.content, size, font));
            let whole = state.paragraph.min_width();
            if whole > room {
                let shown = fit(&self.content, room, |content| {
                    if content == self.content {
                        whole
                    } else {
                        Raw::with_text(line(content, size, font)).min_width()
                    }
                });
                let _ = state.paragraph.update(line(&shown, size, font));
            }
            state.fitted = Some(wanted);
        }
        let bounds = state.paragraph.min_bounds();
        layout::Node::new(limits.resolve(Length::Shrink, Length::Shrink, bounds))
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        defaults: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();
        let style = self.style.as_ref().map(|style| style(theme));
        iced::advanced::widget::text::draw(
            renderer,
            defaults,
            layout.bounds(),
            state.paragraph.raw(),
            style.unwrap_or_default(),
            viewport,
        );
    }

    fn operate(
        &mut self,
        _tree: &mut Tree,
        layout: Layout<'_>,
        _renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        // The whole text: what is cut is still what it says.
        operation.text(None, layout.bounds(), &self.content);
    }
}

impl<'a, Message: 'a> From<Truncated<'a>> for Element<'a, Message> {
    fn from(text: Truncated<'a>) -> Self {
        Element::new(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Seven pixels a character, as a font of one width.
    #[allow(clippy::cast_precision_loss)]
    fn width(content: &str) -> f32 {
        7.0 * content.chars().count() as f32
    }

    #[test]
    fn a_text_that_fits_is_left_alone_and_a_longer_one_ends_with_an_ellipsis() {
        assert_eq!(fit("redis", 70.0, width), "redis");
        assert_eq!(fit("redis", 35.0, width), "redis");
        // Whole characters, then the ellipsis, within the room.
        assert_eq!(fit("shop-web-1-0123", 70.0, width), "shop-web-…");
        assert_eq!(fit("Up 3 hours (healthy)", 56.0, width), "Up 3 ho…");
        // No space is left hanging before the ellipsis.
        assert_eq!(fit("Up 3 hours", 28.0, width), "Up…");
        // Not even one character fits: the ellipsis alone.
        assert_eq!(fit("redis", 10.0, width), "…");
        assert_eq!(fit("", 0.0, width), "");
    }

    #[test]
    fn it_cuts_between_characters_of_any_script() {
        assert_eq!(fit("ميغابايت 128", 49.0, width), "ميغابا…");
        assert_eq!(fit("コンテナ一覧", 28.0, width), "コンテ…");
    }
}
