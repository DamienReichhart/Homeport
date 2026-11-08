---
title: Applying and editing YAML
description: Apply manifests to a Cluster or edit an object's YAML, seeing what changes before anything does, with server-side apply.
---

Two ways to change a Cluster with YAML, for everything the dedicated actions don't cover: **Apply YAML** for manifests, like `kubectl apply -f`, and **Edit** on any object's **YAML** tab, like `kubectl edit`. Both show what would change before anything does.

## Apply YAML

**Apply YAML**, in a Cluster's sidebar, takes one or more manifests: paste them, or **Open file…** to load a `.yaml`, `.yml` or `.json` file from this computer. Several documents separated by `---` are applied together, in order.

A document without a namespace goes to the namespace picked at the top right (or, with **All namespaces**, to the Cluster's default namespace). Objects that aren't in a namespace, such as a Namespace or a ClusterRole, ignore it.

1. **Preview changes** asks the cluster what each document would do, with a server-side dry run: nothing changes yet. Each document shows as **Created**, **Changed** or **No changes**, with the lines it would change.
2. **Apply** asks you to confirm, naming the cluster, then applies.

Applying the same manifests twice reports **No changes** the second time. Editing the YAML after a preview asks for a new one first.

## Editing an object

Every object's **YAML** tab has **Edit**: its YAML as the cluster holds it, without what the server manages (`managedFields`, `status`, its `resourceVersion`, uid and creation time). Change it, **Preview changes** to see only the lines you changed, then **Apply**.

The preview compares your YAML with the object as it is when you preview, so a change made meanwhile shows in the diff. If the edit changes a field another tool manages, the preview lists it as a conflict instead of overwriting it.

Secrets can't be edited in Homeport, since it never shows all their values at once; and Events are written by Kubernetes itself.

## Server-side apply

Homeport applies with Kubernetes' server-side apply, under the field manager `homeport`: the cluster records which fields Homeport set, alongside those set by `kubectl`, Helm, controllers or other tools, which you'll see in an object's `managedFields`.

When a document changes a field another tool manages — `.spec.replicas` set by `kubectl apply` or managed by an autoscaler — the preview says which fields and which managers. Leave those fields out, or **Apply anyway** after a second confirmation: Homeport takes them over, and the other tool may set them back.

## When a document can't be applied

The preview shows each document's problem in place, and **Apply** stays off until every document can be applied:

- **Invalid YAML**, with the line it breaks at.
- **Unknown kind**: the cluster doesn't serve that `apiVersion` and `kind`. A custom resource needs its CRD installed first.
- **Refused by the cluster**: its validation's answer for each field, such as a missing required field or a value of the wrong type.
- **Not allowed**: applying needs `patch` on the resource (and `create` for new objects), with the `kubectl auth can-i` command to check.
