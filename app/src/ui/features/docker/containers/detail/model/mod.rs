//! What the tabs work out of what the Engine says, apart from drawing it
//! (`model/health.ts`, `model/parse.ts`, and what `InspectTab` read of
//! the inspect document).

pub mod inspect;

#[cfg(test)]
pub mod tests;

use jiff::tz::TimeZone;

use homeport_engine::containers::health::HealthConfig;

use crate::t;

/// The clock times are said on: this computer's. In a test it is UTC, so
/// that a snapshot is the same wherever it is drawn.
pub fn zone() -> TimeZone {
    if cfg!(test) {
        TimeZone::UTC
    } else {
        TimeZone::system()
    }
}

/// A duration as `docker run` writes one: `1m30s`, `2h`, `5s`
/// (`formatDuration`).
pub fn format_duration(seconds: u32) -> String {
    let (hours, minutes, rest) = (seconds / 3600, seconds % 3600 / 60, seconds % 60);
    let mut said = String::new();
    if hours > 0 {
        said.push_str(&format!("{hours}h"));
    }
    if minutes > 0 {
        said.push_str(&format!("{minutes}m"));
    }
    if rest > 0 || (hours == 0 && minutes == 0) {
        said.push_str(&format!("{rest}s"));
    }
    said
}

/// The healthcheck's timing, as `every 30s, timeout 30s, 3 retries, start
/// period 0s`: what is unset is the Engine's default (`healthTiming`).
pub fn health_timing(config: &HealthConfig) -> String {
    t!(
        "containers.health.timing",
        interval = format_duration(config.interval_s.unwrap_or(30)),
        timeout = format_duration(config.timeout_s.unwrap_or(30)),
        retries = config.retries.unwrap_or(3),
        start = format_duration(config.start_period_s.unwrap_or(0))
    )
}

/// How long a check took, from its RFC 3339 start and end: `250 ms`,
/// `2.5 s`; `None` when a time doesn't read or the end is before the
/// start (`checkDuration`).
pub fn check_duration(start: &str, end: &str) -> Option<String> {
    let start = start.parse::<jiff::Timestamp>().ok()?;
    let end = end.parse::<jiff::Timestamp>().ok()?;
    let millis = end.as_millisecond() - start.as_millisecond();
    if millis < 0 {
        return None;
    }
    Some(if millis < 1000 {
        format!("{millis} ms")
    } else {
        #[allow(clippy::cast_precision_loss)]
        let seconds = millis as f64 / 1000.0;
        format!("{seconds:.1} s")
    })
}

/// Outputs longer than this many lines are folded (`OUTPUT_LINES`).
pub const OUTPUT_LINES: usize = 6;
