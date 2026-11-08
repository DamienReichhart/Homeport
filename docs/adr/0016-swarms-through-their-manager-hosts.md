# Swarms are reached through their manager Hosts

Docker Swarm turns several Engines into one cluster, managed through the Engine API of any of its managers. Homeport already holds a connection to each Host, so it can manage a Swarm without anything new: no agent on a node, no extra credentials, no stored cluster.

## Decision

- **A Swarm is found, never added.** On connecting, and again on every `node` event and every ping, the supervisor reads the Engine's `/info`: a Host in a Swarm has a `SwarmMembership` (node ID, manager or worker, state, the managers it knows) on its `Connected` status. Managers carry the swarm ID (`Cluster.ID`); workers don't, so a worker is placed by its node ID in a manager's node list, or else by the managers it talks to.
- **Addressed by swarm ID.** `HostRegistry::swarms()` groups the manager Hosts of one swarm ID into one Swarm, however many of them are Hosts. Every Swarm command takes the swarm ID and goes through `HostRegistry::manager(swarm_id)`: any connected manager, the first in the Hosts' order. With none, it fails with `swarm_unreachable`, naming the manager Hosts last seen.
- **Nothing is stored about a Swarm.** The registry keeps each Host's last membership in memory, so a Swarm whose managers all went away still names them; restarting Homeport forgets it until a manager connects. Swarm secrets and join tokens pass through Homeport to the Engine and back, and are never written to `homeport.db`, the log or the query caches.
- **No name of its own.** Docker names every Swarm `default`, so Homeport calls one "Swarm of <first manager Host>" with its short ID.
- **Everything through the Engine API**, as for Hosts, with one exception: deploying a Stack to a Swarm runs `docker stack deploy` on this computer (ADR 0017), since the Engine has no stack endpoint.

## Considered Options

- **Adding a Swarm as its own kind of Host**, by a manager's address: a second connection to an Engine Homeport already reaches, with its own credentials to keep in step, and nothing to do when that manager dies while another is connected.
- **Storing Swarms and their manager list**: useful only offline, where nothing can be done to a Swarm anyway, and a second source of truth next to the Engine's.
- **Reading membership only on connect**: a Swarm joined from a terminal wouldn't show until a reconnect, since a worker gets no event for it.

## Consequences

- A Swarm shows as soon as one of its managers is a connected Host; adding a second manager doesn't show it twice.
- Every Swarm operation works through whichever manager is connected, and fails with one clear error when none is.
- A Swarm whose managers aren't Hosts isn't shown, even if some of its workers are: they show "Worker of a Swarm" only once a manager is added.
- The registry pays one `/info` per Host every 15 s, which every Engine answers from memory.
