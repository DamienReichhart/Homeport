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

### A red banner says no OS keychain is available

Homeport couldn't reach your keychain. On Linux this usually means no Secret Service is running — common on minimal desktops and window managers. Install and start GNOME Keyring (`gnome-keyring`) or enable KWallet's Secret Service, then restart Homeport. Until then, use local socket, TCP or ssh-agent Hosts.

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

### "No ssh-agent is reachable"

Start your SSH agent and add your key with `ssh-add`. If you opened Homeport from a desktop menu, it may not see the agent your terminal uses; log out and in, or start Homeport from a terminal where `ssh-add -l` lists your key.

### Access refused: "TLS handshake failed"

- The CA certificate must be the one that signed the server's certificate.
- The address you entered must match a name or IP address in the server's certificate.
- The server must run with `--tlsverify` and trust your client certificate's CA.

### Server key changed

The SSH server presented a different key from the one you trusted. If you reinstalled the server or changed its keys, choose **Trust the new key**. If you don't know why it changed, don't connect: ask whoever runs the server. See [SSH Hosts](/docs/hosts/ssh/#when-the-servers-key-changes).

### Credentials lost

The key that encrypts your secrets is missing from the keychain or has changed — after a keychain reset, a new user profile, or copying Homeport's data to another computer. Homeport keeps the Host; re-enter its credentials on the Host's page:

- **TLS** — load the three certificate files again and choose **Save**.
- **SSH with an imported key** — paste the private key and passphrase again and choose **Save**.
- **SSH with a Homeport key** — choose **Regenerate key**, then authorise the new key on the server.

Managed Stack environments sealed with the old key need re-entering too.

## Using Homeport

### Deploy is greyed out: "The docker compose CLI is not installed"

Homeport needs `docker compose` on this computer to deploy Stacks. Install Docker Desktop or the Docker CLI with the Compose plugin, then restart Homeport. **Settings → This machine** shows whether it's found.

If it's installed but not found, Homeport may have a different `PATH` from your terminal — common when started from a desktop menu on macOS. Make sure `docker` is in a standard location such as `/usr/local/bin`.

### Deploy fails: "uses build:"

The Host would build from files on this computer. Build and push the image to a registry, then use `image:` instead.

### Pulling a private image fails

Homeport's **Pull image** doesn't log in to registries yet. Log in and pull on the Host, or use the image in a Managed Stack: Compose uses the Docker credentials on this computer.

### The Files tab says the container has no find/stat tools

Minimal and distroless images don't include `find` and `stat`, which Homeport uses to list files. Use `docker cp` for those containers.

### The terminal doesn't open

The container must be running, and have `/bin/sh` or `/bin/bash`. Use **New terminal** to run a different program.

### A volume or network won't delete

The Engine refuses while a container — even a stopped one — still uses it. Remove those containers first; the confirmation lists them.

### "host … is not connected"

The action needed the Host, but it disconnected. Wait for it to reconnect, or choose **Retry now**.
