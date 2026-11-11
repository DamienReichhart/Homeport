-- Clusters: Kubernetes (and, later, Nomad) APIs Homeport connects to. Their
-- credentials, from a kubeconfig, are only ever stored sealed.
CREATE TABLE clusters (
    id         TEXT PRIMARY KEY NOT NULL,
    name       TEXT NOT NULL UNIQUE COLLATE NOCASE,
    kind       TEXT NOT NULL DEFAULT 'kubernetes',
    connection TEXT NOT NULL,          -- JSON ClusterConnection, no secrets
    secrets    BLOB,                   -- ClusterSecrets sealed by crypto::SecretBox
    enabled    INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
) STRICT;
