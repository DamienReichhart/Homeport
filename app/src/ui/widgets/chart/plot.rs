//! The chart on its canvas: the grid, the axes and the lines, kept as
//! they were drawn until what they show changes; and over them, drawn
//! anew for each frame the pointer is on the chart, the crosshair and
//! its tooltip.
//!
//! A canvas is drawn in the window's own pixels (iced scales what is
//! given in logical ones), so a line stays sharp on any display.

use std::{
    hash::{Hash, Hasher},
    rc::Rc,
};

use iced::{
    advanced::text::{self as core_text, Paragraph as _},
    alignment::Vertical,
    mouse,
    widget::canvas::{self, path::Builder, Frame, Geometry, LineDash, LineJoin, Path, Stroke},
    Color, Pixels, Point, Rectangle, Renderer, Size, Vector,
};

use super::{
    lttb,
    scale::DAY,
    scene::{self, Scene, FONT, TICK},
    series::Look,
    state::State,
    tip::{self, Tip},
    unit::Unit,
};
use crate::{
    i18n,
    ui::theme::{color::alpha, font, metrics::radius, Theme},
};

type Raw = <Renderer as core_text::Renderer>::Paragraph;

/// How thick a series' line is: uPlot's `width: 1.5`.
const LINE: f32 = 1.5;
/// How much of its colour a series tints what is under it with.
const AREA: f32 = 0.12;
/// How many points a series draws per pixel of the plot's width at the
/// most: more would land on the same pixels, and every point is the
/// renderer's to draw at each frame.
const POINTS_PER_PIXEL: f32 = 1.0;

/// How wide `content` is drawn at the axes' size.
pub(super) fn measure(content: &str) -> f32 {
    Raw::with_text(core_text::Text {
        content,
        bounds: Size::INFINITE,
        size: Pixels(FONT),
        line_height: core_text::LineHeight::default(),
        font: font::DEFAULT,
        align_x: core_text::Alignment::Default,
        align_y: Vertical::Top,
        shaping: core_text::Shaping::Advanced,
        wrapping: core_text::Wrapping::None,
    })
    .min_bounds()
    .width
}

/// The canvas' program: one chart, as its view describes it.
pub(super) struct Plot<'a> {
    pub state: &'a State,
    /// Each series' name and look, in the order of the samples' values.
    pub series: Vec<(String, Look)>,
    pub unit: Unit,
    pub zone: jiff::tz::TimeZone,
    pub rtl: bool,
}

/// Whether the pointer was on the chart at the last event: a frame is
/// asked for when it moves there, and once more when it leaves.
#[derive(Debug, Default)]
pub(super) struct Pointer {
    over: bool,
}

impl Plot<'_> {
    /// What tells this draw from the last: anything the axes and the
    /// lines show.
    fn signature(&self, size: Size, theme: Theme) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        let state = self.state;
        state.revision.hash(&mut hasher);
        (size.width.to_bits(), size.height.to_bits()).hash(&mut hasher);
        (self.unit.id(), self.rtl, theme).hash(&mut hasher);
        // A language set again is another: its formats are.
        (std::sync::Arc::as_ptr(&i18n::current()) as usize).hash(&mut hasher);
        let newest = state.samples.last_time().unwrap_or_default();
        // Seconds since 1970 fit an i64.
        #[allow(clippy::cast_possible_truncation)]
        let offset = jiff::Timestamp::from_second(newest as i64)
            .map(|at| self.zone.to_offset(at).seconds())
            .unwrap_or(0);
        offset.hash(&mut hasher);
        for (_, look) in &self.series {
            look.hash(&mut hasher);
        }
        hasher.finish()
    }

    /// The scene of this draw: the last one's while nothing it was made
    /// from changed.
    fn scene(&self, signature: u64, size: Size) -> Rc<Scene> {
        let mut kept = self.state.scene.borrow_mut();
        if let Some((of, scene)) = kept.as_ref() {
            if *of == signature {
                return scene.clone();
            }
        }
        let scene = Rc::new(Scene::build(
            &self.state.samples,
            &scene::Ask {
                size,
                hidden: &self.state.hidden,
                window: self.state.window,
                unit: self.unit,
                rtl: self.rtl,
                zone: &self.zone,
                measure: &measure,
            },
        ));
        *kept = Some((signature, scene.clone()));
        scene
    }

    /// The sample the pointer is nearest to, across the plot.
    fn pointed(&self, scene: &Scene, bounds: Rectangle, cursor: mouse::Cursor) -> Option<usize> {
        let at = cursor.position_in(bounds)?;
        let plot = scene.plot;
        if at.x < plot.x || at.x > plot.x + plot.width || plot.width <= 0.0 {
            return None;
        }
        self.state.samples.nearest(scene.time_at(at.x), scene.from)
    }

    fn tip(&self, signature: u64, scene: &Scene, index: usize, room: Size) -> Rc<Tip> {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        (signature, index).hash(&mut hasher);
        for (label, _) in &self.series {
            label.hash(&mut hasher);
        }
        let key = hasher.finish();
        let mut kept = self.state.tip.borrow_mut();
        if let Some(tip) = kept.as_ref().filter(|tip| tip.key == key) {
            return tip.clone();
        }
        let tip = Rc::new(Tip::build(
            &self.state.samples,
            &tip::Ask {
                key,
                index,
                labels: &self.series,
                hidden: &self.state.hidden,
                unit: self.unit,
                zone: &self.zone,
                dated: scene.x.1 - scene.x.0 >= DAY,
                room,
                measure: &measure,
            },
        ));
        *kept = Some(tip.clone());
        tip
    }
}

impl<Message> canvas::Program<Message, Theme> for Plot<'_> {
    type State = Pointer;

    fn update(
        &self,
        pointer: &mut Pointer,
        event: &iced::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<Message>> {
        let iced::Event::Mouse(mouse::Event::CursorMoved { .. } | mouse::Event::CursorLeft) = event
        else {
            return None;
        };
        let over = cursor.is_over(bounds);
        let was = std::mem::replace(&mut pointer.over, over);
        (over || was).then(canvas::Action::request_redraw)
    }

    fn draw(
        &self,
        _pointer: &Pointer,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let state = self.state;
        let size = bounds.size();
        let signature = self.signature(size, *theme);
        let scene = self.scene(signature, size);
        if state.drawn.replace(signature) != signature {
            state.lines.clear();
        }
        let lines = state.lines.draw(renderer, size, |frame| {
            grid(frame, &scene, *theme);
            self.lines(frame, &scene, *theme);
        });
        let Some(index) = self.pointed(&scene, bounds, cursor) else {
            return vec![lines];
        };
        let mut frame = Frame::new(renderer, size);
        // The tooltip stays in the plot: what the axes say is never
        // under it.
        let tip = self.tip(signature, &scene, index, scene.plot.size());
        self.crosshair(&mut frame, &scene, &tip, *theme);
        vec![lines, frame.into_geometry()]
    }

    fn mouse_interaction(
        &self,
        _pointer: &Pointer,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if cursor.is_over(bounds) && !self.state.samples.is_empty() {
            mouse::Interaction::Crosshair
        } else {
            mouse::Interaction::default()
        }
    }
}

/// A line one pixel wide where `at` is: on the middle of its pixel.
fn crisp(at: f32) -> f32 {
    at.floor() + 0.5
}

/// `content`, `width` wide, from `position` (its left end, at mid
/// height). It is given its own width and no more: a line that reads
/// right to left is set against the far end of the room it is given.
fn label(content: &str, width: f32, position: Point, colour: Color) -> canvas::Text {
    canvas::Text {
        content: content.to_owned(),
        position,
        max_width: width.ceil() + 1.0,
        color: colour,
        size: Pixels(FONT),
        line_height: core_text::LineHeight::default(),
        font: font::DEFAULT,
        align_x: core_text::Alignment::Left,
        align_y: Vertical::Center,
        shaping: core_text::Shaping::Advanced,
    }
}

/// The grid, the ticks and what they say.
fn grid(frame: &mut Frame, scene: &Scene, theme: Theme) {
    let tokens = theme.tokens();
    let plot = scene.plot;
    let hairline = Stroke::default().with_color(tokens.border).with_width(1.0);
    let (right, bottom) = (plot.x + plot.width, plot.y + plot.height);
    // The y axis is where text starts: left, and right in Arabic.
    // Each line starts at its tick's mark, outside the plot.
    let (from, to) = if scene.rtl {
        (plot.x, right + TICK)
    } else {
        (plot.x - TICK, right)
    };
    for tick in &scene.y_ticks {
        let y = crisp(tick.at);
        frame.stroke(
            &Path::line(Point::new(from, y), Point::new(to, y)),
            hairline,
        );
        if tick.label.is_empty() {
            continue;
        }
        // The top label stays whole inside the canvas.
        let y = tick.at.clamp(FONT / 2.0, frame.height() - FONT / 2.0);
        frame.fill_text(label(
            &tick.label,
            tick.width,
            Point::new(tick.start, y),
            tokens.muted_foreground,
        ));
    }
    for tick in &scene.x_ticks {
        let x = crisp(tick.at);
        frame.stroke(
            &Path::line(Point::new(x, plot.y), Point::new(x, bottom + TICK)),
            hairline,
        );
        if tick.label.is_empty() {
            continue;
        }
        frame.fill_text(label(
            &tick.label,
            tick.width,
            Point::new(tick.start, bottom + TICK + 4.0 + FONT / 2.0),
            tokens.muted_foreground,
        ));
    }
    // The line the values stand on.
    let base = crisp(bottom);
    frame.stroke(
        &Path::line(Point::new(plot.x, base), Point::new(right, base)),
        hairline,
    );
}

fn stroke(look: Look, theme: Theme, width: f32) -> Stroke<'static> {
    Stroke {
        line_join: LineJoin::Round,
        line_dash: LineDash {
            segments: look.dash.segments(),
            offset: 0,
        },
        ..Stroke::default()
            .with_color(look.tone.colour(theme))
            .with_width(width)
    }
}

/// A short sample of a series' line from `from`, `length` long: what
/// the legend and the tooltip show beside its name.
pub(super) fn swatch(frame: &mut Frame, look: Look, theme: Theme, from: Point, length: f32) {
    let y = crisp(from.y) - 0.5;
    frame.stroke(
        &Path::line(Point::new(from.x, y), Point::new(from.x + length, y)),
        stroke(look, theme, 2.0),
    );
}

impl Plot<'_> {
    /// Every series that shows, back to front: the first on top.
    fn lines(&self, frame: &mut Frame, scene: &Scene, theme: Theme) {
        let plot = scene.plot;
        if plot.width <= 0.0 || plot.height <= 0.0 {
            return;
        }
        let samples = &self.state.samples;
        let in_view = samples.len().saturating_sub(scene.from);
        // The plot's width in points, a thousand or two.
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let budget = ((plot.width * POINTS_PER_PIXEL).ceil() as usize).max(3);
        let place = |index: usize, value: f64| {
            let time = samples.time(index).unwrap_or_default();
            Point::new(scene.px(time), scene.py(value))
        };
        let base = scene.py(0.0f64.clamp(scene.y.0, scene.y.1));
        {
            for (series, (_, look)) in self.series.iter().enumerate().rev() {
                if self.state.is_hidden(series) || series >= samples.series() {
                    continue;
                }
                decimated::count();
                let colour = look.tone.colour(theme);
                for (from, to) in runs(self.state, series, scene.from) {
                    let count = to - from;
                    // The run's share of the points the series may draw.
                    let keep = if in_view <= budget {
                        count
                    } else {
                        (budget * count / in_view).max(3)
                    };
                    let value = |index: usize| samples.value(series, index).unwrap_or_default();
                    let at = |offset: usize| {
                        let index = from + offset;
                        (samples.time(index).unwrap_or_default(), value(index))
                    };
                    let kept = lttb::pick(count, keep, at);
                    let mut points: Vec<Point> = kept
                        .iter()
                        .map(|offset| place(from + offset, value(from + offset)))
                        .collect();
                    enter(&mut points, plot.x);
                    decimated::points(points.len());
                    match points.as_slice() {
                        [] => {}
                        // A sample alone has no line: a dot says it is there.
                        [only] => frame.fill(&Path::circle(*only, LINE + 0.5), colour),
                        [first, .., last] => {
                            let line = |builder: &mut Builder| {
                                builder.move_to(*first);
                                points[1..].iter().for_each(|point| builder.line_to(*point));
                            };
                            if look.area {
                                let under = Path::new(|builder| {
                                    line(builder);
                                    builder.line_to(Point::new(last.x, base));
                                    builder.line_to(Point::new(first.x, base));
                                    builder.close();
                                });
                                frame.fill(&under, alpha(colour, AREA));
                            }
                            frame.stroke(&Path::new(line), stroke(*look, theme, LINE));
                        }
                    }
                }
            }
        }
    }

    /// The line through the sample pointed at, a mark on each series
    /// there, and the tooltip beside it.
    fn crosshair(&self, frame: &mut Frame, scene: &Scene, tip: &Tip, theme: Theme) {
        let tokens = theme.tokens();
        let plot = scene.plot;
        let samples = &self.state.samples;
        let Some(time) = samples.time(tip.index) else {
            return;
        };
        let x = scene.px(time);
        frame.stroke(
            &Path::line(
                Point::new(crisp(x), plot.y),
                Point::new(crisp(x), plot.y + plot.height),
            ),
            Stroke::default()
                .with_color(tokens.muted_foreground)
                .with_width(1.0),
        );
        for (series, (_, look)) in self.series.iter().enumerate() {
            if self.state.is_hidden(series) {
                continue;
            }
            if let Some(value) = samples.value(series, tip.index) {
                let at = Point::new(x, scene.py(value));
                frame.fill(&Path::circle(at, 3.5), tokens.popover);
                frame.fill(&Path::circle(at, 2.5), look.tone.colour(theme));
            }
        }
        let left = plot.x + tip::place(x - plot.x, tip.size.width, plot.width, scene.rtl);
        let top = plot.y;
        frame.with_save(|frame| {
            frame.translate(Vector::new(left.round(), top.round()));
            panel(frame, tip, scene.rtl, theme);
        });
    }
}

/// The tooltip, from its own corner.
fn panel(frame: &mut Frame, tip: &Tip, rtl: bool, theme: Theme) {
    let tokens = theme.tokens();
    let outline = Path::rounded_rectangle(
        Point::new(0.5, 0.5),
        Size::new(tip.size.width - 1.0, tip.size.height - 1.0),
        radius::MD.into(),
    );
    frame.fill(&outline, tokens.popover);
    frame.stroke(
        &outline,
        Stroke::default().with_color(tokens.border).with_width(1.0),
    );
    let first = tip::PAD + tip::ROW / 2.0;
    let time = if rtl {
        tip.size.width - tip::PAD - tip.time_width
    } else {
        tip::PAD
    };
    frame.fill_text(label(
        &tip.time,
        tip.time_width,
        Point::new(time, first),
        tokens.popover_foreground,
    ));
    for (index, row) in tip.rows.iter().enumerate() {
        // Rows and columns are a handful.
        #[allow(clippy::cast_precision_loss)]
        let (column, line) = (
            (index / tip.per_column) as f32,
            (index % tip.per_column) as f32,
        );
        let y = first + (line + 1.0) * tip::ROW;
        // A column from its start: the sample of the line, the name,
        // and the value against its end. Mirrored in Arabic.
        let start = tip::PAD + column * tip.column_step();
        let across = |from_start: f32, width: f32| {
            if rtl {
                tip.size.width - start - from_start - width
            } else {
                start + from_start
            }
        };
        let from = Point::new(across(0.0, tip::SWATCH), y);
        swatch(frame, row.look, theme, from, tip::SWATCH);
        let named = row.label_width;
        let name = tip::SWATCH + tip::AFTER_SWATCH;
        frame.fill_text(label(
            &row.label,
            named,
            Point::new(across(name, named), y),
            tokens.muted_foreground,
        ));
        let valued = row.value_width;
        frame.fill_text(label(
            &row.value,
            valued,
            Point::new(across(tip.column_width() - valued, valued), y),
            tokens.popover_foreground,
        ));
    }
}

/// Cuts a line where it comes into the plot: the sample before the
/// window is to the left of it, and the line from it starts at the
/// plot's edge. (A frame's own clipping is not the same in the two
/// renderers, so nothing is drawn that would need it.)
pub(super) fn enter(points: &mut Vec<Point>, left: f32) {
    let Some(first) = points.first().copied().filter(|first| first.x < left) else {
        return;
    };
    match points.get(1).copied() {
        Some(next) if next.x > left => {
            let share = (left - first.x) / (next.x - first.x);
            points[0] = Point::new(left, first.y + (next.y - first.y) * share);
        }
        // Nothing of it is in the plot.
        _ => {
            points.remove(0);
        }
    }
}

/// The runs of samples of `series` to join, from the sample `from` on:
/// one ends before a gap, and where two samples are further apart than
/// the chart joins.
pub(super) fn runs(state: &State, series: usize, from: usize) -> Vec<(usize, usize)> {
    let samples = &state.samples;
    let mut runs = vec![];
    let mut start: Option<usize> = None;
    for index in from..samples.len() {
        let here = samples.value(series, index).is_some();
        let apart = index > from
            && state.break_after.is_some_and(|most| {
                samples.time(index).unwrap_or_default()
                    - samples.time(index - 1).unwrap_or_default()
                    > most
            });
        if let Some(began) = start.filter(|_| !here || apart) {
            runs.push((began, index));
            start = None;
        }
        if here && start.is_none() {
            start = Some(index);
        }
    }
    if let Some(began) = start {
        runs.push((began, samples.len()));
    }
    runs
}

/// How many series this thread has picked the points of: a test holds
/// a frame that changes nothing to none.
pub(super) mod decimated {
    #[cfg(test)]
    thread_local! {
        pub static SERIES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
        pub static POINTS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    }

    pub fn count() {
        #[cfg(test)]
        SERIES.with(|series| series.set(series.get() + 1));
    }

    /// And how many points it drew of them.
    #[cfg_attr(not(test), allow(unused_variables))]
    pub fn points(count: usize) {
        #[cfg(test)]
        POINTS.with(|points| points.set(points.get() + count));
    }
}
