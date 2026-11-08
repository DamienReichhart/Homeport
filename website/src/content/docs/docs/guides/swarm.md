---
title: Swarms
description: Find your Docker Swarms through their manager hosts and manage their nodes, services, tasks, secrets, configs and swarm stacks.
---

A **Swarm** is a Docker Swarm cluster. Homeport doesn't add a Swarm on its own: it finds it through the hosts that belong to it, and manages it through any connected manager. Nothing is installed on any node, and nothing about a Swarm is stored on this computer.

## Finding a Swarm

Add one manager of the Swarm as a host, the way you add any other host (see [Hosts and connections](/docs/hosts/)). As soon as it's connected, the Swarm appears:

- in the sidebar, under **Swarms**,
- on the **Overview**, as a card of its own,
- on each of its hosts, as **Manager of** or **Worker of** the Swarm.

Docker gives every Swarm the same name, so Homeport calls one **Swarm of** its first manager host, with the start of its swarm ID. Adding a second manager of the same Swarm doesn't show it twice: Swarms are told apart by their swarm ID. A worker host shows which Swarm it belongs to once one of that Swarm's managers is a host too.

The Swarm's page starts with its **Summary**: its manager and worker hosts, how many nodes and managers it has, how many services run on it, and its swarm ID.

### When no manager is connected

Everything Homeport does to a Swarm goes through one of its connected managers, whichever answers first. When none is connected, the Swarm's page says **No manager of this Swarm is connected** and names the manager hosts it last saw: reconnect one of them, or add another manager as a host.

## Creating a Swarm

Homeport can set up a Swarm from hosts it already connects to, with nothing to copy between terminals. Each connected host's page has a **Swarm** card:

1. On the host that will manage the Swarm, choose **Init a Swarm**. Homeport fills in the **advertise address** (the address other nodes reach it at) when it reaches the host by IP; leave it empty to let the Engine pick, which works when the machine has a single address. The host becomes the Swarm's first manager, and the Swarm appears in the sidebar.
2. On each other host, choose **Join a Swarm**, pick the Swarm, and join as a **worker** or a **manager**. Homeport fetches the join token from a connected manager. To join a Swarm Homeport doesn't know, choose **Another Swarm, by its token** and paste the token and a manager's address from `docker swarm join-token worker`.
3. For machines that aren't hosts in Homeport, the Swarm's **Summary** has **Join commands**: reveal them and copy the `docker swarm join` command for a worker or a manager. Tokens are fetched when you reveal them and are never stored or logged.

**Leave the Swarm**, on a host's card, takes it out: a worker's tasks move to other nodes. A manager needs **Force**; demote it first to keep the Swarm healthy. The last manager leaving takes the Swarm with it, with every service, secret and config, so Homeport asks you to type the host's name first.

### Ports between nodes

Nodes talk to each other directly, not through Homeport, and Homeport installs nothing on them, so it can't open ports for you. Between every node of a Swarm, in their firewalls and your network's, allow:

| Port | Protocol | For                                   |
| ---- | -------- | ------------------------------------- |
| 2377 | TCP      | Managing the Swarm (to the managers)  |
| 7946 | TCP, UDP | Nodes finding each other              |
| 4789 | UDP      | Overlay network traffic between nodes |

When a host can't reach the managers, joining fails with **This host couldn't reach the Swarm's managers**, and the hint lists these ports.

## Nodes

The **Nodes** tab lists every node of the Swarm: its hostname, whether it's a manager (and the leader, reachable or unreachable) or a worker, its availability, whether it's up (`ready`) or `down`, its Engine version, address and labels. A node that is also one of your hosts links to the host's page, and the host's page links back to its node. The list refreshes on its own as nodes change.

Each node's menu changes it:

| Action                 | What it does                                                                                     |
| ---------------------- | ------------------------------------------------------------------------------------------------ |
| **Activate**           | The scheduler may place new tasks on it again.                                                   |
| **Pause**              | It keeps its tasks but gets no new ones.                                                         |
| **Drain**              | Its tasks move to other nodes: do this before maintenance.                                       |
| **Promote to manager** | It joins the managers. Keep an odd number of them (1, 3 or 5) so the Swarm survives one failing. |
| **Demote to worker**   | It leaves the managers but keeps running tasks.                                                  |
| **Edit labels**        | Sets the labels placement constraints choose nodes by, such as `node.labels.zone==a`.            |
| **Remove**             | Takes it out of the Swarm.                                                                       |

Homeport refuses, before asking the Engine, to demote the Swarm's last reachable manager (a Swarm without one can't be managed at all) and to drain its last active node (its tasks would have nowhere to go). Removing a node that's still up needs **Force**; a manager must be demoted first. If someone else changed a node since the list was read, the change is refused with **This was changed elsewhere**: refresh and try again, so nothing is overwritten.

## Services

The **Services** tab lists what the Swarm runs: each service's name, image, mode (replicated with its number of replicas, or global: one task on every node), how many of its tasks run against how many should (in warning colour while fewer run), its published ports, the swarm stack it belongs to, and the state of its last update. The list refreshes on its own as services change.

Each service's menu:

- **Logs** opens its logs, merged from every task, each line labelled with its task and node, such as `web.2@node-b`. While following, the tasks the Swarm starts later join in.
- **Scale** sets how many replicas a replicated service runs; 0 stops it without removing it. Global services aren't scaled: they run one task per node.
- **Roll back** puts back the settings the service had before its last update (image, environment, replicas…), the way `docker service rollback` does. A service that was never updated has nothing to roll back to, and says so.
- **Remove** stops and removes its tasks. Its volumes are kept.

Select a service to open it: the **Logs** tab follows its logs, and **Inspect** shows its full definition as the Engine has it, including what Homeport doesn't edit (placement constraints, resource limits, health check, update and rollback policies).

### Create and update a service

**Create service**, on the Services tab, starts a service from an image: its name, whether it's replicated (and how many replicas) or global, when its tasks restart, a command to replace the image's own, environment variables (one `KEY=value` per line), published ports, volumes and bind mounts, the overlay networks it joins, and the secrets and configs it's given, each mounted where you say (`/run/secrets/<name>` for a secret by default). Ports go through the Swarm's routing mesh, reachable on every node, unless you choose to publish them on the node running each task only. Bind mounts are paths on each node, not on this computer. Homeport checks the form before sending it: a valid name, `KEY=value` entries, a port published twice. The new service opens on its **Tasks** tab, where you watch its replicas start.

**Edit**, on a service's page, changes its image, environment and replicas, and only those: its ports, mounts, networks and everything else are sent back as they are. The Swarm then rolls its tasks over to the new settings a few at a time, and the page shows the update's progress. **Redeploy every task** restarts them even when nothing changed, pulling the image again, which is how a new push of the same tag is rolled out. If someone else updated the service since you opened it, Homeport says so instead of overwriting their change: refresh and edit again.

Private images: the nodes pull images themselves, so a private image needs a login on each node (`docker login` there), or a Stack deployed with your registry credentials (see [Deploying Stacks](#deploying-stacks-to-a-swarm)).

## Tasks

A service runs as **tasks**: one per replica (its slot, such as `web.2`), or one per node for a global service (`agent@node-b`). Each task runs as a container on its node. When a service shows fewer tasks running than it should, the answer is in its tasks.

The **Tasks** tab lists the Swarm's tasks, grouped by service then slot, newest first: the node each runs on, what it's doing and what the Swarm wants it to do, since when, its error and its image. A service's page has the same tab for that service only, and a node's menu has **Show its tasks**. By default each slot shows only the task the Swarm wants running; turn on **Show history** to see the earlier attempts it shut down or that failed. A task on a node that's also one of your hosts has **Open container**, which selects that host and opens the container.

Common errors:

| Error                                                     | What it means                                                                                                                            |
| --------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| `No such image`, `pull access denied`                     | The node can't pull the image: check its name, or whether the registry needs a login on that node.                                       |
| `no suitable node`                                        | No node meets the service's constraints, has the resources it reserves, or is active: check placement constraints and node availability. |
| `port is already allocated`, `address already in use`     | A port published in `host` mode is taken on that node.                                                                                   |
| `invalid mount config`, `bind source path does not exist` | A bind mount names a path the node doesn't have. Bind mounts are paths on each node.                                                     |
| `task: non-zero exit (1)`                                 | The container started and exited with an error: open its logs from the service's **Logs** tab.                                           |

## Secrets and configs

Services read credentials from **secrets** and configuration files from **configs**, both mounted as files in their containers (secrets under `/run/secrets/`). The **Secrets** and **Configs** tabs list them with the services that use them, their labels, and when they were created and updated.

**Create secret** asks for a name and the value, masked unless you choose **Show**, or **Load from file…** to send a file from this computer instead (a key, a certificate). The value goes to the Swarm once: Homeport doesn't keep it, and the Engine never sends it back, so no one can read it from the Swarm, Homeport included. **Create config** works the same, with the content shown as you type; a config's content can be viewed again from its row. Both are limited to 500 KB.

Secrets and configs can't be changed once created. To change one, create a new one under a new name, such as `db_password_v2`, point the services at it, then remove the old one. A secret or config a service uses can't be removed: the Swarm refuses, and Homeport names the services that use it.

## Swarm stacks

A **swarm stack** is a Stack deployed to a Swarm: a compose file turned into services, networks, secrets and configs that share a namespace. Homeport finds every one of them, however it was deployed, from the `com.docker.stack.namespace` label `docker stack deploy` puts on them. The Swarm's **Stacks** tab lists them, like the **Stacks** page lists a host's: their kind, state (running when every task of every service runs, stopped when every service is scaled to 0) and services.

A Stack deployed from a terminal with `docker stack deploy` is **External**: Homeport has no compose file for it. Select it to see its services, each linking to its page, and its networks, secrets and configs. **Remove** does what `docker stack rm` does: it removes the services, waits for their tasks to stop, then removes the networks, secrets and configs of the namespace. Anything the Swarm refused to remove is listed. Volumes are kept on each node.

A swarm stack has no start or stop: Docker has no such operation for one. To stop a Stack without removing it, scale its services to 0 from their pages.

On a host's **Containers** page, the containers that a swarm stack's tasks run there are grouped under that Stack, marked **Swarm**, rather than listed one by one.

## Deploying Stacks to a Swarm

Managed, Linked and Synced Stacks can be deployed to a Swarm instead of a host. On **New Stack** (from the Stacks page, or **New Stack** on a Swarm's **Stacks** tab), choose the Swarm under **Deploy to**, then write the compose file, pick a folder or a repository as for a host. The Swarm's **Stacks** tab lists them with the External ones; a Stack's page there has **Deploy** or **Redeploy**, its **Definition** to edit, and its **History** to roll back, as on a host.

Homeport deploys with `docker stack deploy`, run on this computer against whichever of the Swarm's managers is connected, through the same private, per-command connection as compose: nothing is installed or written on any node. It needs the `docker` command on this computer (see [requirements](/docs/requirements/)). With no manager connected, deploying says so. A Stack of a Swarm belongs to the Swarm, not to the manager it was created through: removing that host keeps it.

What changes on a Swarm:

- **Images come from a registry.** Each node pulls the images of the tasks it runs, so a service built from local files (`build:` without `image:`) is refused before anything runs: push the image first and name it in `image:`. **Send my registry credentials**, in a Stack's **Definition**, has the docker CLI send the logins it has on this computer (`docker login`) to the Swarm, so its nodes can pull private images. It's off by default; Homeport's own registry logins are never handed to the CLI.
- **Some compose keys are ignored** by `docker stack deploy`: `container_name`, `restart` (use `deploy.restart_policy`), `depends_on` conditions and `profiles`. The editor warns about them. Bind mounts are paths on whichever node runs the task.
- **Remove** takes the Stack down as `docker stack rm` does, and there's no start or stop.

### Linked and Synced Stacks on a Swarm

Most swarm stacks live in a folder or a repository next to their config files, and that's where Linked and Synced Stacks shine on a Swarm. Homeport runs `docker stack deploy` from the Stack's folder on this computer (a Linked Stack's, or a Synced Stack's checkout), so relative `env_file:`, `configs: file:` and `secrets: file:` are read here and sent to the Swarm. The folder's `.env` is read too, as compose would (`docker stack deploy` doesn't on its own). Nothing is written on any node, and secrets read from files go to the Swarm without Homeport keeping them.

- A service with `build:` needs an `image:` too: Homeport can't hand the nodes an image built here. Build and push it, then deploy.
- A Synced Stack on a Swarm syncs, deploys and auto-deploys as on a host: each deploy uses the commit just fetched.
- An External swarm stack deployed from a folder can be linked to it: on its page, choose **Link to a folder…**. Homeport checks that the folder's compose files declare the services the Stack runs before linking, so a Stack isn't tied to another project's files.

Swarm configs and secrets can't be changed once created. When a `file:` source changes, redeploying fails with **A config or secret of this Stack changed**: give the object a versioned name, for example

```yaml
configs:
  app_conf:
    name: app_conf_${CONF_VERSION}
    file: ./app.conf
```

and change `CONF_VERSION` in the folder's `.env` (or the Stack's environment) each time the file changes. Homeport never removes and recreates one behind your back.
