//! The name a save dialog suggests: one that every system this app runs
//! on takes, whatever it was made from (a container's name, an image's
//! tag, a path inside a volume).
//!
//! The rules are the same on every system, the strictest of the three, so
//! a name suggested on Linux is still a file a Windows share or a macOS
//! disk accepts.

/// The longest name most file systems take, in bytes (ext4, APFS; NTFS
/// counts 255 UTF-16 units, which UTF-8 never undercuts).
const MAX_BYTES: usize = 255;

/// The device names Windows keeps for itself, with any extension. The
/// ports count from 0 and also answer to the superscript digits.
const RESERVED: &[&str] = &[
    "CON", "PRN", "AUX", "NUL", "CONIN$", "CONOUT$", "COM0", "COM1", "COM2", "COM3", "COM4",
    "COM5", "COM6", "COM7", "COM8", "COM9", "COM¹", "COM²", "COM³", "LPT0", "LPT1", "LPT2", "LPT3",
    "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9", "LPT¹", "LPT²", "LPT³",
];

/// What stands for a character no file name may hold.
const STAND_IN: char = '_';

/// The archive formats whose extension is two parts: cut as one.
const COMPOUND: &str = ".tar";

fn forbidden(c: char) -> bool {
    c.is_control() || matches!(c, '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|')
}

/// A character that shows nothing and changes how what follows reads:
/// the format characters (Unicode's Cf: direction marks and overrides,
/// zero-width marks, the byte order mark, tags) and the line and
/// paragraph separators. A name comes from a container, which may hide
/// an extension behind them (`gpj.exe` after a right-to-left override
/// reads `exe.jpg`): they are taken out, not replaced.
fn invisible(c: char) -> bool {
    matches!(
        c,
        '\u{00AD}'
            | '\u{0600}'..='\u{0605}'
            | '\u{061C}'
            | '\u{06DD}'
            | '\u{070F}'
            | '\u{180E}'
            | '\u{200B}'..='\u{200F}'
            | '\u{2028}'..='\u{202E}'
            | '\u{2060}'..='\u{2064}'
            | '\u{2066}'..='\u{206F}'
            | '\u{FEFF}'
            | '\u{FFF9}'..='\u{FFFB}'
            | '\u{E0001}'
            | '\u{E0020}'..='\u{E007F}'
    )
}

/// Whether `name` is one of Windows' devices: `nul`, `COM1.txt`.
fn reserved(name: &str) -> bool {
    let stem = name.split('.').next().unwrap_or(name).trim_end();
    // Case is folded by the letters only: `¹` has none.
    RESERVED
        .iter()
        .any(|r| r.to_lowercase() == stem.to_lowercase())
}

/// The longest start of `text` that fits `bytes`, cut between characters.
fn fitting(text: &str, bytes: usize) -> &str {
    let mut end = text.len().min(bytes);
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    &text[..end]
}

/// `name` as a file name to suggest: no path separator or character a
/// system refuses, none that hides how it reads, no dot or space at its
/// end, never a Windows device, at most 255 bytes with its extension
/// kept, and `fallback` when nothing is left of it (an empty `fallback`
/// suggests no name). Letters of any script stay as they are.
pub fn suggested(name: &str, fallback: &str) -> String {
    let cleaned: String = name
        .chars()
        .filter(|c| !invisible(*c))
        .map(|c| if forbidden(c) { STAND_IN } else { c })
        .collect();
    let trimmed = cleaned.trim_start().trim_end_matches(['.', ' ']);
    // A name of dots and stand-ins only says nothing: `..`, `/`.
    if trimmed.chars().all(|c| c == STAND_IN || c == '.') {
        return fallback.to_owned();
    }
    // One dot hides a file; more before a name are only noise (`..env`).
    let dots = trimmed.len() - trimmed.trim_start_matches('.').len();
    let trimmed = &trimmed[dots.saturating_sub(1)..];
    let guarded = if reserved(trimmed) {
        format!("{STAND_IN}{trimmed}")
    } else {
        trimmed.to_owned()
    };
    shortened(&guarded)
}

/// Where the extension of `name` starts, if it has one: its last part,
/// with `.tar` before it when it is an archive's (`.tar.gz`). An
/// extension is short and has no space; anything else is the name.
fn extension_at(name: &str) -> Option<usize> {
    let dot = name.rfind('.').filter(|&dot| dot > 0)?;
    let last = &name[dot..];
    if last.len() > 16 || last.contains(' ') {
        return None;
    }
    let before = &name[..dot];
    let compound = before.len() > COMPOUND.len()
        && before.is_char_boundary(before.len() - COMPOUND.len())
        && before[before.len() - COMPOUND.len()..].eq_ignore_ascii_case(COMPOUND);
    Some(if compound { dot - COMPOUND.len() } else { dot })
}

/// `name` within [`MAX_BYTES`]: the stem gives way, the extension stays.
fn shortened(name: &str) -> String {
    if name.len() <= MAX_BYTES {
        return name.to_owned();
    }
    match extension_at(name) {
        Some(at) => {
            let (stem, extension) = name.split_at(at);
            let stem = fitting(stem, MAX_BYTES - extension.len());
            format!("{}{extension}", stem.trim_end_matches(['.', ' ']))
        }
        None => fitting(name, MAX_BYTES)
            .trim_end_matches(['.', ' '])
            .to_owned(),
    }
}

/// [`suggested`] for `name` ending in `extension` (`"tar"`): `web` and
/// `tar` give `web.tar`, and so does `web.tar`, which has it already.
/// With nothing left of the name and no fallback, no name is suggested.
pub fn with_extension(name: &str, extension: &str, fallback: &str) -> String {
    let extension = extension.trim_start_matches('.');
    let base = suggested(name, fallback);
    if base.is_empty() {
        return base;
    }
    let ending = format!(".{extension}");
    let has_it = base.len() > ending.len()
        && base.is_char_boundary(base.len() - ending.len())
        && base[base.len() - ending.len()..].eq_ignore_ascii_case(&ending);
    if has_it {
        return base;
    }
    // The stem is cut again if the extension made the whole too long.
    shortened(&format!("{base}{ending}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn separators_and_refused_characters_are_replaced() {
        assert_eq!(
            suggested("etc/nginx/nginx.conf", "file"),
            "etc_nginx_nginx.conf"
        );
        assert_eq!(suggested(r"C:\Users\ada", "file"), "C__Users_ada");
        assert_eq!(suggested("nginx:1.27", "image"), "nginx_1.27");
        assert_eq!(suggested("a*b?c\"d<e>f|g", "file"), "a_b_c_d_e_f_g");
        assert_eq!(suggested("tab\there\nnew\u{7f}", "file"), "tab_here_new_");
    }

    #[test]
    fn windows_devices_are_never_suggested() {
        for name in [
            "CON",
            "con",
            "Nul",
            "COM1",
            "lpt9",
            "aux.txt",
            "PRN.tar.gz",
            "nul .txt",
        ] {
            let safe = suggested(name, "file");
            assert_eq!(safe, format!("_{name}"), "{name}");
        }
        // The ports from 0, their superscript digits, and the console's two ends.
        for name in [
            "COM0",
            "lpt0.txt",
            "COM¹",
            "com².log",
            "LPT³",
            "CONIN$",
            "conout$.txt",
        ] {
            assert_eq!(suggested(name, "file"), format!("_{name}"), "{name}");
        }
        // Names that only start like one are left alone.
        for name in [
            "console.log",
            "COM10",
            "auxiliary",
            "nulls.txt",
            "LPT10",
            "COM⁴",
            "CON$",
        ] {
            assert_eq!(suggested(name, "file"), name);
        }
    }

    #[test]
    fn trailing_dots_and_spaces_go_and_empty_names_fall_back() {
        assert_eq!(suggested("logs.txt. . ", "file"), "logs.txt");
        assert_eq!(suggested("  web  ", "file"), "web");
        assert_eq!(suggested(".env", "file"), ".env");
        for nothing in ["", "   ", ".", "..", "...", "/", "//", r"\", " . "] {
            assert_eq!(suggested(nothing, "logs.txt"), "logs.txt", "{nothing:?}");
        }
        // Trimming can uncover a device.
        assert_eq!(suggested("con.", "file"), "_con");
    }

    #[test]
    fn long_names_are_cut_between_characters_and_keep_their_extension() {
        let long = format!("{}.tar", "a".repeat(400));
        let safe = suggested(&long, "file");
        assert_eq!(safe.len(), 255);
        assert!(safe.ends_with("a.tar"));

        // Three bytes a character: the cut never splits one.
        let wide = format!("{}.txt", "日".repeat(200));
        let safe = suggested(&wide, "file");
        assert!(safe.len() <= 255 && safe.len() > 250, "{}", safe.len());
        assert!(safe.ends_with("日.txt"));
        assert_eq!(safe.chars().filter(|&c| c == '日').count(), 83);

        // No extension: the name itself is cut, and its new end is clean.
        let plain = format!("{} {}", "b".repeat(254), "c".repeat(50));
        assert_eq!(suggested(&plain, "file"), "b".repeat(254));
        // A dot far from the end is part of the name, not an extension.
        let sentence = format!("v1. {}", "d".repeat(300));
        assert_eq!(suggested(&sentence, "file").len(), 255);
    }

    #[test]
    fn a_long_archive_keeps_both_parts_of_its_extension() {
        let long = format!("{}.tar.gz", "a".repeat(400));
        let safe = suggested(&long, "file");
        assert_eq!(safe.len(), 255);
        assert!(safe.ends_with("a.tar.gz"), "{safe}");
        let upper = suggested(&format!("{}.TAR.ZST", "é".repeat(300)), "file");
        assert!(
            upper.len() <= 255 && upper.ends_with("é.TAR.ZST"),
            "{upper}"
        );
        // Only an archive's: a version number is part of the name.
        let versioned = suggested(&format!("{}.1.gz", "b".repeat(400)), "file");
        assert!(
            versioned.ends_with("b.gz") && versioned.len() == 255,
            "{versioned}"
        );
        // `.tar.gz` alone is a hidden file named `.tar` with an extension.
        assert_eq!(suggested(".tar.gz", "file"), ".tar.gz");
    }

    #[test]
    fn what_shows_nothing_and_changes_how_a_name_reads_is_taken_out() {
        // A right-to-left override: `photo_gpj.exe` would read `photo_exe.jpg`.
        assert_eq!(suggested("photo_\u{202E}gpj.exe", "file"), "photo_gpj.exe");
        let marks = [
            '\u{202A}', '\u{202B}', '\u{202C}', '\u{202D}', '\u{202E}', '\u{2066}', '\u{2067}',
            '\u{2068}', '\u{2069}', '\u{200E}', '\u{200F}', '\u{2028}', '\u{2029}', '\u{200B}',
            '\u{FEFF}', '\u{00AD}', '\u{061C}',
        ];
        for mark in marks {
            let hidden = format!("lo{mark}gs.t{mark}xt");
            assert_eq!(
                suggested(&hidden, "file"),
                "logs.txt",
                "U+{:04X}",
                mark as u32
            );
        }
        // Nothing but marks is nothing, and a device behind one is still one.
        assert_eq!(suggested("\u{202E}\u{200F}", "logs.txt"), "logs.txt");
        assert_eq!(suggested("N\u{200B}UL.txt", "file"), "_NUL.txt");
        // Text in a right-to-left script has no need of them to be kept.
        assert_eq!(suggested("سجلات.txt", "file"), "سجلات.txt");
    }

    #[test]
    fn dots_before_a_name_hide_it_once() {
        assert_eq!(suggested(".env", "file"), ".env");
        assert_eq!(suggested("..env", "file"), ".env");
        assert_eq!(suggested("....txt", "file"), ".txt");
        // A stem that is only dots is no name.
        for nothing in ["..", "...", ". . .", "._.", "/../"] {
            assert_eq!(
                with_extension(nothing, "tar", "export"),
                "export.tar",
                "{nothing:?}"
            );
        }
    }

    #[test]
    fn letters_of_any_script_stay() {
        for name in [
            "日本語のログ.txt",
            "سجلات.txt",
            "journal-été.log",
            "🚀.tar",
            "Ünïcödé",
        ] {
            assert_eq!(suggested(name, "file"), name);
        }
    }

    #[test]
    fn an_extension_is_added_once_and_the_whole_still_fits() {
        assert_eq!(with_extension("web", "tar", "export"), "web.tar");
        // A name that ends with it already has it.
        assert_eq!(with_extension("web.tar", "tar", "export"), "web.tar");
        assert_eq!(with_extension("web.TAR", ".tar", "export"), "web.TAR");
        assert_eq!(with_extension("web.tar.", "tar", "export"), "web.tar");
        // One that only looks like it does not.
        assert_eq!(with_extension("webtar", "tar", "export"), "webtar.tar");
        assert_eq!(with_extension(".tar", "tar", "export"), ".tar.tar");
        // With no fallback, nothing is suggested rather than a bare extension.
        assert_eq!(with_extension("/", "tar", ""), "");
        assert_eq!(suggested("..", ""), "");
        assert_eq!(with_extension("web", ".tar", "export"), "web.tar");
        assert_eq!(with_extension("/", "tar", "export"), "export.tar");
        assert_eq!(with_extension("nul", "txt", "logs"), "_nul.txt");
        let long = with_extension(&"e".repeat(255), "tar", "export");
        assert_eq!(long.len(), 255);
        assert!(long.ends_with("e.tar"));
    }
}
