use super::*;
use homeport_core::error::ErrorCode;

#[test]
fn transactions_keep_nothing_of_a_failure() {
    let db = Db::open_in_memory().unwrap();
    let put = |tx: &rusqlite::Transaction<'_>, key: &str| {
        tx.execute("INSERT INTO settings (key, value) VALUES (?1, '{}')", [key])
            .map_err(sqlite_error)
    };
    let failed: AppResult<()> = db.transaction(|tx| {
        put(tx, "a")?;
        Err(homeport_core::error::AppError::validation(
            homeport_core::messages::CONTAINER_GONE,
        ))
    });
    assert!(failed.is_err());
    db.transaction(|tx| put(tx, "b")).unwrap();
    let keys: Vec<String> = db
        .with(|c| {
            c.prepare("SELECT key FROM settings")?
                .query_map([], |r| r.get(0))?
                .collect()
        })
        .unwrap();
    assert_eq!(keys, ["b"]);
}

#[test]
fn copies_are_snapshots_of_the_database() {
    let dir = tempfile::tempdir().unwrap();
    let db = Db::open(&dir.path().join("homeport.db")).unwrap();
    db.with(|c| c.execute("INSERT INTO settings (key, value) VALUES ('k', '1')", []))
        .unwrap();
    let copy = dir.path().join("copy.db");
    db.copy_to(&copy).unwrap();
    let copied = Db::open(&copy).unwrap();
    let value: String = copied
        .with(|c| {
            c.query_row("SELECT value FROM settings WHERE key = 'k'", [], |r| {
                r.get(0)
            })
        })
        .unwrap();
    assert_eq!(value, "1");
}

#[test]
fn sealed_secrets_are_found_wherever_they_are_kept() {
    let local = r#"{"kind":"local","socketPath":"/var/run/docker.sock"}"#;
    let rows = [
            format!("INSERT INTO hosts (id, name, connection, secrets) VALUES ('s', 's', '{local}', x'01')"),
            "INSERT INTO stacks (id, host_id, name, env) VALUES ('m', 'h', 'shop', x'01')".into(),
            "INSERT INTO registries (id, address, username, secret) VALUES ('r', 'ghcr.io', 'me', x'01')".into(),
            "INSERT INTO git_repositories (id, url, auth, token) VALUES ('g', 'https://e.com/a.git', 'https_token', x'01')".into(),
            "INSERT INTO clusters (id, name, connection, secrets) VALUES ('c', 'c', '{}', x'01')".into(),
        ];
    for row in rows {
        let db = Db::open_in_memory().unwrap();
        db.with(|c| {
            c.execute(
                &format!("INSERT INTO hosts (id, name, connection) VALUES ('h', 'h', '{local}')"),
                [],
            )
        })
        .unwrap();
        assert!(!db.holds_sealed_data().unwrap(), "a Host without secrets");
        db.with(|c| c.execute(&row, [])).unwrap();
        assert!(db.holds_sealed_data().unwrap(), "{row}");
    }
}

#[test]
fn migrations_are_valid() {
    MIGRATIONS.validate().unwrap();
}

/// Every file in the folder, by name, with its bytes.
fn files_in(dir: &Path) -> Vec<(String, Vec<u8>)> {
    let mut files: Vec<_> = std::fs::read_dir(dir)
        .unwrap()
        .map(|entry| {
            let entry = entry.unwrap();
            (
                entry.file_name().to_string_lossy().into_owned(),
                std::fs::read(entry.path()).unwrap(),
            )
        })
        .collect();
    files.sort();
    files
}

/// A database one migration past the latest, as a newer Homeport that
/// records the version it needs would leave it.
fn newer_database(path: &Path, requires: Option<&str>) {
    let db = Db::open(path).unwrap();
    db.with(|c| {
        c.execute("INSERT INTO settings VALUES ('k', '1')", [])?;
        match requires {
            Some(version) => c.execute(
                "UPDATE database_info SET value = ?1 WHERE key = 'requires'",
                [version],
            )?,
            None => c.execute("DROP TABLE database_info", [])?,
        };
        c.pragma_update(None, "user_version", MIGRATION_SLICE.len() as i64 + 1)
    })
    .unwrap();
}

#[test]
fn a_database_from_a_newer_homeport_is_refused_untouched() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("homeport.db");
    newer_database(&path, Some("9.4.0"));
    let before = files_in(dir.path());
    assert_eq!(before.len(), 1, "closing leaves the database file alone");

    let error = Db::open(&path).err().expect("refused");
    assert_eq!(error.code, ErrorCode::DbFromNewerVersion);
    assert_eq!(
        error.message,
        "This database was last used by Homeport 9.4.0 or later."
    );
    let hint = error.hint.expect("says what to do");
    assert!(hint.text.contains("Install Homeport 9.4.0 or newer"));
    assert!(hint.text.contains("hasn't been changed"));
    assert!(!hint.text.to_lowercase().contains("delete"));
    assert_eq!(files_in(dir.path()), before, "nothing on disk changed");
}

#[test]
fn a_newer_database_that_names_no_version_is_refused_too() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("homeport.db");
    newer_database(&path, None);
    let before = files_in(dir.path());

    let error = Db::open(&path).err().expect("refused");
    assert_eq!(error.code, ErrorCode::DbFromNewerVersion);
    assert!(error.message.contains("a newer Homeport than this one"));
    assert_eq!(files_in(dir.path()), before);
}

#[test]
fn a_newer_database_left_open_by_a_crash_is_refused_without_losing_its_log() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("homeport.db");
    let live = dir.path().join("live.db");
    // The newer Homeport is still holding the file, so its last writes are
    // only in the write-ahead log; copying now is what a crash leaves.
    let conn = Connection::open(&live).unwrap();
    conn.pragma_update(None, "journal_mode", "WAL").unwrap();
    conn.pragma_update(None, "wal_autocheckpoint", 0).unwrap();
    conn.execute_batch("CREATE TABLE settings (key TEXT, value TEXT)")
        .unwrap();
    conn.pragma_update(None, "user_version", MIGRATION_SLICE.len() as i64 + 1)
        .unwrap();
    for suffix in ["", "-wal"] {
        let name = |base: &Path| {
            let mut name = base.as_os_str().to_owned();
            name.push(suffix);
            std::path::PathBuf::from(name)
        };
        std::fs::copy(name(&live), name(&path)).unwrap();
    }
    drop(conn);
    let before = std::fs::read(&path).unwrap();
    let log = std::fs::read(dir.path().join("homeport.db-wal")).unwrap();
    assert!(!log.is_empty());

    let error = Db::open(&path).err().expect("refused");
    assert_eq!(error.code, ErrorCode::DbFromNewerVersion);
    assert_eq!(std::fs::read(&path).unwrap(), before);
    assert_eq!(
        std::fs::read(dir.path().join("homeport.db-wal")).unwrap(),
        log
    );
}

#[test]
fn opening_records_the_oldest_homeport_that_can_read_the_file() {
    let db = Db::open_in_memory().unwrap();
    let requires: String = db
        .with(|c| {
            c.query_row(
                "SELECT value FROM database_info WHERE key = 'requires'",
                [],
                |r| r.get(0),
            )
        })
        .unwrap();
    assert_eq!(requires, release_of(MIGRATION_SLICE.len()));
}

/// The promise behind every upgrade: a migration that shipped is never
/// edited, because databases that already ran it wouldn't get the change.
#[test]
fn released_migrations_are_unchanged() {
    use sha2::{Digest, Sha256};
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/db/migrations");
    let mut files: Vec<String> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .filter(|name| name.ends_with(".sql"))
        .collect();
    files.sort();
    assert_eq!(
        files.len(),
        MIGRATION_SLICE.len(),
        "every migration file is in MIGRATION_SLICE"
    );

    let released: Vec<Released> = released().collect();
    assert!(released.len() <= files.len());
    for (migration, file) in released.iter().zip(&files) {
        assert_eq!(
            migration.file, file,
            "RELEASED lists the migrations in order, without gaps"
        );
        let bytes = std::fs::read(dir.join(file)).unwrap();
        let hash: String = Sha256::digest(&bytes)
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect();
        assert_eq!(
            hash, migration.sha256,
            "{file} was released in {} and must never change: \
                 fix it with a new migration (docs/RELEASING.md, Data compatibility)",
            migration.release
        );
    }
}

#[test]
fn released_migrations_name_their_release() {
    assert_eq!(release_of(1), "0.1.0");
    assert_eq!(release_of(4), "0.7.0");
    assert_eq!(release_of(17), "0.18.0");
    // One past the latest isn't released by definition.
    assert_eq!(
        release_of(MIGRATION_SLICE.len() + 1),
        env!("CARGO_PKG_VERSION")
    );
}

#[test]
fn databases_from_0_11_keep_engine_events_until_their_host_goes() {
    let mut conn = Connection::open_in_memory().unwrap();
    MIGRATIONS.to_version(&mut conn, 13).unwrap();
    let local = r#"{"kind":"local","socketPath":"/var/run/docker.sock"}"#;
    conn.execute(
        "INSERT INTO hosts (id, name, connection) VALUES ('h', 'local', ?1)",
        [local],
    )
    .unwrap();
    migrate(&mut conn).unwrap();

    let insert = "INSERT OR IGNORE INTO engine_events
            (host_id, time_nano, kind, action, actor_id, attributes)
            VALUES ('h', 42, 'container', 'start', 'abc', '{}')";
    assert_eq!(conn.execute(insert, []).unwrap(), 1);
    assert_eq!(conn.execute(insert, []).unwrap(), 0, "kept once");
    conn.execute("DELETE FROM hosts WHERE id = 'h'", [])
        .unwrap();
    let left: i64 = conn
        .query_row("SELECT count(*) FROM engine_events", [], |r| r.get(0))
        .unwrap();
    assert_eq!(left, 0);
}

#[test]
fn databases_from_0_14_gain_clusters_and_keep_their_hosts() {
    let mut conn = Connection::open_in_memory().unwrap();
    MIGRATIONS.to_version(&mut conn, 15).unwrap();
    let local = r#"{"kind":"local","socketPath":"/var/run/docker.sock"}"#;
    conn.execute(
        "INSERT INTO hosts (id, name, connection) VALUES ('h', 'local', ?1)",
        [local],
    )
    .unwrap();
    migrate(&mut conn).unwrap();

    let hosts: i64 = conn
        .query_row("SELECT count(*) FROM hosts", [], |r| r.get(0))
        .unwrap();
    assert_eq!(hosts, 1);
    conn.execute(
        "INSERT INTO clusters (id, name, connection) VALUES ('c', 'kind-dev', '{}')",
        [],
    )
    .unwrap();
    let (kind, enabled): (String, bool) = conn
        .query_row("SELECT kind, enabled FROM clusters", [], |r| {
            Ok((r.get(0)?, r.get(1)?))
        })
        .unwrap();
    assert_eq!((kind.as_str(), enabled), ("kubernetes", true));
    let duplicate = conn.execute(
        "INSERT INTO clusters (id, name, connection) VALUES ('d', 'KIND-DEV', '{}')",
        [],
    );
    assert!(duplicate.is_err(), "names are unique whatever their case");
}

#[test]
fn hosts_from_0_17_open_as_docker_compatible_engines() {
    let mut conn = Connection::open_in_memory().unwrap();
    MIGRATIONS.to_version(&mut conn, 16).unwrap();
    let local = r#"{"kind":"local","socketPath":"/var/run/docker.sock"}"#;
    conn.execute(
            "INSERT INTO hosts (id, name, connection, host_key, enabled) VALUES ('h', 'local', ?1, 'k', 0)",
            [local],
        )
        .unwrap();
    migrate(&mut conn).unwrap();

    let (name, connection, host_key, enabled, engine): (String, String, String, bool, String) =
        conn.query_row(
            "SELECT name, connection, host_key, enabled, engine FROM hosts",
            [],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?)),
        )
        .unwrap();
    assert_eq!(
        (
            name.as_str(),
            connection.as_str(),
            host_key.as_str(),
            enabled
        ),
        ("local", local, "k", false),
        "nothing else changes"
    );
    assert_eq!(engine, "docker");
    let unknown = conn.execute(
        "INSERT INTO hosts (id, name, connection, engine) VALUES ('x', 'x', ?1, 'lxd')",
        [local],
    );
    assert!(unknown.is_err(), "only known kinds are stored");
}

#[test]
fn stacks_from_0_13_keep_their_host_and_files() {
    let mut conn = Connection::open_in_memory().unwrap();
    MIGRATIONS.to_version(&mut conn, 14).unwrap();
    let local = r#"{"kind":"local","socketPath":"/var/run/docker.sock"}"#;
    conn.execute_batch(&format!(
            "INSERT INTO hosts (id, name, connection) VALUES ('h', 'local', '{local}');
             INSERT INTO stacks (id, host_id, name, env, profiles) VALUES ('m', 'h', 'shop', NULL, '[\"debug\"]');
             INSERT INTO stack_files (stack_id, position, name, content) VALUES ('m', 0, 'compose.yaml', 'services: {{}}');
             INSERT INTO linked_stacks (id, host_id, name, working_dir, config_files, fingerprint)
                 VALUES ('l', 'h', 'blog', '/srv/blog', '[]', 'f');
             INSERT INTO git_repositories (id, url, auth) VALUES ('r', 'https://example.com/a.git', 'none');
             INSERT INTO synced_stacks (id, host_id, name, repository_id, ref_kind, ref_name, compose_path, auto_deploy)
                 VALUES ('s', 'h', 'site', 'r', 'branch', 'main', 'compose.yaml', 1);
             INSERT INTO stack_deploys (id, host_id, stack_id, stack_kind, stack_name, action, definition)
                 VALUES ('d', 'h', 'm', 'managed', 'shop', 'up', '[]');"
        ))
        .unwrap();
    migrate(&mut conn).unwrap();

    let owner = |table: &str| -> (Option<String>, Option<String>) {
        conn.query_row(&format!("SELECT host_id, swarm_id FROM {table}"), [], |r| {
            Ok((r.get(0)?, r.get(1)?))
        })
        .unwrap()
    };
    for table in ["stacks", "linked_stacks", "synced_stacks", "stack_deploys"] {
        assert_eq!(owner(table), (Some("h".into()), None), "{table}");
    }
    let files: i64 = conn
        .query_row("SELECT count(*) FROM stack_files", [], |r| r.get(0))
        .unwrap();
    assert_eq!(files, 1, "rebuilding stacks keeps its files");
    let kept: (String, bool) = conn
        .query_row("SELECT profiles, auto_deploy FROM synced_stacks", [], |r| {
            Ok((r.get::<_, String>(0)?, r.get(1)?))
        })
        .unwrap();
    assert_eq!(kept, ("[]".to_owned(), true));

    // A Stack belongs to a Host or a Swarm, never both or neither.
    for (host, swarm) in [("NULL", "NULL"), ("'h'", "'s1'")] {
        let insert = format!(
            "INSERT INTO stacks (id, host_id, swarm_id, name) VALUES ('x', {host}, {swarm}, 'x')"
        );
        assert!(conn.execute(&insert, []).is_err(), "{host} {swarm}");
    }
    conn.execute(
        "INSERT INTO stacks (id, swarm_id, name) VALUES ('w', 's1', 'shop')",
        [],
    )
    .unwrap();
    // Removing the Host keeps the Swarm's Stack and drops its own.
    conn.execute("DELETE FROM hosts WHERE id = 'h'", [])
        .unwrap();
    let left: Vec<String> = conn
        .prepare("SELECT id FROM stacks")
        .unwrap()
        .query_map([], |r| r.get(0))
        .unwrap()
        .collect::<rusqlite::Result<_>>()
        .unwrap();
    assert_eq!(left, ["w"]);
}

#[test]
fn ssh_hosts_from_before_bastions_get_an_empty_list() {
    let mut conn = Connection::open_in_memory().unwrap();
    MIGRATIONS.to_version(&mut conn, 4).unwrap();
    let ssh = r#"{"kind":"ssh","host":"h","port":22,"user":"u","socketPath":"/var/run/docker.sock","auth":"agent","publicKey":null}"#;
    let local = r#"{"kind":"local","socketPath":"/var/run/docker.sock"}"#;
    conn.execute(
            "INSERT INTO hosts (id, name, connection, host_key) VALUES ('a', 'a', ?1, 'ssh-ed25519 KEY'), ('b', 'b', ?2, NULL)",
            (ssh, local),
        )
        .unwrap();
    migrate(&mut conn).unwrap();

    let read = |id: &str| -> (serde_json::Value, Option<String>) {
        conn.query_row(
            "SELECT connection, host_key FROM hosts WHERE id = ?1",
            [id],
            |r| {
                Ok((
                    serde_json::from_str(&r.get::<_, String>(0)?).unwrap(),
                    r.get(1)?,
                ))
            },
        )
        .unwrap()
    };
    let (a, key) = read("a");
    assert_eq!(a["bastions"], serde_json::json!([]));
    assert_eq!(a["host"], "h");
    assert_eq!(key.as_deref(), Some("ssh-ed25519 KEY"));
    assert!(read("b").0.get("bastions").is_none());
}

#[test]
fn databases_from_0_9_keep_their_hosts_and_stacks() {
    let mut conn = Connection::open_in_memory().unwrap();
    MIGRATIONS.to_version(&mut conn, 5).unwrap();
    let local = r#"{"kind":"local","socketPath":"/var/run/docker.sock"}"#;
    conn.execute(
        "INSERT INTO hosts (id, name, connection) VALUES ('h', 'local', ?1)",
        [local],
    )
    .unwrap();
    conn.execute(
        "INSERT INTO stacks (id, host_id, name, compose) VALUES ('s', 'h', 'shop', 'services: {}')",
        [],
    )
    .unwrap();
    migrate(&mut conn).unwrap();

    let stacks: i64 = conn
        .query_row("SELECT count(*) FROM stacks WHERE host_id = 'h'", [], |r| {
            r.get(0)
        })
        .unwrap();
    assert_eq!(stacks, 1);
    // Its one definition is now its first compose file.
    let file: (i64, String, String) = conn
        .query_row(
            "SELECT position, name, content FROM stack_files WHERE stack_id = 's'",
            [],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
        )
        .unwrap();
    assert_eq!(file, (0, "compose.yaml".into(), "services: {}".into()));
    // It deploys no profile until one is turned on.
    let profiles: String = conn
        .query_row("SELECT profiles FROM stacks WHERE id = 's'", [], |r| {
            r.get(0)
        })
        .unwrap();
    assert_eq!(profiles, "[]");
    // No deploy was recorded before history existed.
    let deploys: i64 = conn
        .query_row("SELECT count(*) FROM stack_deploys", [], |r| r.get(0))
        .unwrap();
    assert_eq!(deploys, 0);
    let linked: i64 = conn
        .query_row("SELECT count(*) FROM linked_stacks", [], |r| r.get(0))
        .unwrap();
    assert_eq!(linked, 0);
}

#[test]
fn databases_from_0_10_keep_their_stacks_and_history() {
    let mut conn = Connection::open_in_memory().unwrap();
    MIGRATIONS.to_version(&mut conn, 9).unwrap();
    let local = r#"{"kind":"local","socketPath":"/var/run/docker.sock"}"#;
    conn.execute_batch(&format!(
            "INSERT INTO hosts (id, name, connection) VALUES ('h', 'local', '{local}');
             INSERT INTO stacks (id, host_id, name) VALUES ('s', 'h', 'shop');
             INSERT INTO linked_stacks (id, host_id, name, working_dir, config_files, fingerprint)
                 VALUES ('l', 'h', 'blog', '/srv/blog', '[]', 'fp');
             INSERT INTO stack_deploys (id, host_id, stack_id, stack_kind, stack_name, action, definition)
                 VALUES ('d', 'h', 's', 'managed', 'shop', 'up', '[]');"
        ))
        .unwrap();
    migrate(&mut conn).unwrap();

    let count = |table: &str| -> i64 {
        conn.query_row(&format!("SELECT count(*) FROM {table}"), [], |r| r.get(0))
            .unwrap()
    };
    assert_eq!(
        (
            count("stacks"),
            count("linked_stacks"),
            count("stack_deploys")
        ),
        (1, 1, 1)
    );
    assert_eq!((count("git_repositories"), count("synced_stacks")), (0, 0));
    let commit: Option<String> = conn
        .query_row(
            "SELECT commit_hash FROM stack_deploys WHERE id = 'd'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(commit, None, "deploys from before 0.11 have no commit");
    // A repository can't go while a Synced Stack uses it.
    conn.execute_batch(
            "INSERT INTO git_repositories (id, url, auth) VALUES ('r', 'https://example.com/r.git', 'none');
             INSERT INTO synced_stacks (id, host_id, name, repository_id, ref_kind, ref_name, compose_path)
                 VALUES ('y', 'h', 'site', 'r', 'branch', 'main', 'compose.yaml');",
        )
        .unwrap();
    assert!(conn
        .execute("DELETE FROM git_repositories WHERE id = 'r'", [])
        .is_err());
    // Synced Stacks poll by default, and never deploy on their own until asked.
    let (poll, auto): (bool, bool) = conn
        .query_row(
            "SELECT poll, auto_deploy FROM synced_stacks WHERE id = 'y'",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .unwrap();
    assert_eq!((poll, auto), (true, false));
    // Removing the Host removes its Synced Stacks.
    conn.execute("DELETE FROM hosts WHERE id = 'h'", [])
        .unwrap();
    assert_eq!(count("synced_stacks"), 0);
}

#[test]
fn reopening_a_file_keeps_data() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("nested").join("homeport.db");
    Db::open(&path)
        .unwrap()
        .with(|c| c.execute("INSERT INTO settings VALUES ('k', '1')", []))
        .unwrap();

    let value: String = Db::open(&path)
        .unwrap()
        .with(|c| {
            c.query_row("SELECT value FROM settings WHERE key = 'k'", [], |r| {
                r.get(0)
            })
        })
        .unwrap();
    assert_eq!(value, "1");
}
