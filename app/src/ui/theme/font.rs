//! The two typefaces (docs/BRAND.md, Type), bundled so the app looks the
//! same on a machine that has neither. Both are under the SIL Open Font
//! License; the texts sit beside the files in `app/assets/fonts`.
//!
//! **Host Grotesk** is a variable font (300 to 800) upstream, but
//! cosmic-text 0.15 takes a family's face as the default only when its
//! *registered* weight is the one asked for, and a variable file registers
//! one weight (300): every other weight would fall to a system font. So the
//! four weights the brand uses are bundled as static instances of the
//! variable file (see the README beside them).
//!
//! **Fragment Mono** has one weight on purpose: emphasis in machine text is
//! colour or italic, never bold, so [`MONO`] is never asked for a weight
//! that would be faked.
//!
//! # Other scripts
//!
//! Neither face has Arabic, Devanagari, Hangul, kana or Han glyphs, and the
//! app bundles none: cosmic-text falls back per script to the fonts of the
//! OS (Noto Sans Arabic/Devanagari/CJK on Linux; Nirmala UI, Yu Gothic,
//! Malgun Gothic and Microsoft YaHei on Windows; Geeza Pro, Devanagari
//! Sangam MN, Hiragino Sans and PingFang on macOS), as the Tauri webview did.
//! Noto Sans Arabic and Devanagari would cost a few hundred kilobytes each,
//! but one weight of Noto Sans CJK is about 16 MB per language, to serve
//! machines set to Japanese, Korean or Chinese, which have those fonts.
//! Bundling only the two small scripts would make the gap less predictable,
//! not smaller. What is lost: on a Linux without Noto, text in those scripts
//! shows as boxes, as it does in every other application there.

use std::borrow::Cow;

use iced::{
    font::{Family, Stretch, Style, Weight},
    Font,
};

/// The family names, as the files declare them.
pub const SANS_FAMILY: &str = "Host Grotesk";
pub const MONO_FAMILY: &str = "Fragment Mono";

/// The interface: body text at 400.
pub const SANS: Font = sans(Weight::Normal);
/// Labels and buttons (`font-medium`).
pub const SANS_MEDIUM: Font = sans(Weight::Medium);
/// Titles and the wordmark.
pub const SANS_SEMIBOLD: Font = sans(Weight::Semibold);
/// Display headlines.
pub const SANS_BOLD: Font = sans(Weight::Bold);

/// Anything a machine wrote or will read: ids, paths, ports, commands, logs.
pub const MONO: Font = Font {
    family: Family::Name(MONO_FAMILY),
    weight: Weight::Normal,
    stretch: Stretch::Normal,
    style: Style::Normal,
};
/// Comments in the editor, and emphasis in machine text.
pub const MONO_ITALIC: Font = Font {
    style: Style::Italic,
    ..MONO
};

/// Emphasis in the app's language: [`SANS_MEDIUM`], [`SANS_SEMIBOLD`] and
/// [`SANS_BOLD`] where the bundled face has the language's script, and
/// otherwise the nearest weight every OS has for that script.
///
/// cosmic-text 0.15 falls back to a script's font (Noto Sans Arabic, Segoe
/// UI, Geeza Pro…) only when it has a face at *exactly* the weight asked
/// for; without one it takes any font that holds the code points, and for
/// Arabic that may be one that draws the letters apart. The OS's fonts for
/// other scripts come in Regular and Bold, seldom in 500 or 600. So text in
/// such a language asks for 400 where Latin text asks for 500, and for 700
/// where it asks for 600: a label loses a shade of weight, never its
/// letters. Text that is Latin whatever the language (the wordmark, machine
/// text) keeps the constants.
pub fn medium() -> Font {
    in_language(Weight::Medium)
}

/// Titles: see [`medium`].
pub fn semibold() -> Font {
    in_language(Weight::Semibold)
}

/// Display headlines: see [`medium`].
pub fn bold() -> Font {
    in_language(Weight::Bold)
}

fn in_language(weight: Weight) -> Font {
    let language = crate::i18n::current();
    sans(if is_bundled_script(&language.locale.name) {
        weight
    } else {
        fallback_weight(weight)
    })
}

/// Whether text like `sample` is drawn by the bundled face, which has
/// Latin and nothing else: a language is judged by its own name.
pub fn is_bundled_script(sample: &str) -> bool {
    // Up to Latin Extended-B; everything past it is another script.
    sample.chars().all(|c| (c as u32) < 0x0250)
}

/// The weight to ask a fallback font for in place of `weight`.
pub const fn fallback_weight(weight: Weight) -> Weight {
    match weight {
        Weight::Semibold | Weight::Bold | Weight::ExtraBold | Weight::Black => Weight::Bold,
        _ => Weight::Normal,
    }
}

/// What the application draws text in unless told otherwise.
pub const DEFAULT: Font = SANS;

const fn sans(weight: Weight) -> Font {
    Font {
        family: Family::Name(SANS_FAMILY),
        weight,
        stretch: Stretch::Normal,
        style: Style::Normal,
    }
}

/// The weights of [`SANS_FAMILY`] the app ships: asking for another draws a
/// system font.
pub const SANS_WEIGHTS: [Weight; 4] = [
    Weight::Normal,
    Weight::Medium,
    Weight::Semibold,
    Weight::Bold,
];

const FILES: [&[u8]; 6] = [
    include_bytes!("../../../assets/fonts/HostGrotesk-Regular.ttf"),
    include_bytes!("../../../assets/fonts/HostGrotesk-Medium.ttf"),
    include_bytes!("../../../assets/fonts/HostGrotesk-SemiBold.ttf"),
    include_bytes!("../../../assets/fonts/HostGrotesk-Bold.ttf"),
    include_bytes!("../../../assets/fonts/FragmentMono-Regular.ttf"),
    include_bytes!("../../../assets/fonts/FragmentMono-Italic.ttf"),
];

/// Every bundled font file, for `iced::application(..).font(..)`.
pub fn files() -> impl Iterator<Item = Cow<'static, [u8]>> {
    FILES.into_iter().map(Cow::Borrowed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{i18n::LOCALES, test_support::snapshot::in_language as language};

    #[test]
    fn a_language_in_another_script_asks_for_weights_its_fonts_have() {
        {
            let _french = language("fr");
            assert_eq!(medium(), SANS_MEDIUM);
            assert_eq!(semibold(), SANS_SEMIBOLD);
            assert_eq!(bold(), SANS_BOLD);
        }
        for tag in ["ar", "hi", "ja", "ko", "ru", "zh-CN"] {
            let _other = language(tag);
            assert_eq!(medium(), SANS, "{tag}");
            assert_eq!(semibold(), SANS_BOLD, "{tag}");
            assert_eq!(bold(), SANS_BOLD, "{tag}");
        }
        // Every language is on one side or the other by its own name.
        let latin: Vec<&str> = LOCALES
            .iter()
            .filter(|l| is_bundled_script(&l.name))
            .map(|l| l.tag)
            .collect();
        assert_eq!(latin, ["en", "de", "es", "fr", "it", "pt-BR"]);
    }

    /// A woff2 (`wOF2`) copied here by mistake would load nothing, silently:
    /// cosmic-text reads TrueType, whose files start with version 1.0.
    #[test]
    fn every_bundled_font_is_a_truetype_file() {
        for file in FILES {
            assert_eq!(&file[..4], [0, 1, 0, 0]);
            assert!(file.len() > 40_000);
        }
    }
}
