---
title: Troubleshooting
description: Fixes for the problems you're most likely to meet, by the message Homeport shows.
---

If your problem isn't here, check the [log file](/docs/reference/security/#logs) and [open an issue](https://github.com/DamienReichhart/Homeport/issues) with your Homeport version from **Settings**.

## Installing and starting

### Windows says "Windows protected your PC"

The installer isn't signed with a commercial certificate yet. Choose **More info**, then **Run anyway**. You can [verify the download](/docs/reference/verifying/) first.

### macOS says Homeport "can't be opened"

Homeport isn't notarised by Apple yet. Open **System Settings → Privacy & Security** and choose **Open Anyway** next to Homeport, or run:

```sh
xattr -dr com.apple.quarantine /Applications/Homeport.app
```

### The AppImage doesn't start

If it mentions FUSE, install `libfuse2` (`libfuse2t64` on Ubuntu 24.04 and later). Check the file is executable: `chmod +x Homeport_*.AppImage`.

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

## Updates and the package repository

### "Couldn't check for updates"

Homeport couldn't download the update information from this website: you may be offline, or behind a proxy or firewall that blocks `damienreichhart.github.io`. Homeport keeps working; it checks again later, or choose **Settings → Updates → Check now**. You can always download the latest version from the [download page](/download/).

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

### Unreachable: Permission denied

Your user can't open the Docker socket. On Linux, add it to the `docker` group and log in again:

```sh
sudo usermod -aG docker $USER
```

### Unreachable: No such file or directory

Nothing is listening at the socket path. Start your Engine — Docker Desktop, Colima, OrbStack, Rancher Desktop — or check the path. See [local engines](/docs/hosts/local/#adding-a-socket-by-hand).

### "The Engine did not answer in time" or "SSH connection timed out"

Homeport waits 10 seconds for a connection. Check the address and port, that the server is up, and that no firewall is in the way. Homeport keeps retrying on its own.

### Access refused: "The server refused the key for …"

The server doesn't accept Homeport's key yet. Run the **Add the public key** command from the Host's page on the server, then choose **Reconnect**. See [SSH Hosts](/docs/hosts/ssh/#authorise-homeports-key).

### "The SSH server couldn't open /var/run/docker.sock"

Homeport logged in, but couldn't reach Docker. Check:

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

### Access refused: "TLS handshake failed"

- The CA certificate must be the one that signed the server's certificate.
- The address you entered must match a name or IP address in the server's certificate.
- The server must run with `--tlsverify` and trust your client certificate's CA.

### Server key changed

The SSH server presented a different key from the one you trusted. If you reinstalled the server or changed its keys, choose **Trust the new key**. If you don't know why it changed, don't connect: ask whoever runs the server. See [SSH Hosts](/docs/hosts/ssh/#when-the-servers-key-changes).

### Credentials lost

The key that encrypts your secrets is missing from the keychain or has changed — after a keychain reset, a new user profile, or copying Homeport's data to another computer (use [Export and Import](/docs/reference/security/#moving-to-another-computer) for that). Homeport keeps the Host; re-enter its credentials on the Host's page:

- **TLS** — load the three certificate files again and choose **Save**.
- **SSH with an imported key** — paste the private key and passphrase again and choose **Save**.
- **SSH with a Homeport key** — choose **Regenerate key**, then authorise the new key on the server.

Managed Stack environments sealed with the old key need re-entering too.

## Clusters

### Access refused: "… refused the credentials from its kubeconfig"

The API server answered 401: the certificate or token has expired or was revoked. Check that `kubectl --context <context> get namespaces` still works, then use **Refresh from kubeconfig** on the Cluster's page. For an exec plugin, Homeport already asked it for a new token once.

### "… isn't installed on this computer"

The Cluster's [exec plugin](/docs/clusters/exec-plugins/) (`aws`, `gke-gcloud-auth-plugin`, `kubelogin`…) wasn't found on `PATH` or in its usual install places. Install it, check it runs in a terminal, then **Reconnect**.

### "… needs you to sign in, which it can't do inside Homeport"

The exec plugin wants a browser or a device code. Run `kubectl --context <context> get namespaces` once in a terminal, sign in there, then **Reconnect**.

### "… couldn't get credentials for …"

The exec plugin ran and failed. **Details** shows the end of what it printed, often an expired profile or a wrong region. Homeport tries again on its own.

### "… signs in with the gcp auth provider, which Kubernetes no longer supports"

The `gcp` and `azure` auth providers were removed from Kubernetes. Fetch the credentials again with `gke-gcloud-auth-plugin` installed (GKE), or run `kubelogin convert-kubeconfig` (AKS), then add the context again.

### "The certificate of … isn't signed by the CA in its kubeconfig"

kind, k3d and minikube make new certificates each time they recreate a cluster. Use **Refresh from kubeconfig** on the Cluster's page.

## Using Homeport

### Deploy is greyed out: "The docker compose CLI is not installed"

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

### "host … is not connected"

The action needed the Host, but it disconnected. Wait for it to reconnect, or choose **Retry now**.
