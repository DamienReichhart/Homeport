//! What the tooltip under the pointer says: when the sample was taken
//! and what each series that shows was then, laid out to fit the chart.

use iced::Size;

use super::{samples::Samples, series::Look, unit::Unit};
use crate::{i18n, ui::widgets::basic::truncate};

/// What stands for a value that is missing, as the tables write it: a
/// mark of no language.
pub const NONE: &str = "—";

pub const PAD: f32 = 8.0;
pub const ROW: f32 = 18.0;
/// The sample of a series' line before its label, and the room after.
pub const SWATCH: f32 = 28.0;
pub const AFTER_SWATCH: f32 = 6.0;
/// Between a label and its value, and between two columns.
const BEFORE_VALUE: f32 = 12.0;
const BETWEEN: f32 = 16.0;
/// The least a label keeps when the chart is too narrow for it whole.
const LEAST_LABEL: f32 = 24.0;

#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    pub series: usize,
    /// The series' line; none for the row that counts those left out.
    pub look: Option<Look>,
    /// The series' name, cut with an ellipsis if the chart is narrower
    /// than it: the legend has it whole.
    pub label: String,
    pub value: String,
    /// How wide each is drawn, measured once.
    pub label_width: f32,
    pub value_width: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tip {
    /// What it was built from: the same sample of the same chart.
    pub key: u64,
    /// The sample it is about.
    pub index: usize,
    /// When the sample was taken: the tooltip's first line.
    pub time: String,
    pub time_width: f32,
    /// One row per series that shows.
    pub rows: Vec<Row>,
    /// How many rows a column holds, and how wide a label and a value
    /// are given.
    pub per_column: usize,
    pub label_width: f32,
    pub value_width: f32,
    pub size: Size,
}

/// What a tooltip is made from.
pub struct Ask<'a> {
    pub key: u64,
    pub index: usize,
    pub labels: &'a [(String, Look)],
    pub hidden: &'a [bool],
    pub unit: Unit,
    pub zone: &'a jiff::tz::TimeZone,
    /// Whether the chart spans days: the time then comes with its date.
    pub dated: bool,
    /// The room the tooltip may take.
    pub room: Size,
    pub measure: &'a dyn Fn(&str) -> f32,
}

impl Tip {
    pub fn build(samples: &Samples, ask: &Ask<'_>) -> Self {
        let time = samples.time(ask.index).unwrap_or_default();
        // Seconds since 1970 fit an i64.
        #[allow(clippy::cast_possible_truncation)]
        let time = {
            let (formats, at) = (&i18n::current().formats, time.round() as i64);
            if ask.dated {
                formats.date_time(at, ask.zone)
            } else {
                formats.time(at, ask.zone)
            }
        };
        let time_width = (ask.measure)(&time);
        let mut rows: Vec<Row> = ask
            .labels
            .iter()
            .enumerate()
            .filter(|(index, _)| !ask.hidden.get(*index).copied().unwrap_or(false))
            .map(|(index, (label, look))| {
                let value = samples
                    .value(index, ask.index)
                    .map_or_else(|| NONE.to_owned(), |value| ask.unit.format(value));
                Row {
                    series: index,
                    look: Some(*look),
                    label_width: (ask.measure)(label),
                    value_width: (ask.measure)(&value),
                    label: label.clone(),
                    value,
                }
            })
            .collect();
        // Under the time, as many rows as the chart is tall; more go in
        // further columns, as even as they can be.
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let fit = (((ask.room.height - 2.0 * PAD) / ROW).floor() as usize)
            .saturating_sub(1)
            .max(1);
        let widest = |rows: &[Row], of: fn(&Row) -> f32| rows.iter().map(of).fold(0.0, f32::max);
        let value_width = widest(&rows, |row| row.value_width);
        // As many columns as the chart is wide, each with the least a
        // name keeps; the series past that are counted in a last row,
        // so the tooltip is never larger than the plot. The legend
        // names them all.
        let least = SWATCH + AFTER_SWATCH + LEAST_LABEL + BEFORE_VALUE + value_width + BETWEEN;
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let room = (((ask.room.width - 2.0 * PAD + BETWEEN) / least).floor() as usize).max(1) * fit;
        if rows.len() > room {
            let left_out = rows.len() - (room - 1);
            rows.truncate(room - 1);
            let label = crate::t!("ui.chart.more", count = left_out);
            rows.push(Row {
                series: usize::MAX,
                look: None,
                label_width: (ask.measure)(&label),
                value_width: 0.0,
                label,
                value: String::new(),
            });
        }
        let columns = rows.len().div_ceil(fit).max(1);
        let per_column = rows.len().div_ceil(columns).max(1);
        let mut label_width = widest(&rows, |row| row.label_width);
        // Columns are counted on one hand.
        #[allow(clippy::cast_precision_loss)]
        let count = columns as f32;
        let fixed = SWATCH + AFTER_SWATCH + BEFORE_VALUE + value_width;
        let width_of = |label: f32| {
            (2.0 * PAD + count * (fixed + label) + (count - 1.0) * BETWEEN)
                .max(2.0 * PAD + time_width)
        };
        if width_of(label_width) > ask.room.width {
            let spare = ask.room.width - 2.0 * PAD - count * fixed - (count - 1.0) * BETWEEN;
            label_width = (spare / count).max(LEAST_LABEL).min(label_width);
            for row in &mut rows {
                row.label = truncate::fit(&row.label, label_width, ask.measure);
                row.label_width = (ask.measure)(&row.label);
            }
        }
        // A column holds a handful of rows.
        #[allow(clippy::cast_precision_loss)]
        let height = 2.0 * PAD + (1 + rows.len().min(per_column)) as f32 * ROW;
        Self {
            key: ask.key,
            index: ask.index,
            time,
            time_width,
            rows,
            per_column,
            label_width,
            value_width,
            size: Size::new(width_of(label_width), height),
        }
    }

    /// How wide one column is.
    pub fn column_width(&self) -> f32 {
        SWATCH + AFTER_SWATCH + self.label_width + BEFORE_VALUE + self.value_width
    }

    /// How far a column starts from the one before.
    pub fn column_step(&self) -> f32 {
        self.column_width() + BETWEEN
    }
}

/// Where the tooltip goes across a chart `width` wide, beside the
/// crosshair at `at`: after it the way the language reads if there is
/// room, else before it, and inside the chart in any case. It never
/// stands on the crosshair unless the chart is narrower than both.
pub fn place(at: f32, tip: f32, width: f32, rtl: bool) -> f32 {
    const AWAY: f32 = 12.0;
    let after = at + AWAY;
    let before = at - AWAY - tip;
    let (first, second) = if rtl {
        (before, after)
    } else {
        (after, before)
    };
    let fits = |x: f32| x >= 0.0 && x + tip <= width;
    let x = if fits(first) || !fits(second) {
        first
    } else {
        second
    };
    x.clamp(0.0, (width - tip).max(0.0))
}
