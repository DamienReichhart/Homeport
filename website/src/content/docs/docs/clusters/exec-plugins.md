---
title: Exec plugins
description: How Homeport signs in to EKS, GKE and AKS clusters with the exec credential plugin their kubeconfig names, and what runs where.
---

Cloud kubeconfigs don't hold a password or a certificate: their user names an **exec plugin**, a program on your computer that prints a short-lived token when asked. Homeport runs the same program `kubectl` would, on this computer, following the Kubernetes `ExecCredential` protocol (`client.authentication.k8s.io/v1` and `v1beta1`).

## Common providers

| Provider | The kubeconfig's user runs           | Install                                                                       |
| -------- | ------------------------------------ | ----------------------------------------------------------------------------- |
| EKS      | `aws eks get-token --cluster-name …` | the AWS CLI, version 2                                                        |
| GKE      | `gke-gcloud-auth-plugin`             | the Google Cloud CLI, then `gcloud components install gke-gcloud-auth-plugin` |
| AKS      | `kubelogin get-token …`              | `az aks install-cli`, or kubelogin from its releases                          |

Write the kubeconfig as usual — `aws eks update-kubeconfig`, `gcloud container clusters get-credentials`, `az aks get-credentials` followed by `kubelogin convert-kubeconfig` — then [add its contexts](/docs/clusters/).

## What runs where

- The plugin runs **on this computer**, as you, with the arguments and environment variables the kubeconfig gives it (`AWS_PROFILE`, for example). Homeport never asks it to be interactive and gives it no input.
- A desktop launcher starts Homeport with a short `PATH`, so Homeport also looks in the usual install places of the AWS, Google Cloud and Azure CLIs and kubelogin (Homebrew, the SDK's own folder, `~/.local/bin`, `~/.krew/bin`, Program Files…). A plugin given by full path in the kubeconfig runs from there.
- The token or certificate it prints is kept **in memory only**, until shortly before it expires; the plugin then runs again. It runs once per token lifetime, not once per request. When the API refuses a token, Homeport asks the plugin for a new one once before showing **Access refused**.
- The plugin's own caches (`~/.kube/cache`, `~/.aws/sso/cache`, the gcloud config) are its business; Homeport writes nothing there.

## When it can't sign in

- **"… isn't installed on this computer"** — the program wasn't found. Install it (the message says how, using the kubeconfig's `installHint` when it has one), check it works in a terminal, then **Reconnect**.
- **"… needs you to sign in, which it can't do inside Homeport"** — the plugin wants a browser or a device code (AWS SSO, Azure device login, an expired `gcloud` session). Run `kubectl` once in a terminal with that context, sign in there, then choose **Reconnect**: the plugin then finds its cached sign-in.
- **"… couldn't get credentials for …"** — it ran but failed; **Details** shows the end of what it printed. Homeport tries again on its own.

See [Troubleshooting](/docs/reference/troubleshooting/#clusters) for the other messages.
