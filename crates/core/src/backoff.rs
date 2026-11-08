//! How long a supervised connection (a Host's, a Cluster's) waits before
//! trying again.

const MAX_BACKOFF_SECS: u32 = 60;

/// The next wait after `current` seconds: doubling from 1 up to 60.
pub fn next_backoff(current: u32) -> u32 {
    (current.max(1) * 2).min(MAX_BACKOFF_SECS)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backoff_doubles_up_to_a_minute() {
        let waits: Vec<_> = std::iter::successors(Some(1), |&b| Some(next_backoff(b)))
            .take(8)
            .collect();
        assert_eq!(waits, [1, 2, 4, 8, 16, 32, 60, 60]);
    }
}
