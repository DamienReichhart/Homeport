# Helm via the `helm` CLI, with a private proxy to the Cluster's API

## Context

v0.17.0 manages Helm releases on Kubernetes Clusters: list them and their history, install a chart, upgrade, roll back and uninstall. Helm is a Go program and library with no usable Rust counterpart, and it only reaches a cluster through a kubeconfig. Homeport keeps every Cluster credential sealed in SQLite (ADR 0002, ADR 0018) and builds its kube-rs client in memory, exec-plugin tokens included, so there is no kubeconfig with credentials it could hand to `helm`, and writing one would break ADR 0002. ADR 0001 met the same problem with `docker compose` and solved it with a per-command proxy; this decision extends it to Clusters.

## Decision

- **Homeport runs the user's `helm` 3 CLI** for every Helm action, with `--kubeconfig <temp>` and `KUBECONFIG=<temp>`.
- **The temporary kubeconfig points at a loopback proxy.** For each command, `homeport_clusters::proxy` listens on `127.0.0.1` on a random port and forwards every request through the Cluster's authenticated kube-rs client: the same server, CA, TLS server name, HTTP proxy and credentials as the Cluster's pages, exec-plugin tokens included and renewed as they are there. Streaming responses (watches, `--wait`) and upgraded connections (SPDY or WebSocket) pass through.
- **The kubeconfig holds no Cluster credential.** It has one cluster (`server: http://127.0.0.1:<port>`), one user with a random 256-bit bearer token for the proxy, one context and the Cluster's default Namespace. It is written as a 0600 file in a fresh 0700 temporary folder, deleted with the proxy when the command ends.
- **The proxy refuses other local processes.** A request without `Authorization: Bearer <that token>` gets a 401; the header is removed before the request goes on. The token is only in that file, and grants nothing once the command has ended and the listener is closed.
- **Values go on stdin** (`-f -`), never in a file.
- **helm's own configuration stays the user's.** Chart repositories, registry logins and caches are helm's (`repositories.yaml`, `helm env`), shared with the terminal. Homeport lists, adds (public only), updates and removes repositories through `helm repo`, and stores no chart repository credentials.

## Considered Options

- **A Helm SDK in Rust**: none is mature, and reimplementing chart rendering, hooks and the release storage is out of reach.
- **Writing a real kubeconfig with the Cluster's credentials**: the least code, but certificates, tokens and exec plugins' output would sit in clear on disk, which ADR 0002 rules out.
- **Reading the release Secrets directly** (`owner=helm`, gzipped and base64-encoded): works for listing and history, not for install, upgrade or rollback, which would still need `helm`.

## Consequences

- Helm features need the `helm` 3 CLI on this machine, as deploying Stacks needs compose. Without it, Settings and the Helm page say how to install it, and the rest of a Cluster's pages work.
- Helm keeps releases as Secrets in each release's Namespace, written through the Kubernetes API: Homeport installs nothing on the Cluster, and releases made from a terminal and from Homeport are the same.
- The proxy must carry streaming and upgraded connections, and keep working for long commands (`--wait`): it asks the Cluster's supervisor for its current client on each request, so renewed exec-plugin tokens are used.
- Private chart repositories and OCI registries needing a login are added with `helm repo add` or `helm registry login` in a terminal, where helm keeps their credentials as it always does; Homeport then sees them.
