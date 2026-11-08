---
title: Podman pods
description: List, start, stop and remove Podman pods, create one with its published ports, and run containers in it.
---

A **pod** is Podman's way of grouping containers: they share one network (and their hostname and IPC), so they reach each other on `localhost`, and they start and stop together. Each pod has an **infra container** that holds what they share and the ports the pod publishes. Homeport shows pods on Hosts whose Engine is Podman, through Podman's own API on the socket the Host already uses; nothing is installed on the Host.

The **Pods** page appears in the sidebar only while a Podman Host is selected. On any other Docker-compatible Engine it isn't there, and a link to it says why. See [Engine kinds](/docs/hosts/#engine-kinds) for how Homeport tells Podman apart.

## The Pods page

| Column         | Shows                                                                                                          |
| -------------- | -------------------------------------------------------------------------------------------------------------- |
| **Name**       | The pod's name, with its status as a dot.                                                                      |
| **Status**     | **Running**, **Partly running** (some containers stopped), **Paused**, **Stopped**, **Created** or **Failed**. |
| **Containers** | How many of its own containers run, the infra container not counted.                                           |
| **Networks**   | The networks the pod is on.                                                                                    |
| **Created**    | When it was created.                                                                                           |

Each row has the action that fits its status — **Start** for a stopped pod, **Stop** for a running one — and **Remove**. Select a pod to open its page. The list updates on its own as the pod's containers start and stop, whether from Homeport or a terminal.

On the **Containers** page, a container that runs in a pod shows the pod's name next to its own; select the name to open the pod.

## A pod's page

The header has **Run container** and the pod's actions: **Start**, **Stop**, **Restart**, **Pause**, **Resume** and **Remove**, as its status allows. Each acts on every container of the pod at once, as `podman pod start` and friends do.

- **Containers** lists the pod's containers, its infra container last. Select one to open its container page, with its logs, terminal and files.
- **Settings** shows the ports the pod publishes, the namespaces its containers share, its networks, hostname and labels, and the `podman pod create` command it was made with when Podman kept it.

## Removing a pod

**Remove** asks first and says how many containers go with the pod. A pod that's running is stopped first: the button reads **Stop and remove**. The pod's volumes are kept.

## Creating a pod

**New pod** (or `n` on the Pods page, or **New pod** in the command palette) asks for:

- a **Name**: letters, digits, `_`, `.` and `-`, starting with a letter or digit;
- **Ports** to publish, host port to container port, as on the Run dialog. Leave the host port empty to let Podman pick a free one;
- **Labels**, one `key=value` per line.

Ports belong to the pod: they are published on its infra container and reach whichever container in the pod listens on that port. The new pod's page opens, ready to run containers in it. A pod has no containers of its own until you run one; Podman starts it with the first.

## Running a container in a pod

On a Podman Host, the **Run** dialog has a **Pod** field, and a pod's **Run container** button opens the dialog with that pod picked. A container in a pod:

- shares the pod's network, so it can't publish ports or join another network: the dialog hides those fields once a pod is picked;
- uses the pod's hostname and DNS settings, so Homeport refuses its own;
- keeps everything else the Run dialog offers: command, environment, mounts, restart policy, limits, capabilities, devices, healthcheck and logging.

Duplicate and Edit never move a container into a pod: Podman can't move an existing container into one.

## What Homeport needs

Pods work on Podman 4.0 and later, over every kind of connection — local socket, SSH, TLS or TCP — since they go through the same socket as everything else.
