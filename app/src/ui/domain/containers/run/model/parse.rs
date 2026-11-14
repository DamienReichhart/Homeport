//! The Run dialog's sizes, durations and counts, read as `docker run`
//! takes them (`model/parse.ts`). Each reader gives `Ok(None)` for an
//! empty field and [`Unreadable`] for one it can't read.

/// A field whose text says no value of its kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Unreadable;

/// What a field says: nothing, a value, or what can't be read.
pub type Read<T> = Result<Option<T>, Unreadable>;

/// The number at the start of `text` (`\d+(\.\d+)?`) and what follows.
fn number(text: &str) -> Option<(f64, &str)> {
    let digits = text
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(text.len());
    if digits == 0 {
        return None;
    }
    let mut end = digits;
    if let Some(rest) = text[digits..].strip_prefix('.') {
        let fraction = rest
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(rest.len());
        if fraction == 0 {
            return None;
        }
        end = digits + 1 + fraction;
    }
    Some((text[..end].parse().ok()?, &text[end..]))
}

/// What a unit letter stands for, in bytes: powers of 1024, as Docker
/// counts them.
fn unit(letter: &str) -> Option<f64> {
    let power = match letter {
        "" | "b" => 0,
        "k" => 1,
        "m" => 2,
        "g" => 3,
        "t" => 4,
        _ => return None,
    };
    Some(1024_f64.powi(power))
}

/// `512m`, `2g`, `1.5GB`, `64MiB` or `1024` (bytes) as bytes, above 0.
pub fn parse_size(text: &str) -> Read<i64> {
    let text = text.trim().to_lowercase();
    if text.is_empty() {
        return Ok(None);
    }
    let (value, rest) = number(&text).ok_or(Unreadable)?;
    let rest = rest.trim_start();
    // A unit letter, then `b` or `ib`; or `b` alone.
    let (letter, suffix) = match rest.chars().next() {
        Some(c @ ('k' | 'm' | 'g' | 't')) => (c.to_string(), &rest[1..]),
        _ => (String::new(), rest),
    };
    if !matches!(suffix, "" | "b" | "ib") {
        return Err(Unreadable);
    }
    let bytes = (value * unit(&letter).ok_or(Unreadable)?).round();
    #[allow(clippy::cast_possible_truncation)]
    let bytes = bytes as i64;
    if bytes > 0 {
        Ok(Some(bytes))
    } else {
        Err(Unreadable)
    }
}

/// Bytes as the shortest exact [`parse_size`] text: `512m`, `2g`, or
/// plain bytes.
pub fn format_size(bytes: i64) -> String {
    for (letter, power) in [("t", 4), ("g", 3), ("m", 2), ("k", 1)] {
        let size = 1024_i64.pow(power);
        if bytes % size == 0 {
            return format!("{}{letter}", bytes / size);
        }
    }
    bytes.to_string()
}

/// `30s`, `1m30s`, `2h` or `45` (seconds) as whole seconds above 0.
pub fn parse_duration(text: &str) -> Read<u32> {
    let text = text.trim().to_lowercase();
    if text.is_empty() {
        return Ok(None);
    }
    let positive = |seconds: u64| match u32::try_from(seconds) {
        Ok(seconds) if seconds > 0 => Ok(Some(seconds)),
        _ => Err(Unreadable),
    };
    if text.chars().all(|c| c.is_ascii_digit()) {
        return positive(text.parse().map_err(|_| Unreadable)?);
    }
    let mut seconds: u64 = 0;
    let mut rest = text.as_str();
    while !rest.is_empty() {
        let digits = rest
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(rest.len());
        let unit = match rest[digits..].chars().next() {
            Some('h') => 3600,
            Some('m') => 60,
            Some('s') => 1,
            _ => return Err(Unreadable),
        };
        if digits == 0 {
            return Err(Unreadable);
        }
        let count: u64 = rest[..digits].parse().map_err(|_| Unreadable)?;
        seconds = seconds.saturating_add(count.saturating_mul(unit));
        rest = &rest[digits + 1..];
    }
    positive(seconds)
}

/// Seconds as `1m30s`, `2h` or `45s`.
pub fn format_duration(seconds: u32) -> String {
    let (h, m, s) = (seconds / 3600, seconds % 3600 / 60, seconds % 60);
    let mut text = String::new();
    if h > 0 {
        text.push_str(&format!("{h}h"));
    }
    if m > 0 {
        text.push_str(&format!("{m}m"));
    }
    if s > 0 || (h == 0 && m == 0) {
        text.push_str(&format!("{s}s"));
    }
    text
}

/// A number above 0, such as a CPU count (`0.5`); `integer` refuses
/// fractions.
pub fn parse_positive(text: &str, integer: bool) -> Read<f64> {
    let text = text.trim();
    if text.is_empty() {
        return Ok(None);
    }
    match number(text) {
        Some((value, "")) if value > 0.0 && (!integer || value.fract() == 0.0) => Ok(Some(value)),
        _ => Err(Unreadable),
    }
}

/// [`parse_positive`] for a whole number of a type of its own.
pub fn parse_whole<T: TryFrom<u64>>(text: &str) -> Read<T> {
    match parse_positive(text, true)? {
        None => Ok(None),
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        Some(value) => T::try_from(value as u64).map(Some).map_err(|_| Unreadable),
    }
}
