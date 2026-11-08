---
title: Workloads and Pods
description: Browse a Cluster's Namespaces, Deployments, StatefulSets, DaemonSets, Jobs, CronJobs and Pods, live and read-only.
---

Select a Cluster in the switcher and the sidebar shows its pages. **Workloads** and **Pods** list what runs on it; every list follows the cluster live through a Kubernetes watch, so a Deployment you scale or a Pod that crashes shows its new state within seconds without refreshing. These pages are read-only: logs, a shell, scaling and deleting come in a later version.

## Namespaces

Each Cluster page shows one Namespace, or **All namespaces**, picked at the top right. It starts at the namespace of the Cluster's kubeconfig context (else `default`), and Homeport remembers your choice per Cluster.

If your credentials may only see some Namespaces, pick one of those: the pages work there. When they can't even list Namespaces, the picker becomes a field where you type the one you're allowed to use.

## Workloads

**Workloads** has a tab per kind: **Deployments**, **StatefulSets**, **DaemonSets**, **Jobs** and **CronJobs**. Each row shows its health, how many Pods are ready of how many it wants, how many run its newest template, its images and its age.

| Health          | Means                                                                             |
| --------------- | --------------------------------------------------------------------------------- |
| **Ready**       | Every wanted Pod is ready and up to date; a Job finished; a CronJob is scheduled. |
| **Progressing** | Rolling out, scheduling, or a Job still running.                                  |
| **Degraded**    | A Deployment's rollout passed its progress deadline.                              |
| **Failed**      | A Job failed.                                                                     |
| **Paused**      | Scaled to zero, or a suspended Job or CronJob.                                    |

A workload's page has:

- **Overview**: its namespace, replicas, rollout strategy, the labels it selects Pods by, its labels and annotations, and its containers with their images and ports.
- **Pods**: the Pods it selects (for a CronJob, **Jobs**: the Jobs it started, newest first).
- **Conditions**, **Events** (warnings first; Kubernetes keeps them for about an hour) and **YAML**, read-only and without `managedFields`.

## Pods

**Pods** lists each Pod with its status as `kubectl get pods` shows it — `Running`, `Pending`, `CrashLoopBackOff`, `ImagePullBackOff`, `Init:…`, `Terminating` — its ready containers, restarts, Node and age. Long lists show 100 rows a page, so a cluster with thousands of Pods stays responsive; the search box filters all of them.

A Pod's page shows its Node, IPs, QoS class, service account and what controls it, then each container (init containers first) with its image, state, why it waits or ended, its restarts and why it last restarted. **Conditions**, **Events** and **YAML** follow, as for workloads.

## When your credentials can't see something

A list your credentials aren't allowed to read says so — "Your credentials can't list daemonsets in shop." — with the `kubectl auth can-i` command to check; the other pages keep working.
