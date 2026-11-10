//! Every released version has a sample database (`tests/fixtures/<version>.db`,
//! made by the `db_fixture` example), and today's Homeport opens each one
//! without losing a row. What the rows hold is checked through the stores, in
//! the shell's `upgrade` test.

use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    process::Command,
};

use homeport_storage::Db;

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

fn fixtures() -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = std::fs::read_dir(fixtures_dir())
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|e| e == "db"))
        .collect();
    files.sort();
    files
}

/// The number of rows in each table.
fn row_counts(conn: &rusqlite::Connection) -> BTreeMap<String, i64> {
    let tables: Vec<String> = conn
        .prepare("SELECT name FROM sqlite_master WHERE type = 'table' AND name NOT LIKE 'sqlite_%'")
        .unwrap()
        .query_map([], |r| r.get(0))
        .unwrap()
        .collect::<rusqlite::Result<_>>()
        .unwrap();
    tables
        .into_iter()
        .map(|table| {
            let count = conn
                .query_row(&format!("SELECT count(*) FROM {table}"), [], |r| r.get(0))
                .unwrap();
            (table, count)
        })
        .collect()
}

#[test]
fn every_fixture_migrates_without_losing_a_row() {
    let fixtures = fixtures();
    assert!(!fixtures.is_empty(), "no fixture in {:?}", fixtures_dir());
    for fixture in fixtures {
        let name = fixture.file_name().unwrap().to_string_lossy().into_owned();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("homeport.db");
        std::fs::copy(&fixture, &path).unwrap();
        let before = row_counts(&rusqlite::Connection::open(&path).unwrap());
        assert!(
            before.values().sum::<i64>() > 0,
            "{name} holds no row to lose"
        );

        let db = Db::open(&path).unwrap_or_else(|e| panic!("{name} doesn't open: {e:?}"));
        let after = db.with(|c| Ok(row_counts(c))).unwrap();
        // Opening may add rows (database_info records the version it needs),
        // never remove one.
        for (table, count) in &before {
            let kept = after.get(table).copied().unwrap_or(0);
            assert!(
                kept >= *count,
                "{name}: {table} had {count} rows, {kept} after migrating"
            );
        }
    }
}

/// The versions `git tag` lists, or `None` where there is no git or no tag
/// (a source archive, a shallow clone without tags).
fn released_versions() -> Option<Vec<String>> {
    let output = Command::new("git")
        .args(["tag", "--list", "v*"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .ok()?;
    let tags = String::from_utf8_lossy(&output.stdout);
    let versions: Vec<String> = tags
        .lines()
        .map(|tag| tag.trim().trim_start_matches('v').to_owned())
        .filter(|version| !version.is_empty())
        .collect();
    (output.status.success() && !versions.is_empty()).then_some(versions)
}

#[test]
fn every_release_has_a_fixture() {
    let Some(versions) = released_versions() else {
        eprintln!("no release tag here: skipped (CI fetches the tags)");
        return;
    };
    let missing: Vec<&String> = versions
        .iter()
        .filter(|version| !fixtures_dir().join(format!("{version}.db")).exists())
        .collect();
    assert!(
        missing.is_empty(),
        "no database fixture for {missing:?}. Make them with:\n  \
         cargo run -p homeport-storage --features test-support --example db_fixture -- --missing\n\
         and commit crates/storage/tests/fixtures/ (docs/RELEASING.md, Data compatibility)"
    );
}
