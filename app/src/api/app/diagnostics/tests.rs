use std::io::Read;

use super::*;

#[test]
fn the_zip_holds_the_summary_and_logs_without_their_secrets() {
    let mut aliases = Aliases::default();
    aliases.add("prod", "host-1");
    let bundle = build(
        r#"{"app":{"version":"1.0.0"}}"#,
        vec![(
            "Homeport.log".to_owned(),
            "[WARN] host prod: Authorization: Bearer abcdefgh12345678 refused\n".to_owned(),
        )],
        &aliases,
    );
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("homeport-diagnostics.zip");
    write_zip(&path, &bundle).unwrap();

    let mut zip = zip::ZipArchive::new(std::fs::File::open(&path).unwrap()).unwrap();
    let names: Vec<String> = zip.file_names().map(str::to_owned).collect();
    assert_eq!(names.len(), 2, "{names:?}");
    let mut read = |name: &str| {
        let mut text = String::new();
        zip.by_name(name)
            .unwrap()
            .read_to_string(&mut text)
            .unwrap();
        text
    };
    assert_eq!(read("summary.json"), r#"{"app":{"version":"1.0.0"}}"#);
    assert_eq!(
        read("logs/Homeport.log"),
        "[WARN] host host-1: Authorization: [redacted] refused\n"
    );
}

#[test]
fn a_missing_log_folder_leaves_the_summary_alone() {
    assert!(logs(Path::new("/nonexistent/homeport/logs")).is_empty());
}

/// A log folder holding one log with a bearer token, and a file that isn't a log.
fn log_dir(dir: &Path) -> PathBuf {
    let logs = dir.join("logs");
    std::fs::create_dir_all(&logs).unwrap();
    std::fs::write(
        logs.join("Homeport.log"),
        "[WARN] Authorization: Bearer abcdefgh12345678 refused\n",
    )
    .unwrap();
    std::fs::write(logs.join("notes.txt"), "not a log").unwrap();
    logs
}

#[tokio::test]
async fn the_preview_lists_the_summary_then_each_log() {
    let app = crate::app::testing::app();
    let logs = log_dir(app.dir.path());
    crate::api::app::history::testing::host(&app.state);

    let named = diagnostics_preview(&app.state, Installer::Deb, &logs, true).unwrap();
    assert!(named.summary.contains("prod"), "{}", named.summary);
    let preview = diagnostics_preview(&app.state, Installer::Deb, &logs, false).unwrap();
    assert!(!preview.summary.contains("prod"), "{}", preview.summary);

    let names: Vec<&str> = preview.files.iter().map(|f| f.name.as_str()).collect();
    assert_eq!(names, ["summary.json", "logs/Homeport.log"]);
    assert_eq!(preview.files[0].size as usize, preview.summary.len());
    let summary: serde_json::Value = serde_json::from_str(&preview.summary).unwrap();
    assert_eq!(summary["app"]["version"], env!("CARGO_PKG_VERSION"));
    assert_eq!(summary["app"]["bundle"], "deb");
    assert_eq!(summary["app"]["os"], std::env::consts::OS);
    assert_eq!(summary["hosts"][0]["connection"], "local");
}

#[tokio::test]
async fn the_export_writes_what_the_preview_showed() {
    let app = crate::app::testing::app();
    let logs = log_dir(app.dir.path());
    let path = app.dir.path().join("homeport-diagnostics.zip");

    let exported = diagnostics_export(
        &app.state,
        Installer::Unknown,
        &logs,
        path.to_string_lossy().into_owned(),
        true,
    );
    exported.unwrap();

    let mut zip = zip::ZipArchive::new(std::fs::File::open(&path).unwrap()).unwrap();
    let mut log = String::new();
    zip.by_name("logs/Homeport.log")
        .unwrap()
        .read_to_string(&mut log)
        .unwrap();
    assert_eq!(log, "[WARN] Authorization: [redacted] refused\n");
    let mut summary = String::new();
    zip.by_name("summary.json")
        .unwrap()
        .read_to_string(&mut summary)
        .unwrap();
    let preview = diagnostics_preview(&app.state, Installer::Unknown, &logs, true).unwrap();
    assert_eq!(summary, preview.summary);
}

#[tokio::test]
async fn a_folder_that_cant_be_written_to_fails_the_export() {
    let app = crate::app::testing::app();
    let path = app.dir.path().join("missing/homeport-diagnostics.zip");

    let exported = diagnostics_export(
        &app.state,
        Installer::Unknown,
        app.dir.path(),
        path.to_string_lossy().into_owned(),
        false,
    );

    assert!(exported.is_err());
}

#[test]
fn every_installer_has_its_name_in_the_summary() {
    let names = [
        (Installer::AppImage, "appimage"),
        (Installer::Deb, "deb"),
        (Installer::Rpm, "rpm"),
        (Installer::Msi, "msi"),
        (Installer::Nsis, "nsis"),
        (Installer::App, "app"),
        (Installer::Unknown, "unknown"),
    ];
    for (installer, name) in names {
        assert_eq!(bundle_type(installer), name);
    }
}
