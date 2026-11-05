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

**Stack**:
A Compose project running on a Host, identified by its project name. Every Stack is either Managed or External.
_Avoid_: Compose, project, application

**Managed Stack**:
A Stack whose compose definition (and environment) Homeport holds, so it can be deployed, redeployed and edited from Homeport.
_Avoid_: Owned stack, local stack

**External Stack**:
A Stack found running on a Host whose compose definition Homeport doesn't have. It can be started, stopped and removed, but not redeployed or edited.
_Avoid_: Unmanaged stack, foreign stack, discovered stack

**Adopt**:
To turn an External Stack into a Managed Stack by supplying its compose definition.
_Avoid_: Import, claim

**Registry login**:
The user name and password or access token Homeport keeps for one container registry address (Docker Hub, GHCR, a private registry), app-wide rather than per Host, and hands to the Engine when it pulls, pushes or builds. Docker Hub is always stored as `docker.io`.
_Avoid_: Registry credential (in the UI), account, auth
