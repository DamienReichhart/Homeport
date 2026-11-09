---
title: 术语表
description: Homeport 使用的词语及其含义。
---

Homeport 对事物的命名很精确。这些术语贯穿整个应用和本文档。每个条目先列出产品的英文术语，再列出简体中文应用中使用的词。

### Host（主机）

Homeport 连接的 Docker Engine，由一个名称和访问方式定义：本地套接字、TCP、TLS 或 SSH。每个容器、镜像、卷、网络和堆栈都只属于一台主机。参见[主机与连接](/docs/hosts/)。

### Engine kind（引擎类型）

主机所指向的引擎的类型：Docker 兼容（Docker Engine，或通过其兼容 API 的 Podman）、containerd 或 Incus。它决定主机显示哪些页面：containerd 主机和 Incus 主机有各自的页面。Podman 不是单独的类型：Homeport 每次连接时都会识别它。参见[引擎类型](/docs/hosts/#engine-kinds)。

### Namespace (containerd's)（containerd 命名空间）

在 containerd 主机上，每个容器、镜像和快照所在的空间：nerdctl 使用 `default`，Kubernetes 使用 `k8s.io`，Docker 使用 `moby`。containerd 主机的页面一次显示一个命名空间。集群的 Namespace 是 Kubernetes 的 Namespace。参见 [containerd 命名空间](/docs/hosts/containerd/#namespaces)。

### Snapshot（快照）

在 containerd 主机上，磁盘上的一层文件：已解包的镜像层，或容器的可写层。参见[快照](/docs/hosts/containerd/#snapshots)。在 Incus 主机上，实例的一个已保存状态，实例可以恢复到该状态；参见[实例快照](/docs/guides/instances/#snapshots)。

### Instance（实例）

Incus 主机运行的东西：在主机内核上启动完整 Linux 发行版的系统容器，或拥有自己内核的虚拟机。它不是容器，容器只运行一个应用。参见[实例](/docs/guides/instances/)。

### Project (Incus's)（项目）

在 Incus 主机上，其实例、镜像、配置文件、自定义卷和网络所在的空间。Incus 主机的页面一次显示一个项目，在主机切换器下方选择。参见[项目](/docs/hosts/incus/#projects)。

### Profile（配置文件）

在 Incus 主机上，由列出它的实例共享的配置和设备。参见[配置文件](/docs/guides/instances/#profiles)。

### Trust token（信任令牌）

在 Incus 服务器上由 `incus config trust add` 输出的内容，粘贴到 Homeport 中一次，即可通过 HTTPS 添加该服务器。参见[信任令牌](/docs/hosts/incus/#over-https-with-a-trust-token)。

### Pod（Pod）

在 Podman 主机上，一组共享同一网络、一起启动和停止的容器，围绕一个持有该 Pod 所发布端口的 infra 容器。集群的 Pod 是 Kubernetes 的 Pod。参见 [Podman Pod](/docs/guides/podman-pods/)。

### Quadlet unit（Quadlet 单元）

Podman 主机上的一个文件，例如 `web.container`，systemd 会将它转换为一个服务 `web.service`，从而让 Podman 的容器在开机时启动，并由 systemd 保持运行。参见 [Quadlet 单元](/docs/guides/quadlet/)。

### Cluster（集群）

Homeport 通过其 API 连接的 Kubernetes 集群或 Nomad 集群。Kubernetes 集群从 kubeconfig 的某个上下文添加：一个名称、API 服务器的地址，以及 kubeconfig 提供的凭据，加密保存。Nomad 集群通过其某个 agent 的地址添加，附带 ACL 令牌和 TLS 材料，加密保存。Homeport 不会在任何一种集群上安装任何东西，并且只能访问这些凭据所允许的内容。它不是 Swarm，Swarm 是通过其主机发现的，从不单独添加。Kubernetes 自身的对象保留 Kubernetes 的名称：Namespace、Pod、Deployment、Service、Node…Nomad 的对象保留 Nomad 的名称：job、task group、task、allocation、client node、namespace。参见[集群](/docs/clusters/)和 [Nomad 集群](/docs/clusters/nomad/)。

### Allocation（allocation）

在 Nomad 集群上，job 的某个 task group 在某个 client node 上的一次放置，运行其 task。其日志以及其中的 shell 会经过 Nomad 的服务器到达该节点。参见[日志和 shell](/docs/clusters/nomad/#logs-and-shell)。

### Swarm（Swarm）

一个 Docker Swarm 集群，通过属于它的主机发现，从不单独添加。Homeport 通过任何一台已连接的管理节点主机访问它，按 swarm ID 区分不同的 Swarm，并且不保存关于 Swarm 的任何信息。由于 Docker 给每个 Swarm 起的名称都相同，它以其第一台管理节点主机命名。参见 [Swarm](/docs/guides/swarm/)。

### Port forward（端口转发）

本机回环地址上的一个端口，Homeport 通过 Kubernetes API 将它连接到某个 Pod 的端口，或通过某个 Service 的一个就绪 Pod 连接到该 Service 的端口，与 `kubectl port-forward` 相同。Homeport 退出时它就会结束。参见[端口转发](/docs/clusters/port-forward/)。

### Release（Release）

一个 Helm release：chart 在集群上的一次安装，在其 Namespace 中命名，每次安装、升级和回滚都有一个编号的修订版本。Helm 将它保存在集群上；在终端中创建的 release 和在 Homeport 中创建的 release 是一样的。参见 [Helm Release](/docs/clusters/helm/)。

### Chart repository（Chart 仓库）

helm 从中下载 chart 的服务器，位于本机上 helm 自己的列表中，即 `helm repo list` 显示的列表。OCI 镜像仓库中的 chart 则通过其 `oci://` 引用安装。参见 [Chart 仓库](/docs/clusters/helm/#chart-repositories)。

### Node（节点）

Swarm 中的一个引擎：管理节点或工作节点。集群的 Node 是 Kubernetes 的 Node，即运行 Pod 的机器。只有当 Homeport 连接到某个节点时，它才是一台主机，主机页面会链接到其节点。

### Service（服务）

Swarm 运行的东西：一个镜像、其设置和一定数量的副本；对于全局服务，则是每个节点上一个任务。

### Task（任务）

服务在某个节点上的一个槽位，在该节点上作为容器运行。当服务无法收敛时，它的任务会说明原因。在 containerd 主机上，任务是为容器运行的东西，而容器只是元数据：启动容器会创建其任务。

### Stack（堆栈）

在主机上运行的 Compose 项目，或在 Swarm 上运行的 swarm stack，由其项目名称标识。每个堆栈都是托管、关联、同步或外部堆栈之一。参见[堆栈](/docs/guides/stacks/)和 [Swarm 堆栈](/docs/guides/swarm/#swarm-stacks)。

### Managed Stack（托管堆栈）

compose 定义和环境变量由 Homeport 保存的堆栈，因此可以在 Homeport 中部署、重新部署和编辑。

### Linked Stack（关联堆栈）

compose 文件位于本机某个文件夹中的堆栈。Homeport 记录该文件夹及其 compose 文件的位置，在原处读写这些文件，并以该文件夹作为项目目录进行部署，因此 `build:`、`env_file:` 和相对路径都能正常工作。可以从正在运行的容器的 compose 标签中发现，也可以从文件夹创建。其文件缺失期间，它会变为外部堆栈。参见[关联堆栈](/docs/guides/stacks/#linked-stacks)。

### Synced Stack（同步堆栈）

compose 定义来自 git 仓库的堆栈。Homeport 在本机上保存它的一个克隆，在 Homeport 打开期间按需或按间隔获取某个分支或标签，显示变更内容，并从 compose 文件所在的文件夹部署某个提交，因此 `build:` 和相对路径都能正常工作。它从不在 Homeport 中编辑：其文件通过 git 变更；只有其环境变量和同步设置属于 Homeport。参见[同步堆栈](/docs/guides/synced-stacks/)。

### Repository（仓库）

Homeport 可以为同步堆栈读取的 git 仓库：其 URL 以及 Homeport 向它认证的方式（Homeport 生成的部署密钥、用户名和令牌，或者对于公开仓库则无需认证）。对所有主机只保存一次，并由基于它创建的每个同步堆栈共享。

### Deploy key（部署密钥）

Homeport 为某个仓库生成的 Ed25519 密钥，你需要在 git 托管平台上将它添加到该仓库，并且只授予读取权限。它从不是你自己的 SSH 密钥。

### External Stack（外部堆栈）

在主机上发现正在运行、但 Homeport 没有其 compose 定义的堆栈：既没有保存，也不在本机的某个文件夹中，也不在它关注的某个仓库中。它可以启动、停止和删除，但不能重新部署或编辑。

### Adopt（接管）

通过提供 compose 定义，将外部堆栈变为托管堆栈。

### Registry login（镜像仓库登录）

Homeport 为某个容器镜像仓库（例如 Docker Hub 或 GHCR）保存的用户名和密码或访问令牌，在每台主机上拉取、推送和构建时使用。参见[设置](/docs/guides/settings/#registries)。

### Helper container（辅助容器）

Homeport 在主机上创建的一个短期 `busybox` 容器，用于打开卷（浏览、备份、恢复或克隆卷），因为 Engine API 无法读取卷。它没有网络，除非必须写入，否则以只读方式挂载卷，并在操作结束时被删除。它从不会出现在容器列表或事件页面中。参见[浏览卷的文件](/docs/guides/volumes-networks/#the-helper-container)。

### Board（看板）

在一个表格中列出所有已连接主机的容器的页面，每个容器都标有其主机。参见[看板](/docs/guides/overview/#board)。

### Command palette（命令面板）

使用 <kbd>Mod</kbd> <kbd>K</kbd> 打开的搜索框：跳转到某个页面、切换主机、运行命令，或在任意已连接主机上查找容器、镜像、卷、网络或堆栈。参见[键盘](/docs/guides/keyboard/#command-palette)。

### Engine（引擎）

在机器上运行容器的 Docker Engine（`dockerd`），或与之兼容的引擎，例如 Podman。Homeport 通过 Docker Engine API 与它通信。

### Socket（套接字）

引擎监听的地址：Unix 套接字文件（例如 `/var/run/docker.sock`），或 Windows 上的命名管道（例如 `//./pipe/docker_engine`）。

### Keychain（系统钥匙串）

操作系统中用于保存密码和密钥的安全存储（macOS 钥匙串、Windows 凭据管理器或 Linux Secret Service），Homeport 在其中保存用于加密其机密的密钥。

### Trust on first use（首次使用即信任）

Homeport 检查 SSH 服务器的方式：首次连接时由你确认服务器的密钥指纹，之后如果密钥发生变化，Homeport 会拒绝连接，直到你再次确认。

## 中文术语

简体中文的应用、文档和网站都使用以下术语，因此界面和页面上的说法始终一致。

| 英文           | 简体中文 |
| -------------- | -------- |
| Host           | 主机     |
| Engine         | 引擎     |
| Instance       | 实例     |
| Swarm          | Swarm    |
| Cluster        | 集群     |
| Stack          | 堆栈     |
| Managed Stack  | 托管堆栈 |
| Linked Stack   | 关联堆栈 |
| Synced Stack   | 同步堆栈 |
| External Stack | 外部堆栈 |
| Adopt          | 接管     |
| Board          | 看板     |
| Bastion        | 堡垒机   |
| Deploy key     | 部署密钥 |
| Port forward   | 端口转发 |
| Release (Helm) | Release  |
