---
title: FAQ
description: Common questions about Homeport.
---

### Is Homeport free?

Yes. Homeport is free to use, for personal and commercial work, and open source under the [GNU Affero General Public License, version 3](https://github.com/DamienReichhart/Homeport/blob/main/LICENSE) (AGPL).

### What does the AGPL mean for me?

Nothing changes if you use Homeport: install it, run it at home or at work, on as many computers as you like. The AGPL only asks something of those who **distribute** Homeport, modified or not, or offer a modified version to others over a network: they must publish its full source code under the same licence. Every copy of Homeport links to the source of its exact version in **Settings → About → Licenses**.

Versions up to 0.11 were released under the MIT licence and stay available under it.

### Does Homeport replace Docker Desktop?

Not entirely. Homeport is a manager: it doesn't run a Docker Engine itself. On Linux, install Docker Engine and use Homeport as its interface. On macOS and Windows you still need something that runs the Engine — Docker Desktop, Colima, OrbStack, Rancher Desktop or Podman — and Homeport can sit on top of any of them, along with your remote servers.

### Does it work with containerd?

Yes, directly, without Docker: a Host's Engine can be [containerd](/docs/hosts/containerd/), reached through its socket on this machine or over SSH — k3s's, Docker Engine's own, or containerd's service on a server. Homeport lists its namespaces, its containers and their tasks (start, stop, pause, signal, remove), their logs where nerdctl or Kubernetes wrote them, a terminal in them, its images (pull, remove) and its snapshots. It doesn't create containers or deploy Stacks there; nerdctl does that.

### Does it work with Incus?

Yes: a Host's Engine can be [Incus](/docs/hosts/incus/) 6.0 or later, reached through its socket on this machine or over SSH, or over HTTPS with a trust token from `incus config trust add`. Homeport lists the [Instances](/docs/guides/instances/) of a project, system containers and virtual machines, starts, stops, freezes, deletes and creates them, opens their console and a terminal in them, takes and restores snapshots, and manages images, profiles, storage pools and networks. It doesn't deploy Stacks there, and installs nothing on the Host.

### Does it work with LXD?

Not as such: Homeport speaks Incus, the community fork of LXD, and refuses an LXD server, whose API has drifted. Migrate it with `lxd-to-incus`, then add it as an Incus Host.

### Does it work with Podman?

Yes, through Podman's Docker-compatible API socket. Homeport detects the usual Podman sockets on Linux, every running [Podman machine on macOS](/docs/hosts/local/#podman-machines-on-macos), and the default Podman machine pipe on Windows. Anything else can be [added by hand](/docs/hosts/local/#adding-a-socket-by-hand).

Homeport also recognises Podman when it connects, and then offers what only Podman has, through Podman's own API on the same socket: [pods](/docs/guides/podman-pods/), which you can list, start, stop, create and remove, and run containers in from the Run dialog; and [Quadlet units](/docs/guides/quadlet/), which you can list, read, install, edit and remove, and start or stop over SSH.

### Does it work with Kubernetes, Nomad or Docker Swarm?

Docker Swarm, yes. Homeport finds a Swarm through its manager hosts, once one of them is a host, and manages it fully: its nodes, services, tasks, secrets, configs and swarm stacks, including Managed, Linked and Synced Stacks deployed to it. It can also start a Swarm on a host, and join or leave one. Everything goes through a connected manager's Engine API; nothing is installed on the nodes. See [Swarms](/docs/guides/swarm/).

Kubernetes, yes. Add a cluster from your kubeconfig — this computer's or one you paste — and Homeport finds local ones (Docker Desktop, OrbStack, Rancher Desktop, kind, k3d, minikube) for you. EKS, GKE and AKS work through the exec plugin their kubeconfig names. You can browse Namespaces, workloads, Pods, Services, Ingresses, ConfigMaps, Secrets, Nodes, Events and any other API resource, custom ones included, live; follow logs, open a terminal in a Pod, forward ports, scale, restart and delete, and apply or edit YAML with a preview. Nothing is installed on the cluster: Homeport only talks to the Kubernetes API your kubeconfig already reaches, with its credentials' rights. With the `helm` CLI on this computer, Homeport also lists, installs, upgrades, rolls back and uninstalls [Helm releases](/docs/clusters/helm/), and manages helm's chart repositories. See [Clusters](/docs/clusters/).

Nomad, yes. Add a cluster by its agent's address, with an ACL token and TLS material when it uses them. You can list jobs and their allocations, live; run a job from HCL or JSON after seeing Nomad's plan; edit and re-run one, stop or purge it, and scale its task groups; follow a task's logs and open a shell in it; drain nodes and switch their eligibility; and pick a namespace. Nothing is installed on the cluster and no `nomad` CLI is needed: Homeport talks to Nomad's HTTP API with the token's rights. See [Nomad clusters](/docs/clusters/nomad/).

### Do I need kubectl?

No. Homeport reads your kubeconfig and talks to the Kubernetes API itself. The only programs it runs are the [exec plugins](/docs/clusters/exec-plugins/) a kubeconfig names, such as the AWS CLI for EKS, `gke-gcloud-auth-plugin` for GKE or `kubelogin` for AKS — the same ones `kubectl` would run. Having `kubectl` around still helps to check what your credentials can do (`kubectl auth can-i`).

### Do I need to install anything on my servers?

No — only a Docker Engine and an SSH server, which you most likely already have. Homeport doesn't install agents.

A Swarm needs nothing more than the Engine either. Its nodes must reach each other, though, which Homeport can't set up for you: open 2377/tcp (to the managers), 7946/tcp and 7946/udp, and 4789/udp between them. See [ports between nodes](/docs/guides/swarm/#ports-between-nodes).

### Do I need Docker installed on my computer to manage remote servers?

No. Homeport speaks the Engine API and SSH itself. The only exceptions are deploying Managed, Linked and Synced Stacks, which uses the `docker compose` command on your computer, and deploying them to a Swarm, which uses the `docker` command (`docker stack deploy`). Neither needs an Engine on your computer.

### Does Homeport keep my Stacks in sync when it's closed?

No. [Synced Stacks](/docs/guides/synced-stacks/) are checked for new commits, and deployed automatically if you turn that on, only while Homeport is open. Homeport installs nothing on your servers — no agent, no webhook receiver — so when it's closed, nothing watches your repositories. When you open it again, it waits one interval, then checks them all. For deploys that must follow every push around the clock, run them from your CI.

To keep Homeport running without its window, turn on **Close to the system tray**, and **Start at login** with **Start hidden in the tray**, in [Settings → Desktop](/docs/guides/settings/#desktop).

### Can Homeport tell me when a container crashes?

Yes, while it runs — even hidden in the system tray. It shows an OS notification when a container stops with an error, runs out of memory or turns unhealthy, and when a Host becomes unreachable or comes back. Each kind can be turned off in [Settings → Notifications](/docs/guides/settings/#notifications). It learns this from the event stream it already holds with each Host, so nothing is installed on your servers; when Homeport isn't running, nobody is told.

### Does Homeport need git?

Only for [Synced Stacks](/docs/guides/synced-stacks/), which run the `git` installed on your computer, with each repository's own deploy key or token. **Settings → This machine** says whether Homeport found it. Your own git configuration, credential helpers and SSH keys are never used.

### Can several people share Homeport's Hosts?

Each person runs their own Homeport with their own Hosts and keys. With a Homeport-created key per person and per server, you can see and revoke each one's access in `authorized_keys`.

### Can I move Homeport to another computer?

Yes, in two ways:

- **Everything**: **Settings → Data → Back up…** saves every Host, Cluster, Stack, registry login, repository and setting, with their secrets, to one file protected by a passphrase of at least 12 characters. On the new computer, **Restore…** in the same place opens it, lists what it holds, and replaces what's there. Everything connects and deploys without asking you for a key or a password again. See [backing up and restoring](/docs/guides/settings/#backing-up-and-restoring).
- **Some Hosts**: on the **Hosts** page, **Export** the Hosts you pick, and **Import** them on the other computer, next to the Hosts it already has. See [moving to another computer](/docs/reference/security/#moving-to-another-computer).

Copying `homeport.db` doesn't work for secrets: they're encrypted with a key kept in this computer's keychain, and show as **Credentials lost** elsewhere.

### Does a backup include my containers and volumes?

No. A backup holds what Homeport keeps — how to reach your Hosts, your Stacks' definitions and environments, logins and settings — not what runs on your Hosts. Your containers, images and volumes stay on the Hosts. To copy a volume's data, [back it up to a file](/docs/guides/volumes-networks/#backing-up-and-restoring) from the Volumes page.

### Can Homeport build images?

Yes, in two places. The **Images** page builds an image from a folder on this computer. And a [Linked Stack](/docs/guides/stacks/#linked-stacks) — a Stack whose compose files are in a folder on this computer — can use `build:`: Homeport runs `docker compose` in that folder, so the build context is sent from there to the Host's Engine, which builds it. **Rebuild and deploy** rebuilds after a code change. A Managed Stack can't build, since Homeport holds only its YAML and there is no folder to build from: link the folder instead, or push the image to a registry.

### Why does my Stack show as External on my other computer?

A Stack started with `docker compose up` in a folder shows as [Linked](/docs/guides/stacks/#how-linked-stacks-are-found) only on a computer where that folder and its compose files exist, at the same path, and define the services that run. On another computer the path doesn't exist, so Homeport has no compose file for it and shows it as External. To manage it from there too, clone the project to that computer and link it with **New stack → Use a folder on this computer**, or adopt it as a Managed Stack.

### Does Homeport collect data about me?

No. There's no account, telemetry or analytics. The only request Homeport makes on its own from your computer is the update check, which downloads a small file from this website and sends nothing about you; you can turn it off in **Settings → Updates**. Separately, the newer-image check asks your images' registries, through each Host's Engine, which version their tags point to; for a local Host that request leaves from your computer too. See [security and your data](/docs/reference/security/#what-homeport-connects-to).

### Can Homeport use my private registries?

Yes. Add a login for each registry in **Settings → Registries** — Docker Hub, GHCR or your own. Homeport hands it to the Engine to pull, push and build on any Host, and pulls a Managed, Linked or Synced Stack's images with it before deploying. Passwords and tokens are encrypted with your OS keychain. Homeport doesn't read the logins of `docker login` on this computer.

### How does Homeport know a newer image exists?

It asks the image's registry, through the Host's Engine, which digest the tag points to now, and compares it with the one the Host pulled. Nothing is downloaded, but Docker Hub counts each question against its pull rate limit, like a `docker pull` that finds nothing new. So Homeport keeps each answer for 6 hours, asks at most four registries at once, and only asks again sooner when you choose **Check for newer images**. A Docker Hub login in **Settings → Registries** raises the limit. If a registry can't be reached or is rate-limiting, Homeport shows no mark for its images and writes a line to its log.

### How do I update?

On Windows, macOS and with the AppImage, Homeport tells you when a new version is out: choose **Install and restart**. On Linux, the `.deb` and `.rpm` packages update with the rest of your system from the Homeport package repository. See [updating](/docs/reference/updating/).

### Is Homeport on Homebrew, winget, Flathub or the AUR?

Not yet. They're planned; the [installation page](/docs/installation/#coming-later) links to each one's issue.

### Is Homeport available in my language?

Homeport speaks English, French, Spanish, German, Italian, Brazilian Portuguese, Russian, Simplified Chinese, Japanese, Korean, Hindi and Arabic, and follows your operating system's language unless you pick one under [Settings → Appearance](/docs/guides/settings/#appearance). The website is in all of them too; the documentation is fully translated into French, and other languages show the English page where a translation doesn't exist yet. To add a language, see [Contributing](/docs/project/contributing/#translating-homeport).

### Is Homeport accessible?

It can be used with the keyboard alone, follows your system's reduced-motion setting, meets WCAG 2.2 AA contrast, and every page is checked with automated accessibility tools. A hands-on screen-reader audit is under way. See [Keyboard and accessibility](/docs/reference/accessibility/) for what works, the known limits and how to report a barrier.

### Is there a command-line interface?

No — Homeport is a desktop app. Your existing `docker` CLI works alongside it, and Homeport updates live when you change things from the terminal.

### How can I help?

Report bugs and suggest features in [GitHub issues](https://github.com/DamienReichhart/Homeport/issues), or see [contributing](/docs/project/contributing/).
