-- Linked Stacks (ADR 0008): Stacks whose compose files are a folder on this
-- machine. Only where they are is kept; their files stay in the folder.
-- A Stack name is unique per Host across this table and `stacks`, which the
-- store checks.
CREATE TABLE linked_stacks (
    id           TEXT PRIMARY KEY NOT NULL,
    host_id      TEXT NOT NULL REFERENCES hosts (id) ON DELETE CASCADE,
    name         TEXT NOT NULL,
    working_dir  TEXT NOT NULL,
    config_files TEXT NOT NULL, -- JSON array of absolute paths, in order
    fingerprint  TEXT NOT NULL, -- of the files as Homeport last read or wrote them
    created_at   TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at   TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    UNIQUE (host_id, name)
) STRICT;
