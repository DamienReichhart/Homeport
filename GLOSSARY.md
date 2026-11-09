# Homeport

A desktop app for managing Docker Engines, wherever they run, from one place.

## Language

Each entry gives the French term the app, the docs and the website use (agreed in #101). Every language's terms, with their gender, plural and the words to avoid, are in [`docs/i18n/`](docs/i18n/): translate with that language's table, and add a row there for a new term.

**Host**:
An Engine that Homeport connects to, defined by a name and how to reach it (local socket, TCP, TLS or SSH). Every container, image, volume, network and Stack belongs to exactly one Host. Messages refer to a Host by its name.
_Avoid_: Instance, endpoint, engine, context, server
_French_: hôte (m.); _avoid_ instance, serveur, point de terminaison, contexte

**Engine**:
The Docker-compatible daemon a Host points to (Docker Engine, Docker Desktop, Podman, Colima, OrbStack…): the software that gets installed, started and stopped. Not a synonym for Host: "Start the Docker Engine" talks about the Engine, "Homeport can't reach prod-server" talks about the Host.
_Avoid_: Daemon, server, backend
_French_: moteur (m.); _avoid_ démon, serveur, backend

**Engine kind**:
What sort of Engine a Host points to, stored with it: Docker-compatible (Docker Engine, or Podman through its compatible API), containerd or Incus. It decides which pages a Host shows; containerd Hosts get their own (ADR 0020), and so do Incus Hosts (ADR 0021). Called `engine` in code, never `kind`, which is the connection's. Podman is not a kind of its own: it's a Docker-compatible Engine recognised on every connect, because the same socket can change product.
_French_: type de moteur (m.); _avoid_ sorte

**Namespace (containerd)**:
The space every object of a containerd Host lives in (`default` for nerdctl, `k8s.io` for Kubernetes, `moby` for Docker); every call names one. A containerd Host's pages show one at a time, remembered per Host. A Cluster's Namespace is the Kubernetes one: say "containerd namespace" wherever both could be meant.
_French_: espace de noms containerd (m.); _avoid_ namespace (for containerd)

**Task (containerd)**:
What runs for a containerd container, which is only metadata (image, spec, snapshot). Starting a container creates and starts its task; stopping leaves a stopped task until the next start. A Swarm's task is one slot of a service: say "containerd task" wherever both could be meant.
_French_: tâche containerd (f.)

**Snapshot**:
On a containerd Host, a layer of files a snapshotter keeps: a committed layer of an unpacked image, or a container's active, writable one, each built on a parent. On an Incus Host, a saved state of an Instance's disk (and, stateful, its memory) that the Instance can be restored to: say "Instance snapshot" wherever both could be meant.
_French_: instantané (m.); _avoid_ snapshot

**Instance**:
What an Incus Host runs: a system container, which boots a whole distribution on the host's kernel, or a virtual machine. Not a Container, which is the Docker Engine's (or containerd's) and runs one application.
_Avoid_: VM (for both), Incus container (for both)
_French_: instance (f.); _avoid_ VM, conteneur Incus

**Project (Incus)**:
The space an Incus Host keeps Instances, images, profiles, custom volumes and networks in; every request about them names one. An Incus Host's pages show one at a time, remembered per Host, like a containerd namespace.
_French_: projet (m.)

**Profile**:
A set of configuration keys and devices that Incus applies to the Instances listing it, in order: shared limits, networks, storage, cloud-init.
_French_: profil (m.)

**Trust token**:
What `incus config trust add <name>` prints: base64 JSON naming the client, the Incus server's addresses and certificate fingerprint, a secret and an expiry. Homeport adds an Incus Host over HTTPS with it, once.
_French_: jeton de confiance (m.)

**Pod**:
On a Podman Host, a group of containers that share one network namespace (and their hostname and IPC) and start and stop together, around an infra container that holds the namespaces and the published ports. Shown on the Pods page through Podman's own API. A Cluster's Pod is the Kubernetes one: in code and text about Hosts, say "Podman pod" wherever both could be meant.
_French_: pod Podman / pod (m.)

**Quadlet unit**:
A file on a Podman Host (`web.container`, `shop.pod`, `cache.volume`…) that systemd's generator turns into a service (`web.service`); rootful Podman's are system units, rootless Podman's the user's. Listed, read, installed and removed through Podman's API; its service is started and stopped with `systemctl` over SSH (ADR 0019). The unit is the file, the service is what systemd runs: "start the unit" starts its service.
_French_: unité Quadlet (f.); _avoid_ fichier Quadlet (for the unit)

**Bastion**:
An SSH server an SSH Host is reached through when its own SSH server only accepts connections from it (OpenSSH's `ProxyJump`). A Host has an ordered list of them, possibly empty; each has its own trusted server key and either uses the Host's credentials or its own. Not a Host: it runs no Engine Homeport talks to.
_Avoid_: Jump host, jump server, proxy, gateway, hop (in the UI)
_French_: bastion (m.); _avoid_ hôte de rebond, proxy, passerelle

**Swarm**:
A Docker Swarm cluster, found through the Hosts that belong to it and never added on its own: Homeport reaches it through any connected manager Host, addresses it by its swarm ID, and stores nothing about it (ADR 0016). Named "Swarm of <its first manager Host>", since Docker gives every Swarm the same name.
_Avoid_: Cluster (alone), swarm mode, orchestrator
_French_: Swarm (m.); _avoid_ cluster (seul), mode swarm, orchestrateur

**Cluster**:
A Kubernetes cluster or a Nomad cluster Homeport connects to through its API; its kind says which, and decides the pages it has. A Kubernetes Cluster is added from a context of a kubeconfig: a name, the API server's address and the credentials the kubeconfig gives, stored sealed. A Nomad Cluster is added by the address of one of its agents: a name, the address, an optional region, and an ACL token and TLS material, stored sealed. Homeport installs nothing on either and only reaches what those credentials allow. Not a Swarm, which is found through its Hosts and never added. The UI may say where a Kubernetes Cluster came from ("context kind-dev of ~/.kube/config"), but a Cluster is never called a context.
_Avoid_: Context (for a Cluster), kube cluster, k8s, environment, orchestrator
_French_: cluster (m.); _avoid_ contexte, grappe, environnement

Kubernetes' own objects (Namespace, Pod, Deployment, StatefulSet, DaemonSet, Job, CronJob, Service, Ingress, ConfigMap, Secret, Node, Event) keep their Kubernetes names and capitals. A Cluster's Node, Service or Secret is the Kubernetes one, never a Swarm's.

Nomad's own objects (job, task group, task, allocation, evaluation, deployment, client node, node pool, namespace, ACL token and policy) keep Nomad's names, in lower case as Nomad writes them. A Nomad job is never a Stack, and an allocation is never a container: it runs one or more tasks, each with its driver.

**Port forward**:
A local port on this computer's loopback that Homeport connects, through the Kubernetes API, to a port of a Pod, or of a Service through one of its ready Pods, like `kubectl port-forward`. Kept in memory: it ends when Homeport quits or its Cluster is removed.
_Avoid_: Tunnel, proxy, published port (that's a container's)
_French_: redirection de port (f.); _avoid_ tunnel, proxy, port publié

**Release**:
A Helm release: one install of a chart on a Cluster, named in its Namespace, with a numbered revision for each install, upgrade and rollback. Helm keeps it in Secrets in that Namespace; Homeport manages it through the user's `helm` CLI (ADR 0009) and stores nothing about it. A release made in a terminal and one made in Homeport are the same.
_Avoid_: Deployment (that's a Kubernetes object), app, install
_French_: release (f.); _avoid_ déploiement, application, installation

**Chart repository**:
A server helm downloads charts from, in helm's own list on this computer (`helm repo list`), shared with the terminal. An OCI registry holding charts isn't one: its charts are installed by their `oci://` reference.
_Avoid_: Helm repo (in the UI), registry (for an http one)
_French_: dépôt de charts (m.); _avoid_ dépôt Helm, registre

**Node**:
One Engine in a Swarm, a manager or a worker. A node is a Host only when Homeport connects to it; a Host's page links to its node and back.
_Avoid_: Member, machine, server
_French_: nœud (m.); _avoid_ membre, machine, serveur

**Service**:
What a Swarm runs: an image, its settings and a number of replicas (or one per node, for a global service), carried out as tasks.
_Avoid_: App, deployment, workload
_French_: service (m.); _avoid_ application, déploiement

**Task**:
One slot of a service on one node, run as a container there. A service that isn't converging says why in its tasks.
_Avoid_: Replica (for a single one), job, pod
_French_: tâche (f.); _avoid_ réplica, job

**Board**:
The page listing the containers of every connected Host in one table, each row labelled with its Host. The other resource pages show the selected Host only.
_Avoid_: Dashboard, all hosts view
_French_: Tableau (m.); _avoid_ tableau de bord

**Command palette**:
The search box opened with Mod+K that goes to a page, switches Host, runs a command, and finds resources on every connected Host.
_Avoid_: Quick open, launcher, omnibox
_French_: palette de commandes (f.); _avoid_ lanceur

**Stack**:
A Compose project running on a Host, or a swarm stack running on a Swarm, identified by its project name (its namespace on a Swarm). Every Stack is Managed, Linked, Synced or External.
_Avoid_: Compose, project, application
_French_: stack (f.); _avoid_ projet Compose, application, pile

**Managed Stack**:
A Stack whose compose definition (and environment) Homeport holds, so it can be deployed, redeployed and edited from Homeport.
_Avoid_: Owned stack, local stack
_French_: stack gérée (f.); _avoid_ stack locale, stack possédée

**Linked Stack**:
A Stack whose compose definition is a folder on this machine: Homeport keeps where the folder and its compose files are, reads and writes the files in place, and deploys with that folder as the project directory, so `build:`, `env_file:` and relative paths work. Found from the compose labels of running containers, or created from a folder. It becomes External while its files are missing.
_Avoid_: Local stack, folder stack, bound stack
_French_: stack liée (f.); _avoid_ stack locale, stack dossier

**Synced Stack**:
A Stack whose compose definition comes from a git repository: Homeport keeps a clone of it on this machine, fetches a branch or tag on demand or on an interval while Homeport is open, shows what changed, and deploys a commit from the compose file's folder, so `build:` and relative paths work. Never edited in Homeport: its files change through git. Only its environment and its sync settings are Homeport's.
_Avoid_: Git stack, GitOps stack, remote stack
_French_: stack synchronisée (f.); _avoid_ stack git, stack GitOps

**Repository**:
A git repository Homeport may read for Synced Stacks: its URL and how Homeport authenticates to it (a deploy key Homeport generated, a user name and token, or nothing for a public one), app-wide rather than per Host, and shared by every Synced Stack made from it.
_Avoid_: Repo (in the UI), remote, source
_French_: dépôt (m.); _avoid_ repo, source, remote

**Deploy key**:
The Ed25519 key Homeport generates for one Repository, which the user adds to it on their git forge with read access only. Never the user's own SSH key.
_Avoid_: SSH key (alone), access key
_French_: clé de déploiement (f.); _avoid_ clé SSH (seule), clé d’accès

**External Stack**:
A Stack found running on a Host whose compose definition Homeport doesn't have, neither stored nor in a folder on this machine. It can be started, stopped and removed, but not redeployed or edited.
_Avoid_: Unmanaged stack, foreign stack, discovered stack
_French_: stack externe (f.); _avoid_ stack non gérée, stack étrangère

**Adopt**:
To turn an External Stack into a Managed Stack by supplying its compose definition.
_Avoid_: Import, claim
_French_: adopter; _avoid_ importer, réclamer

**Registry login**:
The user name and password or access token Homeport keeps for one container registry address (Docker Hub, GHCR, a private registry), app-wide rather than per Host, and hands to the Engine when it pulls, pushes or builds. Docker Hub is always stored as `docker.io`.
_Avoid_: Registry credential (in the UI), account, auth
_French_: identifiants de registre / connexion au registre; _avoid_ compte, authentification

**Helper container**:
A short-lived `busybox` container Homeport creates on a Host to open a volume (browse, back up, restore or clone it), since the Engine API can't read one. It mounts the volume read-only unless it must write to it, has no network, and is removed when the action ends (ADR 0015). Hidden from the Containers list and the Events page. Not a container of the user's.
_Avoid_: Sidecar, agent, temporary container

_French_: conteneur auxiliaire (m.); _avoid_ sidecar, agent
