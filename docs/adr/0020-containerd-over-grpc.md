# containerd over its gRPC API, with commands over SSH for logs and exec

## Context

v0.19.0 lets a Host's Engine be containerd: its namespaces, containers and tasks, logs, exec, images and snapshots. containerd is a library-shaped Engine: its gRPC API stores containers, images, content and snapshots, but a client is expected to do much of the work itself and to share the Host's filesystem. Homeport is a remote client that installs nothing on a Host and reaches an SSH Host's socket through a `direct-streamlocal` channel (ADR 0002). Since ADR 0019, it may also run a short, fixed list of commands on the SSH session it already holds, for actions the Engine's API lacks.

What containerd offers, from its API and source:

- **The API** is gRPC over HTTP/2 on `/run/containerd/containerd.sock`, root-owned, with the group set by `[grpc] gid` in `config.toml`. It isn't served over the network by default. Every call names its namespace in the `containerd-namespace` metadata.
- **Pulling**: the Images service only stores records. A pull is resolve, fetch into the content store and unpack into a snapshotter. containerd 2.0's Transfer service does all three on the Engine's side; the client answers registry credentials on a stream of the Streaming service and reads progress from another. containerd 1.7 has Transfer behind a flag.
- **Task I/O**: `Tasks.Create` and `Tasks.Exec` take stdin, stdout and stderr as paths (FIFOs) or `binary://` and `file://` URIs on the Host's filesystem. A gRPC client can't create a FIFO on the Host.
- **Logs**: containerd keeps none. nerdctl writes JSON lines under its data root (its `nerdctl/state-dir` label points there); Kubernetes' CRI containers (`k8s.io`) log in the CRI format under `/var/log/pods/`, the path held in the container's CRI metadata extension; containers made with `ctr` have no output kept anywhere.

## Decision

Options 1 and 2 below together: **containerd's gRPC API for everything it covers, and commands on the Host's machine for logs and exec, where Homeport can run them.**

- **Client**: Homeport's own crate, `homeport-containerd`, with tonic and prost code generated from the containerd API's `.proto` files, vendored in the crate. protoc comes from `protoc-bin-vendored` at build time, so building Homeport needs nothing installed. The `containerd-client` crate isn't used: its build script needs `protoc` on every machine that builds Homeport. HTTP/2 travels over the Host's `Dialer` (#74), so local sockets and SSH reach containerd the same way.
- **Connections**: local sockets and SSH only. TCP and TLS are refused when the Host is saved, since containerd doesn't serve its API over the network.
- **Minimum version**: **containerd 1.7** to connect, the oldest still supported upstream; **2.0** to pull, where Transfer is stable. Pulling on 1.7 is refused with `ContainerdTooOld`, which names the version needed. Below 1.7 the Host doesn't connect and says why with the same code.
- **Namespace**: chosen per Host on this computer and remembered with the selected Host. `default` first, or the only namespace holding something when `default` is empty.
- **Pull path**: Transfer, from an OCI registry to the image store, unpacked for the Host's platform into its default snapshotter. Registry credentials come from Homeport's registry logins (v0.7.0), answered on the auth stream for the registry asked about, in memory only (ADR 0002).
- **Logs** come from the file the creating tool wrote, read with `tail` (`tail -n <lines> [-F] -- <file>`), parsed as nerdctl's JSON lines or CRI lines. A container whose output isn't kept says so instead of showing an empty viewer.
- **Exec** runs a short POSIX script that makes a private folder with `mktemp -d`, two FIFOs in it with `mkfifo -m 600`, prints the folder, then copies the session's input into one and the other's output back with `cat`, and removes the folder on exit. Homeport then asks containerd, through gRPC, to start the process with those FIFOs and a terminal. Closing the terminal kills and deletes the process; the script removes the FIFOs.
- **Where the commands run**:
  - on an **SSH Host**, on a session channel of the SSH session Homeport already holds, as ADR 0019 does for `systemctl`;
  - on a **local socket** on Linux, on this computer, since the containerd is this computer's;
  - **nowhere else**: logs and exec explain that they need a local or SSH connection (`ContainerdNeedsShell`).
- **Only these commands, quoted, never `sudo`.** They run with the SSH user's rights (or this computer's user), so a log file the user can't read fails with the reason the shell gave. They're listed in the security page and the containerd page. Adding one is a decision to record here.
- **Stacks** aren't deployed to containerd Hosts in 1.0; nerdctl compose stays in **Later**.

## Considered Options

1. **gRPC only**: everything through the API, pulls through Transfer (containerd 2.0 and later), and no exec or logs on remote Hosts, both possible on local Hosts where Homeport can create FIFOs and read files itself. Simple, but exec and logs are the first things users open, and most containerd Hosts are servers reached over SSH.
2. **gRPC plus commands over SSH** (chosen, with 1): as 1, plus exec and logs on SSH Hosts by running commands on the session (`mkfifo` and `cat`, `tail -F`), the way ADR 0019 runs `systemctl`. Local Linux Hosts run the same commands on this computer, so there's one code path.
3. **Client-side pull** for containerd 1.7: resolve, fetch and unpack in Rust, writing to the content store and asking the snapshotter to apply each layer. The widest support, but large, fragile and a second implementation of what containerd 2.0 does itself.

## Consequences

- A containerd Host is reached through the same `Dialer` as every other Host; the supervisor watches `Events.Subscribe` and pings with `Version`.
- containerd 1.7 Hosts can do everything but pull. The containerd page has a table of what each version and kind of connection allows.
- Homeport now runs `tail`, `mktemp`, `mkfifo`, `cat` and `rm` on SSH Hosts, for logs and exec, beside `systemctl` (ADR 0019) and the socket probe; the security page lists them all.
- No file is written on a Host except the exec FIFOs, in a private temporary folder removed when the session closes.
- Building Homeport needs no `protoc`: the vendored one runs at build time. Updating the API means replacing the `.proto` files in `crates/containerd/proto/`.
