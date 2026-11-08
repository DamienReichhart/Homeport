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
