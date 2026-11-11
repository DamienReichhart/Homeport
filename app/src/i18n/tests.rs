use jiff::tz::TimeZone;

use homeport_core::error::{AppError, ErrorCode, Message};

use super::*;

/// 2026-11-04T14:05:09Z.
const WHEN: i64 = 1_793_801_109;

fn language(tag: &str) -> Language {
    Language::new(tag, &[])
}

fn args() -> Args {
    Args::new()
}

#[test]
fn a_message_is_looked_up_by_its_dotted_key() {
    let english = language("en");
    assert_eq!(english.text("containers.title", None), "Containers");
    let summary = args().with("running", 3).with("count", 12);
    assert_eq!(
        english.text("containers.summary", Some(&summary.0)),
        "3 running of 12"
    );
    assert_eq!(language("fr").text("containers.title", None), "Conteneurs");
}

#[test]
fn plurals_follow_the_language() {
    let count = |n: u32| args().with("count", n);
    let english = language("en");
    assert_eq!(
        english.text("hosts.import.submit", Some(&count(0).0)),
        "Import"
    );
    assert_eq!(
        english.text("hosts.import.submit", Some(&count(1).0)),
        "Import 1 host"
    );
    assert_eq!(
        english.text("hosts.import.submit", Some(&count(1200).0)),
        "Import 1,200 hosts"
    );
    let arabic = language("ar");
    let two = arabic.text("hosts.import.submit", Some(&count(2).0));
    let eleven = arabic.text("hosts.import.submit", Some(&count(11).0));
    assert_ne!(two, eleven);
    assert!(eleven.contains("11"), "{eleven}");
}

#[test]
fn an_unknown_key_reads_as_itself() {
    assert_eq!(language("fr").text("nothing.here", None), "nothing.here");
    assert!(!language("fr").has("nothing.here"));
    assert!(language("fr").has("containers.title"));
}

#[test]
fn the_languages_are_english_first_and_arabic_is_right_to_left() {
    let tags: Vec<_> = LOCALES.iter().map(|l| l.tag).collect();
    assert_eq!(
        tags,
        ["en", "ar", "de", "es", "fr", "hi", "it", "ja", "ko", "pt-BR", "ru", "zh-CN"]
    );
    for locale in LOCALES.iter() {
        assert!(!locale.name.is_empty());
        assert_eq!(
            locale.direction.is_rtl(),
            locale.tag == "ar",
            "{}",
            locale.tag
        );
    }
    assert_eq!(LOCALES[4].name, "Français");
}

#[test]
fn a_message_with_tags_becomes_spans_and_arguments_stay_text() {
    let english = language("en");
    let given = args().rich("running", "<mono>3</mono>").rich("total", 5);
    let text = english.text("overview.host.running", Some(&given.0));
    assert_eq!(
        rich::spans(&text),
        [
            Span {
                text: "<mono>3</mono>".to_owned(),
                tag: Some(Tag::Strong),
            },
            Span {
                text: " / 5 running".to_owned(),
                tag: None,
            },
        ]
    );
}

#[test]
fn errors_read_in_the_language_or_in_english_without_a_key() {
    const GONE: Message = Message {
        key: "errors.nothing.here",
        en: "It is gone.",
    };
    let unknown = AppError::new(ErrorCode::NotFound, GONE);
    assert_eq!(errors::message(&unknown), "It is gone.");
}

#[test]
fn switching_the_language_changes_what_the_app_shows() {
    // The language is the whole process's: no other test may draw meanwhile.
    let _language = crate::test_support::snapshot::in_language("en");
    assert_eq!(set_language("ar", &[]), "ar");
    assert_eq!(direction(), Direction::Rtl);
    assert_eq!(
        crate::t!("containers.title"),
        language("ar").text("containers.title", None)
    );
    assert_eq!(set_language("system", &["de-AT".to_owned()]), "de");
    assert_eq!(current().formats.tag(), "de-AT");
    assert_eq!(crate::t!("containers.title"), "Container");
    assert_eq!(format::bytes(1536.0), "1,5 KB");
    // The crates render the text the OS draws: it follows too.
    let localize = |key: &str| homeport_core::error::localize(key, &Default::default());
    assert_eq!(set_language("fr", &[]), "fr");
    assert_eq!(
        localize("native.tray.status.connected").as_deref(),
        Some("Connecté")
    );
    let host = homeport_core::error::Params::from([("host".to_owned(), "prod".into())]);
    let line = homeport_core::error::localize("native.tray.host", &host).unwrap();
    assert!(line.starts_with("prod — "), "{line}");
    assert_eq!(set_language("en", &[]), "en");
    assert_eq!(localize("native.tray.status.connected"), None);
    assert_eq!(direction(), Direction::Ltr);
    assert_eq!(
        crate::t!("containers.summary", running = 1, count = 2),
        "1 running of 2"
    );
}

#[test]
fn formats_in_english() {
    let formats = &language("en").formats;
    assert_eq!(formats.bytes_parts(0.0, 1).1, "0");
    assert_eq!(
        formats.bytes_parts(512.0, 1),
        ("format.bytes.b", "512".to_owned())
    );
    assert_eq!(
        formats.bytes_parts(1536.0, 1),
        ("format.bytes.kb", "1.5".to_owned())
    );
    let three_gb = 3.0 * 1024f64.powi(3);
    assert_eq!(
        formats.bytes_parts(three_gb, 1),
        ("format.bytes.gb", "3.0".to_owned())
    );
    assert_eq!(formats.bytes_parts(1000.0, 1).1, "1000");
    assert_eq!(formats.percent(3.456), "3.5%");
    assert_eq!(formats.percent(250.0), "250%");
    assert_eq!(formats.number(12345.0), "12,345");
    let now = 1_700_000_000;
    assert_eq!(formats.relative(now - 1, now), "1 second ago");
    assert_eq!(formats.relative(now - 90, now), "2 minutes ago");
    assert_eq!(formats.relative(now - 3 * 86400, now), "3 days ago");
    assert_eq!(formats.relative(now + 3600, now), "in 1 hour");
    assert_eq!(formats.relative(now - 86400, now), "yesterday");
    assert_eq!(formats.date(WHEN, &TimeZone::UTC), "Nov 4, 2026");
    assert_eq!(
        format::short_id("sha256:0123456789abcdef0123"),
        "0123456789ab"
    );
    assert_eq!(format::short_id("abc"), "abc");
}

#[test]
fn formats_in_french() {
    let formats = &language("fr").formats;
    assert_eq!(formats.bytes_parts(1536.0, 1).1, "1,5");
    assert_eq!(formats.percent(3.456), "3,5\u{a0}%");
    assert_eq!(formats.number(12345.0), "12\u{202f}345");
    let now = 1_700_000_000;
    assert_eq!(formats.relative(now - 1, now), "il y a 1 seconde");
    assert_eq!(formats.relative(now - 90, now), "il y a 2 minutes");
    assert_eq!(formats.date(WHEN, &TimeZone::UTC), "4 nov. 2026");
    assert_eq!(formats.time(WHEN, &TimeZone::UTC), "14:05:09");
}

/// One number, percentage, size, relative time and date in every language:
/// each is written, and not the way English writes it unless the language
/// does the same.
#[test]
fn every_language_formats_numbers_dates_sizes_and_relative_times() {
    let now = 1_700_000_000;
    let expected = [
        ("en", "12,345", "3 days ago", "Nov 4, 2026"),
        ("ar", "12,345", "قبل 3 أيام", "04\u{200f}/11\u{200f}/2026"),
        ("de", "12.345", "vor 3 Tagen", "04.11.2026"),
        ("es", "12.345", "hace 3 días", "4 nov 2026"),
        ("fr", "12\u{202f}345", "il y a 3 jours", "4 nov. 2026"),
        ("hi", "12,345", "3 दिन पहले", "4 नव॰ 2026"),
        ("it", "12.345", "3 giorni fa", "4 nov 2026"),
        ("ja", "12,345", "3 日前", "2026/11/04"),
        ("ko", "12,345", "3일 전", "2026/11/4"),
        ("pt-BR", "12.345", "há 3 dias", "4 de nov. de 2026"),
        ("ru", "12\u{a0}345", "3 дня назад", "4 нояб. 2026\u{202f}г."),
        ("zh-CN", "12,345", "3天前", "2026年11月4日"),
    ];
    assert_eq!(expected.len(), LOCALES.len());
    for (tag, number, relative, date) in expected {
        let language = language(tag);
        let formats = &language.formats;
        assert_eq!(formats.number(12345.0), number, "{tag} number");
        assert_eq!(
            formats.relative(now - 3 * 86400, now),
            relative,
            "{tag} relative"
        );
        assert_eq!(formats.date(WHEN, &TimeZone::UTC), date, "{tag} date");
        let (unit, value) = formats.bytes_parts(1536.0, 1);
        let size = language.text(unit, Some(&args().with("value", value).0));
        assert!(
            size.chars().count() >= 5 && !size.contains("format."),
            "{tag} size {size}"
        );
        assert!(!formats.percent(3.456).is_empty(), "{tag} percent");
        assert!(
            !formats.date_time(WHEN, &TimeZone::UTC).is_empty(),
            "{tag} date and time"
        );
        assert!(!formats.time(WHEN, &TimeZone::UTC).is_empty(), "{tag} time");
    }
}
