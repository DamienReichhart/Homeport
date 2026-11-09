---
title: Nomad clusters
description: How Homeport connects to a Nomad cluster's HTTP API with an address, an ACL token and TLS material, and what it stores.
---

A Cluster can be a **Nomad** cluster as well as a Kubernetes one. Homeport talks to the HTTP API that a Nomad agent, server or client, already serves, from your computer: nothing is installed on the cluster, and no `nomad` CLI is needed on this machine. Everything it does goes through the API with the rights of the ACL token you give it.

Nomad clusters sit next to your Hosts and Kubernetes clusters in the switcher. Selecting one shows its dashboard on the Overview; the Kubernetes pages are hidden while it is selected.

## Adding a Nomad cluster

Choose **Add cluster…** in the switcher, then the **Nomad** tab, and fill in:

- **Name**: what the switcher and the Overview show, such as `nomad-prod`.
- **Address**: the agent's HTTP API, as `NOMAD_ADDR` gives it to the `nomad` CLI. `nomad.example.com` becomes `http://nomad.example.com:4646`: port 4646 is added when none is given, and `https://` is used when you give TLS material.
- **Region**: leave it empty to use the agent's own region. Set it to manage another region through a federated agent, as `NOMAD_REGION` does.
- **ACL token**: the token's secret ID, when the cluster has ACLs enabled. See [the token](#the-acl-token) below.

**Test connection** connects once without saving and shows the Nomad version and region, or why it can't connect. **Add cluster** saves it and connects. Its page then shows its status, version and region, and lets you change any of these settings: the token and TLS key are never shown again, and a field left empty keeps the stored one. **Remove** next to a stored credential deletes it when you save.

## TLS

An agent that serves HTTPS with a certificate a public CA, or a CA your computer already trusts, needs nothing more than an `https://` address. Open **TLS settings** for the rest, pasting the contents of the PEM files the `nomad` CLI is given:

| Field                                     | The `nomad` CLI's variable              | When you need it                                                                                                                          |
| ----------------------------------------- | --------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| **CA certificate**                        | `NOMAD_CACERT`                          | The agent's certificate is signed by your own CA, such as `nomad-agent-ca.pem`.                                                           |
| **Client certificate** and **Client key** | `NOMAD_CLIENT_CERT`, `NOMAD_CLIENT_KEY` | The agent asks for client certificates (`verify_https_client`). Give both or neither.                                                     |
| **TLS server name**                       | `NOMAD_TLS_SERVER_NAME`                 | The certificate isn't made for the address's name. Nomad's own are made for `server.global.nomad`, with your region in place of `global`. |

When a CA certificate is given, only that CA is trusted for this cluster, as with the `nomad` CLI.

## The ACL token

Homeport sends the token in the `X-Nomad-Token` header of each request, never in a URL. While connected it checks the token every 15 seconds; a token Nomad doesn't know (deleted, expired, or mistyped) stops the connection with **Unauthorized** until you paste a new one.

A token that can't read the agent (`agent:read`) still connects: the cluster's page then shows its region but not its version.

## Statuses and problems

The status follows the same rules as for [Kubernetes clusters](/docs/clusters/#statuses): Homeport retries on its own while the agent can't be reached, and waits for you when only you can fix it.

- **The address is wrong**: a name that can't be found, a refused connection or a timeout says which, with the address it tried.
- **An expired certificate, an unknown CA or the wrong name**: each says what to change in **TLS settings**.
- **The agent only answers over HTTPS**, or doesn't use TLS at all: change the address's `http://` or `https://`.
- **No leader**: the cluster's servers can't agree on a leader, so nothing can be read. Nomad needs a majority of its servers running; check them with `nomad server members` on a server.

## What is stored where

| Kept in clear in Homeport's database                    | Kept only encrypted (like SSH keys)                    |
| ------------------------------------------------------- | ------------------------------------------------------ |
| the name, the address, the region, the TLS server name  | the ACL token                                          |
| whether a token, a CA or a client certificate is stored | the CA certificate, the client certificate and its key |

Homeport's [data backup](/docs/reference/security/) includes Nomad clusters with their credentials, as it does Kubernetes clusters.
