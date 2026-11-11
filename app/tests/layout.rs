//! The iced app's layout rules (`docs/agents/iced-migration.md`, The target
//! architecture), checked on every test run: which part may use which, that
//! nothing of Tauri comes back, how long a file and how full a folder may
//! get, and where tests and modules sit.

use std::{
    fs,
    path::{Path, PathBuf},
};

/// Lines of a file before its tests. Past this it holds more than one thing.
const MAX_LINES: usize = 600;

/// Files and folders directly in one folder of sources.
const MAX_ENTRIES: usize = 14;

/// Lines of tests a file may hold itself; longer ones go to `tests.rs`.
const MAX_INLINE_TESTS: usize = 100;

/// What each part of `src/` may not name, as (part, paths it must not use, why).
const FORBIDDEN: &[(&str, &[&str], &str)] = &[
    (
        "api",
        &[
            "crate::ui",
            "iced::",
            "iced_",
            "crate::data",
            "crate::platform",
        ],
        "a use case knows no window: `cargo test` runs it without one",
    ),
    (
        "app",
        &[
            "crate::ui",
            "crate::api",
            "iced::",
            "crate::data",
            "crate::platform",
        ],
        "the state stands below the use cases and the window",
    ),
    (
        "i18n",
        &[
            "crate::ui",
            "crate::api",
            "iced::",
            "crate::data",
            "crate::platform",
        ],
        "text is looked up by anything, so it depends on nothing above the crates",
    ),
    (
        "data",
        &["crate::ui::features", "crate::ui::shell"],
        "the data layer serves every page, so it knows none",
    ),
    (
        "ui/widgets",
        &[
            "crate::ui::features",
            "crate::ui::shell",
            "crate::api",
            "crate::data",
        ],
        "a widget has no domain knowledge",
    ),
    (
        "ui/theme",
        &[
            "crate::ui::features",
            "crate::ui::shell",
            "crate::api",
            "crate::data",
        ],
        "the theme styles widgets, whatever they show",
    ),
];

/// Crates and paths that belong to the Tauri app and must not come back.
const TAURI: &[&str] = &["tauri", "specta", "tauri_specta"];

fn src() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

/// Every folder of modules under `folder`, itself included.
fn folders(folder: &Path) -> Vec<PathBuf> {
    let mut all = vec![];
    let mut pending = vec![folder.to_owned()];
    while let Some(folder) = pending.pop() {
        for entry in fs::read_dir(&folder).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                pending.push(path);
            }
        }
        all.push(folder);
    }
    all
}

fn rust_files(folder: &Path) -> Vec<PathBuf> {
    if !folder.is_dir() {
        return vec![];
    }
    folders(folder)
        .iter()
        .flat_map(|folder| fs::read_dir(folder).unwrap())
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|e| e == "rs"))
        .collect()
}

fn shown(path: &Path) -> String {
    path.strip_prefix(src()).unwrap().display().to_string()
}

/// The line a file's own `mod tests { … }` starts at, if it holds one.
fn tests_start(text: &str) -> Option<usize> {
    let lines: Vec<&str> = text.lines().collect();
    (1..lines.len()).find(|&i| {
        lines[i] == "mod tests {"
            && lines[i - 1].starts_with("#[cfg(")
            && lines[i - 1].contains("test")
    })
}

/// The lines of code of a file: no comment, and nothing of its tests.
fn code(path: &Path) -> Vec<String> {
    if path.file_name().is_some_and(|n| n == "tests.rs") {
        return vec![];
    }
    let text = fs::read_to_string(path).unwrap();
    let end = tests_start(&text).unwrap_or(usize::MAX);
    text.lines()
        .take(end)
        .map(str::trim_start)
        .filter(|line| !line.starts_with("//"))
        .map(str::to_owned)
        .collect()
}

#[test]
fn dependencies_point_inward() {
    let mut wrong = vec![];
    for (part, forbidden, why) in FORBIDDEN {
        for path in rust_files(&src().join(part)) {
            for line in code(&path) {
                if let Some(used) = forbidden.iter().find(|f| line.contains(**f)) {
                    wrong.push(format!("{} uses {used} ({why})", shown(&path)));
                }
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "features use widgets, the shell, data and api; api uses app; app uses the crates: {wrong:#?}"
    );
}

#[test]
fn a_feature_never_uses_another_feature() {
    let features = src().join("ui/features");
    let mut wrong = vec![];
    if features.is_dir() {
        for entry in fs::read_dir(&features).unwrap() {
            let feature = entry.unwrap().path();
            if !feature.is_dir() {
                continue;
            }
            let own = format!(
                "crate::ui::features::{}",
                feature.file_name().unwrap().to_string_lossy()
            );
            for path in rust_files(&feature) {
                for line in code(&path) {
                    let other = line.contains("crate::ui::features::") && !line.contains(&own);
                    if other || line.contains("super::super::") {
                        wrong.push(format!("{}: {line}", shown(&path)));
                    }
                }
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "what two features share goes to ui/shell or data: {wrong:#?}"
    );
}

#[test]
fn nothing_of_tauri_comes_back() {
    let manifest = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"));
    let mut wrong: Vec<String> = manifest
        .unwrap()
        .lines()
        .filter(|line| TAURI.iter().any(|t| line.starts_with(t)))
        .map(str::to_owned)
        .collect();
    for path in rust_files(&src()) {
        for line in code(&path) {
            let names = TAURI
                .iter()
                .any(|t| line.contains(&format!("{t}::")) || line.contains(&format!("use {t}")));
            if names {
                wrong.push(format!("{}: {line}", shown(&path)));
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "the iced app has no webview and no IPC: {wrong:#?}"
    );
}

#[test]
fn files_stay_short() {
    let mut long = vec![];
    for path in rust_files(&src()) {
        if path.file_name().is_some_and(|n| n == "tests.rs") {
            continue;
        }
        let text = fs::read_to_string(&path).unwrap();
        let lines = tests_start(&text).unwrap_or_else(|| text.lines().count());
        if lines > MAX_LINES {
            long.push(format!("{}: {lines} lines", shown(&path)));
        }
    }
    assert!(
        long.is_empty(),
        "split a file past {MAX_LINES} lines before its tests: {long:#?}"
    );
}

#[test]
fn long_tests_sit_beside_the_module() {
    let mut inline = vec![];
    for path in rust_files(&src()) {
        let text = fs::read_to_string(&path).unwrap();
        let Some(start) = tests_start(&text) else {
            continue;
        };
        let lines = text.lines().count() - start;
        if lines > MAX_INLINE_TESTS {
            inline.push(format!("{}: {lines} lines of tests", shown(&path)));
        }
    }
    assert!(
        inline.is_empty(),
        "move tests past {MAX_INLINE_TESTS} lines to a tests.rs beside the module: {inline:#?}"
    );
}

#[test]
fn folders_stay_small() {
    let mut full = vec![];
    for folder in folders(&src()) {
        let entries = fs::read_dir(&folder).unwrap().count();
        if entries > MAX_ENTRIES {
            full.push(format!("{}: {entries} entries", shown(&folder)));
        }
    }
    assert!(
        full.is_empty(),
        "group a folder past {MAX_ENTRIES} entries into subfolders: {full:#?}"
    );
}

#[test]
fn a_module_with_a_folder_is_its_mod_rs() {
    let beside: Vec<String> = rust_files(&src())
        .into_iter()
        .filter(|path| path.with_extension("").is_dir())
        .map(|path| shown(&path))
        .collect();
    assert!(
        beside.is_empty(),
        "name it mod.rs inside its folder, as every other module: {beside:?}"
    );
}
