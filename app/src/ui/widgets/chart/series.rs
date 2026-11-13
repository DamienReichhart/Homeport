//! What a series looks like: its colour, its dashes, and whether what is
//! under its line is filled.
//!
//! docs/BRAND.md keeps charts neutral: a series is Ink or grey, a
//! second one in the same chart is dashed, and none is a status colour.
//! A page that ports a React chart says so itself ([`Series::tone`],
//! [`Series::dashed`]). A chart that names no look gets
//! [`Look::of`]: four quiet colours that stay apart for eyes that lack
//! one of the three cones, and five dash patterns, so that no two of
//! twenty series share both and colour is never the only cue. The
//! legend shows the line as it is drawn, beside its name.

use iced::Color;

use crate::ui::theme::Theme;

/// A series' colour, by what it is in either theme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tone {
    /// The text colour: `--chart-1` to `--chart-3`.
    Ink,
    Blue,
    Olive,
    /// `--chart-4` and `--chart-5`.
    Grey,
}

impl Tone {
    pub const ALL: [Self; 4] = [Self::Ink, Self::Blue, Self::Olive, Self::Grey];

    /// The colour in `theme`: 3:1 or more on the page and on a card
    /// (WCAG 1.4.11), which `chart/tests/palette.rs` holds it to.
    pub fn colour(self, theme: Theme) -> Color {
        let tokens = theme.tokens();
        match self {
            Self::Ink => tokens.chart[0],
            Self::Blue => tokens.series[0],
            Self::Olive => tokens.series[1],
            Self::Grey => tokens.chart[3],
        }
    }
}

/// How a line is dashed, so two series differ by more than their colour.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Dash {
    #[default]
    Solid,
    /// uPlot's `dash: [5, 3]`, what `ChartSeries.dashed` drew.
    Dashed,
    Dotted,
    DashDot,
    Long,
}

impl Dash {
    pub const ALL: [Self; 5] = [
        Self::Solid,
        Self::Dashed,
        Self::Dotted,
        Self::DashDot,
        Self::Long,
    ];

    /// The lengths drawn and left out, in turn; none for a solid line.
    pub fn segments(self) -> &'static [f32] {
        match self {
            Self::Solid => &[],
            Self::Dashed => &[5.0, 3.0],
            Self::Dotted => &[1.5, 3.0],
            Self::DashDot => &[7.0, 3.0, 1.5, 3.0],
            Self::Long => &[11.0, 4.0],
        }
    }
}

/// A series' look.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Look {
    pub tone: Tone,
    pub dash: Dash,
    /// Whether what is under the line is tinted.
    pub area: bool,
}

impl Look {
    /// The look of the series at `index` when its chart names none: no
    /// two of the first twenty are alike in both colour and dashes, and
    /// neighbours differ in both.
    pub fn of(index: usize) -> Self {
        Self {
            tone: Tone::ALL[index % Tone::ALL.len()],
            dash: Dash::ALL[index % Dash::ALL.len()],
            area: false,
        }
    }
}

/// One series of a chart, as its view describes it: the label is read
/// from the catalogue at each draw, so the state keeps none.
#[derive(Debug, Clone, PartialEq)]
pub struct Series {
    pub label: String,
    pub(super) tone: Option<Tone>,
    pub(super) dash: Option<Dash>,
    pub(super) area: bool,
}

impl Series {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            tone: None,
            dash: None,
            area: false,
        }
    }

    #[must_use]
    pub fn tone(mut self, tone: Tone) -> Self {
        self.tone = Some(tone);
        self
    }

    #[must_use]
    pub fn dash(mut self, dash: Dash) -> Self {
        self.dash = Some(dash);
        self
    }

    /// `ChartSeries.dashed`.
    #[must_use]
    pub fn dashed(self) -> Self {
        self.dash(Dash::Dashed)
    }

    /// Tints what is under the line: an area chart.
    #[must_use]
    pub fn filled(mut self) -> Self {
        self.area = true;
        self
    }

    /// How the series at `index` is drawn.
    pub fn look(&self, index: usize) -> Look {
        let default = Look::of(index);
        Look {
            tone: self.tone.unwrap_or(default.tone),
            // A chart that names its colours names its dashes too.
            dash: self.dash.unwrap_or(if self.tone.is_some() {
                Dash::Solid
            } else {
                default.dash
            }),
            area: self.area,
        }
    }
}
