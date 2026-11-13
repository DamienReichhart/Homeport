//! What the tabs work out of what the Engine says, apart from drawing it
//! (`model/health.ts`, `model/parse.ts`, and what `InspectTab` read of
//! the inspect document).

use jiff::tz::TimeZone;

/// The clock times are said on: this computer's. In a test it is UTC, so
/// that a snapshot is the same wherever it is drawn.
pub fn zone() -> TimeZone {
    if cfg!(test) {
        TimeZone::UTC
    } else {
        TimeZone::system()
    }
}
