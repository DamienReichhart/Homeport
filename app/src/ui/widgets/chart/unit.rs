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
