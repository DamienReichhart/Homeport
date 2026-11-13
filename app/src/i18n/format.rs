//! Numbers, sizes, dates and times as the app's language writes them. Every
//! page formats through here, so none builds a formatter of its own.

use fixed_decimal::Decimal;
use icu::{
    datetime::{
        fieldsets::{T, YMD, YMDT},
        input::{Date, DateTime, Time},
        options::TimePrecision,
        DateTimeFormatter,
    },
    decimal::DecimalFormatter,
    list::{
        options::{ListFormatterOptions, ListLength},
        ListFormatter,
    },
};
use icu_experimental::{
    dimension::percent::formatter::PercentFormatter,
    relativetime::{options::Numeric, RelativeTimeFormatter, RelativeTimeFormatterOptions},
};
use jiff::{tz::TimeZone, Timestamp};

use super::{number, Args};

/// The size units, by power of 1024.
pub(super) const UNITS: [&str; 5] = [
    "format.bytes.b",
    "format.bytes.kb",
    "format.bytes.mb",
    "format.bytes.gb",
    "format.bytes.tb",
];

/// The units of a bit rate, by power of 1000, as networks count.
pub(super) const BIT_RATES: [&str; 5] = [
    "format.bits.b",
    "format.bits.kb",
    "format.bits.mb",
    "format.bits.gb",
    "format.bits.tb",
];

/// The steps of a relative time: how many of one unit make the next.
const STEPS: [i64; 5] = [60, 60, 24, 30, 12];

/// The formatters of one locale, built once when the language is chosen.
pub struct Formats {
    tag: String,
    grouped: Option<DecimalFormatter>,
    ungrouped: Option<DecimalFormatter>,
    percent: Option<PercentFormatter<DecimalFormatter>>,
    /// Seconds, minutes, hours, days, months, years; `None` for a unit the
    /// locale has no data for, so the others keep their place.
    relative: [Option<RelativeTimeFormatter>; 6],
    date_time: Option<DateTimeFormatter<YMDT>>,
    date: Option<DateTimeFormatter<YMD>>,
    time: Option<DateTimeFormatter<T>>,
    /// Names one after the other: `a, b, and c`.
    list: Option<ListFormatter>,
}

impl Formats {
    /// The formatters of the locale `tag` (`fr`, `en-GB`).
    pub fn new(tag: &str) -> Self {
        let locale = number::locale(tag);
        let options = RelativeTimeFormatterOptions::default().with_numeric(Numeric::Auto);
        let relative = [
            RelativeTimeFormatter::try_new_long_second,
            RelativeTimeFormatter::try_new_long_minute,
            RelativeTimeFormatter::try_new_long_hour,
            RelativeTimeFormatter::try_new_long_day,
            RelativeTimeFormatter::try_new_long_month,
            RelativeTimeFormatter::try_new_long_year,
        ]
        .map(|new| new((&locale).into(), options).ok());
        let seconds = TimePrecision::Second;
        Self {
            tag: tag.to_owned(),
            grouped: number::formatter(tag),
            ungrouped: number::ungrouped(tag),
            percent: PercentFormatter::try_new((&locale).into(), Default::default()).ok(),
            relative,
            date_time: DateTimeFormatter::try_new(
                (&locale).into(),
                YMDT::medium().with_time_precision(seconds),
            )
            .ok(),
            date: DateTimeFormatter::try_new((&locale).into(), YMD::medium()).ok(),
            time: DateTimeFormatter::try_new(
                (&locale).into(),
                T::medium().with_time_precision(seconds),
            )
            .ok(),
            list: ListFormatter::try_new_and(
                (&locale).into(),
                ListFormatterOptions::default().with_length(ListLength::Wide),
            )
            .ok(),
        }
    }

    /// `items` one after the other as the language lists them
    /// (`Intl.ListFormat`): `web, db, and cache`, `web、db、cache` in
    /// Japanese.
    pub fn list(&self, items: &[&str]) -> String {
        match &self.list {
            Some(list) => list.format_to_string(items.iter().copied()),
            None => items.join(", "),
        }
    }

    /// The locale these format in.
    pub fn tag(&self) -> &str {
        &self.tag
    }

    /// A count or other plain number with the language's grouping:
    /// `12,345`, `12 345`.
    pub fn number(&self, value: f64) -> String {
        match &self.grouped {
            Some(formatter) if value.is_finite() => number::at_most(formatter, value, 3),
            _ => value.to_string(),
        }
    }

    fn decimals(&self, value: f64, digits: u8) -> String {
        match &self.ungrouped {
            Some(formatter) => number::fixed(formatter, value, digits),
            None => format!("{value:.*}", usize::from(digits)),
        }
    }

    /// `1536` → the unit's key and `1.5` (`1,5` in French): powers of 1024,
    /// like the Docker CLI. A size is under 1024 of its unit, so never
    /// grouped: `1000 B`.
    pub(super) fn bytes_parts(&self, bytes: f64, digits: u8) -> (&'static str, String) {
        if !bytes.is_finite() || bytes <= 0.0 {
            return (UNITS[0], self.decimals(0.0, 0));
        }
        // The exponent is a small whole number by construction.
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let exp = ((bytes.ln() / 1024f64.ln()).floor().max(0.0) as usize).min(UNITS.len() - 1);
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        let value = bytes / 1024f64.powi(exp as i32);
        (
            UNITS[exp],
            self.decimals(value, if exp == 0 { 0 } else { digits }),
        )
    }

    /// `1500` bits a second → the unit's key and `1.5`: powers of 1000,
    /// never grouped, whole under a thousand.
    pub(super) fn bit_rate_parts(&self, bits: f64, digits: u8) -> (&'static str, String) {
        if !bits.is_finite() || bits <= 0.0 {
            return (BIT_RATES[0], self.decimals(0.0, 0));
        }
        // The exponent is a small whole number by construction.
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let exp = ((bits.log10() / 3.0).floor().max(0.0) as usize).min(BIT_RATES.len() - 1);
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        let value = bits / 1000f64.powi(exp as i32);
        (
            BIT_RATES[exp],
            self.decimals(value, if exp == 0 { 0 } else { digits }),
        )
    }

    /// `3.456` → `3.5%` (`3,5 %` in French); no decimals from 100 up.
    pub fn percent(&self, value: f64) -> String {
        self.percent_with(value, if value >= 100.0 { 0 } else { 1 })
    }

    /// A percentage with `digits` decimals, whatever its size: the
    /// ticks of one axis are all written alike.
    pub fn percent_with(&self, value: f64, digits: u8) -> String {
        let rounded = number::rounded(value, digits);
        match &self.percent {
            Some(formatter) if value.is_finite() => formatter.format(&rounded).to_string(),
            _ => format!("{value}%"),
        }
    }

    /// A Unix time in seconds as "3 minutes ago", seen from `now`.
    pub fn relative(&self, unix_seconds: i64, now: i64) -> String {
        let mut value = now.saturating_sub(unix_seconds);
        let mut unit = STEPS.len();
        for (index, size) in STEPS.iter().enumerate() {
            if value.abs() < *size {
                unit = index;
                break;
            }
            value = div_round(value, *size);
        }
        match self.relative.get(unit).and_then(Option::as_ref) {
            Some(formatter) => formatter
                .format(Decimal::from(value.saturating_neg()))
                .to_string(),
            None => String::new(),
        }
    }

    /// Date and time: `Nov 9, 2026, 2:05:09 PM`, `9 nov. 2026, 14:05:09`.
    pub fn date_time(&self, unix_seconds: i64, zone: &TimeZone) -> String {
        let Some(when) = local(unix_seconds, zone) else {
            return String::new();
        };
        match &self.date_time {
            Some(formatter) => formatter.format(&when).to_string(),
            None => String::new(),
        }
    }

    /// The date alone: `Nov 9, 2026`.
    pub fn date(&self, unix_seconds: i64, zone: &TimeZone) -> String {
        let Some(when) = local(unix_seconds, zone) else {
            return String::new();
        };
        match &self.date {
            Some(formatter) => formatter.format(&when.date).to_string(),
            None => String::new(),
        }
    }

    /// The time alone: `2:05:09 PM`, `14:05:09`.
    pub fn time(&self, unix_seconds: i64, zone: &TimeZone) -> String {
        let Some(when) = local(unix_seconds, zone) else {
            return String::new();
        };
        match &self.time {
            Some(formatter) => formatter.format(&when.time).to_string(),
            None => String::new(),
        }
    }
}

/// `a / b` rounded to the nearest, halves up, as JavaScript's `Math.round`.
fn div_round(a: i64, b: i64) -> i64 {
    a.saturating_mul(2).saturating_add(b).div_euclid(2 * b)
}

/// A Unix time as the date and time a clock in `zone` shows.
fn local(unix_seconds: i64, zone: &TimeZone) -> Option<DateTime<icu::calendar::Iso>> {
    let at = Timestamp::from_second(unix_seconds)
        .ok()?
        .to_zoned(zone.clone());
    let date = Date::try_new_iso(
        i32::from(at.year()),
        at.month().unsigned_abs(),
        at.day().unsigned_abs(),
    )
    .ok()?;
    let time = Time::try_new(
        at.hour().unsigned_abs(),
        at.minute().unsigned_abs(),
        at.second().unsigned_abs(),
        0,
    )
    .ok()?;
    Some(DateTime { date, time })
}

fn formats() -> std::sync::Arc<super::Language> {
    super::current()
}

/// `1536` → `1.5 KB` (`1,5 Ko` in French), in the app's language.
pub fn bytes(bytes: f64) -> String {
    bytes_with(bytes, 1)
}

/// [`bytes`] with `digits` decimals.
pub fn bytes_with(bytes: f64, digits: u8) -> String {
    let language = formats();
    let (unit, value) = language.formats.bytes_parts(bytes, digits);
    language.text(unit, Some(&Args::new().with("value", value).0))
}

/// Bytes a second: `1536` → `1.5 KB/s`, in the app's language.
pub fn rate(bytes_per_second: f64) -> String {
    crate::t!("format.perSecond", value = bytes(bytes_per_second))
}

/// Bits a second: `1500` → `1.5 kbit/s`, in the app's language. Powers
/// of 1000, as networks count.
pub fn bit_rate(bits_per_second: f64) -> String {
    let language = formats();
    let (unit, value) = language.formats.bit_rate_parts(bits_per_second, 1);
    language.text(unit, Some(&Args::new().with("value", value).0))
}

/// `3.456` → `3.5%`, in the app's language.
pub fn percent(value: f64) -> String {
    formats().formats.percent(value)
}

/// A percentage with `digits` decimals: `50` → `50%`, `0.5` → `0.5%`.
pub fn percent_with(value: f64, digits: u8) -> String {
    formats().formats.percent_with(value, digits)
}

/// `12345` → `12,345`, in the app's language.
pub fn number(value: f64) -> String {
    formats().formats.number(value)
}

/// Names one after the other, as the app's language lists them.
pub fn list(items: &[&str]) -> String {
    formats().formats.list(items)
}

/// A Unix time in seconds as "3 minutes ago".
pub fn relative(unix_seconds: i64) -> String {
    formats()
        .formats
        .relative(unix_seconds, Timestamp::now().as_second())
}

/// Date and time on this computer's clock.
pub fn date_time(unix_seconds: i64) -> String {
    formats()
        .formats
        .date_time(unix_seconds, &TimeZone::system())
}

/// The date alone, on this computer's clock.
pub fn date(unix_seconds: i64) -> String {
    formats().formats.date(unix_seconds, &TimeZone::system())
}

/// The time alone, on this computer's clock.
pub fn time(unix_seconds: i64) -> String {
    formats().formats.time(unix_seconds, &TimeZone::system())
}

/// The time alone, on a clock in `zone`: an axis that must read the same
/// wherever it is drawn.
pub fn time_in(unix_seconds: i64, zone: &TimeZone) -> String {
    formats().formats.time(unix_seconds, zone)
}

/// First 12 hex characters of an id, without any `sha256:` prefix.
pub fn short_id(id: &str) -> &str {
    let id = id.strip_prefix("sha256:").unwrap_or(id);
    id.get(..12).unwrap_or(id)
}
