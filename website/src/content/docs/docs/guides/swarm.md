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
