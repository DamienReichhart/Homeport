# Hosts move between computers in one file sealed with a passphrase

Secrets are sealed under a key kept in the OS keychain (ADR 0002), so copying `homeport.db` to another computer leaves every Host **Credentials lost**. Homeport instead exports the Hosts the user picks to one file encrypted with a passphrase they type, and imports such a file on any computer, where the secrets are sealed again under that computer's own keychain key.

## The format

```
"HOMEPORT-BUNDLE" | format version (1) | Argon2id memory KiB, iterations, lanes (u32 LE each) | salt (16) | nonce (24) | ciphertext
```

- The key is Argon2id over the passphrase and the random salt (RustCrypto `argon2`): 64 MiB, 3 iterations, 1 lane for new files. The cost is in the header so it can rise without a new format; a file asking for more than 1 GiB, 16 iterations or 16 lanes is refused before deriving.
- The ciphertext is XChaCha20-Poly1305 over the whole JSON payload, with everything before it as associated data, so the cost, salt and nonce can't be changed either. A wrong passphrase and a damaged file both fail authentication, and are reported together (`bundle_unreadable`).
- The payload is `{ "version": 1, "createdAt", "appVersion", "hosts": [...] }`. Each Host is its name, its connection as stored (bastions and their trusted keys included, ADR 0012), whether it's enabled, its trusted server key, and its opened secrets. A later Homeport adds sections next to `hosts` (Stacks, settings: the v0.12 backup), which this one skips; a payload `version` above 1, or a format byte above 1, is reported as coming from a newer Homeport (`bundle_too_new`).
- Passphrases have at least 12 characters. They are used, never stored or logged.

## Considered Options

- **Copying the database and the keychain key**: no new format, but it moves every Host or none, needs the user to export a keychain item by hand on three systems, and puts a key that opens everything in a file.
- **Sealing each field like `SecretBox`, with a passphrase-derived key**: names and addresses would stay readable in the file. Encrypting the whole payload reveals nothing but its size.
- **age or another external format**: a second encryption scheme to review for one feature, where the AEAD already used for secrets does the job with a key-derivation step in front.

## Consequences

- An imported Host connects with no trust prompt and no key to re-enter: its server keys came along. A Host whose secrets couldn't be opened when exporting is written without them, the export says which, and it imports with its credentials missing until re-entered.
- On a name clash the preview proposes `prod (2)`; unpicking a Host skips it.
- Deriving the key takes a fraction of a second in release builds; `argon2` and `blake2` are optimised in dev builds too, so tests and `tauri dev` stay fast.
- Losing the passphrase loses the file. Nothing in Homeport can open it without one.
- The v0.12 backup of all of Homeport's data uses this format and payload version, with `"backup": true` and its sections (`registries`, `repositories`, `stacks`, `settings`) next to `hosts`, whose entries then keep their `id` so Stacks find their Host. Hosts import reads a backup's Hosts like an export's; restoring refuses an export, which would replace everything with some Hosts.
