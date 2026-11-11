---
title: RBAC
description: See a Cluster's Roles, bindings and subjects, what each subject can do and where, and what your own credentials can do.
---

**RBAC**, in a Kubernetes Cluster's sidebar, shows who can do what on the cluster. It is read-only: Homeport doesn't create, change or delete RBAC objects (you can still edit them as YAML from [API resources](/docs/clusters/api-resources/)).

Like the other pages, it follows the namespace picker: with a namespace picked, it shows that namespace's Roles and RoleBindings, plus the ClusterRoles and ClusterRoleBindings, which apply everywhere.

## Roles

**Roles** lists the Roles (each in its namespace) and the ClusterRoles, with their number of rules. Open one for its page:

- its **rules**, as Kubernetes writes them: resources, API groups (`core` for the core group), resource names and verbs, or non-resource URLs such as `/healthz`;
- the **bindings using it**, each linked to its subjects.

A ClusterRole marked **aggregated** has an `aggregationRule`: its page shows its label selectors, the ClusterRoles they match, and its rules include theirs. Kubernetes' own `admin`, `edit` and `view` are built this way, so a CRD's ClusterRole labelled `rbac.authorization.k8s.io/aggregate-to-edit: "true"` shows up in `edit`.

## Bindings

**Bindings** lists the RoleBindings and ClusterRoleBindings, each with the role it grants, linked, and the subjects it names: Users, Groups and ServiceAccounts. A binding whose role doesn't exist is marked **not found**; it grants nothing.

## Subjects

**Subjects** lists everyone a binding names. Open a User, a Group or a ServiceAccount to see what it can do, as a matrix of resources and verbs, grouped by where it applies: **Cluster-wide** first, then each namespace. Under it, the bindings that grant it, and how: naming it directly, or through a group.

Homeport merges the rules of every binding that applies:

- A ClusterRoleBinding grants its ClusterRole everywhere: in every namespace, on cluster-scoped resources such as Nodes, and on non-resource URLs.
- A RoleBinding grants its role in its own namespace only, even when it binds a ClusterRole. A ServiceAccount bound to `edit` by a RoleBinding in `prod` can edit in `prod`, and nowhere else.
- Bindings to the groups Kubernetes puts every subject in count too: a ServiceAccount is in `system:serviceaccounts`, `system:serviceaccounts:<its namespace>` and `system:authenticated`, and is also the user `system:serviceaccount:<namespace>:<name>`; a User is in `system:authenticated`.
- `*` stands for every verb, API group or resource, `*/scale` for that subresource of every resource, and a rule with resource names applies to those objects only (the matrix shows them under **Only**). A row a broader one already covers is left out.

**This is a best-effort view.** A subject's permissions are computed from the bindings your credentials can read, not asked of the API server, so they need no permission to impersonate anyone. They can miss what other authorizers allow, such as a webhook (common on managed clusters) or ABAC, and the groups an authenticator gives a user (from OIDC claims or a client certificate's organisation), which only the API server knows. To be sure about your own credentials, use **What can I do here?**.

With a namespace picked, only that namespace's RoleBindings are read; pick **All namespaces** to see a subject's permissions everywhere.

## What can I do here?

**What can I do here?** is the API server's own answer for the credentials Homeport uses, in the namespace picked (the cluster's default one with **All namespaces**), as `kubectl auth can-i --list` shows it. It says who they sign in as and in which groups, on clusters that tell (Kubernetes 1.28 and later), and links to that subject's page.

When the API server says its list may be incomplete — an authorizer such as a webhook can allow more than it can list — the page says so.

**Can I…?** asks one question, as `kubectl auth can-i` does: a verb, a resource and a namespace (empty for every namespace), and optionally an API group, a subresource and an object's name. The resource can be its plural (`secrets`), a short name (`deploy`), or its plural with its group (`deployments.apps`); Homeport resolves it as kubectl does. The answer comes with the same question as a command to copy, such as `kubectl auth can-i delete secrets --namespace prod`, which gives the same answer.

When your credentials can't list RBAC objects at all, the page shows only **What can I do here?**, under a message naming what was refused. When only some can be listed, such as RoleBindings but not ClusterRoles, the page says which, and what it shows leaves them out.
