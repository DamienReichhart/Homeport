-- Synced Stacks (ADR 0014): Stacks whose compose file comes from a git
-- repository. A Stack name is unique per Host across this table, `stacks`
-- and `linked_stacks`, which the store checks. The environment is sealed
-- like a Managed Stack's (ADR 0002); the repository can't be removed while
-- a Synced Stack uses it.
CREATE TABLE synced_stacks (
    id              TEXT PRIMARY KEY NOT NULL,
    host_id         TEXT NOT NULL REFERENCES hosts (id) ON DELETE CASCADE,
    name            TEXT NOT NULL,
    repository_id   TEXT NOT NULL REFERENCES git_repositories (id) ON DELETE RESTRICT,
    ref_kind        TEXT NOT NULL CHECK (ref_kind IN ('branch', 'tag')),
    ref_name        TEXT NOT NULL,
    compose_path    TEXT NOT NULL, -- relative to the repository's root, with /
    env             BLOB,          -- .env text sealed by crypto::SecretBox; NULL when empty
    profiles        TEXT NOT NULL DEFAULT '[]',
    deployed_commit TEXT,          -- the commit of the last successful deploy
    fetched_commit  TEXT,          -- the ref's commit as last fetched
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    UNIQUE (host_id, name)
) STRICT;

CREATE INDEX synced_stacks_by_repository ON synced_stacks (repository_id);
