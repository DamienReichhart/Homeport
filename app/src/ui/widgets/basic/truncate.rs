//! `truncate`: text that ends with an ellipsis where its room does,
//! instead of being cut through a letter. iced 0.14 has no ellipsis of
//! its own; this shows as many whole characters as fit before one. It is
//! one line unless told ([`Truncated::lines`]): given two, it wraps onto
//! the second before anything is cut (`line-clamp-2`).
//!
//! The whole text stays what a test or a screen reader finds, and
//! [`Truncated::with_tip`] shows it under the pointer while it is cut: a
//! cut text is never the only way to read what it says.

use iced::{
    advanced::{
        layout, mouse, renderer,
        text::{self, paragraph, LineHeight, Paragraph, Shaping, Wrapping},
        widget::{tree, Operation, Tree},
        Layout, Widget,
    },
    alignment, Font, Length, Pixels, Rectangle, Size,
};

use crate::ui::{
    theme::{metrics::text as size, Element, Theme},
    widgets::overlay::{tooltip, Anchored, Side},
};

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
    lines: usize,
}

/// `content` on one line of `text-sm`, ending with `…` when it is longer
/// than the room it is given.
pub fn truncated<'a>(content: impl Into<String>) -> Truncated<'a> {
    Truncated {
        content: content.into(),
        size: size::SM,
        font: None,
        style: None,
        lines: 1,
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

    /// The most lines it takes: it wraps onto them between words, and
    /// only what is past the last one is cut.
    #[must_use]
    pub fn lines(mut self, lines: usize) -> Self {
        self.lines = lines.max(1);
        self
    }

    /// The same with the whole text in a tooltip, which shows only while
    /// the text is cut.
    pub fn with_tip<Message: Clone + 'a>(self) -> Anchored<'a, Message> {
        let whole = self.content.clone();
        tooltip(self, whole, Side::Top).only_while(is_cut)
    }

    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme) -> iced::widget::text::Style + 'a) -> Self {
        self.style = Some(Box::new(style));
        self
    }
}

/// What a text shows of what it says: for a test that holds a page to
/// showing a text whole, and for the tooltip of one that is cut.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Fitted {
    pub whole: String,
    pub shown: String,
}

impl Fitted {
    pub fn is_cut(&self) -> bool {
        self.whole != self.shown
    }
}

/// What was fitted last, so a view that changes nothing shapes nothing.
#[derive(Default)]
struct State {
    paragraph: paragraph::Plain<Raw>,
    /// The text, its size, its font, its lines and the room it was fitted to.
    wanted: Option<(String, f32, Font, usize, f32)>,
    fitted: Fitted,
}

/// Whether the text whose state `tree` is shows less than it says.
fn is_cut(tree: &Tree) -> bool {
    tree.tag == tree::Tag::of::<State>() && tree.state.downcast_ref::<State>().fitted.is_cut()
}

/// `content` laid out in `room`: on one line however long when `room` is
/// endless, wrapped between words otherwise.
fn text(content: &str, size: f32, font: Font, room: f32) -> text::Text<&str, Font> {
    text::Text {
        content,
        bounds: Size::new(room, f32::INFINITY),
        size: Pixels(size),
        line_height: LineHeight::default(),
        font,
        align_x: text::Alignment::Default,
        align_y: alignment::Vertical::Top,
        // Shaped as every other text is, so a text that is cut is set
        // as the one beside it that isn't.
        shaping: Shaping::Advanced,
        wrapping: if room.is_finite() {
            Wrapping::Word
        } else {
            Wrapping::None
        },
    }
}

/// The longest start of `content` that fits with an ellipsis after it:
/// `content` cut after `guess` of its characters to start with, then a
/// character at a time until it is the longest `fits` takes.
fn longest(content: &str, guess: f32, fits: impl Fn(&str) -> bool) -> String {
    let ends: Vec<usize> = content.char_indices().map(|(at, _)| at).collect();
    if ends.is_empty() {
        return String::new();
    }
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
    let mut count = ((ends.len() as f32 * guess).floor().max(0.0) as usize).min(ends.len() - 1);
    if fits(&cut(count)) {
        // It fits: as many more as still do.
        while count + 1 < ends.len() && fits(&cut(count + 1)) {
            count += 1;
        }
    } else {
        // Too long: fewer, until it fits or nothing is left but the sign.
        while count > 0 {
            count -= 1;
            if fits(&cut(count)) {
                break;
            }
        }
    }
    cut(count)
}

/// The longest start of `content` that fits in `room` on one line with
/// an ellipsis after it, given how wide a text is. `content` itself when
/// it fits.
///
/// Measuring is shaping, which is what a cell costs: the cut is first
/// guessed from how wide the whole text is, then moved a character at a
/// time until it is the longest that fits, a measure or two for most.
pub fn fit(content: &str, room: f32, width: impl Fn(&str) -> f32) -> String {
    let whole = width(content);
    if whole <= room {
        return content.to_owned();
    }
    longest(content, room / whole, |cut| width(cut) <= room)
}

/// The same over `lines` lines `room` wide, given how many lines a text
/// wraps onto there and how wide it is on one: it wraps before it is cut.
pub fn fit_lines(
    content: &str,
    room: f32,
    lines: usize,
    wrapped: impl Fn(&str) -> usize,
    width: impl Fn(&str) -> f32,
) -> String {
    if wrapped(content) <= lines {
        return content.to_owned();
    }
    // About as many characters as the lines have room for.
    #[allow(clippy::cast_precision_loss)]
    let guess = lines as f32 * room / width(content).max(1.0);
    longest(content, guess.min(1.0), |cut| wrapped(cut) <= lines)
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
        let wanted = (self.content.clone(), self.size, font, self.lines, room);
        if state.wanted.as_ref() != Some(&wanted) {
            #[cfg(test)]
            FITS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let size = self.size;
            let on_one_line = |content: &str| {
                Raw::with_text(text(content, size, font, f32::INFINITY)).min_width()
            };
            let shown = if self.lines == 1 {
                // The whole text first, as any text is laid out: most
                // fit, and cost no more than a text that is never cut.
                let _ = state
                    .paragraph
                    .update(text(&self.content, size, font, f32::INFINITY));
                let whole = state.paragraph.min_width();
                (whole > room).then(|| {
                    fit(&self.content, room, |content| {
                        if content == self.content {
                            whole
                        } else {
                            on_one_line(content)
                        }
                    })
                })
            } else {
                // Wrapped in its room: how tall it is says how many lines.
                let line = LineHeight::default().to_absolute(Pixels(size)).0;
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let lines_of = |height: f32| (height / line).round().max(1.0) as usize;
                let _ = state
                    .paragraph
                    .update(text(&self.content, size, font, room));
                let whole = lines_of(state.paragraph.min_height());
                (whole > self.lines).then(|| {
                    let wrapped = |content: &str| {
                        if content == self.content {
                            whole
                        } else {
                            lines_of(Raw::with_text(text(content, size, font, room)).min_height())
                        }
                    };
                    fit_lines(&self.content, room, self.lines, wrapped, on_one_line)
                })
            };
            let room = if self.lines == 1 { f32::INFINITY } else { room };
            if let Some(shown) = &shown {
                let _ = state.paragraph.update(text(shown, size, font, room));
            }
            state.fitted = Fitted {
                shown: shown.unwrap_or_else(|| self.content.clone()),
                whole: self.content.clone(),
            };
            state.wanted = Some(wanted);
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
        tree: &mut Tree,
        layout: Layout<'_>,
        _renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        // The whole text: what is cut is still what it says.
        operation.text(None, layout.bounds(), &self.content);
        // And what it shows of it, for who asks.
        let state = tree.state.downcast_mut::<State>();
        operation.custom(None, layout.bounds(), &mut state.fitted);
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

    /// Lines of ten characters, broken between words.
    fn wrapped(content: &str) -> usize {
        let (mut lines, mut used) = (1, 0);
        for word in content.split(' ') {
            let length = word.chars().count();
            if used > 0 && used + 1 + length > 10 {
                lines += 1;
                used = length;
            } else {
                used += usize::from(used > 0) + length;
            }
        }
        lines
    }

    #[test]
    fn given_two_lines_it_wraps_before_it_is_cut() {
        // Whole on two lines: nothing is cut.
        assert_eq!(
            fit_lines("1 minute ago", 70.0, 2, wrapped, width),
            "1 minute ago"
        );
        assert_eq!(
            fit_lines("prod server", 70.0, 2, wrapped, width),
            "prod server"
        );
        // Three lines' worth in two: the second ends with the ellipsis.
        let cut = fit_lines("Exited (0) 2 days ago today", 70.0, 2, wrapped, width);
        assert_eq!(cut, "Exited (0) 2 days ag…");
        assert_eq!(wrapped(&cut), 2);
        // On one line it would have been cut much sooner.
        assert_eq!(
            fit("Exited (0) 2 days ago today", 70.0, width),
            "Exited (0…"
        );
    }
}
