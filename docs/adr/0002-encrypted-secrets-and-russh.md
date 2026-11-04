# Secrets encrypted in SQLite; SSH through russh, not bollard's `ssh` feature

Homeport generates an Ed25519 keypair for each SSH Host, and the user can also import keys, TLS client material and Stack `.env` files, and store registry passwords and tokens. All of these are stored in SQLite, each field sealed separately with XChaCha20-Poly1305 under a random 256-bit key kept in the OS keychain. There's no master password. Bollard's built-in SSH support runs the system `ssh` binary and needs a key _file_, which would mean writing decrypted keys to disk. So SSH Hosts use `russh` to open a `direct-streamlocal` channel to the remote Docker socket, plugged into bollard via `connect_with_custom_transport`. Keys never leave memory, and SSH works on Windows too. Server host keys are trusted on first use and stored in SQLite rather than `~/.ssh/known_hosts`.

## Consequences

- Without an OS keychain, SSH, TLS and `.env` are unavailable; local and plain-TCP Hosts still work.
- If the keychain entry is lost, stored secrets can't be decrypted. Affected Hosts show as "credentials lost" and offer re-entry or key regeneration rather than being wiped.
