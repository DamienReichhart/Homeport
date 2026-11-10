//! Kubernetes quantities (`250m`, `512Mi`, `1.5Gi`, `100n`, `2k`, `1e3`) as
//! numbers: a signed decimal number, then a binary suffix (`Ki`…`Ei`), a
//! decimal one (`n`, `u`, `m`, none, `k`, `M`…`E`) or an exponent (`e3`, `E-2`).

/// A quantity's value: `"250m"` is 0.25, `"1Ki"` 1024, `"1e3"` 1000; `None`
/// when the text isn't a quantity.
pub fn parse(text: &str) -> Option<f64> {
    let (number, suffix) = split(text)?;
    let value = number.parse::<f64>().ok()? * scale(suffix)?;
    value.is_finite().then_some(value)
}

/// A CPU quantity in millicores: `"250m"` is 250, `"2"` 2000, `"1500000n"` 1.5.
pub fn millicores(text: &str) -> Option<f64> {
    parse(text).map(|cores| cores * 1000.0)
}

/// A CPU quantity in whole millicores, rounded up as Kubernetes does; `None`
/// when negative.
pub fn whole_millicores(text: &str) -> Option<u64> {
    whole(millicores(text)?)
}

/// A memory quantity in bytes, rounded up as Kubernetes does: `"512Mi"` is
/// 536870912, `"1k"` 1000; `None` when negative.
pub fn bytes(text: &str) -> Option<u64> {
    whole(parse(text)?)
}

fn whole(value: f64) -> Option<u64> {
    // A value as large as a u64 or larger saturates, which no Cluster reaches.
    (value >= 0.0).then(|| value.ceil() as u64)
}

/// The number and the suffix after it; `None` without a digit.
fn split(text: &str) -> Option<(&str, &str)> {
    let b = text.as_bytes();
    let mut i = usize::from(matches!(b.first(), Some(b'+' | b'-')));
    let digits = |i: &mut usize| {
        let start = *i;
        while b.get(*i).is_some_and(u8::is_ascii_digit) {
            *i += 1;
        }
        *i - start
    };
    let mut count = digits(&mut i);
    if b.get(i) == Some(&b'.') {
        i += 1;
        count += digits(&mut i);
    }
    (count > 0).then(|| text.split_at(i))
}

/// What a suffix multiplies its number by.
fn scale(suffix: &str) -> Option<f64> {
    Some(match suffix {
        "" => 1.0,
        "n" => 1e-9,
        "u" => 1e-6,
        "m" => 1e-3,
        "k" => 1e3,
        "M" => 1e6,
        "G" => 1e9,
        "T" => 1e12,
        "P" => 1e15,
        "E" => 1e18,
        "Ki" => 1024f64,
        "Mi" => 1024f64.powi(2),
        "Gi" => 1024f64.powi(3),
        "Ti" => 1024f64.powi(4),
        "Pi" => 1024f64.powi(5),
        "Ei" => 1024f64.powi(6),
        _ => {
            let exponent = suffix.strip_prefix(['e', 'E'])?;
            let digits = exponent.strip_prefix(['+', '-']).unwrap_or(exponent);
            if digits.is_empty() || !digits.bytes().all(|c| c.is_ascii_digit()) {
                return None;
            }
            10f64.powi(exponent.parse().ok()?)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quantities_read_as_numbers() {
        let cases: &[(&str, Option<f64>)] = &[
            ("0", Some(0.0)),
            ("4", Some(4.0)),
            ("250m", Some(0.25)),
            ("100n", Some(100e-9)),
            ("12u", Some(12e-6)),
            ("2k", Some(2000.0)),
            ("3M", Some(3e6)),
            ("1G", Some(1e9)),
            ("1T", Some(1e12)),
            ("1P", Some(1e15)),
            ("2E", Some(2e18)),
            ("1Ki", Some(1024.0)),
            ("512Mi", Some(536_870_912.0)),
            ("1.5Gi", Some(1_610_612_736.0)),
            ("1Ti", Some(1_099_511_627_776.0)),
            ("1Pi", Some(1024f64.powi(5))),
            ("1Ei", Some(1024f64.powi(6))),
            ("16302048Ki", Some(16_693_297_152.0)),
            ("1e3", Some(1000.0)),
            ("1E3", Some(1000.0)),
            ("5e-3", Some(0.005)),
            ("2e+2", Some(200.0)),
            ("0.5", Some(0.5)),
            (".5", Some(0.5)),
            ("5.", Some(5.0)),
            ("+3", Some(3.0)),
            ("-1", Some(-1.0)),
            ("-500m", Some(-0.5)),
            ("", None),
            ("m", None),
            ("Mi", None),
            (".", None),
            ("-", None),
            ("1e", None),
            ("1e+", None),
            ("1e3.5", None),
            ("1KI", None),
            ("1 Mi", None),
            (" 1", None),
            ("1Mib", None),
            ("1.2.3", None),
            ("abc", None),
            ("1e999", None),
        ];
        for (text, expected) in cases {
            let got = parse(text);
            match (got, expected) {
                (Some(g), Some(e)) => assert!(
                    (g - e).abs() <= e.abs() * 1e-12,
                    "{text:?} read as {g}, not {e}"
                ),
                _ => assert_eq!(got, *expected, "{text:?}"),
            }
        }
    }

    #[test]
    fn cpu_reads_in_millicores_and_memory_in_bytes() {
        assert_eq!(millicores("250m"), Some(250.0));
        assert_eq!(millicores("2"), Some(2000.0));
        let nano = millicores("123456789n").unwrap();
        assert!((nano - 123.456_789).abs() < 1e-9);
        assert_eq!(whole_millicores("1.5"), Some(1500));
        assert_eq!(
            whole_millicores("100u"),
            Some(1),
            "rounded up, as Kubernetes does"
        );
        assert_eq!(whole_millicores("-1"), None);
        assert_eq!(bytes("512Mi"), Some(536_870_912));
        assert_eq!(bytes("1k"), Some(1000));
        assert_eq!(bytes("100m"), Some(1), "a fraction of a byte rounds up");
        assert_eq!(bytes("-1Ki"), None);
        assert_eq!(bytes("lots"), None);
    }
}
