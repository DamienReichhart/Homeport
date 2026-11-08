---
title: API resources
description: Browse every type of object a Cluster serves, custom resources included, with the columns kubectl get shows.
---

Homeport has pages of its own for workloads, Pods, Services, Ingresses, ConfigMaps, Secrets, Nodes and Events. **API resources**, in a Cluster's sidebar, covers everything else: PersistentVolumes and their claims, NetworkPolicies, ServiceAccounts and RBAC, and the resources operators add with their CRDs — cert-manager's Certificates, Argo CD's Applications, Prometheus' ServiceMonitors.

## Finding a type

The left column lists every type the cluster serves, grouped by API group (`core` first), at each group's preferred version, as `kubectl api-resources` does. Search by kind, plural or group, or type a short name: `pvc`, `netpol`, `sa`, `cert`.

A lock marks the types you can't list: those your credentials aren't allowed to list in the namespace picked (as the cluster tells Homeport through a `SelfSubjectRulesReview`), and the few whose API can't list at all. Opening one says why. Some clusters, such as those whose authorizer is a webhook, can't tell; then nothing is marked and a list that's refused says so.

Installing or removing a CRD updates the list on its own; **Refresh** reads it again.

## Listing objects

Pick a type and its objects are listed with the same columns as `kubectl get`: the API server says which, including a CRD's `additionalPrinterColumns`, so cert-manager's Certificates show **Ready**, **Secret** and **Age** as they do in a terminal. The list follows changes live, through a watch.

Namespaced types have the namespace picker, as Homeport's other pages do; with **All namespaces**, a **Namespace** column is added.

## An object's page

Click an object for its page:

- **Overview**: its namespace, the objects that own it — linked to their page — its age, labels and annotations.
- **Conditions**, when its `status` has them, as for Homeport's own pages.
- **Events** about it, and its **YAML**, without `managedFields`.

When its type allows it, **Edit** opens its YAML to change it through a preview and server-side apply (see [Applying and editing YAML](/docs/clusters/yaml/)), and **Delete** removes it after a confirmation naming it and the cluster (see [Deleting](/docs/clusters/actions/#deleting)). On the list, tick several objects to delete them together.
