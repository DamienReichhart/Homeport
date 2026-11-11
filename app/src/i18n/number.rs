//! Numbers as a language writes them: its digits, its decimal mark and its
//! grouping.

use fixed_decimal::{Decimal, FloatPrecision, SignedRoundingMode, UnsignedRoundingMode};
use icu::{
    decimal::{
        options::{DecimalFormatterOptions, GroupingStrategy},
        DecimalFormatter,
    },
    locale::Locale,
};

/// The ICU locale of a language tag; English for one that doesn't parse.
pub fn locale(tag: &str) -> Locale {
    tag.parse().unwrap_or(icu::locale::locale!("en"))
}

/// A formatter with the language's grouping (`12,345`, `12 345`).
pub fn formatter(tag: &str) -> Option<DecimalFormatter> {
    DecimalFormatter::try_new(locale(tag).into(), DecimalFormatterOptions::default()).ok()
}

/// A formatter that never groups: `1000`, not `1,000`.
pub fn ungrouped(tag: &str) -> Option<DecimalFormatter> {
    DecimalFormatter::try_new(locale(tag).into(), GroupingStrategy::Never.into()).ok()
}

/// `value` with as many decimals as it has.
pub fn format(formatter: &DecimalFormatter, value: f64) -> String {
    let decimal = Decimal::try_from_f64(value, FloatPrecision::RoundTrip).unwrap_or_default();
    formatter.format(&decimal).to_string()
}

/// `value` with exactly `digits` decimals.
pub fn rounded(value: f64, digits: u8) -> Decimal {
    let places = -i16::from(digits);
    let mut decimal = Decimal::try_from_f64(value, FloatPrecision::RoundTrip)
        .unwrap_or_default()
        .rounded_with_mode(
            places,
            SignedRoundingMode::Unsigned(UnsignedRoundingMode::HalfExpand),
        );
    decimal.pad_end(places);
    decimal
}

/// `value` with at most `digits` decimals and no trailing zero, as
/// `Intl.NumberFormat` writes a plain number.
pub fn at_most(formatter: &DecimalFormatter, value: f64, digits: u8) -> String {
    let mut decimal = rounded(value, digits);
    decimal.absolute.trim_end();
    formatter.format(&decimal).to_string()
}

/// `value` rounded to exactly `digits` decimals, half away from zero as
/// `Intl.NumberFormat` does.
pub fn fixed(formatter: &DecimalFormatter, value: f64, digits: u8) -> String {
    formatter.format(&rounded(value, digits)).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_the_way_each_language_does() {
        let english = formatter("en").unwrap();
        assert_eq!(format(&english, 12345.0), "12,345");
        assert_eq!(format(&english, 3.0), "3");
        let german = formatter("de").unwrap();
        assert_eq!(format(&german, 12345.5), "12.345,5");
        let egyptian = formatter("ar-EG").unwrap();
        assert_eq!(format(&egyptian, 12.0), "١٢");
    }

    #[test]
    fn rounds_to_a_fixed_number_of_decimals() {
        let english = ungrouped("en").unwrap();
        assert_eq!(fixed(&english, 1.5, 1), "1.5");
        assert_eq!(fixed(&english, 3.0, 1), "3.0");
        assert_eq!(fixed(&english, 1000.4, 0), "1000");
        assert_eq!(fixed(&english, 2.25, 1), "2.3");
        let grouped = formatter("en").unwrap();
        assert_eq!(at_most(&grouped, 1.234_56, 3), "1.235");
        assert_eq!(at_most(&grouped, 0.1 + 0.2, 3), "0.3");
        assert_eq!(at_most(&grouped, 12345.0, 3), "12,345");
    }
}
