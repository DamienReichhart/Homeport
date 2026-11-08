---
title: Clusters
description: How Homeport connects to Kubernetes clusters from your kubeconfig, what it stores, and what each status means.
---

A **Cluster** is a Kubernetes cluster Homeport connects to through its API server. You add one from a context of a kubeconfig — the file `kubectl` already uses — so there are no new credentials to create, and Homeport installs nothing on the cluster: it only talks to the Kubernetes API your kubeconfig already reaches, with the rights its credentials have.

Clusters sit next to your Hosts in the switcher at the top of the sidebar. Selecting one shows the pages about it; selecting a Host shows the Host pages again.

## Adding a Cluster

Choose **Add cluster…** in the switcher, or **Add a cluster** on the Clusters page. Each context you pick becomes a Cluster, named after the context; you can rename it before or after importing.

- **From this machine's kubeconfig** reads the files listed in `KUBECONFIG`, merged as `kubectl` merges them, or `~/.kube/config` when `KUBECONFIG` isn't set. The current context is picked for you.
- **Paste a kubeconfig** reads text you paste, in memory only: it's never saved as a file. Files it refers to (`certificate-authority: /…`) must be full paths; the simplest is to paste one with everything inlined:

  ```bash
  kubectl config view --raw --flatten --minify
  ```

A context that is already a Cluster can't be imported twice: Homeport offers **Refresh it** instead. A context that can't be used says why, for example a user whose `auth-provider` is `gcp` or `azure`, which Kubernetes removed (switch it to `gke-gcloud-auth-plugin` or `kubelogin`, as `kubectl` now requires).

Users can sign in with a client certificate, a token (or `tokenFile`), a user name and password, an OpenID Connect `auth-provider` (its ID token is refreshed in memory), or an exec plugin.

## What is stored where

| Kept in clear in Homeport's database                                     | Kept only encrypted (like SSH keys)                                         |
| ------------------------------------------------------------------------ | --------------------------------------------------------------------------- |
| the name, the API server's URL, the context's namespace                  | the CA certificate, client certificate and key                              |
| how its user signs in (certificate, token…), the context it was imported | the token or password, the OIDC tokens, the exec plugin's arguments and env |

Files the kubeconfig refers to are read once, when you import, and stored encrypted with the rest. Your kubeconfig is never changed. Without an OS keychain, Homeport can't store credentials and won't add a Cluster; see [Security and your data](/docs/reference/security/).

## Refreshing from the kubeconfig

kind, k3d and minikube give a cluster new certificates each time they recreate it, and cloud credentials get renewed. **Refresh from kubeconfig** on a Cluster's page reads its context again — from this machine's kubeconfig, or from a kubeconfig you paste again for one that was pasted — keeps its name, and reconnects.

## Managing Clusters

- **Manage clusters** in the switcher lists every Cluster with its status, API server and how it signs in.
- On a Cluster's page, the **Enabled** switch disconnects it without forgetting it, **Reconnect** tries again at once, and **Rename** changes its name in Homeport only.
- **Remove cluster** forgets the Cluster and its stored credentials. Nothing changes on the cluster or in your kubeconfig.

Cluster names must be unique, ignoring case.

## Statuses

| Status               | Meaning                                                                                            |
| -------------------- | -------------------------------------------------------------------------------------------------- |
| **Connected**        | The API server answers; its Kubernetes version shows next to it.                                   |
| **Connecting**       | Homeport is reaching it.                                                                           |
| **Unreachable**      | The API server didn't answer. Homeport tries again on its own, waiting 1 s and up to a minute.     |
| **Access refused**   | The API server refused the credentials. Check that `kubectl` works with the context, then refresh. |
| **Misconfigured**    | Something only you can fix, such as a certificate that doesn't match. Homeport waits until you do. |
| **Credentials lost** | The OS keychain's key that protects them is gone; refresh the Cluster from its kubeconfig.         |
| **Disabled**         | Turned off with the **Enabled** switch.                                                            |

A connected Cluster is checked every 15 seconds, so one that goes away shows it and reconnects on its own when it's back, like a Host.
