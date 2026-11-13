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

/// The device names Windows keeps for itself, with any extension.
const RESERVED: &[&str] = &[
    "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
    "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
];

/// What stands for a character no file name may hold.
const STAND_IN: char = '_';

fn forbidden(c: char) -> bool {
    c.is_control() || matches!(c, '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|')
}

/// Whether `name` is one of Windows' devices: `nul`, `COM1.txt`.
fn reserved(name: &str) -> bool {
    let stem = name.split('.').next().unwrap_or(name).trim_end();
    RESERVED.iter().any(|r| r.eq_ignore_ascii_case(stem))
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
/// system refuses, no dot or space at its end, never a Windows device,
/// at most 255 bytes with its extension kept, and `fallback` when nothing
/// is left of it. Letters of any script stay as they are.
pub fn suggested(name: &str, fallback: &str) -> String {
    let cleaned: String = name
        .chars()
        .map(|c| if forbidden(c) { STAND_IN } else { c })
        .collect();
    let trimmed = cleaned.trim_start().trim_end_matches(['.', ' ']);
    // A name of dots and stand-ins only says nothing: `..`, `/`.
    if trimmed.chars().all(|c| c == STAND_IN || c == '.') {
        return fallback.to_owned();
    }
    let guarded = if reserved(trimmed) {
        format!("{STAND_IN}{trimmed}")
    } else {
        trimmed.to_owned()
    };
    shortened(&guarded)
}

/// `name` within [`MAX_BYTES`]: the stem gives way, the extension stays.
fn shortened(name: &str) -> String {
    if name.len() <= MAX_BYTES {
        return name.to_owned();
    }
    // An extension is short and has no space; anything else is the name.
    let extension = name
        .rfind('.')
        .filter(|&dot| dot > 0)
        .map(|dot| &name[dot..])
        .filter(|ext| ext.len() <= 16 && !ext.contains(' '));
    match extension {
        Some(ext) => {
            let stem = fitting(&name[..name.len() - ext.len()], MAX_BYTES - ext.len());
            format!("{}{ext}", stem.trim_end_matches(['.', ' ']))
        }
        None => fitting(name, MAX_BYTES)
            .trim_end_matches(['.', ' '])
            .to_owned(),
    }
}

/// [`suggested`] for `name` with `extension` after it (`"tar"`):
/// `web` and `tar` give `web.tar`.
pub fn with_extension(name: &str, extension: &str, fallback: &str) -> String {
    let extension = extension.trim_start_matches('.');
    let base = suggested(name, fallback);
    // The stem is cut again if the extension made the whole too long.
    shortened(&format!("{base}.{extension}"))
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
        // Names that only start like one are left alone.
        for name in ["console.log", "COM10", "auxiliary", "nulls.txt", "LPT0"] {
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
        assert_eq!(with_extension("web", ".tar", "export"), "web.tar");
        assert_eq!(with_extension("/", "tar", "export"), "export.tar");
        assert_eq!(with_extension("nul", "txt", "logs"), "_nul.txt");
        let long = with_extension(&"e".repeat(255), "tar", "export");
        assert_eq!(long.len(), 255);
        assert!(long.ends_with("e.tar"));
    }
}
