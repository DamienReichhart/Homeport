//! The backend's layout rules (`docs/ARCHITECTURE.md`, Backend layout),
//! checked on every test run so the workspace keeps its shape as it grows:
//! which crate may depend on which, how long a file and how full a folder
//! may get, and where tests and modules sit.

use std::{
    fs,
    path::{Path, PathBuf},
};

/// The folders of `crates/`, from the bottom of the dependency graph up. A
/// crate depends only on crates of the layers before its own.
const LAYERS: &[&str] = &["foundation", "runtimes", "domains", "app"];

/// The dependencies between two crates of one layer, as (crate, on).
const WITHIN_A_LAYER: &[(&str, &str)] = &[
    ("storage", "core"),
    ("transport", "core"),
    // A Synced Stack's compose files come from a repository.
    ("stacks", "git"),
];

/// Lines of a file before its tests. Past this it holds more than one thing.
const MAX_LINES: usize = 600;

/// Files and folders directly in one folder of sources.
const MAX_ENTRIES: usize = 14;

/// Lines of tests a file may hold itself; longer ones go to `tests.rs`.
const MAX_INLINE_TESTS: usize = 100;

/// Folders that hold data, not modules.
const DATA_FOLDERS: &[&str] = &["testdata", "migrations", "proto", "fixtures"];

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
}

/// Every crate under `crates/`, as (layer, name, manifest).
fn crates() -> Vec<(usize, String, String)> {
    let mut found = vec![];
    for (layer, folder) in LAYERS.iter().enumerate() {
        for entry in fs::read_dir(root().join("crates").join(folder)).unwrap() {
            let path = entry.unwrap().path();
            let Ok(manifest) = fs::read_to_string(path.join("Cargo.toml")) else {
                continue;
            };
            let name = path.file_name().unwrap().to_string_lossy().into_owned();
            found.push((layer, name, manifest));
        }
    }
    found
}

/// What a manifest depends on outside its tests, by the name before `=` or `.`.
fn dependencies(manifest: &str) -> Vec<String> {
    let mut names = vec![];
    let mut counted = false;
    for line in manifest.lines() {
        if line.starts_with('[') {
            counted = line.trim_end().ends_with("dependencies]") && !line.contains("dev-");
        } else if counted {
            let name: String = line
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_'))
                .collect();
            if !name.is_empty() {
                names.push(name);
            }
        }
    }
    names
}

/// The source folders: each crate's `src`, and the shell's.
fn source_roots() -> Vec<PathBuf> {
    let mut roots = vec![root().join("src-tauri/src")];
    for folder in LAYERS {
        for entry in fs::read_dir(root().join("crates").join(folder)).unwrap() {
            let src = entry.unwrap().path().join("src");
            if src.is_dir() {
                roots.push(src);
            }
        }
    }
    roots
}

/// Every folder of modules under the source folders, the roots included.
fn folders() -> Vec<PathBuf> {
    let mut all = vec![];
    let mut pending = source_roots();
    while let Some(folder) = pending.pop() {
        for entry in fs::read_dir(&folder).unwrap() {
            let path = entry.unwrap().path();
            let data = path
                .file_name()
                .is_some_and(|n| DATA_FOLDERS.contains(&&*n.to_string_lossy()));
            if path.is_dir() && !data {
                pending.push(path);
            }
        }
        all.push(folder);
    }
    all
}

fn rust_files() -> Vec<PathBuf> {
    folders()
        .iter()
        .flat_map(|folder| fs::read_dir(folder).unwrap())
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|e| e == "rs"))
        .collect()
}

fn shown(path: &Path) -> String {
    let root = root().canonicalize().unwrap();
    path.canonicalize()
        .unwrap()
        .strip_prefix(root)
        .unwrap()
        .display()
        .to_string()
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

#[test]
fn crates_depend_only_on_the_layers_below() {
    let crates = crates();
    let layer_of = |package: &str| {
        crates
            .iter()
            .find(|(_, name, _)| format!("homeport-{name}") == package)
            .map(|(layer, name, _)| (*layer, name.as_str()))
    };
    let mut wrong = vec![];
    for (layer, name, manifest) in &crates {
        for dependency in dependencies(manifest) {
            let Some((below, on)) = layer_of(&dependency) else {
                continue;
            };
            let allowed = below < *layer
                || (below == *layer && WITHIN_A_LAYER.contains(&(name.as_str(), on)));
            if !allowed {
                wrong.push(format!(
                    "{name} ({}) depends on {on} ({})",
                    LAYERS[*layer], LAYERS[below]
                ));
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "a crate depends only on the layers below its own (docs/ARCHITECTURE.md): {wrong:#?}"
    );
}

#[test]
fn only_the_shell_knows_tauri() {
    let wrong: Vec<String> = crates()
        .into_iter()
        .filter(|(_, _, manifest)| {
            dependencies(manifest)
                .iter()
                .any(|d| d.starts_with("tauri"))
        })
        .map(|(_, name, _)| name)
        .collect();
    assert!(
        wrong.is_empty(),
        "a crate that must tell the app something defines a trait the shell implements: {wrong:?}"
    );
}

#[test]
fn files_stay_short() {
    let mut long = vec![];
    for path in rust_files() {
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
    for path in rust_files() {
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
    for folder in folders() {
        let entries = fs::read_dir(&folder)
            .unwrap()
            .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
            .filter(|name| !DATA_FOLDERS.contains(&name.as_str()))
            .count();
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
    let beside: Vec<String> = rust_files()
        .into_iter()
        .filter(|path| path.with_extension("").is_dir())
        .map(|path| shown(&path))
        .collect();
    assert!(
        beside.is_empty(),
        "name it mod.rs inside its folder, as every other module: {beside:?}"
    );
}
