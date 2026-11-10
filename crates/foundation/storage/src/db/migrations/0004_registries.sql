-- Registry credentials: one login per registry address, for every Host.
CREATE TABLE registries (
    id         TEXT PRIMARY KEY NOT NULL,
    address    TEXT NOT NULL UNIQUE, -- normalised, e.g. docker.io, ghcr.io, registry.local:5000
    username   TEXT NOT NULL,
    secret     BLOB NOT NULL,        -- password or token sealed by crypto::SecretBox
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
) STRICT;
