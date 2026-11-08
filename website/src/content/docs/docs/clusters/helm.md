---
title: Helm releases
description: List a Cluster's Helm releases and their history, values, notes, manifest and resources, through the helm CLI on this computer.
---

**Helm**, in the sidebar of a Cluster, shows the [Helm](https://helm.sh) releases on it: the same releases `helm list` shows in a terminal, whether they were installed from Homeport or not.

## What Homeport needs

Homeport runs the `helm` command, version 3 or later, on **this** computer. Install it with `brew install helm` on macOS, `winget install Helm.Helm` (or Chocolatey, or Scoop) on Windows, and `snap install helm --classic`, your distribution's package or the script from helm.sh on Linux. **Settings → This machine** shows the helm Homeport found; without one, the Helm page says what to install for your system, and every other page of the Cluster works.

Homeport doesn't hand helm your kubeconfig. For each helm command, it opens a private connection to the Cluster on this computer's loopback address and gives helm a temporary kubeconfig that points at it, holding a one-time token and no cluster credential. Homeport adds the Cluster's own credentials — certificates, tokens, exec plugins' tokens — to each request on the way, as for every other page, and deletes the kubeconfig when the command ends. So helm reaches every Cluster Homeport does, cloud clusters with exec plugins included, and no credential is written to disk.

Helm keeps its releases as Secrets in each release's Namespace: Homeport installs nothing on the Cluster.

## Releases

The list shows each release's name, Namespace, chart and chart version, app version, revision, status and when it was last changed. Pick a Namespace at the top right, or all of them. helm has no live updates, so the list asks again every 30 seconds while it's open, and **Refresh** asks at once.

A **failed** release is marked in red, and one **pending** an install, upgrade or rollback (`pending-install`, `pending-upgrade`, `pending-rollback`) in amber: a pending release is locked until that operation ends.

If your credentials can't list Secrets in every Namespace, Homeport lists the releases of the Cluster's default Namespace instead and says so; pick another Namespace to see its releases.

## A release's page

Click a release to open it:

- **History** — every revision, newest first, with its date, status, chart, app version and helm's description of what happened (`Install complete`, `Upgrade "web" failed: …`).
- **Values** — the values given at install or upgrade (**User-supplied**), or every value the chart was rendered with, its defaults included (**Computed**).
- **Notes** — what the chart prints after an install (its `NOTES.txt`).
- **Manifest** — the Kubernetes objects the revision rendered, as YAML.
- **Resources** — those objects by kind, name and Namespace, each opening its page in Homeport: its own page for Deployments, Pods, Services and the like, its page under [API resources](/docs/clusters/api-resources/) for the rest.

Values, Notes, Manifest and Resources show the current revision; pick an earlier one above them to see what it was installed with.

## Chart repositories

**Chart repositories**, the second tab of the Helm page, lists the repositories helm knows on this computer: the same list as `helm repo list` in a terminal, shared by every Cluster. Homeport keeps no list of its own, so a repository added in a terminal appears here, and one added here appears in `helm repo list`.

- **Add repository** asks for a name (letters, digits, dots, dashes and underscores) and the repository's `https://` URL, such as `https://charts.bitnami.com/bitnami`, and downloads its index.
- **Update all**, or **Update** on one repository, downloads the latest indexes, as `helm repo update` does, with helm's output shown live. Charts published since the last update only show up after one.
- **Remove** forgets a repository, after you confirm. Releases installed from it keep running.

**Find charts** searches the charts of these repositories by name or description, as `helm search repo` does: type `postgres` after adding Bitnami's repository to see its PostgreSQL charts, with their latest chart and app versions.

Homeport only adds public repositories. One that needs a login is added in a terminal, with `helm repo add --username …`, where helm keeps its password as it always does; it then appears here and works like the others. A chart in an OCI registry (`oci://…`) needs no repository.

## When it doesn't work

- **The helm CLI isn't installed**: install helm 3 as above, then reopen the page.
- **Helm 2**: Homeport only drives Helm 3 and later. Install a current helm.
- **Not allowed**: helm reads releases from Secrets; listing them needs `list` on `secrets` in the Namespace. Details names what was refused.
- **Not a chart repository, or can't be reached**: the URL serves no `index.yaml`, or this computer can't reach it. Details shows what helm said.
- **Not connected**: helm goes through the Cluster's connection in Homeport; the Cluster's page says why it isn't connected.
