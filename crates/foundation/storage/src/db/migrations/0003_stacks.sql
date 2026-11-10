-- Managed Stacks: compose definitions Homeport holds for a Host.
CREATE TABLE stacks (
    id         TEXT PRIMARY KEY NOT NULL,
    host_id    TEXT NOT NULL REFERENCES hosts (id) ON DELETE CASCADE,
    name       TEXT NOT NULL,
    compose    TEXT NOT NULL,
    env        BLOB, -- .env text sealed by crypto::SecretBox
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    UNIQUE (host_id, name)
) STRICT;
