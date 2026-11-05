# Bastions live inline in the SSH Host, each with its own trusted key and optional key

An SSH Host can be reached through an ordered list of bastions, OpenSSH's `ProxyJump`. Homeport chains the sessions itself with russh: it logs in to the first bastion, opens a `direct-tcpip` channel from it to the next server and runs an SSH session over that channel (`client::connect_stream`), and so on until the Host, whose session carries the Engine channels and the compose proxy as before. The system `ssh` is never run, keys stay in memory and nothing is installed on a bastion (ADR 0002).

Each bastion is stored inline in `HostConnection::Ssh` as `{ host, port, user, auth, publicKey, hostKey }`, in order. `auth` is `sameAsHost` (the Host's generated key, imported key or ssh-agent), `imported` (a key of its own) or `agent`. A bastion's own private key is sealed with the Host's other secrets, in `HostSecrets::Ssh.bastions`, keyed by the bastion's `host:port`. Its server key is trusted on first use and kept in its `hostKey`, next to it.

## Considered Options

- **Bastions inline, every bastion using the Host's credentials**: the simplest, and right for the common "same key everywhere" setup, but a `~/.ssh/config` with a different `IdentityFile` per jump, or a personal key in the agent for the bastion and a generated key for the Host, couldn't be imported or entered.
- **Inline, with optional credentials per bastion** (chosen): covers both, with one more choice in the form, defaulting to the Host's.
- **Bastions as their own records, shared by several Hosts**: one trusted key per bastion for every Host behind it, but a new entity with its own store, page, glossary term and export section, for a saving (trusting a bastion once rather than once per Host) that a first-use prompt already makes cheap.

## Decisions

- **Data model.** `HostConnection::Ssh` gains `bastions: Vec<Bastion>`, empty for a direct connection. Two bastions can't share a `host:port`, and none may be the Host's own address. The order is the order of the hops.
- **Migration.** Migration `0005_bastions.sql` adds `"bastions": []` to every stored SSH connection, so existing Hosts keep their single trusted key in `hosts.host_key` and connect as before. Deserialising also defaults a missing list to empty, for bundles from 0.8.
- **Trusted keys.** The frontend never sets a `hostKey`: on update, `HostStore::update` gives each bastion the key stored for a bastion at the same `host:port`, so moving a bastion keeps its key, and changing its address forgets only that one. The Host's own key keeps its column and its rule (forgotten when the Host's address changes).
- **Statuses.** `HostKeyUntrusted` and `HostKeyMismatch` carry `bastion: Option<String>`, the `host:port` of the bastion that presented the key, or `null` for the Host. `hosts_trust_key` takes the same value, and the trust prompt names the server. Failures on a hop are classified with that hop's address, so "Homeport can't reach jump.example.com:22" is told apart from the Host. A bastion that won't open the channel to the next server gets its own message and hint (`AllowTcpForwarding`, reachability from the bastion).
- **Credentials.** `HostInput.bastionKeys` carries the keys typed for `imported` bastions, by their index in the input; an omitted key keeps the stored one for that address. A bastion's `publicKey` shows the public half of its imported key, as for the Host. When the Host's key is generated, the setup card lists every server that needs it: each `sameAsHost` bastion and the Host.
- **Export.** The export bundle (ADR 0013) carries the connection JSON as stored, bastions and their trusted keys included, and the opened secrets, bastion keys included.

## Consequences

- An SSH Host behind bastions is an SSH Host like any other: same page, same supervisor, the compose proxy forwards through the chain without knowing it.
- Two Hosts behind the same bastion trust its key separately, and a changed bastion key is reported on each of them.
- `ProxyCommand` stays out of scope: it runs a program, which a GUI app can't do portably and which would break the "nothing runs but Homeport" property.
