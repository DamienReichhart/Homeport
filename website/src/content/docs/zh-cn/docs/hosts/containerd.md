---
title: containerd
description: 连接本机上或通过 SSH 访问的 containerd，选择命名空间，运行其容器的任务，阅读日志，打开 shell，并管理镜像和快照。
---

**containerd** 运行在 k3s、nerdctl、BuildKit 以及许多没有 Docker 的服务器之下。Homeport 通过它自己的 gRPC API 直接连接它，这是 [ADR 0020](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0020-containerd-over-grpc.md) 的决定。containerd 主机在侧边栏中有自己的页面：**容器**、**镜像**、**快照**和**命名空间**。Docker 的页面、堆栈和看板在其上不可用。

## 添加 containerd 主机

在表单的**引擎**字段中选择 **containerd**。它只能通过套接字访问 containerd，因此连接方式为**本地套接字**或 **SSH**；TCP 和 TLS 显示为灰色，因为 containerd 不会通过网络提供其 API。

| containerd                      | 套接字                                                          |
| ------------------------------- | --------------------------------------------------------------- |
| containerd 自身的服务           | `/run/containerd/containerd.sock`                               |
| k3s                             | `/run/k3s/containerd/containerd.sock`                           |
| Docker Engine 自带的 containerd | `/run/containerd/containerd.sock`，其容器位于 `moby` 命名空间中 |

在 Linux 上，当本机的 containerd 和 k3s 的 containerd 有响应时，**在本机上找到**会列出它们。你暂时还不能使用的套接字会连同修复方法一起列出。

### 让你的用户使用套接字

containerd 的套接字属于 root。要以其他用户身份（在本地或通过 SSH）使用它，请在 `/etc/containerd/config.toml` 中为某个组授予访问权限，并将该用户加入该组：

```bash
sudo groupadd -f containerd
sudo usermod -aG containerd "$USER"
getent group containerd | cut -d: -f3
```

将输出的数字填入 `/etc/containerd/config.toml` 的 `[grpc]` 下，重启 containerd，然后重新登录：

```toml
[grpc]
  gid = 1001
```

```bash
sudo systemctl restart containerd
```

**Rootless containerd**（`containerd-rootless.sh`）将其套接字保存在 RootlessKit 的命名空间内，外部的任何东西都无法访问，Homeport 也不例外。Homeport 发现它时会说明这一点；请改用 containerd 的系统服务。

### 各版本和连接方式支持的功能

Homeport 可连接 containerd **1.7** 及更高版本。拉取镜像通过 containerd 的 Transfer 服务进行，该服务从 **2.0** 起稳定。

| 操作                             | containerd     | 连接                         |
| -------------------------------- | -------------- | ---------------------------- |
| 命名空间、容器和任务、镜像、快照 | 1.7 及更高版本 | 本地套接字或 SSH             |
| 拉取镜像                         | 2.0 及更高版本 | 本地套接字或 SSH             |
| 日志和终端                       | 1.7 及更高版本 | SSH，或 Linux 上的本地套接字 |

containerd 版本过旧而无法连接时，主机会显示**配置错误**，并写明其版本；版本过旧而无法拉取时，会在你拉取时说明。

## 命名空间

每个 containerd 对象都位于一个**命名空间**中：nerdctl 使用 `default`，Kubernetes 使用 `k8s.io`，Docker 使用 `moby`，BuildKit 使用 `buildkit`。页面一次显示一个命名空间，在主机切换器下方选择，那里会列出每个命名空间及其包含的容器和镜像数量。Homeport 会在每次启动之间记住每台主机的选择。在你选择之前，主机显示 `default`；如果 `default` 为空，则显示唯一包含内容的命名空间，因此 k3s 主机会打开 `k8s.io`。

**命名空间**页面可以创建空命名空间，也可以删除空的命名空间。containerd 拒绝删除仍包含任何内容的命名空间，Homeport 会说明其中还有什么：请先删除其容器、镜像和快照。

## 容器和任务

在 containerd 中，**容器**只是元数据：它的镜像、规范和快照。真正运行的是它的**任务**。**容器**页面列出命名空间中的容器及其任务：**运行中**、**已暂停**、**已停止**（附退出码）或**无任务**。每个容器都会说明是哪个工具创建的：**Kubernetes**（附其 Pod）、**nerdctl**、**Docker** 或 **ctr 或其他**；Pod 的 pause 容器默认隐藏，除非你要求显示。

| 操作         | Homeport 的行为                                                                                                                                                                                |
| ------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **启动**     | 在容器的快照上创建任务，日志写到 nerdctl 写日志的位置，然后启动它。已停止的任务会被替换，这是 `ctr task start` 的要求。                                                                        |
| **停止**     | 发送镜像的停止信号（除非镜像指定了其他信号，否则为 SIGTERM），10 秒后发送 SIGKILL。之后仍存活的任务会被报告。                                                                                  |
| **暂停**     | 冻结任务；**恢复**会解冻它。                                                                                                                                                                   |
| **发送信号** | 向任务的主进程发送任意信号，可按名称（`SIGHUP`）或编号指定。                                                                                                                                   |
| **删除**     | 先询问确认；强制终止正在运行的任务，删除任务和容器，以及其快照和它更改的文件，除非你取消勾选**同时删除**：保留的快照会被标记为 `containerd.io/gc.root`，因此 containerd 的垃圾回收器会保留它。 |

容器页面会显示其运行时、快照、标签和 OCI 规范。Homeport 不会在 containerd 上创建容器：containerd 本身没有网络功能，网络由 nerdctl 和 CNI 添加。

Kubernetes 的容器由 **kubelet** 管理：停止后它会重新启动它们，删除后它会重新创建它们。它们的页面会说明这一点；请改为在其集群上修改 Pod。从这里删除 nerdctl 容器会留下 nerdctl 自己对它的记录（其名称和网络），与 `ctr` 的行为相同；对于这类容器，`nerdctl rm` 是更干净的方式。

## 日志

containerd 不保存日志：启动任务的工具决定其输出去向。**日志**标签页会从该工具写入的位置读取：

| 创建者                 | 输出去向                                                       | Homeport 显示的内容                    |
| ---------------------- | -------------------------------------------------------------- | -------------------------------------- |
| nerdctl                | nerdctl 状态文件夹中的 JSON 日志文件，由其 `nerdctl/` 标签命名 | 该文件，持续跟踪                       |
| Kubernetes（`k8s.io`） | `/var/log/pods/<pod>/<container>/N.log`，由 CRI 的记录指明     | 该文件，持续跟踪                       |
| `ctr` 和其他客户端     | 它们指定的任何位置，通常没有                                   | **此容器的输出未被记录。**，并说明原因 |

使用其他驱动（journald、fluentd）记录日志的 nerdctl 容器会改为写明该驱动。Homeport 在主机所在的机器上用 `tail` 读取文件（见[下文](#在主机上运行的内容)）；日志文件通常属于 root，因此 SSH 用户必须有权读取它们。

## 容器中的终端

**Exec** 标签页会在正在运行的容器中打开一个 shell（镜像有 bash 时用 bash，否则用 sh），或运行你指定的命令，以数字用户（`uid` 或 `uid:gid`）身份并在你选择的文件夹中运行。**重新打开**会使用你输入的内容启动一个新的终端。

containerd 以其所在机器上的路径接收进程的输入和输出。因此 Homeport 会在那里运行一个简短脚本，在私有临时文件夹中创建两个 FIFO，并通过它们传递终端数据；containerd 会根据容器自身的进程规范，在这些 FIFO 上启动进程。关闭标签页会强制终止进程并删除该文件夹。

## 镜像

**镜像**页面列出命名空间中的镜像及其平台、由它们创建的容器，以及按 `ctr images ls` 方式计算的、针对主机平台的大小。

**拉取镜像**通过 containerd 本身获取镜像，然后针对主机的平台将其解包到默认 snapshotter 中，因此 `nerdctl run` 可以立即使用它。可以提供完整引用（`ghcr.io/home/app:1.2`）或简短引用（`nginx`，会变成 `docker.io/library/nginx:latest`）。私有镜像仓库中的镜像会使用**设置 → 镜像仓库**中的登录信息拉取，只有在镜像仓库要求时才发送给 containerd，并且从不保存在主机上。

命名空间中有容器基于它创建的镜像无法删除；请先删除这些容器。一旦没有任何东西持有某个镜像的内容，containerd 就会释放它。

## 快照

快照是 containerd 磁盘空间的去处：已解包镜像的各层，以及每个容器的可写层。**快照**页面显示一个 snapshotter 的快照（优先为 `overlayfs`，或主机加载的任何其他 snapshotter），以按父级组织的树形呈现，最大的排在最前，并按 `ctr snapshot usage` 的方式显示其用量，总量显示在顶部。

每个快照都会说明谁在使用它：容器（通过其快照）或镜像（通过其层）。没有被使用、也没有被作为基础的快照（例如在这里删除容器时保留的快照）可以通过**仅未使用**单独显示，并可删除。其他快照无法从这里删除。没有被任何东西引用、也没有被标记为根的快照（例如 `ctr containers rm --keep-snapshot` 留下的快照）不会长久存在：containerd 的垃圾回收器会在下次运行时删除它。

## 在主机上运行的内容

上述所有操作都通过 containerd 的 API 进行，只有日志和终端除外，它们需要 containerd 所在机器上的文件。为此，Homeport 会通过已建立的 SSH 会话（或者对于 Linux 上的本地套接字，在本机上）运行以下命令，使用 SSH 用户的权限，从不使用 `sudo`：

| 用途 | 命令                                                                                              |
| ---- | ------------------------------------------------------------------------------------------------- |
| 日志 | `tail -n <lines> [-F] -- '<log file>'`；对于没有 CRI 记录的 Kubernetes 容器，先 `ls` 其日志文件夹 |
| 终端 | `mktemp -d`，在该文件夹中用 `mkfifo -m 600` 创建两个 FIFO，通过它们 `cat`，最后 `rm -rf` 该文件夹 |

macOS 或 Windows 上的本地套接字无法运行这些命令，此时日志和终端会说明需要 SSH。[安全页面](/docs/reference/security/#homeport-在你的主机上运行的内容)列出了 Homeport 在主机上运行的每条命令。

## 事件

页面会根据 containerd 在所有命名空间中的自身事件刷新：任务启动或退出、容器或镜像被创建或删除、快照被提交。containerd 不保存事件历史，因此 Homeport 关闭期间发生的事情不会被补上。
