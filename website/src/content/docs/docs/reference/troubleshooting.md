---
title: Troubleshooting
description: Fixes for the problems you're most likely to meet, by the message Homeport shows.
---

If your problem isn't here, [open an issue](https://github.com/DamienReichhart/Homeport/issues/new/choose) with the [diagnostics bundle](#exporting-diagnostics-for-a-bug-report) from **Settings → About**.

## Installing and starting

### Windows says "Windows protected your PC"

The installer isn't signed with a commercial certificate yet. Choose **More info**, then **Run anyway**. You can [verify the download](/docs/reference/verifying/) first.

### macOS says Homeport "can't be opened"

Homeport isn't notarised by Apple yet. Open **System Settings → Privacy & Security** and choose **Open Anyway** next to Homeport, or run:

```sh
xattr -dr com.apple.quarantine /Applications/Homeport.app
```

### The AppImage doesn't start

If it mentions FUSE, install `libfuse2` (`libfuse2t64` on Ubuntu 24.04 and later). Check the file is executable: `chmod +x Homeport*.AppImage`.

### Homeport opens in the wrong place

Homeport reopens with the size, position and maximised state its window had when you closed it. If the display it was on is gone, it opens centred on one that's left. To forget the saved place, quit Homeport and delete `.window-state.json` from its settings folder:

| System  | File                                                                |
| ------- | ------------------------------------------------------------------- |
| Linux   | `~/.config/com.homeport.app/.window-state.json`                     |
| macOS   | `~/Library/Application Support/com.homeport.app/.window-state.json` |
| Windows | `%APPDATA%\com.homeport.app\.window-state.json`                     |

### Opening Homeport again doesn't open a second window

Only one Homeport runs at a time: opening it while it's already running brings its window to the front, even when it's hidden in the [system tray](/docs/guides/overview/#system-tray). Two copies would connect to every Host twice and write to the same data.

### There's no tray icon

On Linux, the tray needs the appindicator library, and some desktops hide tray icons: on GNOME, install the AppIndicator Support extension. If Homeport finds no appindicator library, **Settings → Desktop** says so and closing the window quits. See [requirements](/docs/requirements/#optional-a-system-tray).

### A red banner says no OS keychain is available

Homeport couldn't reach your keychain. On Linux this usually means no Secret Service is running — common on minimal desktops and window managers. Install and start GNOME Keyring (`gnome-keyring`) or enable KWallet's Secret Service, then restart Homeport. Until then, use local socket, TCP or ssh-agent Hosts.

### "This database was last used by Homeport … or later"

You installed an older Homeport than the one that last opened your data, for example by reinstalling an earlier release. Each version upgrades the database to its own format when it starts, and an older version can't read a newer format, so Homeport stops before it changes anything: the window shows this message, the path of the database and **Open folder**, and nothing else starts.

Your Hosts, Stacks and secrets are intact. Install the version the message names, or any newer one, from the [download page](/download/), and Homeport opens them as before. Don't delete or move `homeport.db`: that is the only way to lose what's in it. Downgrading isn't supported; see [Your data across versions](/docs/reference/updating/#your-data-across-versions).

The same window appears for any other failure to open the database (damaged file, full disk, another Homeport holding it), with that failure's own message and what to do about it.

## Updates and the package repository

### "Homeport couldn't download the update"

When it shows while checking, Homeport couldn't download the update information from this website: you may be offline, or behind a proxy or firewall that blocks `damienreichhart.github.io`. Homeport keeps working; it checks again later, or choose **Settings → Updates → Check now**. You can always download the latest version from the [download page](/download/).

### An update fails to install

Homeport only installs an update whose signature matches its update key, and stops if the download is incomplete or altered. Try again later; if it keeps failing, download the new version from the [download page](/download/) and install it over the old one.

### apt says "NO_PUBKEY" or "The repository is not signed"

The Homeport key is missing from `/etc/apt/keyrings/homeport.asc`. Run the [quick install](/docs/installation/#quick-install) again, or the first two commands of the [manual setup](/docs/installation/#manual-setup).

### dnf or zypper asks whether to import a key

That's the Homeport package key, the first time your package manager uses the repository. Check that the fingerprint it shows matches the one on [verifying downloads](/docs/reference/verifying/#package-signatures), then accept.

### apt says "Hash Sum mismatch" right after a release

The repository was being updated while apt read it. Run `sudo apt update` again a few minutes later.

### My package manager doesn't see the new version

Check the repository is still set up: `/etc/apt/sources.list.d/homeport.sources`, `/etc/yum.repos.d/homeport.repo` or `/etc/zypp/repos.d/homeport.repo`. If you deleted it, run the [quick install](/docs/installation/#quick-install) to add it again. If you installed 0.2.0 or earlier from a downloaded file, the repository was never added: see [updating from 0.2.0](/docs/reference/updating/#updating-from-020-or-earlier).

## Connecting

### "Homeport isn't allowed to use the Docker Engine at …"

Your user can't open the Docker socket. On Linux, add it to the `docker` group and log in again:

```sh
sudo usermod -aG docker $USER
```

### "No Docker Engine is running at …"

Nothing is listening at the socket path. When nothing exists there at all, the message is _The Docker Engine doesn't seem to be installed_ instead. Start your Engine — Docker Desktop, Colima, OrbStack, Rancher Desktop — or check the path. See [local engines](/docs/hosts/local/#adding-a-socket-by-hand).

### "The Docker Engine didn't answer in time" or "… didn't answer in time"

Homeport waits 10 seconds for a connection. Check the address and port, that the server is up, and that no firewall is in the way. Homeport keeps retrying on its own.

### Access refused: "The SSH server refused to let … log in"

The server doesn't accept Homeport's key yet. Run the **Add the public key** command from the Host's page on the server, then choose **Reconnect**. See [SSH Hosts](/docs/hosts/ssh/#authorise-homeports-key).

### "… can't open the Docker Engine socket at /var/run/docker.sock on …"

Homeport logged in, but couldn't reach Docker. When it can tell why, the message says so instead: the Engine isn't installed or isn't running on the server, the user isn't allowed to use it, or the SSH server doesn't allow forwarding to its socket. Check:

- the **Remote socket path** — rootless Docker uses `/run/user/<uid>/docker.sock`;
- that the user can use Docker: `sudo usermod -aG docker <user>` on the server;
- that the server allows socket forwarding (`AllowStreamLocalForwarding` in `sshd_config`, on by default).

### "No ssh-agent is running"

Start your SSH agent and add your key with `ssh-add`. If you opened Homeport from a desktop menu, it may not see the agent your terminal uses; log out and in, or start Homeport from a terminal where `ssh-add -l` lists your key.

On Windows, Homeport uses either the OpenSSH Authentication Agent service or PuTTY's Pageant. Open Pageant and load your key, or start the service in an administrator PowerShell:

```powershell
Get-Service ssh-agent | Set-Service -StartupType Automatic
Start-Service ssh-agent
ssh-add
```

### Access refused: "The secure connection to … failed"

- The CA certificate must be the one that signed the server's certificate.
- The address you entered must match a name or IP address in the server's certificate.
- The server must run with `--tlsverify` and trust your client certificate's CA.

### Server key changed

The SSH server presented a different key from the one you trusted. If you reinstalled the server or changed its keys, choose **Trust the new key**. If you don't know why it changed, don't connect: ask whoever runs the server. See [SSH Hosts](/docs/hosts/ssh/#when-the-servers-key-changes).

### Credentials lost

The key that encrypts your secrets is missing from the keychain or has changed — after a keychain reset, a new user profile, or copying Homeport's data to another computer (use a [backup](/docs/reference/data-and-backups/#backing-up-in-homeport), or [Export and Import](/docs/reference/security/#moving-to-another-computer), for that). Homeport keeps the Host; re-enter its credentials on the Host's page:

- **TLS** — load the three certificate files again and choose **Save**.
- **SSH with an imported key** — paste the private key and passphrase again and choose **Save**.
- **SSH with a Homeport key** — choose **Regenerate key**, then authorise the new key on the server.

Managed and Synced Stack environments, registry logins and repositories' keys and tokens encrypted with the old key need re-entering too, and Clusters refreshing from their kubeconfig; [If the secrets are lost](/docs/reference/data-and-backups/#if-the-secrets-are-lost) lists each one.

## Clusters

### Access refused: "… refused the credentials from its kubeconfig"

The API server answered 401: the certificate or token has expired or was revoked. Check that `kubectl --context <context> get namespaces` still works, then use **Refresh from kubeconfig** on the Cluster's page. For an exec plugin, Homeport already asked it for a new token once.

### "… isn't installed on this computer"

The Cluster's [exec plugin](/docs/clusters/exec-plugins/) (`aws`, `gke-gcloud-auth-plugin`, `kubelogin`…) wasn't found on `PATH` or in its usual install places. Install it, check it runs in a terminal, then **Reconnect**.

### "… needs you to sign in, which it can't do inside Homeport"

The exec plugin wants a browser or a device code. Run `kubectl --context <context> get namespaces` once in a terminal, sign in there, then **Reconnect**.

### "… couldn't get credentials for …"

The exec plugin ran and failed. **Details** shows the end of what it printed, often an expired profile or a wrong region. Homeport tries again on its own.

### "Its user signs in with the gcp auth provider, which Kubernetes no longer supports"

The `gcp` and `azure` auth providers were removed from Kubernetes. Fetch the credentials again with `gke-gcloud-auth-plugin` installed (GKE), or run `kubelogin convert-kubeconfig` (AKS), then add the context again.

### "Your credentials don't allow “…” on … in …"

The cluster's RBAC doesn't let your kubeconfig's user do that. Pick a namespace you're allowed to see in the picker at the top right, or ask the cluster's administrator. The message gives the `kubectl auth can-i` command that checks it. Other pages keep working.

### Unreachable: "… refused the connection" or "… didn't answer in time"

The API server isn't reachable from this computer: the cluster is stopped (a local one), behind a VPN, or the kubeconfig's server address changed. Homeport keeps retrying on its own; check `kubectl cluster-info` with the same context.

### "The certificate of … isn't signed by the CA in its kubeconfig"

kind, k3d and minikube make new certificates each time they recreate a cluster. Use **Refresh from kubeconfig** on the Cluster's page.

### Nomad: "… doesn't know the ACL token Homeport sent"

The token was mistyped, has expired, or was deleted. Make a new one with the policy from [ACL policies](/docs/clusters/nomad/#acl-policies) (`nomad acl token create -name=homeport -policy=homeport`) and paste its secret ID in the cluster's **Settings**. The cluster waits as **Access refused** until you do.

### Nomad: "… refused: the ACL token lacks a permission this needs"

The token's policies don't grant what this page or action needs; the message says which (reading jobs or logs, running or stopping jobs, scaling, opening a shell, changing nodes). Add the capability to the token's policy; [ACL policies](/docs/clusters/nomad/#acl-policies) lists what each needs. Other pages keep working.

### Nomad: "… has no leader: its servers can't agree on one"

Fewer than a majority of the cluster's servers are running or can reach each other, so Nomad can't answer. Check them with `nomad server members` on a server, and bring enough back. Homeport keeps retrying.

### Nomad: "… has no job …"

The job was stopped and purged, or garbage-collected, or it lives in another namespace than the one picked. Open the Jobs page again, with **All namespaces** if need be.

### Nomad: "Task group … can't run more than …" or "fewer than …"

The task group's `scaling` block sets a `min` and a `max`, and the count asked is outside them. Change the bounds in the job's specification and run it again to go further.

### Nomad: "Nomad won't scale … while a deployment of it is in progress"

Running a new version of a service job starts a deployment, and Nomad refuses to change its count until every allocation of the new version is healthy. Wait for the job's allocations to settle, then scale again.

### Nomad: "The job specification doesn't parse, at line …, column …"

Nomad's parser found a problem at that place; **Details** has its message. `variable` blocks need a default, since Homeport doesn't pass variable files. A JSON specification must be a job in Nomad's API format, as `nomad job inspect` prints it.

### Nomad: "Nomad can't place every allocation of …"

The plan found no eligible node with the resources the task group asks for, or matching its constraints; the plan says which. **Run anyway** queues what can't be placed until a node can take it, or lower the resources, change the constraints, or add nodes.

### Nomad: "… changed since it was planned, so it wasn't run"

Someone else ran or stopped the job between your plan and **Run**, and Homeport doesn't overwrite their change. Go back, plan again, and check the new diff before running.

### Nomad: "Task … of allocation … isn't running"

A shell needs a running task: this one hasn't started, has finished, or failed. Its logs stay readable until the allocation is garbage-collected.

### Nomad: "… can't reach the node allocation … runs on"

Logs and shells go through the servers to the client node, and the servers can't reach it: the node is down, disconnected, or its agent stopped. Check it on the Nodes page or with `nomad node status`.

### Helm: "The helm CLI isn't installed on this computer"

[Helm releases](/docs/clusters/helm/) need `helm` 3 or later on this computer. Install it (`brew install helm`, `winget install Helm.Helm`, or `snap install helm --classic`), then reopen the Helm page; **Settings → This machine** shows the one Homeport found. A `helm` 2 is reported as unsupported.

### Helm: "Another install, upgrade or rollback of this release is still in progress"

The release is `pending-…`: an install, upgrade or rollback didn't finish, often because it was interrupted. helm refuses to change it until that ends. Wait for it, or roll the release back to its last deployed revision from its **History**.

## Using Homeport

### Deploy is greyed out: "The docker compose CLI isn't installed on this computer"

Homeport needs `docker compose` on this computer to deploy Stacks. Install Docker Desktop or the Docker CLI with the Compose plugin, then restart Homeport. **Settings → This machine** shows whether it's found.

If it's installed but not found, Homeport may have a different `PATH` from your terminal — common when started from a desktop menu on macOS. Make sure `docker` is in a standard location such as `/usr/local/bin`.

### Deploy fails: "uses build:"

A Managed Stack has no folder to build from. Either [link the project folder](/docs/guides/stacks/#creating-a-linked-stack) instead — a Linked Stack builds — or build and push the image to a registry, then use `image:`.

### My Stack became External

A [Linked Stack](/docs/guides/stacks/#when-the-folder-moves-or-disappears) shows as External, marked ⚠ **Its compose files are no longer on this computer**, when its folder was moved, renamed or deleted, or a compose file in it was. Nothing is lost: put the folder back where it was and the Stack is Linked again at the next refresh. If the folder moved for good, choose **Forget** in the Stack's **Definition** tab, then link it again from its new place with **New stack → Use a folder on this computer**.

A Stack started from a folder on another computer is always External here, since this computer doesn't have that path.

### "Changed outside Homeport"

A Linked Stack's compose files were edited outside Homeport — in your editor, by `git pull` — since Homeport last read them. Choose **Reload** to see them as they are. If you were editing in Homeport at the same time, saving is refused with **… changed outside Homeport since you opened it** so neither change is lost silently: reload and redo your edit, or choose **Overwrite** to keep yours. See [editing a Linked Stack](/docs/guides/stacks/#editing-a-linked-stack).

### Pulling a private image fails

Add a login for the image's registry in **Settings → Registries**, and use **Check login** to confirm the registry accepts it. GHCR, and Docker Hub accounts with two-factor authentication, need an access token rather than the password. The pull dialog says which login it will use; if it says **anonymously**, the image's registry doesn't match a stored login — `ghcr.io/org/app` needs a login for `ghcr.io`, and `org/app` one for Docker Hub.

### A build fails

The message names the step that failed, such as **The build failed at step 3/5 (RUN make)**; **Details** holds the end of the build's output. Homeport uses the Engine's classic builder, so Dockerfiles that need BuildKit (`RUN --mount`, `--secret`, heredocs) fail; build those from a terminal. A `FROM` image that's private needs a login for its registry in **Settings → Registries**.

### No newer-image mark appears

The mark only shows when the registry was asked and answered. Images built locally or pulled by digest never show one. If a registry can't be reached or limits requests (Docker Hub counts these checks against its pull limit), Homeport writes a line to its [log](/docs/reference/security/#logs) and shows nothing; answers are also kept for 6 hours, so choose **Check for newer images** to ask again now. A Docker Hub login in **Settings → Registries** raises the limit.

### Searching Docker Hub shows an error

The search runs on the selected Host, so it needs that Host's internet access, and tags are read from `hub.docker.com` by this computer. Either can be rate-limited or blocked by a proxy. The dialog still pulls any full reference you type, such as `postgres:17-alpine`.

### The Files tab says the container has no find/stat tools

Minimal and distroless images don't include `find` and `stat`, which Homeport uses to list files. Use `docker cp` for those containers.

### The terminal doesn't open

The container must be running, and have `/bin/sh` or `/bin/bash`. Use **New terminal** to run a different program.

### A volume or network won't delete

The Engine refuses while a container — even a stopped one — still uses it. Remove those containers first; the confirmation lists them.

### "This needs Podman 5.8 or later"

Podman's API reads, installs, edits and removes [Quadlet units](/docs/guides/quadlet/) from Podman 5.8, and lists them from 5.7. Update Podman on the Host, or use `podman quadlet` in a terminal there. Starting and stopping units works on every Podman.

### Quadlet: Start and Stop are greyed out

Homeport starts and stops a unit's service with `systemctl`, which it can only run over SSH or for a Podman on this computer (on Linux). Reach the Host over SSH instead of TCP or TLS. A unit whose service shows **not generated yet** has nothing to start: systemd's generator skipped its file.

### Quadlet: "The user's systemd isn't running on the host"

User units (rootless Podman) need the user's systemd instance, which only runs while the user is logged in unless lingering is on. As that user on the Host, run `loginctl enable-linger "$USER"`, then try again.

### Quadlet: "systemd didn't let this user manage …"

System units (rootful Podman) can only be started and stopped by a user allowed to manage them, such as `root`, and Homeport never uses `sudo`. Connect as such a user, or use rootless Podman, whose units are the user's own.

### containerd: "isn't allowed to use containerd at /run/containerd/containerd.sock"

containerd's socket belongs to root. Give a group access to it with `gid` under `[grpc]` in `/etc/containerd/config.toml`, add your user (or the SSH user) to that group, restart containerd and log in again; [the containerd page](/docs/hosts/containerd/#letting-your-user-use-the-socket) has the commands. Rootless containerd can't be reached at all: its socket only exists inside RootlessKit's namespace.

### containerd: "This container's output isn't recorded"

containerd keeps no logs; the tool that started the task decides where its output goes. Containers made with `ctr` usually send it nowhere. nerdctl's and Kubernetes' containers log to files Homeport reads; see [Logs](/docs/hosts/containerd/#logs).

### containerd: logs or the terminal say they need SSH

They need the files of containerd's machine, which Homeport reaches over SSH, or on this computer for a local socket on Linux. Add the Host over SSH. If the log file can't be read, it usually belongs to root: connect as a user allowed to read it.

### containerd: "can't pull images for Homeport"

Pulling goes through containerd's Transfer service, stable from containerd 2.0. Update containerd, or pull with `ctr images pull` or `nerdctl pull` on the Host; the image then shows on the Images page.

### Incus: "isn't allowed to use Incus at /var/lib/incus/unix.socket"

Incus's admin socket is for the `incus-admin` group, and the restricted `unix.socket.user` for the `incus` group. Add your user (or the SSH user) with `sudo usermod -aG incus-admin "$USER"`, then log out and in again, or reconnect over SSH. See [Incus's sockets](/docs/hosts/incus/#its-sockets).

### Incus: "Homeport couldn't reach Incus at …" when adding with a trust token

Incus doesn't listen on the network until `core.https_address` is set. On the server, run `incus config set core.https_address :8443`, check that a firewall lets port 8443 through, and make a new token: the token lists the addresses Incus listened on when it was made.

### Incus: "Incus refused the trust token"

A token works once, and expires. Make a new one on the server with `incus config trust add homeport` and paste it within its lifetime.

### Incus: "… presented a certificate with fingerprint …, not the trusted …"

The server's certificate isn't the one Homeport trusted: Incus was reinstalled, its certificate was renewed, or something else answers at that address. Check the fingerprint on the server with `incus info`; if it's the one shown, remove the Host and add it again with a new token. See [pinned certificates](/docs/hosts/incus/#the-servers-certificate-is-pinned).

### Incus: "This host can't run virtual machines"

Incus needs KVM (`/dev/kvm`) and QEMU for virtual machines. Enable virtualization in the server's firmware (or nested virtualization for a virtual server), or create a system container instead.

### Incus: "The virtual machine's Incus agent isn't running"

Exec in a virtual machine goes through `incus-agent` inside it, which Incus's own images start on boot. Wait for the boot to finish, or use the **Console** tab, which needs no agent.

### "Something went wrong"

A page of Homeport hit a problem it didn't expect and shows this instead of its content; the sidebar still works. **Reload** usually brings the page back, and **Back to overview** leaves it. If it keeps happening, copy the report from **Details** into [an issue](https://github.com/DamienReichhart/Homeport/issues/new/choose), with the [diagnostics bundle](#exporting-diagnostics-for-a-bug-report).

### "… isn't connected"

The action needed the Host, but it disconnected. Wait for it to reconnect, or choose **Retry now**.

## Exporting diagnostics for a bug report

**Settings → About → Export diagnostics…** makes one zip to attach to a GitHub issue. Homeport sends it nowhere: it saves the file where you choose, and sharing it is up to you.

The dialog lists the files and shows `summary.json` before you save:

- `summary.json`: Homeport's version and how it was installed, your operating system, the database's schema version, whether a keychain is available, your settings, and for each Host and Cluster its kind of connection, its Engine and version, its status and the code of its last error, plus how many Stacks of each kind you have.
- `logs/`: Homeport's [log files](/docs/reference/security/#logs), with private keys and certificates, passwords, tokens, `Authorization` headers and the credentials in URLs replaced by `[redacted]`.

Hosts and Clusters appear as `host-1`, `cluster-1`… and their names and addresses are replaced the same way in the logs, unless you tick **Include names and addresses**. Tick it when the name or address matters to the problem and you're comfortable sharing it.

Stored secrets are never read for the bundle. Removing secrets from logs works by recognising their usual shapes, so look through the files before attaching them to a public issue if your logs may quote something unusual.
