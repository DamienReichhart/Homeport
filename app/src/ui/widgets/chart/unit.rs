//! What a chart's values are, so its axis and its tooltip say them as
//! the rest of the app does: through `i18n::format`, in the app's
//! language.

use crate::i18n::format;

/// The unit of a chart's values.
#[derive(Debug, Clone, Copy, Default)]
pub enum Unit {
    /// A count: `12,345`.
    #[default]
    Number,
    /// A share out of a hundred: `3.5%`.
    Percent,
    /// A size, in powers of 1024: `1.5 KB`.
    Bytes,
    /// Bytes a second: `1.5 KB/s`.
    BytesPerSecond,
    /// Bits a second, in powers of 1000: `1.5 kbit/s`.
    BitsPerSecond,
    /// Anything else, said by the page (cores and millicores).
    Custom(fn(f64) -> String),
}

impl Unit {
    /// `value` as text.
    pub fn format(self, value: f64) -> String {
        match self {
            Self::Number => format::number(value),
            Self::Percent => format::percent(value),
            Self::Bytes => format::bytes(value),
            Self::BytesPerSecond => format::rate(value),
            Self::BitsPerSecond => format::bit_rate(value),
            Self::Custom(say) => say(value),
        }
    }

    /// What a tick of an axis that steps by `step` says: as
    /// [`Unit::format`], but with one precision for the whole axis. A
    /// percentage is written with the decimals its step needs and no
    /// more (`0%`, `50%`, `100%`), where `format` alone drops them
    /// from 100 up.
    pub fn tick(self, value: f64, step: f64) -> String {
        match self {
            Self::Percent => {
                // The decimals of a step of 1, 2 or 5 times a power of ten.
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let digits = (-step.log10().floor()).clamp(0.0, 6.0) as u8;
                format::percent_with(value, digits)
            }
            _ => self.format(value),
        }
    }

    /// The least step its axis takes: a byte and a bit are written
    /// whole, so two ticks less than one apart would say the same.
    pub(super) fn least_step(self) -> f64 {
        match self {
            Self::Bytes | Self::BytesPerSecond | Self::BitsPerSecond => 1.0,
            Self::Number | Self::Percent | Self::Custom(_) => 0.0,
        }
    }

    /// Whether the unit's multiples are powers of 1024: its axis then
    /// steps by 512 MB rather than by 500.
    pub(super) fn is_binary(self) -> bool {
        matches!(self, Self::Bytes | Self::BytesPerSecond)
    }

    /// What tells one unit from another, for what is kept between draws.
    pub(super) fn id(self) -> usize {
        match self {
            Self::Number => 0,
            Self::Percent => 1,
            Self::Bytes => 2,
            Self::BytesPerSecond => 3,
            Self::BitsPerSecond => 4,
            // Where the function is: two charts that format alike share it.
            Self::Custom(say) => say as *const () as usize,
        }
    }
}
