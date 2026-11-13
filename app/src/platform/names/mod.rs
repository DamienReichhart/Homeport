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
mod tests;
