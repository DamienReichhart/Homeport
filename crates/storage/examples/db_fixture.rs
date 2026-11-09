//! Makes the sample database of a release, for the upgrade tests.
//!
//!   cargo run -p homeport-storage --features test-support --example db_fixture -- 1.2.0
//!   cargo run -p homeport-storage --features test-support --example db_fixture -- --missing
//!
//! Writes `crates/storage/tests/fixtures/<version>.db` from that release's
//! own migrations: those of the `v<version>` tag when it exists, or the
//! working tree's for a version about to be released. `--missing` makes one
//! for every release tag that has none. A fixture is never overwritten: it
//! is what that release left behind, and stays as it was made.

use std::{
    path::{Path, PathBuf},
    process::Command,
};

use homeport_storage::fixtures;

/// Where the migrations have lived, newest first.
const MIGRATION_DIRS: &[&str] = &[
    "crates/storage/src/db/migrations",
    "src-tauri/src/services/db_migrations",
];

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn git(args: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(root())
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).into_owned())
}

/// The SQL of the migrations in `tag`, in order.
fn migrations_at(tag: &str) -> Option<Vec<String>> {
    MIGRATION_DIRS.iter().find_map(|dir| {
        let listing = git(&["ls-tree", "--name-only", &format!("{tag}:{dir}")])?;
        let mut files: Vec<&str> = listing.lines().filter(|f| f.ends_with(".sql")).collect();
        files.sort_unstable();
        files
            .iter()
            .map(|file| git(&["show", &format!("{tag}:{dir}/{file}")]))
            .collect()
    })
}

/// The SQL of the migrations in the working tree, in order.
fn migrations_here() -> Vec<String> {
    let dir = root().join(MIGRATION_DIRS[0]);
    let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
        .expect("the migrations folder")
        .map(|entry| entry.expect("a folder entry").path())
        .filter(|path| path.extension().is_some_and(|e| e == "sql"))
        .collect();
    files.sort();
    files
        .iter()
        .map(|file| std::fs::read_to_string(file).expect("a migration file"))
        .collect()
}

fn make(version: &str) {
    let path = root()
        .join("crates/storage/tests/fixtures")
        .join(format!("{version}.db"));
    if path.exists() {
        println!("{version}: already there, left as it is");
        return;
    }
    let tag = format!("v{version}");
    let (migrations, from) = match migrations_at(&tag) {
        Some(migrations) => (migrations, tag),
        None => (migrations_here(), "the working tree".to_owned()),
    };
    fixtures::build(&path, &migrations, version).expect("the fixture");
    println!(
        "{version}: {} migrations from {from} -> {}",
        migrations.len(),
        path.display()
    );
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("--missing") => {
            let tags = git(&["tag", "--list", "v*"]).expect("git tag");
            for tag in tags.lines() {
                make(tag.trim_start_matches('v'));
            }
        }
        Some(version) if !version.starts_with('-') => make(version.trim_start_matches('v')),
        _ => {
            eprintln!("usage: db_fixture <version> | --missing");
            std::process::exit(2);
        }
    }
}
