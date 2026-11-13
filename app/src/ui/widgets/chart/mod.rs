//! The time-series chart: lines (or areas) of live values over a time
//! axis that moves with them. What a container's CPU, memory, network
//! and disk, a Pod's and a Node's use are drawn with; `TimeChart`
//! (uPlot) in the React app.
//!
//! The page keeps a [`State`] per chart, pushes it the samples of its
//! stream and hands it the chart's [`Message`]s:
//!
//! ```ignore
//! // state, made once: two counters, five minutes of samples
//! network: chart::State::new(2, 300).counter(0).counter(1),
//!
//! // in update, for each sample of the stream:
//! self.network.push(point.time, &[Some(sample.net_rx), Some(sample.net_tx)]);
//! // and when the page follows another container:
//! self.network.reset();
//! Message::Network(message) => self.network.update(message),
//!
//! // in view:
//! Chart::new(
//!     &self.network,
//!     vec![
//!         Series::new(t!("containers.stats.received")).tone(Tone::Ink),
//!         Series::new(t!("containers.stats.sent")).tone(Tone::Grey).dashed(),
//!     ],
//!     Message::Network,
//! )
//! .unit(Unit::BytesPerSecond)
//! .view()
//! ```
//!
//! - **Samples** are kept in rings (`samples.rs`): appending one moves
//!   none of the others. A value that is missing, `NaN` or infinite is a
//!   gap in the line, never a zero; a counter is drawn as its rate, with
//!   a gap where it went down; a clock that went back drops what it
//!   overtakes.
//! - **The axes** (`scale.rs`, `scene.rs`): time runs left to right in
//!   every language; the values go from zero to a tenth over the
//!   highest in view, in the chart's [`Unit`], said by `i18n::format`.
//!   The y axis is where text starts: on the right in Arabic.
//! - **Long series** are thinned to a point per pixel with
//!   Largest-Triangle-Three-Buckets (`lttb.rs`), which keeps the peaks.
//!   That, the pass over the samples for the range, and the lines
//!   themselves are redone when a sample comes or a series is hidden,
//!   not on each frame (`plot.rs`).
//! - **The pointer** shows a crosshair on the nearest sample and a
//!   tooltip with what each series was then.
//! - **The legend** names each series beside a sample of its line (its
//!   colour and its dashes: `series.rs`); an entry is a button that
//!   hides its series and shows it again, reached with Tab and pressed
//!   with Enter or Space.

mod lttb;
mod plot;
pub mod samples;
pub mod scale;
mod scene;
pub mod series;
pub mod state;
mod tip;
pub mod unit;

use std::rc::Rc;

use iced::{
    mouse,
    widget::{
        canvas::{self, Canvas, Frame, Geometry},
        container,
    },
    Length, Point, Rectangle, Renderer,
};

pub use self::{
    samples::{Kind, Samples},
    series::{Dash, Look, Series, Tone},
    state::{Message, State},
    unit::Unit,
};
use crate::{
    i18n, t,
    ui::{
        theme::{
            self,
            metrics::{control, space, text as size},
            Element, Theme,
        },
        widgets::{
            basic::{flow, truncated},
            focus::pressable,
            line,
            overlay::{position::Side, tooltip},
            stack,
        },
    },
};

/// How tall the plot is unless told: the React chart's.
pub const HEIGHT: f32 = 160.0;

/// How long the sample of a line is in the legend: long enough for
/// the longest dash pattern to show twice over, so each of the five
/// reads as itself (`chart/tests/palette.rs` draws them and compares).
pub(super) const SWATCH: f32 = 28.0;

/// A chart being described; [`Chart::view`] makes it an element.
pub struct Chart<'a, Message> {
    state: &'a State,
    series: Vec<Series>,
    on_event: Rc<dyn Fn(self::Message) -> Message + 'a>,
    unit: Unit,
    height: f32,
    zone: Option<jiff::tz::TimeZone>,
    legend: bool,
}

impl<'a, Message: Clone + 'a> Chart<'a, Message> {
    /// A chart of `state`'s samples: `series` names and styles them, in
    /// the order their values are pushed in.
    pub fn new(
        state: &'a State,
        series: Vec<Series>,
        on_event: impl Fn(self::Message) -> Message + 'a,
    ) -> Self {
        Self {
            state,
            series,
            on_event: Rc::new(on_event),
            unit: Unit::default(),
            height: HEIGHT,
            zone: None,
            legend: true,
        }
    }

    /// What the values are: how the axis and the tooltip say them.
    #[must_use]
    pub fn unit(mut self, unit: Unit) -> Self {
        self.unit = unit;
        self
    }

    /// How tall the plot is; the legend comes under it.
    #[must_use]
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// The clock the times are said on; this computer's unless told.
    #[must_use]
    pub fn zone(mut self, zone: jiff::tz::TimeZone) -> Self {
        self.zone = Some(zone);
        self
    }

    /// Leaves the legend out: a chart of one series under a title that
    /// already names it.
    #[must_use]
    pub fn without_legend(mut self) -> Self {
        self.legend = false;
        self
    }

    pub fn view(self) -> Element<'a, Message> {
        let state = self.state;
        let looks: Vec<(String, Look)> = self
            .series
            .iter()
            .enumerate()
            .map(|(index, series)| (series.label.clone(), series.look(index)))
            .collect();
        let entries: Vec<Element<'a, Message>> = if self.legend {
            looks
                .iter()
                .enumerate()
                .map(|(index, (label, look))| {
                    entry(
                        label,
                        *look,
                        state.is_hidden(index),
                        (self.on_event)(self::Message::Toggle(index)),
                    )
                })
                .collect()
        } else {
            vec![]
        };
        let plot = Canvas::new(plot::Plot {
            state,
            series: looks,
            unit: self.unit,
            zone: self.zone.unwrap_or_else(jiff::tz::TimeZone::system),
            rtl: i18n::direction().is_rtl(),
        })
        .width(Length::Fill)
        .height(self.height);
        // The legend keeps its place in the tree with or without entries.
        let legend = flow(entries).spacing(space::S1);
        stack(vec![plot.into(), legend.into()])
            .spacing(space::S1)
            .into()
    }
}

/// One entry of the legend: a sample of the series' line and its name,
/// in a button that hides the series or shows it again.
fn entry<'a, Message: Clone + 'a>(
    label: &str,
    look: Look,
    hidden: bool,
    toggle: Message,
) -> Element<'a, Message> {
    let sample = Canvas::new(Swatch { look, hidden })
        .width(SWATCH)
        .height(size::XS);
    // A hidden series is told by more than its colour: its line is
    // gone from the entry too.
    let name = truncated(label).size(size::XS).style(if hidden {
        theme::text::muted
    } else {
        theme::text::foreground
    });
    let face = line(vec![sample.into(), name.into()]).spacing(space::S1_5);
    let button = pressable(container(face).center_y(Length::Fill))
        .height(control::HEIGHT_XS)
        .padding([0.0, space::S1_5])
        .style(theme::button::small(theme::button::ghost))
        .on_press(toggle);
    let says = if hidden {
        t!("ui.chart.show", name = label)
    } else {
        t!("ui.chart.hide", name = label)
    };
    tooltip(button, says, Side::Top).into()
}

/// The sample of a line in the legend.
struct Swatch {
    look: Look,
    hidden: bool,
}

impl<Message> canvas::Program<Message, Theme> for Swatch {
    type State = ();

    fn draw(
        &self,
        (): &(),
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let middle = Point::new(0.0, bounds.height / 2.0);
        if self.hidden {
            // Where the line was: a hairline in the border's colour.
            let y = middle.y.floor() + 0.5;
            frame.stroke(
                &canvas::Path::line(Point::new(0.0, y), Point::new(bounds.width, y)),
                canvas::Stroke::default()
                    .with_color(theme.tokens().border)
                    .with_width(1.0),
            );
        } else {
            plot::swatch(&mut frame, self.look, *theme, middle, bounds.width);
        }
        vec![frame.into_geometry()]
    }
}

#[cfg(test)]
mod tests;
