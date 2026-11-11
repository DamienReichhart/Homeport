//! Snapshots of what a page draws: its view rendered without a window, in a
//! language, and compared with the PNG kept under `app/tests/snapshots`.
//!
//! The software renderer and built-in fonts draw the same on every OS but
//! for the last bit of anti-aliasing, which the comparison tolerates.
//! `UPDATE_SNAPSHOTS=1 cargo test -p homeport-app` rewrites the PNGs.

use std::{
    fs,
    io::BufReader,
    path::{Path, PathBuf},
    sync::{Mutex, MutexGuard},
};

use iced::{Element, Settings, Size, Theme};
use iced_test::Simulator;

use crate::i18n;

/// A font with the Arabic letters, which the default font lacks: without
/// it Arabic would be drawn in whatever the machine has installed.
const ARABIC: &[u8] = include_bytes!("../../tests/fonts/NotoSansArabic.ttf");

/// How far a colour channel may be off before a pixel counts as different.
const CHANNEL_TOLERANCE: u8 = 24;

/// The share of pixels that may differ: edges of glyphs, not a moved label.
const PIXEL_TOLERANCE: f64 = 0.002;

/// The app's language belongs to the whole process, and tests run side by
/// side: whoever draws holds this until it's done.
static LANGUAGE: Mutex<()> = Mutex::new(());

/// The app in the language `tag` until the guard is dropped, then in
/// English again. Take it in any test that reads or sets the app's language.
pub struct InLanguage(MutexGuard<'static, ()>);

pub fn in_language(tag: &str) -> InLanguage {
    let guard = LANGUAGE
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    i18n::set_language(tag, &[]);
    InLanguage(guard)
}

impl Drop for InLanguage {
    fn drop(&mut self) {
        i18n::set_language(i18n::DEFAULT_LOCALE, &[]);
    }
}

/// A decoded picture: its size and its RGBA bytes.
struct Picture {
    width: u32,
    height: u32,
    rgba: Vec<u8>,
}

fn read(path: &Path) -> Picture {
    let file = fs::File::open(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let mut reader = png::Decoder::new(BufReader::new(file))
        .read_info()
        .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let size = reader
        .output_buffer_size()
        .expect("a snapshot fits in memory");
    let mut rgba = vec![0; size];
    let info = reader.next_frame(&mut rgba).expect("a readable PNG");
    rgba.truncate(info.buffer_size());
    Picture {
        width: info.width,
        height: info.height,
        rgba,
    }
}

/// The share of pixels of `drawn` that differ from `kept`, or why the two
/// can't be compared.
fn difference(drawn: &Picture, kept: &Picture) -> Result<f64, String> {
    if (drawn.width, drawn.height) != (kept.width, kept.height) {
        return Err(format!(
            "it is {}×{}, the snapshot {}×{}",
            drawn.width, drawn.height, kept.width, kept.height
        ));
    }
    let different = drawn
        .rgba
        .chunks_exact(4)
        .zip(kept.rgba.chunks_exact(4))
        .filter(|(a, b)| {
            a.iter()
                .zip(b.iter())
                .any(|(a, b)| a.abs_diff(*b) > CHANNEL_TOLERANCE)
        })
        .count();
    // Pixel counts are far below 2^53.
    #[allow(clippy::cast_precision_loss)]
    Ok(different as f64 / (drawn.rgba.len() / 4) as f64)
}

fn snapshots() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/snapshots")
}

/// Draws what `view` builds at `size` in the language `tag` with `theme`,
/// and holds it to `tests/snapshots/<name>.png`.
///
/// `view` runs once the language is set, so its text is in that language.
pub fn assert_snapshot<'a, Message: 'a>(
    name: &str,
    tag: &str,
    theme: &Theme,
    size: impl Into<Size>,
    view: impl FnOnce() -> Element<'a, Message>,
) {
    let _language = in_language(tag);
    // Every snapshot is drawn in software, whatever the machine has.
    std::env::set_var("ICED_TEST_BACKEND", "tiny-skia");
    let settings = Settings {
        fonts: vec![ARABIC.into()],
        ..Settings::default()
    };
    let mut ui = Simulator::with_size(settings, size, view());
    let snapshot = ui.snapshot(theme).expect("the page draws without a window");

    // iced_test only writes a picture where none exists, and only compares
    // exactly: have it write a new one, then compare it here.
    let scratch = tempfile::tempdir().expect("a temporary folder");
    snapshot
        .matches_image(scratch.path().join(name))
        .expect("the snapshot is written");
    let drawn_at = scratch.path().join(format!("{name}-tiny-skia.png"));
    let kept_at = snapshots().join(format!("{name}.png"));

    if std::env::var_os("UPDATE_SNAPSHOTS").is_some() {
        fs::create_dir_all(snapshots()).expect("the snapshots folder");
        fs::copy(&drawn_at, &kept_at).expect("the snapshot is kept");
        return;
    }
    assert!(
        kept_at.exists(),
        "no snapshot is kept as {}: run the tests with UPDATE_SNAPSHOTS=1, look at it, commit it",
        kept_at.display()
    );
    let result = difference(&read(&drawn_at), &read(&kept_at));
    if !matches!(result, Ok(share) if share <= PIXEL_TOLERANCE) {
        let left = std::env::temp_dir().join("homeport-snapshots");
        fs::create_dir_all(&left).expect("a folder for what was drawn");
        let actual = left.join(format!("{name}.actual.png"));
        fs::copy(&drawn_at, &actual).expect("what was drawn is kept");
        let why = match result {
            Ok(share) => format!("{:.2} % of its pixels differ", share * 100.0),
            Err(why) => why,
        };
        panic!(
            "{name} no longer looks like its snapshot: {why}. Drawn: {}. \
             If the change is meant, run with UPDATE_SNAPSHOTS=1 and commit the PNG.",
            actual.display()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn picture(pixels: &[[u8; 4]]) -> Picture {
        Picture {
            width: u32::try_from(pixels.len()).unwrap(),
            height: 1,
            rgba: pixels.concat(),
        }
    }

    #[test]
    fn a_faint_difference_is_none_and_a_moved_pixel_is_one() {
        let kept = picture(&[[10, 10, 10, 255], [200, 200, 200, 255]]);
        let faint = picture(&[[20, 10, 10, 255], [200, 190, 200, 255]]);
        assert_eq!(difference(&faint, &kept), Ok(0.0));
        let moved = picture(&[[200, 200, 200, 255], [200, 200, 200, 255]]);
        assert_eq!(difference(&moved, &kept), Ok(0.5));
    }

    #[test]
    fn another_size_cannot_be_compared() {
        let kept = picture(&[[0, 0, 0, 255]]);
        let wider = picture(&[[0, 0, 0, 255], [0, 0, 0, 255]]);
        assert!(difference(&wider, &kept).is_err());
    }
}
