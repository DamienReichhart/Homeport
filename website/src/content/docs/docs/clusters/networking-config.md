---
title: Services and configuration
description: Browse a Cluster's Services, Ingresses, ConfigMaps and Secrets, with Secret values revealed one key at a time.
---

## Services and Ingresses

**Network** lists the selected Namespace's **Services** — their type, cluster IP, external addresses (load balancers' and `externalIPs`) and ports as `kubectl` shows them (`443:30443/TCP` with a node port) — and, on its second tab, **Ingresses** with their class, hosts and the addresses their controller published.

- A Service's page shows its ports and target ports, its session affinity, the labels it selects Pods by, and a **Pods** tab with those Pods. A Service without a selector says so: its endpoints are managed by hand.
- An Ingress's page lists every rule's paths (the default backend first) with where each sends traffic, linked to the Service's page, and its TLS hosts and certificate Secrets.

## ConfigMaps and Secrets

**Configuration** lists **ConfigMaps** and **Secrets** with their keys. A ConfigMap's page shows each value; binary data shows its size only.

A Secret's page shows its type and each key with its size, masked. **Reveal** fetches that one value from the cluster and shows it, decoded (binary values as base64), with **Copy** and **Hide**. Values are never part of a list, a Secret's page or its YAML; a revealed one isn't cached, stored or logged, and is hidden again when you leave the page. See [Security and your data](/docs/reference/security/#clusters).

Editing ConfigMaps and Secrets comes with YAML editing in a later version.

## What isn't shown yet

Endpoints and EndpointSlices, NetworkPolicies and PersistentVolumes have no page of their own yet; a later version adds a browser for any API resource, custom ones included.
