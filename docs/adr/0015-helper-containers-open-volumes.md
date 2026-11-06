# Volumes are opened by short-lived helper containers

The Engine API can list, create and remove a volume, but not read or write what's in it. Browsing a volume's files, backing it up to a file, restoring it and cloning it all need its contents. Homeport installs nothing on a Host and talks only to the Engine API, so the only way in is a container that mounts the volume: a **helper container**. It is the one thing Homeport runs on a Host of its own accord, so it comes with guarantees.

## The helper

- **Image**: `busybox:1.37`, one constant (`homeport_engine::volumes::helper::IMAGE`). It's multi-arch, about 2 MB, and has `find`, `stat`, `cp` and `sleep`, which is all the helper needs. Its `find … -exec stat` output parses like the container file browser's (a test checks it).
- **Name and labels**: `homeport-helper-<random>`, with `com.homeport.helper=true` and `com.homeport.helper.owner=<this computer's hostname>`.
- **Confinement**: no network (`NetworkMode: none`), a read-only root filesystem, and the volume mounted **read-only** unless the action must write to it (a restore, the target of a clone). Browsing and backing up can't change a volume, even one a running container uses.
- **Lifetime**, by action:
  - Browsing runs `sleep 3600` with `AutoRemove`: Homeport runs `find` and `stat` in it and reads files through the archive API, and removes it when the browser closes. If Homeport goes away, the Engine removes it within the hour.
  - Backups and restores use a helper that is created but never started, because the archive API reads and writes a stopped container's volumes. It is removed when the transfer ends, whether it succeeded, failed or was cancelled.
  - Clones and emptying a volume run one command (`cp -a`, `find -delete`) to completion, then the helper is removed.
- **Leftovers**: when this computer connects to a Host, Homeport removes the helpers it owns there (by the owner label), except those of sessions still open. Helpers of another computer using the same Host are left alone, so one person's Homeport never ends another's session.
- **Hidden**: helpers are left out of the Containers list and of a volume's **Used by**, and their events (and the mounts of their volumes) out of the Events page and the live refreshes. A helper isn't something the user made, and showing it would make the volume look used.

## Getting the image

Homeport never pulls the image without asking. When the Host doesn't have it, the action first says which image it needs and how big it is, and pulls it (with the stored registry logins) only once the user agrees. When Homeport pulled it, the image is removed again at the end of the session, unless another helper still runs or another container uses it (the Engine refuses then, and it's kept). A failed pull, such as on a Host without internet access, is reported as `helper_image_unavailable` with the commands to get the image there by hand: `docker pull busybox:1.37`, or `docker save` elsewhere and `docker load` on the Host.

Engines running Windows containers can't run a Linux `busybox`, so opening a volume there is refused with a validation error before anything is created.

## Considered Options

- **Running a command in a container that already mounts the volume**: no new container, but only for volumes in use, only in images that have a shell and `tar`, and with the user's container doing the work under its own user, possibly without permission to read everything.
- **Reading the volume's mountpoint over SSH**: only for SSH Hosts, needs root on the Host, and breaks the rule that Homeport only speaks the Engine API.
- **`alpine` as the image**: it's what the tests use and has the same tools, but is three times the size, for nothing the helper needs.
- **A long-lived helper per Host**: faster to open, but a container that stays on the Host when nothing is being done is exactly what users don't expect from a desktop app.

## Consequences

- Every helper action works on local, TCP, TLS and SSH Hosts (bastions included), since it uses nothing but the Engine API.
- The first use on a Host without the image costs a pull of about 2 MB, after asking.
- A volume browsed for more than an hour loses its helper; the browser then fails to list and is opened again.
- The engine tests that create helpers hold one lock, since one of them removes this computer's leftovers; CI pulls `busybox:1.37` up front and checks that no `homeport-helper-*` container is left.
