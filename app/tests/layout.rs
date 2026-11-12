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
        "ui/shell",
        &["crate::ui::features"],
        "the shell hosts whatever page it is given, and knows no feature",
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

fn shown(src: &Path, path: &Path) -> String {
    path.strip_prefix(src).unwrap().display().to_string()
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

/// What uses a part it must not, under `src`.
fn wrong_dependencies(src: &Path) -> Vec<String> {
    let mut wrong = vec![];
    for (part, forbidden, why) in FORBIDDEN {
        for path in rust_files(&src.join(part)) {
            for line in code(&path) {
                if let Some(used) = forbidden.iter().find(|f| line.contains(**f)) {
                    wrong.push(format!("{} uses {used} ({why})", shown(src, &path)));
                }
            }
        }
    }
    wrong
}

/// The features under `src`: `ui/features/<group>/<name>`, as its path in
/// the crate and its folder.
fn features(src: &Path) -> Vec<(String, PathBuf)> {
    let dirs = |folder: &Path| -> Vec<PathBuf> {
        let Ok(entries) = fs::read_dir(folder) else {
            return vec![];
        };
        entries
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.is_dir())
            .collect()
    };
    let name = |path: &Path| path.file_name().unwrap().to_string_lossy().into_owned();
    let mut features = vec![];
    for group in dirs(&src.join("ui/features")) {
        for feature in dirs(&group) {
            let own = format!("crate::ui::features::{}::{}", name(&group), name(&feature));
            features.push((own, feature));
        }
    }
    features
}

/// The lines of a feature that name another feature.
fn features_using_another(src: &Path) -> Vec<String> {
    let mut wrong = vec![];
    for (own, feature) in features(src) {
        for path in rust_files(&feature) {
            for line in code(&path) {
                let other = line.contains("crate::ui::features::") && !line.contains(&own);
                // Out of its folder by `super`: its group, then the others.
                if other || line.contains("super::super::super::") {
                    wrong.push(format!("{}: {line}", shown(src, &path)));
                }
            }
        }
    }
    wrong
}

fn long_files(src: &Path) -> Vec<String> {
    let mut long = vec![];
    for path in rust_files(src) {
        if path.file_name().is_some_and(|n| n == "tests.rs") {
            continue;
        }
        let text = fs::read_to_string(&path).unwrap();
        let lines = tests_start(&text).unwrap_or_else(|| text.lines().count());
        if lines > MAX_LINES {
            long.push(format!("{}: {lines} lines", shown(src, &path)));
        }
    }
    long
}

fn long_inline_tests(src: &Path) -> Vec<String> {
    let mut inline = vec![];
    for path in rust_files(src) {
        let text = fs::read_to_string(&path).unwrap();
        let Some(start) = tests_start(&text) else {
            continue;
        };
        let lines = text.lines().count() - start;
        if lines > MAX_INLINE_TESTS {
            inline.push(format!("{}: {lines} lines of tests", shown(src, &path)));
        }
    }
    inline
}

fn full_folders(src: &Path) -> Vec<String> {
    let mut full = vec![];
    for folder in folders(src) {
        let entries = fs::read_dir(&folder).unwrap().count();
        if entries > MAX_ENTRIES {
            full.push(format!("{}: {entries} entries", shown(src, &folder)));
        }
    }
    full
}

fn modules_beside_their_folder(src: &Path) -> Vec<String> {
    rust_files(src)
        .into_iter()
        .filter(|path| path.with_extension("").is_dir())
        .map(|path| shown(src, &path))
        .collect()
}

#[test]
fn dependencies_point_inward() {
    let wrong = wrong_dependencies(&src());
    assert!(
        wrong.is_empty(),
        "features use widgets, the shell, data and api; api uses app; app uses the crates: {wrong:#?}"
    );
}

#[test]
fn a_feature_never_uses_another_feature() {
    assert!(
        features(&src()).len() > 30,
        "the features are ui/features/<group>/<name>"
    );
    let wrong = features_using_another(&src());
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
                wrong.push(format!("{}: {line}", shown(&src(), &path)));
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
    let long = long_files(&src());
    assert!(
        long.is_empty(),
        "split a file past {MAX_LINES} lines before its tests: {long:#?}"
    );
}

#[test]
fn long_tests_sit_beside_the_module() {
    let inline = long_inline_tests(&src());
    assert!(
        inline.is_empty(),
        "move tests past {MAX_INLINE_TESTS} lines to a tests.rs beside the module: {inline:#?}"
    );
}

#[test]
fn folders_stay_small() {
    let full = full_folders(&src());
    assert!(
        full.is_empty(),
        "group a folder past {MAX_ENTRIES} entries into subfolders: {full:#?}"
    );
}

#[test]
fn a_module_with_a_folder_is_its_mod_rs() {
    let beside = modules_beside_their_folder(&src());
    assert!(
        beside.is_empty(),
        "name it mod.rs inside its folder, as every other module: {beside:?}"
    );
}

/// Writes `text` as `path` under `src`, with the folders it needs.
fn write(src: &Path, path: &str, text: &str) {
    let path = src.join(path);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, text).unwrap();
}

/// The checks above are only worth what they catch: a tree that breaks
/// each rule once must fail each of them, and say where.
#[test]
fn each_rule_fails_on_a_tree_that_breaks_it() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path();
    // A tree that keeps the rules passes them all.
    write(src, "api/mod.rs", "use crate::app::AppState;\n");
    write(
        src,
        "ui/features/docker/images/mod.rs",
        "use crate::ui::shell::Context;\n",
    );
    let clean = |src: &Path| {
        wrong_dependencies(src).is_empty()
            && features_using_another(src).is_empty()
            && long_files(src).is_empty()
            && long_inline_tests(src).is_empty()
            && full_folders(src).is_empty()
            && modules_beside_their_folder(src).is_empty()
    };
    assert!(clean(src));

    write(
        src,
        "api/hosts.rs",
        "// crate::ui in a comment is fine\nuse crate::ui::theme::Theme;\n",
    );
    write(
        src,
        "ui/shell/pages.rs",
        "use crate::ui::features::docker::images;\n",
    );
    write(src, "ui/widgets/table.rs", "use crate::data::Data;\n");
    let wrong = wrong_dependencies(src);
    assert_eq!(wrong.len(), 3, "{wrong:#?}");
    assert!(wrong
        .iter()
        .any(|w| w.starts_with("api/hosts.rs uses crate::ui")));
    assert!(wrong.iter().any(|w| w.starts_with("ui/shell/pages.rs")));

    write(
        src,
        "ui/features/docker/containers/mod.rs",
        "use crate::ui::features::docker::containers::view;\nuse crate::ui::features::docker::images::State;\n",
    );
    write(
        src,
        "ui/features/docker/volumes/view.rs",
        "use super::super::super::podman::quadlets;\n",
    );
    let wrong = features_using_another(src);
    assert_eq!(wrong.len(), 2, "{wrong:#?}");
    assert!(wrong[0].contains("images::State") || wrong[1].contains("images::State"));

    let long = "fn f() {}\n".repeat(MAX_LINES + 1);
    write(src, "data/long.rs", &long);
    write(src, "data/tests.rs", &long);
    let tested = format!(
        "fn f() {{}}\n#[cfg(test)]\nmod tests {{\n{}}}\n",
        "    // a test\n".repeat(MAX_INLINE_TESTS + 1)
    );
    write(src, "data/tested.rs", &tested);
    assert_eq!(
        long_files(src),
        [format!("data/long.rs: {} lines", MAX_LINES + 1)]
    );
    assert_eq!(long_inline_tests(src).len(), 1);

    for n in 0..=MAX_ENTRIES {
        write(src, &format!("i18n/part{n}.rs"), "");
    }
    assert_eq!(
        full_folders(src),
        [format!("i18n: {} entries", MAX_ENTRIES + 1)]
    );

    write(src, "app/setup.rs", "");
    write(src, "app/setup/steps.rs", "");
    assert_eq!(modules_beside_their_folder(src), ["app/setup.rs"]);
    assert!(!clean(src));
}
