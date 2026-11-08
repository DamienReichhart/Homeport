# Homeport

A desktop app for managing Docker Engines, wherever they run, from one place.

## Language

**Host**:
An Engine that Homeport connects to, defined by a name and how to reach it (local socket, TCP, TLS or SSH). Every container, image, volume, network and Stack belongs to exactly one Host. Messages refer to a Host by its name.
_Avoid_: Instance, endpoint, engine, context, server

**Engine**:
The Docker-compatible daemon a Host points to (Docker Engine, Docker Desktop, Podman, Colima, OrbStack…): the software that gets installed, started and stopped. Not a synonym for Host: "Start the Docker Engine" talks about the Engine, "Homeport can't reach prod-server" talks about the Host.
_Avoid_: Daemon, server, backend

**Bastion**:
An SSH server an SSH Host is reached through when its own SSH server only accepts connections from it (OpenSSH's `ProxyJump`). A Host has an ordered list of them, possibly empty; each has its own trusted server key and either uses the Host's credentials or its own. Not a Host: it runs no Engine Homeport talks to.
_Avoid_: Jump host, jump server, proxy, gateway, hop (in the UI)

**Swarm**:
A Docker Swarm cluster, found through the Hosts that belong to it and never added on its own: Homeport reaches it through any connected manager Host, addresses it by its swarm ID, and stores nothing about it (ADR 0016). Named "Swarm of <its first manager Host>", since Docker gives every Swarm the same name.
_Avoid_: Cluster (alone), swarm mode, orchestrator

**Cluster**:
A Kubernetes cluster Homeport connects to through its API server, added from a context of a kubeconfig: a name, the API server's address and the credentials the kubeconfig gives, stored sealed. Homeport installs nothing on it and only reaches what those credentials allow. Not a Swarm, which is found through its Hosts and never added. The UI may say where a Cluster came from ("context kind-dev of ~/.kube/config"), but a Cluster is never called a context.
_Avoid_: Context (for a Cluster), kube cluster, k8s, environment

**Node**:
One Engine in a Swarm, a manager or a worker. A node is a Host only when Homeport connects to it; a Host's page links to its node and back.
_Avoid_: Member, machine, server

**Service**:
What a Swarm runs: an image, its settings and a number of replicas (or one per node, for a global service), carried out as tasks.
_Avoid_: App, deployment, workload

**Task**:
One slot of a service on one node, run as a container there. A service that isn't converging says why in its tasks.
_Avoid_: Replica (for a single one), job, pod

**Board**:
The page listing the containers of every connected Host in one table, each row labelled with its Host. The other resource pages show the selected Host only.
_Avoid_: Dashboard, all hosts view

**Command palette**:
The search box opened with Mod+K that goes to a page, switches Host, runs a command, and finds resources on every connected Host.
_Avoid_: Quick open, launcher, omnibox

**Stack**:
A Compose project running on a Host, or a swarm stack running on a Swarm, identified by its project name (its namespace on a Swarm). Every Stack is Managed, Linked, Synced or External.
_Avoid_: Compose, project, application

**Managed Stack**:
A Stack whose compose definition (and environment) Homeport holds, so it can be deployed, redeployed and edited from Homeport.
_Avoid_: Owned stack, local stack

**Linked Stack**:
A Stack whose compose definition is a folder on this machine: Homeport keeps where the folder and its compose files are, reads and writes the files in place, and deploys with that folder as the project directory, so `build:`, `env_file:` and relative paths work. Found from the compose labels of running containers, or created from a folder. It becomes External while its files are missing.
_Avoid_: Local stack, folder stack, bound stack

**Synced Stack**:
A Stack whose compose definition comes from a git repository: Homeport keeps a clone of it on this machine, fetches a branch or tag on demand or on an interval while Homeport is open, shows what changed, and deploys a commit from the compose file's folder, so `build:` and relative paths work. Never edited in Homeport: its files change through git. Only its environment and its sync settings are Homeport's.
_Avoid_: Git stack, GitOps stack, remote stack

**Repository**:
A git repository Homeport may read for Synced Stacks: its URL and how Homeport authenticates to it (a deploy key Homeport generated, a user name and token, or nothing for a public one), app-wide rather than per Host, and shared by every Synced Stack made from it.
_Avoid_: Repo (in the UI), remote, source

**Deploy key**:
The Ed25519 key Homeport generates for one Repository, which the user adds to it on their git forge with read access only. Never the user's own SSH key.
_Avoid_: SSH key (alone), access key

**External Stack**:
A Stack found running on a Host whose compose definition Homeport doesn't have, neither stored nor in a folder on this machine. It can be started, stopped and removed, but not redeployed or edited.
_Avoid_: Unmanaged stack, foreign stack, discovered stack

**Adopt**:
To turn an External Stack into a Managed Stack by supplying its compose definition.
_Avoid_: Import, claim

**Registry login**:
The user name and password or access token Homeport keeps for one container registry address (Docker Hub, GHCR, a private registry), app-wide rather than per Host, and hands to the Engine when it pulls, pushes or builds. Docker Hub is always stored as `docker.io`.
_Avoid_: Registry credential (in the UI), account, auth

**Helper container**:
A short-lived `busybox` container Homeport creates on a Host to open a volume (browse, back up, restore or clone it), since the Engine API can't read one. It mounts the volume read-only unless it must write to it, has no network, and is removed when the action ends (ADR 0015). Hidden from the Containers list and the Events page. Not a container of the user's.
_Avoid_: Sidecar, agent, temporary container
