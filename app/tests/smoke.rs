//! The real binary, started as a user would: it opens its window on a new,
//! empty data folder, builds its state and closes cleanly. Needs a display
//! (`xvfb-run` in CI), so it only runs when asked for:
//! `xvfb-run -a cargo test -p homeport-app --test smoke -- --ignored`.

use std::{
    process::{Command, Stdio},
    time::{Duration, Instant},
};

/// Long enough for a debug build on a busy runner to draw its first frame.
const PATIENCE: Duration = Duration::from_secs(90);

#[test]
#[ignore = "needs a display"]
fn the_app_starts_on_an_empty_folder_and_closes_cleanly() {
    let home = tempfile::tempdir().unwrap();
    let mut app = Command::new(env!("CARGO_BIN_EXE_homeport-iced"))
        // Its files go to the temporary folder on every OS.
        .env("HOME", home.path())
        .env("XDG_DATA_HOME", home.path().join("data"))
        .env("XDG_CONFIG_HOME", home.path().join("config"))
        .env("APPDATA", home.path().join("config"))
        .env("LOCALAPPDATA", home.path().join("data"))
        .env("HOMEPORT_EXIT_WHEN_READY", "1")
        .stdout(Stdio::null())
        .spawn()
        .expect("the app starts");

    let started = Instant::now();
    let status = loop {
        if let Some(status) = app.try_wait().unwrap() {
            break status;
        }
        if started.elapsed() > PATIENCE {
            app.kill().unwrap();
            panic!("the app was still running after {PATIENCE:?}");
        }
        std::thread::sleep(Duration::from_millis(100));
    };
    assert!(status.success(), "the app ended with {status}");

    // It got as far as opening its database, so the window was up.
    let database = walk(home.path())
        .into_iter()
        .find(|path| path.file_name().is_some_and(|name| name == "homeport.db"));
    assert!(database.is_some(), "no database under {:?}", home.path());
}

fn walk(dir: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut found = vec![];
    for entry in std::fs::read_dir(dir).into_iter().flatten().flatten() {
        let path = entry.path();
        if path.is_dir() {
            found.extend(walk(&path));
        } else {
            found.push(path);
        }
    }
    found
}
