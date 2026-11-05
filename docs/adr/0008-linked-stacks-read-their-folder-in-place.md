# Linked Stacks keep their compose files in their folder on this machine

Most Stacks are started with `docker compose up` in a project folder, often with `DOCKER_HOST` pointing at a remote Host, and their containers record that folder in `com.docker.compose.project.working_dir` and their files in `com.docker.compose.project.config_files`. Adopting such a Stack copies its YAML into Homeport, where it drifts from the folder the user keeps editing, and Managed Stacks refuse `build:`, `env_file:` and relative paths because they have no folder (ADR 0001). So Homeport adds a third kind, the **Linked Stack**: a Stack whose compose definition is a folder on this machine. Homeport stores only where it is (the Host, the name, the folder, the ordered list of compose files, and a fingerprint of the files as it last read or wrote them), reads and writes those files in place, and deploys with `docker compose --project-directory <folder> --file <each file>` run in that folder, still through the per-command proxy for TLS and SSH Hosts.

A Stack found running is Linked when both labels are present, every listed file exists here, and the files define the services that run: every running service is defined, and every defined service without `profiles:` is running or has a container. The service match keeps a path that exists here by coincidence (a Stack started on the server itself in `/srv/app`, with a `/srv/app` on this machine too) from linking the wrong files. A Linked Stack can also be created from a folder. When its files disappear, it is shown as External, with the reason, until they come back; nothing is deleted from the store or the Host.

## Considered Options

- **Adopt into a Managed Stack**: one kind fewer, but the copy drifts from the folder, and builds and relative paths still can't work.
- **Copy the folder into Homeport's data**: builds would work, but two copies drift the same way, and a folder can be large or hold secrets.
- **Trust the labels alone**: simpler, but a path that happens to exist here would link a Stack to someone else's files.

## Consequences

- Homeport writes to a Linked Stack's compose files when the user saves, atomically (a temporary file in the same folder, then a rename) and keeping the file's permissions. It refuses to overwrite a file that changed on disk since it was opened unless the user says to, and shows "Changed outside Homeport" when the files differ from the stored fingerprint.
- Homeport never reads or writes the folder's `.env`: Compose reads it, and it may hold secrets. Nothing secret is stored for a Linked Stack.
- Forgetting a Linked Stack deletes its record, never its folder or containers.
- Linked Stacks pass the Linked rules of the definition checks: `build:`, `env_file:` and relative paths are allowed. A build runs on the Host's Engine with its context sent from this machine, and a relative bind mount on a remote Host resolves to this machine's path, which the Host may not have; Homeport warns about that rather than refusing.
- Base images of `build:` are pulled by the CLI or BuildKit with the user's own logins (ADR 0010).
- A Stack started from a folder on another computer is External on this one: its labels point at a path that doesn't exist here.
- Detection parses the files with the YAML library only, so it works without the compose CLI; `include:` and `extends:` across files count as no match for now.
