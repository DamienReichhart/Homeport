---
title: Scaling, restarting and deleting
description: Scale Deployments and StatefulSets, restart a rollout, suspend and run CronJobs, and delete any object of a Cluster, each after a confirmation that names it.
---

The everyday changes to a Cluster are a click away on its pages, each sent to the Kubernetes API with the Cluster's own credentials, as `kubectl` would. The lists follow the change live through their watch.

Every action that removes or replaces something asks first, and the confirmation names the cluster, the namespace, the kind and the name, so a change never lands on the wrong cluster or namespace by accident.

## Workloads

A workload's page has its actions in the header; on **Workloads**, each row has them in its **⋯** menu.

| Action      | On                                    | Does, like                                                         |
| ----------- | ------------------------------------- | ------------------------------------------------------------------ |
| **Scale**   | Deployments, StatefulSets             | `kubectl scale --replicas=N`                                       |
| **Restart** | Deployments, StatefulSets, DaemonSets | `kubectl rollout restart`                                          |
| **Suspend** | CronJobs                              | `kubectl patch cronjob … -p '{"spec":{"suspend":true}}'`           |
| **Resume**  | Suspended CronJobs                    | the same, with `false`                                             |
| **Run now** | CronJobs                              | `kubectl create job --from=cronjob/…`                              |
| **Delete**  | All of them                           | `kubectl delete` (its Pods and Jobs go with it, in the background) |

### Scale

**Scale** asks how many Pods to run, showing how many it runs now. Scaling to `0` stops the workload until you scale it up again. If a HorizontalPodAutoscaler manages the workload, the dialog says so: the autoscaler will set the number back on its next check, so change the autoscaler's minimum and maximum instead.

### Restart

**Restart** adds the annotation `kubectl.kubernetes.io/restartedAt` to the Pod template, exactly as `kubectl rollout restart` does: the workload rolls out a new set of Pods following its strategy (a new ReplicaSet for a Deployment), and the old Pods go once the new ones are ready.

### CronJobs

**Suspend** stops a CronJob from starting new Jobs (those already running go on) and **Resume** lets it start them again; a suspended CronJob shows as **Paused**. **Run now** starts a Job from its template at once, named `<cronjob>-manual-<id>`, owned by the CronJob and marked `cronjob.kubernetes.io/instantiate: manual` like kubectl's. It shows under the CronJob's **Jobs** tab.

## Deleting

Every page of an object — a workload, a Pod, a Service, an Ingress, a ConfigMap, a Secret, a Node, or any object under **API resources** — has **Delete** in its header. On a list, tick rows and **Delete N** removes them all after one confirmation that lists them.

Objects are deleted with propagation `Background`, as `kubectl delete` does: the object goes at once and the garbage collector removes what it owns (a Deployment's ReplicaSets and Pods, a CronJob's Jobs).

The confirmation says what happens next:

- **A Pod a controller owns** — a ReplicaSet, StatefulSet, DaemonSet or Job — comes back: the controller starts a new one in its place. Delete or scale the workload to get rid of it.
- **A Namespace** takes everything in it: its workloads, Pods, Services, ConfigMaps and Secrets.
- **A Node** is removed from the cluster's list without being drained first.

## When it doesn't work

- **Not allowed**: a change your credentials may not make says so with its verb and resource — "Your credentials don't allow “patch” on deployments in shop." — and the `kubectl auth can-i` command to check.
- **Changed meanwhile**: when someone else changed the object between your look and your action, Homeport says so; look at it again and retry.
- **Already gone**: deleting something already deleted says it no longer exists.
