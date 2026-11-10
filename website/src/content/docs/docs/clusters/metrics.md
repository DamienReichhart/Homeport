---
title: CPU and memory
description: See what Pods, their containers and Nodes use of CPU and memory, against their requests, limits and allocatable, from metrics-server.
---

When a Cluster runs [metrics-server](https://github.com/kubernetes-sigs/metrics-server), Homeport shows what each Pod and Node uses of CPU and memory: in the lists, and in live charts on their pages. It reads the Metrics API (`metrics.k8s.io`) that metrics-server serves, as `kubectl top` does.

## What you need

metrics-server must already run on the Cluster: Homeport installs nothing on a Cluster. Whether it's there depends on how the Cluster was made:

| Cluster                   | metrics-server                                                       |
| ------------------------- | -------------------------------------------------------------------- |
| GKE                       | Installed                                                            |
| AKS                       | Installed                                                            |
| k3s and k3d               | Installed (unless k3s was started with `--disable metrics-server`)   |
| Docker Desktop            | Installed with its Kubernetes                                        |
| kind                      | Not installed                                                        |
| EKS                       | Not installed: add the metrics-server add-on, or install it yourself |
| minikube                  | Not installed: `minikube addons enable metrics-server`               |
| Other clusters (kubeadm…) | Usually not installed                                                |

To install it yourself, follow [its instructions](https://github.com/kubernetes-sigs/metrics-server#installation); on a local cluster whose kubelets have self-signed certificates (kind), metrics-server also needs `--kubelet-insecure-tls`.

Without metrics-server nothing breaks: the CPU and memory columns aren't shown, and a Pod's or Node's **Metrics** tab says metrics-server isn't installed, with a link to it. Right after it's installed, metrics-server needs about a minute before it has samples; meanwhile the tab says the Metrics API doesn't answer yet.

Your credentials also need to read metrics: `list` and `get` on `pods.metrics.k8s.io` in the Namespace (or every Namespace), and on `nodes.metrics.k8s.io` for Nodes. When they can't, the **Metrics** tab says which permission is missing, and every other page keeps working.

## In the lists

On **Pods**, the **CPU** and **Memory** columns show what each Pod uses, its containers added together, and what share that is of its limit, or of its request when it has no limit. A Pod has a limit only when all its containers have one: a container without a limit can use the whole Node. CPU is in millicores, as `kubectl top` writes it (`250m` is a quarter of a core), and memory is the working set, what the kubelet evicts Pods on.

On **Nodes**, **CPU used** and **Memory used** show what each Node uses, and its share of what's **allocatable** to Pods.

Click a column's header to sort by it: the busiest Pods or Nodes first.

## On a Pod's or Node's page

A Pod's **Metrics** tab charts each container's CPU and memory, with a dashed line at its request and another at its limit, when they're set. A Node's charts its CPU and memory with a line at what's allocatable.

metrics-server reads the kubelets every 15 seconds, and Homeport asks for a new sample at the same pace, so a change of load shows on the chart within 30 seconds. The samples are kept while the page is open, for 15 minutes at most, whichever tab you're on, and only in memory: closing the page stops the reads and forgets them. Homeport keeps no history of metrics; for that, use a monitoring system such as Prometheus.
