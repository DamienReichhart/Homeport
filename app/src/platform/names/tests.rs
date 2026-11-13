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
