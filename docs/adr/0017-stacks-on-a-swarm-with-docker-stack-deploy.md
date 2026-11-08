# Stacks are deployed to a Swarm with `docker stack deploy`

ADR 0016 reaches a Swarm through its manager Hosts. Managed, Linked and Synced Stacks can target a Swarm as well as a Host. The Engine API has no stack endpoint: `docker stack deploy` turns a compose file into services, networks, secrets and configs on the client, and doing that conversion in Rust would be the Compose-in-Rust work ADR 0001 rejected. So a Stack is deployed to a Swarm the way it's deployed to a Host: with the CLI, on this machine, for one command.

## Decision

- **A Stack belongs to a Host or a Swarm, exactly one.** Migration 0015 rebuilds `stacks`, `linked_stacks`, `synced_stacks` and `stack_deploys` with a nullable `host_id` (still cascading from `hosts`) and a `swarm_id`, checked to be exactly one; names are unique per owner. Removing a manager Host doesn't delete a Stack of its Swarm. Existing rows keep their Host. `swarm_id` is the Swarm's ID; nothing else about the Swarm is stored (ADR 0016). Migrations now run with foreign keys off, as SQLite asks for rebuilding a table, and are checked once they've all run.
- **The runtime's `StackTarget::Swarm`** carries the connected manager the command goes through (`HostRegistry::manager_host`), with its connection and dialer. A deploy runs `docker stack deploy --compose-file … --prune [--detach=false] [--with-registry-auth] <name>` with `DOCKER_HOST` pointing at that manager, directly for local and TCP Hosts or through the per-command proxy of ADR 0001 for TLS and SSH ones; `DOCKER_CONTEXT` is removed. `--detach=false` is passed when the CLI has it, so the output shows the services converging.
- **Sources as for a Host.** A Managed Stack's YAML goes in on stdin (several files in its private folder), its environment as process variables. Linked and Synced Stacks run from their folder or worktree, so `env_file:`, `configs: file:` and `secrets: file:` resolve on this machine and are sent to the Swarm. `docker stack deploy` doesn't read a folder's `.env` as compose does, so Homeport reads it and passes it on, the Stack's own environment winning.
- **Images are the nodes' business.** No pull through the Engine API first (ADR 0010 is for a Host): every node pulls its own. **Send my registry credentials**, off by default and stored per Stack, adds `--with-registry-auth`, which sends the docker CLI's own logins (from `docker login` on this machine), not Homeport's stored registry logins: those are never handed to the CLI.
- **Taking a swarm stack down** needs no CLI: it removes the namespace's services, then (once their tasks are gone) its networks, secrets and configs through the Engine API, in `docker stack rm`'s order. Start and stop aren't offered; there is no such operation.
- **Checks in Swarm mode** (`definition::check_for`): `build:` without `image:` is refused, since the nodes can't use an image built here; keys `docker stack deploy` ignores (`container_name`, `restart`, `links`, `depends_on` conditions, `profiles`) are warned about, as are bind mounts, which are paths on each node. A Managed Stack can't read `configs:`/`secrets:` from a file, having no folder.
- Recognised failures: the docker CLI missing (`docker_cli_missing`), and a config or secret whose file changed (Swarm objects are immutable), with a hint to version its `name:`.

## Considered Options

- **Converting compose to services in Rust**: no CLI needed, but the conversion is large, follows the compose spec as it moves, and is what ADR 0001 rejected.
- **Running the CLI on a manager over SSH**: only for SSH Hosts, and it needs the CLI and the files there, against "nothing installed on a Host".
- **Keeping `host_id NOT NULL` with the manager a Stack was created through**: removing that one Host would delete the Swarm's Stack, and another manager couldn't deploy it.
- **Writing Homeport's registry logins to a temporary docker config for `--with-registry-auth`**: secrets on disk, against ADR 0002.

## Consequences

- Deploying to a Swarm needs the `docker` CLI on this machine (not Compose). **Settings → This machine** and the deploy buttons say when it's missing.
- A Stack's history, rollbacks, profiles and Synced Stacks' sync and auto-deploy work the same for a Swarm target; profiles are ignored by `docker stack deploy`, which the run says.
- A backup keeps the Stacks of Swarms in a section of its own, which a Homeport before 0.14 skips.
