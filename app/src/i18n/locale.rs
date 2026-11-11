//! Which language the app shows: the one chosen in Settings, else the first
//! of the OS's preferred languages it has a catalogue for, else English.

use homeport_settings::SYSTEM_LANGUAGE;

use super::catalogue::{Locale, DEFAULT_LOCALE, LOCALES};

fn base(tag: &str) -> String {
    let tag = tag.to_lowercase();
    tag.split(['-', '_']).next().unwrap_or_default().to_owned()
}

/// Chinese written in Traditional characters isn't Simplified Chinese's to show.
fn traditional(tag: &str) -> bool {
    let tag = tag.to_lowercase().replace('_', "-");
    ["zh-hant", "zh-tw", "zh-hk", "zh-mo"]
        .iter()
        .any(|t| tag == *t || tag.starts_with(&format!("{t}-")))
}

/// The supported language for `tag`: exact (`pt-BR`), then by base
/// (`fr-CA` → `fr`).
fn matching(tag: &str) -> Option<&'static Locale> {
    let wanted = tag.replace('_', "-");
    if let Some(exact) = LOCALES.iter().find(|l| l.tag.eq_ignore_ascii_case(&wanted)) {
        return Some(exact);
    }
    if traditional(tag) {
        return None;
    }
    LOCALES.iter().find(|l| base(l.tag) == base(tag))
}

/// The language to show for the Settings value `language`, given the OS's
/// `preferred` languages in order.
pub fn resolve(language: &str, preferred: &[String]) -> &'static str {
    if language != SYSTEM_LANGUAGE {
        if let Some(chosen) = matching(language) {
            return chosen.tag;
        }
    }
    preferred
        .iter()
        .find_map(|tag| matching(tag))
        .map_or(DEFAULT_LOCALE, |l| l.tag)
}

/// The locale numbers and dates are written in: the OS's own variant of the
/// app's language when it prefers one (`en-GB`, `fr-CA`), else the language.
pub fn formatting(language: &str, preferred: &[String]) -> String {
    preferred
        .iter()
        .find(|tag| base(tag) == base(language))
        .map_or_else(|| language.to_owned(), |tag| tag.replace('_', "-"))
}

/// The OS's preferred languages, most wanted first, keeping only real
/// language tags: a `C` or `POSIX` locale (minimal installs, `LANG=C.UTF-8`)
/// is none.
pub fn preferred() -> Vec<String> {
    valid(sys_locale::get_locales())
}

fn valid(tags: impl Iterator<Item = String>) -> Vec<String> {
    tags.filter(|tag| {
        !matches!(base(tag).as_str(), "c" | "posix") && tag.parse::<icu::locale::Locale>().is_ok()
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tags(tags: &[&str]) -> Vec<String> {
        tags.iter().map(|t| (*t).to_owned()).collect()
    }

    #[test]
    fn the_chosen_language_wins_over_the_os() {
        assert_eq!(resolve("fr", &tags(&["de-DE"])), "fr");
        assert_eq!(resolve("pt-br", &[]), "pt-BR");
        assert_eq!(resolve("fr-CA", &[]), "fr");
    }

    #[test]
    fn system_follows_the_first_language_the_app_has() {
        assert_eq!(
            resolve(SYSTEM_LANGUAGE, &tags(&["nl-NL", "de-AT", "fr"])),
            "de"
        );
        assert_eq!(resolve(SYSTEM_LANGUAGE, &tags(&["pt-PT"])), "pt-BR");
        assert_eq!(resolve(SYSTEM_LANGUAGE, &tags(&["nl"])), "en");
        assert_eq!(resolve(SYSTEM_LANGUAGE, &[]), "en");
        // A language no catalogue exists for reads as the OS's.
        assert_eq!(resolve("nl", &tags(&["ja-JP"])), "ja");
    }

    #[test]
    fn traditional_chinese_is_not_shown_in_simplified() {
        assert_eq!(resolve(SYSTEM_LANGUAGE, &tags(&["zh-TW", "ko"])), "ko");
        assert_eq!(resolve(SYSTEM_LANGUAGE, &tags(&["zh-Hant-HK"])), "en");
        assert_eq!(resolve(SYSTEM_LANGUAGE, &tags(&["zh-SG"])), "zh-CN");
        assert_eq!(resolve(SYSTEM_LANGUAGE, &tags(&["zh_CN"])), "zh-CN");
    }

    #[test]
    fn formats_in_the_os_variant_of_the_language() {
        assert_eq!(formatting("en", &tags(&["fr-FR", "en-GB"])), "en-GB");
        assert_eq!(formatting("fr", &tags(&["en-US"])), "fr");
    }

    #[test]
    fn a_c_locale_is_no_language() {
        let kept = valid(tags(&["C", "POSIX", "c.UTF-8", "en-US", "not a tag", "fr"]).into_iter());
        assert_eq!(kept, ["en-US", "fr"]);
    }
}
