---
title: Logs and terminals
description: Follow a Pod's logs or a whole workload's, see why a container crashed, and open a terminal in a Pod's container.
---

A Pod's page has **Logs** and **Terminal** tabs, and a workload's page a **Logs** tab for all its Pods: the `kubectl logs` and `kubectl exec -it` of every day, through the Kubernetes API with the Cluster's own credentials.

## Logs

**Logs** streams a container's logs as they're written. It's the same viewer as a container's on a Host:

- **History**: start from the last 100, 1,000 or 10,000 lines, or all of them.
- **Since**: only lines from the last minutes, hours or days.
- **Timestamps**: the time Kubernetes recorded each line.
- **Follow**: keep the newest line in view; turn it off to read back while new lines keep arriving.
- **Find**, **Clear** and **Download** (as a text file, the lines shown so far).

A Pod with several containers has a **Container** picker, init containers marked. It opens on the container `kubectl` would pick: the one named by the Pod's `kubectl.kubernetes.io/default-container` annotation, else its first app container.

Kubernetes keeps a container's standard output and error together, so there's no stdout/stderr filter as there is for a Host's containers.

The stream stops when you leave the tab, and says so when the container stops.

### Why did it crash?

When a container restarted, **Previous instance** shows the logs of the run before the last restart — for a Pod in `CrashLoopBackOff`, the run that crashed — like `kubectl logs --previous`. It's off for a container that never restarted, since there's nothing to show.

When a container hasn't started yet (`ContainerCreating`, `CrashLoopBackOff` while it waits to start again), the logs say so with the reason, in place. Its Pod's page says why it waits.

### A workload's logs

A Deployment's, StatefulSet's, DaemonSet's or Job's **Logs** tab interleaves the logs of all its Pods as they're written, each line labelled with its Pod in its own colour, like `kubectl logs -l app=api --prefix`. Pick one Pod to read it alone, and the container when its template has several.

## Terminal

**Terminal** opens a shell in one of the Pod's running containers: `bash` when the image has it, `sh` otherwise. Type a command in the field instead (`psql -U app shop`, `top`) and **Connect** runs it, quoted as a shell would.

It's a real TTY: `vi`, `top`, colours and arrow keys work, and resizing the window resizes the terminal in the container. When the process exits, the terminal says with which code; **Reconnect** starts another. Leaving the tab ends the process in the container.

### When it can't open

- **No shell**: minimal (distroless) images have no shell at all. Type a command the image has, or attach a debug container from a terminal with `kubectl debug -it <pod> --image=busybox --target=<container>`; Homeport doesn't create debug containers.
- **Not running**: a container that is waiting or has finished can't run anything. A completed Job's Pods have stopped for good.
- **Not allowed**: opening a terminal needs `create` on `pods/exec`, and reading logs `get` on `pods/log`, which read-only roles often lack. Homeport says which, with the `kubectl auth can-i` command to check.

Homeport doesn't copy files in or out of a Pod (`kubectl cp`).
