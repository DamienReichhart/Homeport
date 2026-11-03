---
title: Settings
description: Theme, what this machine supports, version and licences.
---

**Settings** is the last item in the sidebar.

## Appearance

**Theme** is **Match the system** by default, following your operating system's light or dark mode as it changes. Choose **Light** or **Dark** to fix it.

## This machine

What Homeport can use on this computer:

- **OS keychain** — **Available** when SSH keys, certificates and Stack environments can be stored encrypted; **Unavailable** otherwise. See [requirements](/docs/requirements/#optional-an-os-keychain).
- **Compose CLI** — the Compose version found, such as `docker compose v2.29.1`, or **Not found** if Stacks can be started and stopped but not deployed. See [Stacks](/docs/guides/stacks/#what-deploying-needs).

Homeport checks both when it starts. After installing a keychain or Compose, quit and reopen Homeport.

## About

Shows Homeport's version and the platform it was built for, for example `v0.1.0 · macos/aarch64`. Include it when you [report an issue](https://github.com/DamienReichhart/Homeport/issues).

**Licenses** shows Homeport's MIT licence and the notices of every open-source package it's built with.
