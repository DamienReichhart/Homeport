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
        &[
            "crate::ui::features",
            "crate::ui::shell",
            "crate::ui::domain",
        ],
        "the data layer serves every page, so it knows none",
    ),
    (
        "ui/shell",
        &["crate::ui::features", "crate::ui::domain"],
        "the shell hosts whatever page it is given, and knows no feature",
    ),
    (
        "ui/domain",
        &["crate::ui::features"],
        "what features share stands under them, and knows none",
    ),
    (
        "ui/widgets",
        &[
            "crate::ui::features",
            "crate::ui::domain",
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
            "crate::ui::domain",
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

/// The paths a use-tree names, each written out whole: `a::{b, c::{d, e}}`
/// is `a::b`, `a::c::d` and `a::c::e`.
fn expand(tree: &str) -> Vec<String> {
    let tree = tree.trim();
    let Some(open) = tree.find('{') else {
        return vec![tree.to_owned()];
    };
    // The group that opens here closes at its matching brace.
    let mut depth = 0;
    let mut close = tree.len();
    for (at, c) in tree.char_indices().skip(open) {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    close = at;
                    break;
                }
            }
            _ => {}
        }
    }
    let (prefix, inside) = (&tree[..open], &tree[open + 1..close]);
    let mut parts = vec![];
    let (mut depth, mut from) = (0, 0);
    for (at, c) in inside.char_indices() {
        match c {
            '{' => depth += 1,
            '}' => depth -= 1,
            ',' if depth == 0 => {
                parts.push(&inside[from..at]);
                from = at + 1;
            }
            _ => {}
        }
    }
    parts.push(&inside[from..]);
    parts
        .into_iter()
        .filter(|part| !part.trim().is_empty())
        .flat_map(expand)
        .map(|rest| format!("{prefix}{rest}"))
        .collect()
}

/// A statement of a file: as it is written, on one line, and the paths
/// it names. A `use` names each path of its tree whole, however its
/// braces nest and wherever its lines break, so `use crate::{ui::{domain}}`
/// names `crate::ui::domain`.
struct Statement {
    text: String,
    paths: Vec<String>,
}

/// The statements of a file: no comment, and nothing of its tests.
fn code(path: &Path) -> Vec<Statement> {
    if path.file_name().is_some_and(|n| n == "tests.rs") {
        return vec![];
    }
    let text = fs::read_to_string(path).unwrap();
    let end = tests_start(&text).unwrap_or(usize::MAX);
    let lines = text
        .lines()
        .take(end)
        .map(str::trim)
        .filter(|line| !line.starts_with("//"));
    let mut statements = vec![];
    let mut pending: Option<String> = None;
    for line in lines {
        let is_use =
            line.starts_with("use ") || (line.starts_with("pub") && line.contains(" use "));
        let whole = match pending.take() {
            Some(started) => format!("{started} {line}"),
            None if is_use => line.to_owned(),
            None => {
                statements.push(Statement {
                    text: line.to_owned(),
                    paths: vec![line.to_owned()],
                });
                continue;
            }
        };
        if !whole.ends_with(';') {
            pending = Some(whole);
            continue;
        }
        let tree = &whole[whole.find("use ").map_or(0, |at| at + 4)..whole.len() - 1];
        let squeezed: String = tree.split_whitespace().collect();
        statements.push(Statement {
            paths: expand(&squeezed),
            text: whole,
        });
    }
    statements
}

/// What uses a part it must not, under `src`.
fn wrong_dependencies(src: &Path) -> Vec<String> {
    let mut wrong = vec![];
    for (part, forbidden, why) in FORBIDDEN {
        for path in rust_files(&src.join(part)) {
            for statement in code(&path) {
                let names = |f: &&&str| statement.paths.iter().any(|path| path.contains(**f));
                if let Some(used) = forbidden.iter().find(names) {
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

/// Whether `line` names something of `ui::features` that isn't under
/// `own` (`<group>::<name>`): the name must end there, so `images_extra`
/// is not `images`.
fn names_another_feature(line: &str, own: &str) -> bool {
    const FEATURES: &str = "crate::ui::features::";
    line.match_indices(FEATURES).any(|(at, _)| {
        let rest = &line[at + FEATURES.len()..];
        let inside = rest
            .strip_prefix(own)
            .is_some_and(|after| !after.starts_with(|c: char| c.is_alphanumeric() || c == '_'));
        !inside
    })
}

/// The most `super::` a line chains.
fn supers(line: &str) -> usize {
    let mut most = 0;
    let mut rest = line;
    while let Some(at) = rest.find("super::") {
        let mut chain = 0;
        rest = &rest[at..];
        while let Some(after) = rest.strip_prefix("super::") {
            chain += 1;
            rest = after;
        }
        most = most.max(chain);
    }
    most
}

/// How many modules deep `file` is inside the feature at `feature`: 0 for
/// its `mod.rs`, 1 for `view.rs` or `view/mod.rs`, 2 for `view/row.rs`.
/// That many `super` stay in the feature; one more leaves it.
fn depth(feature: &Path, file: &Path) -> usize {
    let inside = file.strip_prefix(feature).unwrap();
    let folders = inside.components().count() - 1;
    folders + usize::from(inside.file_name().is_some_and(|name| name != "mod.rs"))
}

/// The lines of a feature that reach another feature, by its path in the
/// crate or by climbing out of their own with `super`.
fn features_using_another(src: &Path) -> Vec<String> {
    let mut wrong = vec![];
    for (own, feature) in features(src) {
        let own = own.trim_start_matches("crate::ui::features::").to_owned();
        for path in rust_files(&feature) {
            let allowed = depth(&feature, &path);
            for statement in code(&path) {
                let leaves =
                    |path: &String| names_another_feature(path, &own) || supers(path) > allowed;
                if statement.paths.iter().any(leaves) {
                    wrong.push(format!("{}: {}", shown(src, &path), statement.text));
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
        "features use the domain, widgets, the shell, data and api; api uses app; app uses the crates: {wrong:#?}"
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
        "what two features share goes to ui/domain, ui/shell or data: {wrong:#?}"
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
        for line in code(&path).into_iter().map(|statement| statement.text) {
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

    // What stays inside a feature passes, however deep.
    write(
        src,
        "ui/features/docker/images/view/row.rs",
        "use super::super::State;\nuse crate::ui::features::docker::images::State;\n",
    );
    write(
        src,
        "ui/features/docker/images/view/mod.rs",
        "use super::State;\n",
    );
    assert!(features_using_another(src).is_empty());
    let breaks = [
        // By its path in the crate, a sibling's and one whose name starts
        // with this feature's.
        (
            "docker/containers/mod.rs",
            "use crate::ui::features::docker::images::State;",
        ),
        (
            "docker/images/update.rs",
            "use crate::ui::features::docker::images_extra::State;",
        ),
        (
            "docker/images/state.rs",
            "use crate::ui::features::docker::{containers, images};",
        ),
        // By climbing out of its own folder.
        (
            "docker/containers/list.rs",
            "use super::super::images::State;",
        ),
        ("docker/containers/view/mod.rs", "use super::super::images;"),
        (
            "docker/containers/view/row.rs",
            "use super::super::super::images;",
        ),
        (
            "docker/volumes/view.rs",
            "use super::super::super::podman::quadlets;",
        ),
        ("podman/quadlets/mod.rs", "use super::podman_pods::State;"),
        // Through a nested use-tree.
        (
            "docker/networks/mod.rs",
            "use crate::{data, ui::{features::docker::volumes, theme}};",
        ),
    ];
    for (file, line) in breaks {
        write(src, &format!("ui/features/{file}"), &format!("{line}\n"));
    }
    let wrong = features_using_another(src);
    assert_eq!(wrong.len(), breaks.len(), "{wrong:#?}");
    for (file, line) in breaks {
        let said = format!("ui/features/{file}: {line}");
        assert!(wrong.contains(&said), "{said} passed");
    }

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

/// The same for what features share, and for a use-tree that nests: the
/// check reads whole paths, not lines.
#[test]
fn the_domain_s_rules_fail_and_a_nested_use_is_seen() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path();
    // What features share stands under them and over the shell: it names
    // no feature, and nothing below it names it.
    write(
        src,
        "ui/domain/containers.rs",
        "use crate::ui::features::docker::images::State;\n",
    );
    write(
        src,
        "ui/shell/menu.rs",
        "use crate::ui::domain::containers;\n",
    );
    write(
        src,
        "ui/theme/tone.rs",
        "use crate::ui::domain::containers;\n",
    );
    write(src, "data/usage.rs", "use crate::ui::domain::containers;\n");
    // However the use-tree nests and wherever its lines break.
    write(
        src,
        "ui/widgets/menu.rs",
        "use crate::{\n    i18n,\n    ui::{\n        domain::containers::{model, Target},\n        theme,\n    },\n};\n",
    );
    write(
        src,
        "ui/widgets/fine.rs",
        "pub use crate::{\n    i18n,\n    ui::{theme, widgets::line},\n};\n",
    );
    let wrong = wrong_dependencies(src);
    assert_eq!(wrong.len(), 5, "{wrong:#?}");
    for said in [
        "ui/domain/containers.rs uses crate::ui::features",
        "ui/shell/menu.rs uses crate::ui::domain",
        "ui/theme/tone.rs uses crate::ui::domain",
        "data/usage.rs uses crate::ui::domain",
        "ui/widgets/menu.rs uses crate::ui::domain",
    ] {
        assert!(wrong.iter().any(|w| w.starts_with(said)), "{said} passed");
    }
    assert_eq!(
        expand("crate::{a,ui::{b::{c,d},e}}"),
        [
            "crate::a",
            "crate::ui::b::c",
            "crate::ui::b::d",
            "crate::ui::e"
        ]
    );
}
