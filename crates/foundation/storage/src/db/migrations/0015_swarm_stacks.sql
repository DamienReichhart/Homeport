-- Managed, Linked and Synced Stacks can be deployed to a Swarm (ADR 0017):
-- each row belongs to a Host or to a Swarm, exactly one, so removing one
-- manager Host doesn't delete a Stack of its Swarm. A Swarm is never stored
-- (ADR 0016): `swarm_id` is the swarm ID its managers report. Every existing
-- row keeps its Host. `registry_auth` sends the docker CLI's own registry
-- logins with a swarm deploy, off unless turned on.
--
-- SQLite can't change a column's constraints in place, so each table is
-- rebuilt; migrations run with foreign keys off, so dropping a table doesn't
-- cascade to the rows that point at it (stack_files).

CREATE TABLE stacks_new (
    id            TEXT PRIMARY KEY NOT NULL,
    host_id       TEXT REFERENCES hosts (id) ON DELETE CASCADE,
    swarm_id      TEXT,
    name          TEXT NOT NULL,
    env           BLOB, -- .env text sealed by crypto::SecretBox
    created_at    TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at    TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    profiles      TEXT NOT NULL DEFAULT '[]',
    registry_auth INTEGER NOT NULL DEFAULT 0,
    CHECK ((host_id IS NULL) <> (swarm_id IS NULL)),
    UNIQUE (host_id, name),
    UNIQUE (swarm_id, name)
) STRICT;
INSERT INTO stacks_new (id, host_id, name, env, created_at, updated_at, profiles)
SELECT id, host_id, name, env, created_at, updated_at, profiles FROM stacks;
DROP TABLE stacks;
ALTER TABLE stacks_new RENAME TO stacks;

CREATE TABLE linked_stacks_new (
    id            TEXT PRIMARY KEY NOT NULL,
    host_id       TEXT REFERENCES hosts (id) ON DELETE CASCADE,
    swarm_id      TEXT,
    name          TEXT NOT NULL,
    working_dir   TEXT NOT NULL,
    config_files  TEXT NOT NULL, -- JSON array of absolute paths, in order
    fingerprint   TEXT NOT NULL, -- of the files as Homeport last read or wrote them
    created_at    TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at    TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    profiles      TEXT NOT NULL DEFAULT '[]',
    registry_auth INTEGER NOT NULL DEFAULT 0,
    CHECK ((host_id IS NULL) <> (swarm_id IS NULL)),
    UNIQUE (host_id, name),
    UNIQUE (swarm_id, name)
) STRICT;
INSERT INTO linked_stacks_new
    (id, host_id, name, working_dir, config_files, fingerprint, created_at, updated_at, profiles)
SELECT id, host_id, name, working_dir, config_files, fingerprint, created_at, updated_at, profiles
FROM linked_stacks;
DROP TABLE linked_stacks;
ALTER TABLE linked_stacks_new RENAME TO linked_stacks;

CREATE TABLE synced_stacks_new (
    id                        TEXT PRIMARY KEY NOT NULL,
    host_id                   TEXT REFERENCES hosts (id) ON DELETE CASCADE,
    swarm_id                  TEXT,
    name                      TEXT NOT NULL,
    repository_id             TEXT NOT NULL REFERENCES git_repositories (id) ON DELETE RESTRICT,
    ref_kind                  TEXT NOT NULL CHECK (ref_kind IN ('branch', 'tag')),
    ref_name                  TEXT NOT NULL,
    compose_path              TEXT NOT NULL, -- relative to the repository's root, with /
    env                       BLOB,          -- .env text sealed by crypto::SecretBox
    profiles                  TEXT NOT NULL DEFAULT '[]',
    deployed_commit           TEXT,
    fetched_commit            TEXT,
    created_at                TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at                TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    poll                      INTEGER NOT NULL DEFAULT 1,
    auto_deploy               INTEGER NOT NULL DEFAULT 0,
    last_polled_at            TEXT,
    last_poll_error           TEXT,
    update_available          INTEGER NOT NULL DEFAULT 0,
    auto_deploy_failed_commit TEXT,
    auto_deploy_error         TEXT,
    registry_auth             INTEGER NOT NULL DEFAULT 0,
    CHECK ((host_id IS NULL) <> (swarm_id IS NULL)),
    UNIQUE (host_id, name),
    UNIQUE (swarm_id, name)
) STRICT;
INSERT INTO synced_stacks_new
    (id, host_id, name, repository_id, ref_kind, ref_name, compose_path, env, profiles,
     deployed_commit, fetched_commit, created_at, updated_at, poll, auto_deploy, last_polled_at,
     last_poll_error, update_available, auto_deploy_failed_commit, auto_deploy_error)
SELECT id, host_id, name, repository_id, ref_kind, ref_name, compose_path, env, profiles,
     deployed_commit, fetched_commit, created_at, updated_at, poll, auto_deploy, last_polled_at,
     last_poll_error, update_available, auto_deploy_failed_commit, auto_deploy_error
FROM synced_stacks;
DROP TABLE synced_stacks;
ALTER TABLE synced_stacks_new RENAME TO synced_stacks;
CREATE INDEX synced_stacks_by_repository ON synced_stacks (repository_id);

CREATE TABLE stack_deploys_new (
    id          TEXT PRIMARY KEY NOT NULL,
    host_id     TEXT REFERENCES hosts (id) ON DELETE CASCADE,
    swarm_id    TEXT,
    stack_id    TEXT NOT NULL, -- of `stacks`, `linked_stacks` or `synced_stacks`, by kind
    stack_kind  TEXT NOT NULL,
    stack_name  TEXT NOT NULL,
    action      TEXT NOT NULL,
    definition  TEXT NOT NULL,
    env         BLOB,
    profiles    TEXT NOT NULL DEFAULT '[]',
    started_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    finished_at TEXT,
    success     INTEGER,
    exit_code   INTEGER,
    automatic   INTEGER NOT NULL DEFAULT 0,
    commit_hash TEXT,
    CHECK ((host_id IS NULL) <> (swarm_id IS NULL))
) STRICT;
INSERT INTO stack_deploys_new
    (id, host_id, stack_id, stack_kind, stack_name, action, definition, env, profiles,
     started_at, finished_at, success, exit_code, automatic, commit_hash)
SELECT id, host_id, stack_id, stack_kind, stack_name, action, definition, env, profiles,
     started_at, finished_at, success, exit_code, automatic, commit_hash
FROM stack_deploys;
DROP TABLE stack_deploys;
ALTER TABLE stack_deploys_new RENAME TO stack_deploys;
CREATE INDEX stack_deploys_by_stack ON stack_deploys (stack_id, started_at);
