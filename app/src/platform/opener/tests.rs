use super::*;

#[test]
fn web_mail_and_phone_links_are_opened_and_nothing_else() {
    for url in [
        "http://localhost:8080",
        "https://homeport.example/docs?q=a%20b#top",
        "HTTPS://EXAMPLE.COM",
        "mailto:ada@example.com",
        "tel:+33123456789",
        "  https://example.com  ",
    ] {
        assert_eq!(link(url).unwrap(), url.trim(), "{url}");
    }
    for url in [
        "",
        "https://",
        "file:///etc/passwd",
        "/usr/bin/xterm",
        "javascript:alert(1)",
        "vscode://open",
        "smb://server/share",
        "example.com",
        "https://example.com/a b",
        "https://example.com/\nfile:///etc/passwd",
        "-h",
    ] {
        let refused = link(url).unwrap_err();
        assert_eq!(refused.code, ErrorCode::Validation, "{url}");
        assert_eq!(refused.key, "errors.file.linkRefused", "{url}");
        assert_eq!(
            refused.message,
            format!(
                "{} isn't a web, mail or phone link, so Homeport didn't open it.",
                url.trim()
            )
        );
        // Nothing sends the user to open a `file:` address elsewhere,
        // and nothing in English is added to what the catalogue says.
        assert_eq!((refused.hint, refused.detail), (None, None), "{url}");
    }
    // A link the app does open and the OS could not is another matter.
    let failed = link_not_opened("https://example.com");
    assert_eq!(failed.key, "errors.file.linkNotOpened");
    assert!(failed.hint.is_some());
}

#[test]
fn each_system_is_asked_to_show_a_file_its_own_way() {
    let path = Path::new("/home/ada/My logs/it's, 100%.txt");
    let finder = reveal_command(Os::Macos, path).unwrap();
    assert_eq!(finder.program, "open");
    assert_eq!(finder.arguments, ["-R".into(), OsString::from(path)]);
    assert_eq!(finder.raw, None);

    // Explorer: one argument, the path in quotes of its own, with the
    // comma and the spaces that split it otherwise.
    let explorer = reveal_command(Os::Windows, Path::new(r"C:\Users\ada\My logs\a, b.txt"));
    let explorer = explorer.unwrap();
    assert_eq!(
        explorer.line(),
        OsString::from(r#"explorer.exe /select,"C:\Users\ada\My logs\a, b.txt""#)
    );
    assert!(
        explorer.arguments.is_empty(),
        "nothing is left to the OS to quote"
    );

    let files = reveal_command(Os::Linux, path).unwrap();
    assert_eq!(files.program, "gdbus");
    // Nothing of the path can end the text it is quoted in.
    assert_eq!(
        files.arguments[8],
        OsString::from("['file:///home/ada/My%20logs/it%27s%2C%20100%25.txt']")
    );
    assert_eq!(files.arguments[9], OsString::new());
    assert_eq!(files.arguments.len(), 10);
    assert_eq!(files.raw, None);
}

#[test]
fn a_path_is_a_uri_only_from_the_root() {
    assert_eq!(
        file_uri(Path::new("/tmp/日本.txt")).as_deref(),
        Some("file:///tmp/%E6%97%A5%E6%9C%AC.txt")
    );
    // Every byte that is not plainly safe, whatever it is.
    assert_eq!(
        file_uri(Path::new("/a b/#?&=+;'\"[]/c~d-e_f.g")).as_deref(),
        Some("file:///a%20b/%23%3F%26%3D%2B%3B%27%22%5B%5D/c~d-e_f.g")
    );
    for relative in ["logs.txt", "./logs.txt", "../logs.txt", ""] {
        assert_eq!(file_uri(Path::new(relative)), None, "{relative:?}");
        for os in [Os::Linux, Os::Macos] {
            let refused = reveal_command(os, Path::new(relative)).unwrap_err();
            assert_eq!(refused.kind(), io::ErrorKind::InvalidInput);
        }
    }
}
