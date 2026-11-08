---
title: Port forwarding
description: Reach a Pod's or a Service's port from this computer, like kubectl port-forward, without exposing it.
---

A port forward makes a port of a Pod, or of a Service, reachable on this computer: a database, an admin UI or an internal API, without a `LoadBalancer`, an Ingress or a terminal kept open with `kubectl port-forward`.

## Forwarding a port

- On a **Pod**'s page, the containers list the ports they declare; **Forward…** is next to each TCP port.
- On a **Service**'s page, **Forward…** is next to each of its TCP ports.

Pick the local port: it starts at the same number when that port is free on this computer, and empty otherwise. Leave it empty and Homeport picks a free one. A toast says where it listens, with **Open** to load it in your browser.

The forward listens on the loopback addresses only — `127.0.0.1`, and `[::1]` when the same port is free there — so nothing else on your network can reach it.

### Pods and Services

A forward to a **Pod** reaches that Pod. If the Pod goes away (a rollout replaced it), the forward ends and Homeport says so.

A forward to a **Service** goes through one of its ready Pods, as `kubectl port-forward svc/…` does, to the port the Service sends that port to (its `targetPort`, by number or by the name of a container port). When that Pod goes away, the next connection picks another ready one, so a forward to a Service survives rollouts. It doesn't balance connections across Pods.

Only TCP can be forwarded. Services without a selector (pointing at external endpoints) can't be.

## Port forwards

**Port forwards**, in the sidebar of a Cluster, lists every forward running, across all your Clusters: its local address, the Pod or Service and port it reaches (and, for a Service, the Pod it goes through now), its Namespace and Cluster, and how many connections are open through it. **Open** loads it in your browser; **Stop** ends it and closes its connections.

Forwards live only as long as Homeport: quitting it stops them all, and none comes back at the next launch. Removing or disabling a Cluster stops its forwards too.

## When it doesn't work

- **Port in use**: another program already listens on that local port. Pick another, or leave it empty.
- **Ports below 1024**: most systems don't let an app listen on them; pick a higher local port.
- **Not running**, **no ready Pod**: there's nothing to forward to yet; the Pod's page says why.
- **Not allowed**: forwarding needs `create` on `pods/portforward`; Homeport names it, with the `kubectl auth can-i` command to check.
