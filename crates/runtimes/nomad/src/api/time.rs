//! Nomad's times, which it gives in nanoseconds since the Unix epoch, as the
//! RFC 3339 text the frontend reads.

/// `nanos` as RFC 3339; `None` for zero, Nomad's "never".
pub(crate) fn iso(nanos: i64) -> Option<String> {
    if nanos <= 0 {
        return None;
    }
    jiff::Timestamp::from_nanosecond(i128::from(nanos))
        .ok()
        .map(|t| t.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nanoseconds_become_rfc_3339() {
        assert_eq!(
            iso(1_762_000_000_000_000_000).as_deref(),
            Some("2025-11-01T12:26:40Z")
        );
        assert_eq!(iso(0), None);
    }
}
