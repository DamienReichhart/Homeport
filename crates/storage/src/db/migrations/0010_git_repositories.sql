-- Git repositories Homeport may read for Synced Stacks (ADR 0014), app-wide.
-- The deploy key's private half and the token are sealed by crypto::SecretBox
-- (ADR 0002); the public key and the server's host key are not secret.
CREATE TABLE git_repositories (
    id          TEXT PRIMARY KEY NOT NULL,
    url         TEXT NOT NULL,
    auth        TEXT NOT NULL CHECK (auth IN ('ssh_key', 'https_token', 'none')),
    private_key BLOB,          -- sealed OpenSSH private key, for 'ssh_key'
    public_key  TEXT,          -- its OpenSSH public half, to add to the forge
    username    TEXT,          -- for 'https_token'
    token       BLOB,          -- sealed, for 'https_token'
    host_key    TEXT,          -- the SSH server's known_hosts line, trusted on first use
    created_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    UNIQUE (url)
) STRICT;
