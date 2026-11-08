# Incus through its REST API, with pinned certificates for HTTPS

## Context

v0.20.0 lets a Host's Engine be Incus: its projects, Instances (system containers and virtual machines) with their console, exec and snapshots, and its images, profiles, storage pools and networks. Homeport is a remote client that installs nothing on a Host (ADR 0002), and every Host is reached through a `Dialer` (#74).

What Incus offers, from its API and documentation:

- **The API** is REST over HTTP/1.1. Every answer is an envelope: `sync` with the result in `metadata`, `async` with an operation to wait on (`/1.0/operations/{id}/wait`), or `error` with Incus's own words. Changes travel as JSON; a few resources (profiles, instances) answer with an `ETag` that a `PUT` may send back as `If-Match`.
- **Sockets**: `/var/lib/incus/unix.socket`, full access, for the group `incus-admin`; `/var/lib/incus/unix.socket.user`, served by `incus-user`, restricted to the user's own project, for the group `incus`.
- **HTTPS**, once `core.https_address` is set (port 8443 by default), with client certificates. The server's certificate is usually self-signed; `incus remote add` pins it by its SHA-256 fingerprint. A client is trusted with a token from `incus config trust add <name>`: base64 JSON naming the client, the server certificate's fingerprint, its addresses, a secret and an expiry. The client presents any certificate over TLS and sends the token to `POST /1.0/certificates`.
- **Events, console and exec** are websockets: `/1.0/events`, and the data and control websockets of an exec or console operation, opened with the secrets in its metadata.
- **Projects** scope instances, images, profiles, storage volumes and networks; every request names its project with `?project=`.

## Decision

- **Client**: a crate of Homeport's own, `homeport-incus`, speaking the REST API with the HTTP/1.1 client `homeport-transport` already has (`http`), and websockets with `tokio-tungstenite` over a stream from the same dialer. No Incus client library exists in Rust, and the Go one can't be used.
- **Connections**: the local sockets and SSH, as for Docker and containerd, and **HTTPS** with a client certificate. Plain TCP is refused when the Host is saved: Incus doesn't serve its API without TLS.
- **Server certificates are pinned.** An Incus Host over HTTPS keeps the server's certificate in its sealed TLS material, where a Docker Host keeps its CA, and the dialer accepts exactly that certificate, compared by SHA-256 fingerprint, whatever the name it's for. A changed certificate is refused with both fingerprints, never accepted silently.
- **Trust tokens**: adding an Incus Host over HTTPS takes the token by default. Homeport reads it, generates an ECDSA P-256 client certificate in memory with `rcgen`, tries the token's addresses in order until one presents the certificate the token names, sends the token, and checks that Incus now answers `auth: trusted`. The certificate, its key and the server's certificate are sealed like every other secret (ADR 0002) and never written to disk in clear. Pasting an existing client certificate and key, with the server certificate to pin, stays possible.
- **Projects**: chosen per Host on this computer and remembered with the selected Host, like containerd's namespaces: `default` when the user may use it, else the first project Incus lists (the restricted socket's own). Every project-scoped request names it.
- **Operations** are waited on in Homeport, with a timeout, and their progress (image downloads) is streamed to the page that started them.
- **Remote images** are listed from the simplestreams index of the image server (`https://images.linuxcontainers.org` by default), fetched from this computer and kept a few minutes. Incus itself downloads the image.
- **Stacks** aren't deployed to Incus Hosts in 1.0.

## Considered Options

1. **REST API over the dialer, with HTTPS pinned** (chosen).
2. **The `incus` CLI over SSH**, as Helm's CLI is used (ADR 0009). It would need the CLI on the Host or on this computer, and its remotes' configuration; output parsing would follow its versions. Local and HTTPS Hosts wouldn't have it.
3. **HTTPS verified against a CA**, as for Docker Hosts. Incus's certificates are self-signed, so the user would have to export and paste them, and a name mismatch (an address, not a name) would refuse most Hosts.

## Consequences

- Incus Hosts are reached through the same `Dialer` as every other Host; the supervisor reads `GET /1.0`, follows `/1.0/events` over a websocket and pings with `GET /1.0`.
- `TlsDialer` has a pinned mode beside its CA mode.
- Homeport runs nothing on an Incus Host: console and exec go through Incus's own websockets.
- The client certificate Homeport registers shows in `incus config trust list` under the name the token was made for; removing it there revokes Homeport.
