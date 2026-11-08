//! Signal names as images and users write them, as Linux numbers them.

pub(crate) const SIGTERM: u32 = 15;
pub(crate) const SIGKILL: u32 = 9;

const SIGNALS: &[(&str, u32)] = &[
    ("HUP", 1),
    ("INT", 2),
    ("QUIT", 3),
    ("ILL", 4),
    ("TRAP", 5),
    ("ABRT", 6),
    ("BUS", 7),
    ("FPE", 8),
    ("KILL", 9),
    ("USR1", 10),
    ("SEGV", 11),
    ("USR2", 12),
    ("PIPE", 13),
    ("ALRM", 14),
    ("TERM", 15),
    ("STKFLT", 16),
    ("CHLD", 17),
    ("CONT", 18),
    ("STOP", 19),
    ("TSTP", 20),
    ("TTIN", 21),
    ("TTOU", 22),
    ("URG", 23),
    ("XCPU", 24),
    ("XFSZ", 25),
    ("VTALRM", 26),
    ("PROF", 27),
    ("WINCH", 28),
    ("IO", 29),
    ("PWR", 30),
    ("SYS", 31),
];

/// `SIGTERM`, `term` or `15` as 15; `None` for a name Linux doesn't have.
pub fn signal_number(name: &str) -> Option<u32> {
    let name = name.trim();
    if let Ok(n) = name.parse::<u32>() {
        return (1..=64).contains(&n).then_some(n);
    }
    let upper = name.to_ascii_uppercase();
    let bare = upper.strip_prefix("SIG").unwrap_or(&upper);
    SIGNALS.iter().find(|(n, _)| *n == bare).map(|(_, v)| *v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signals_are_read_by_name_or_number() {
        assert_eq!(signal_number("SIGTERM"), Some(15));
        assert_eq!(signal_number("quit"), Some(3));
        assert_eq!(signal_number("SIGWINCH"), Some(28));
        assert_eq!(signal_number("9"), Some(9));
        assert_eq!(signal_number("SIGNOPE"), None);
        assert_eq!(signal_number("0"), None);
    }
}
