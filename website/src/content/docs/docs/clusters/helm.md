---
title: Helm releases
description: List, install, upgrade, roll back and uninstall a Cluster's Helm releases, and manage chart repositories, through the helm CLI on this computer.
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

## Installing a chart

**Install chart** (or <kbd>n</kbd>) on the Releases tab installs a chart on the selected Cluster:

1. **Pick the chart.** Search the charts of helm's [repositories](#chart-repositories) and click **Use**, or, under **From an OCI registry**, give a reference such as `oci://registry-1.docker.io/bitnamicharts/nginx`, which installs without adding a repository.
2. **Choose its version, name and Namespace.** A repository chart's versions are listed, the latest first; an OCI chart's version is typed, or left empty for the latest. The release is named after the chart to start with: at most 53 lowercase letters, digits, dashes and dots. The Namespace is the one the page shows (or the Cluster's default), and is created if it doesn't exist unless you untick that.
3. **Edit its values.** The editor starts with the chart's default values (`helm show values`); change what you need, and **Reset to the chart's defaults** puts them back. Values are checked as YAML before helm runs, and go to helm on its standard input — never in a file.
4. **Preview**, optionally: helm renders the chart with your values and the Cluster's help (`--dry-run=server`), and Homeport shows the manifest it would apply, hooks included, and the chart's notes. Nothing changes on the Cluster.
5. **Install.** helm's output streams in, as `helm install` prints it. With **Wait until its resources are ready**, helm waits for the release's Deployments, Pods and Services to be ready, for at most the minutes given (5 by default), and marks the release failed if they aren't. **Open release** then opens its page.

Charts from a local folder or a `.tgz` file aren't installed from Homeport; neither are charts from OCI registries that need a login unless you signed in with `helm registry login` in a terminal.

## A release's page

Click a release to open it:

- **History** — every revision, newest first, with its date, status, chart, app version and helm's description of what happened (`Install complete`, `Upgrade "web" failed: …`).
- **Values** — the values given at install or upgrade (**User-supplied**), or every value the chart was rendered with, its defaults included (**Computed**).
- **Notes** — what the chart prints after an install (its `NOTES.txt`).
- **Manifest** — the Kubernetes objects the revision rendered, as YAML.
- **Resources** — those objects by kind, name and Namespace, each opening its page in Homeport: its own page for Deployments, Pods, Services and the like, its page under [API resources](/docs/clusters/api-resources/) for the rest.

Values, Notes, Manifest and Resources show the current revision; pick an earlier one above them to see what it was installed with.

## Upgrading a release

**Upgrade**, on a release's page, changes its chart version, its values, or both:

- **Chart** — the chart to upgrade from. helm doesn't record where a chart came from, so Homeport looks for a chart of that name in your repositories, the ones that have the release's current version first. Pick another with **Another chart…**, as `repository/chart` or an `oci://` reference — which is what a release installed from an OCI registry needs.
- **Chart version** — the versions of that chart, the latest first, or typed for an OCI chart.
- **Values** — the editor starts with the release's user-supplied values. They replace the release's: the chart's defaults fill in the rest. **Changes to the values** shows them side by side with the current ones, and **Reset to the current values** puts them back.
- **Wait until its resources are ready**, for at most the minutes given.

**Preview** renders the upgrade with the Cluster's help without changing anything, and shows the new manifest and what changes in it against the current one. **Upgrade** runs `helm upgrade`, with helm's output live. Each upgrade is a new revision in **History**.

## Rolling back

In **History**, **Roll back to revision N** on any earlier revision asks first, naming the chart and app versions now and after, then runs `helm rollback`: helm puts back that revision's chart and values as a new revision, so a rollback can itself be rolled back. Tick **Wait until its resources are ready** to have helm wait for them.

A release stuck **pending** after an interrupted install, upgrade or rollback is locked: helm refuses to change it until it ends. Rolling it back to its last deployed revision unlocks it.

## Uninstalling

**Uninstall**, on a release's page, names the release, its Namespace and Cluster and lists the objects that will be deleted with it, before `helm uninstall` runs. Objects a chart marks to keep (`helm.sh/resource-policy: keep`) and the PersistentVolumeClaims of StatefulSets stay on the Cluster. **Keep its history** leaves the release listed as `uninstalled`, so it can be rolled back; without it, the release is gone from the list. **Wait until its objects are deleted** has helm wait for them.

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
- **Another install, upgrade or rollback is in progress**: the release is locked by a pending operation. Wait for it to finish, or roll back to its last deployed revision.
- **A release with this name already exists**: release names are unique in a Namespace. Pick another, or upgrade the existing release.
- **Chart or version not found**: check the name and version; a repository chart needs its repository added and [updated](#chart-repositories). To upgrade a release whose chart is in no repository of yours, add that repository first.
- **The values aren't valid YAML**: the message names the line; fix it and retry.
- **Gave up waiting**: with **Wait**, a resource wasn't ready in time. Look at the release's Pods and Events for what is stuck.
- **Not a chart repository, or can't be reached**: the URL serves no `index.yaml`, or this computer can't reach it. Details shows what helm said.
- **Not connected**: helm goes through the Cluster's connection in Homeport; the Cluster's page says why it isn't connected.
