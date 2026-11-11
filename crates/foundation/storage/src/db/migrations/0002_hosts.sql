CREATE TABLE hosts (
    id         TEXT PRIMARY KEY NOT NULL,
    name       TEXT NOT NULL UNIQUE COLLATE NOCASE,
    connection TEXT NOT NULL,          -- JSON HostConnection, no secrets
    secrets    BLOB,                   -- HostSecrets sealed by crypto::SecretBox
    host_key   TEXT,                   -- trusted SSH server key, OpenSSH format
    enabled    INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
) STRICT;
