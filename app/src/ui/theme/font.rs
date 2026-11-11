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
    use super::FILES;

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
