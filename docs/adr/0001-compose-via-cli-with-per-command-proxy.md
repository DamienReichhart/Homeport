# Compose via the `docker compose` CLI, with a per-command proxy

Bollard speaks only the Docker Engine API and has no Compose support, and reimplementing Compose in Rust would take months. So Homeport finds Stacks through the `com.docker.compose.project` label and does lifecycle work through the API, while deploy, pull and `down` for Managed Stacks shell out to the local `docker compose` CLI (YAML sent on stdin). Since ADR 0010, a Stack's images are pulled through the Engine API with the stored registry logins, before `up` and in place of `compose pull`. The CLI can't use Homeport's in-memory SSH keys or encrypted TLS material. So for SSH and TLS Hosts, Homeport opens a private local proxy socket (Unix socket with 0600 permissions, or a named pipe on Windows) that forwards over the Host's existing connection, and points `DOCKER_HOST` at it for that one command only. Local and plain-TCP Hosts pass `DOCKER_HOST` through directly.

## Consequences

- Deploying needs the Compose CLI installed on the user's machine. Without it, Stacks still show up and can be started, stopped and removed.
- Compose resolves relative paths on the client, so Managed Stacks refuse `build:` and warn on relative bind mounts. Linked Stacks (ADR 0008) run compose in their folder on this machine, where those paths mean what they say.
