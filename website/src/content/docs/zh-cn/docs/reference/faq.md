---
title: 常见问题
description: 关于 Homeport 的常见问题。
---

### Homeport 免费吗？

是的。Homeport 可免费用于个人和商业用途，并以 [GNU Affero 通用公共许可证第 3 版](https://github.com/DamienReichhart/Homeport/blob/main/LICENSE)（AGPL）开源。

### AGPL 对我意味着什么？

如果你只是使用 Homeport，什么都不会改变：安装它，在家里或工作中运行它，想装在多少台计算机上都可以。AGPL 只对**分发** Homeport（无论是否修改）或通过网络向他人提供修改版本的人提出要求：他们必须以相同的许可证公开其完整源代码。每个 Homeport 副本都会在**设置 → 关于 → 许可证**中链接到其确切版本的源代码。

0.11 及之前的版本以 MIT 许可证发布，并继续以该许可证提供。

### Homeport 能取代 Docker Desktop 吗？

不完全能。Homeport 是一个管理器：它本身不运行 Docker Engine。在 Linux 上，安装 Docker Engine，并把 Homeport 用作它的界面。在 macOS 和 Windows 上，你仍然需要一个运行引擎的东西（Docker Desktop、Colima、OrbStack、Rancher Desktop 或 Podman），Homeport 可以搭配其中任何一个使用，同时管理你的远程服务器。

### 它支持 containerd 吗？

支持，直接支持，无需 Docker：主机的引擎可以是 [containerd](/docs/hosts/containerd/)，通过本机上的套接字或通过 SSH 访问，可以是 k3s 的、Docker Engine 自带的，或服务器上的 containerd 服务。Homeport 会列出其命名空间、容器及其任务（启动、停止、暂停、发送信号、删除）、nerdctl 或 Kubernetes 写入的日志、容器中的终端、镜像（拉取、删除）以及快照。它不会在其上创建容器或部署堆栈；这些由 nerdctl 完成。

### 它支持 Incus 吗？

支持：主机的引擎可以是 [Incus](/docs/hosts/incus/) 6.0 或更高版本，通过本机上的套接字、通过 SSH，或通过带有 `incus config trust add` 生成的信任令牌的 HTTPS 访问。Homeport 会列出某个项目的[实例](/docs/guides/instances/)（系统容器和虚拟机），启动、停止、冻结、删除和创建它们，打开其控制台以及其中的终端，创建和恢复快照，并管理镜像、配置文件、存储池和网络。它不会在其上部署堆栈，也不会在主机上安装任何东西。

### 它支持 LXD 吗？

不直接支持：Homeport 使用的是 Incus（LXD 的社区分支）的 API，并会拒绝 LXD 服务器，因为其 API 已经有所偏离。请使用 `lxd-to-incus` 迁移，然后将其添加为 Incus 主机。

### 它支持 Podman 吗？

支持，通过 Podman 的 Docker 兼容 API 套接字。Homeport 会检测 Linux 上常见的 Podman 套接字、每个正在运行的 [macOS 上的 Podman machine](/docs/hosts/local/#podman-machines-on-macos)，以及 Windows 上默认 Podman machine 的管道。其他任何套接字都可以[手动添加](/docs/hosts/local/#adding-a-socket-by-hand)。

Homeport 在连接时还会识别 Podman，然后通过同一套接字上 Podman 自身的 API，提供只有 Podman 才有的功能：[Pod](/docs/guides/podman-pods/)，你可以列出、启动、停止、创建和删除它们，并通过“运行”对话框在其中运行容器；以及 [Quadlet 单元](/docs/guides/quadlet/)，你可以列出、阅读、安装、编辑和删除它们，并通过 SSH 启动或停止它们。

### 它支持 Kubernetes、Nomad 或 Docker Swarm 吗？

Docker Swarm：支持。只要其中一个管理节点是主机，Homeport 就会通过管理节点主机发现 Swarm，并全面管理它：节点、服务、任务、机密、配置和 swarm stack，包括部署到其上的托管、关联和同步堆栈。它还可以在主机上创建 Swarm，以及加入或离开 Swarm。一切都通过已连接的管理节点的 Engine API 进行；不会在节点上安装任何东西。参见 [Swarm](/docs/guides/swarm/)。

Kubernetes：支持。从你的 kubeconfig（本机的或你粘贴的）添加集群，Homeport 还会为你找到本地集群（Docker Desktop、OrbStack、Rancher Desktop、kind、k3d、minikube）。EKS、GKE 和 AKS 通过其 kubeconfig 指定的 exec 插件工作。你可以实时浏览 Namespace、工作负载、Pod、Service、Ingress、ConfigMap、Secret、Node、事件以及任何其他 API 资源（包括自定义资源）；跟踪日志、在 Pod 中打开终端、转发端口、扩缩容、重启和删除，以及在预览后应用或编辑 YAML。不会在集群上安装任何东西：Homeport 只与你的 kubeconfig 已能访问的 Kubernetes API 通信，并且只拥有其凭据的权限。如果本机上有 `helm` CLI，Homeport 还可以列出、安装、升级、回滚和卸载 [Helm Release](/docs/clusters/helm/)，并管理 helm 的 Chart 仓库。参见[集群](/docs/clusters/)。

Nomad：支持。通过其 agent 的地址添加集群，如果集群使用 ACL 令牌和 TLS 材料，也一并提供。你可以实时列出 job 及其 allocation；在查看 Nomad 的计划后从 HCL 或 JSON 运行 job；编辑并重新运行 job，停止或清除它，以及扩缩其 task group；跟踪 task 的日志并在其中打开 shell；腾空节点并切换其调度资格；以及选择 namespace。不会在集群上安装任何东西，也不需要 `nomad` CLI：Homeport 以令牌的权限与 Nomad 的 HTTP API 通信。参见 [Nomad 集群](/docs/clusters/nomad/)。

### 我需要 kubectl 吗？

不需要。Homeport 读取你的 kubeconfig，并自己与 Kubernetes API 通信。它唯一会运行的程序是 kubeconfig 指定的 [exec 插件](/docs/clusters/exec-plugins/)，例如 EKS 的 AWS CLI、GKE 的 `gke-gcloud-auth-plugin` 或 AKS 的 `kubelogin`，也就是 `kubectl` 同样会运行的那些程序。不过，手边有 `kubectl` 仍有助于检查你的凭据能做什么（`kubectl auth can-i`）。

### 我需要在服务器上安装任何东西吗？

不需要，只需要 Docker Engine 和 SSH 服务器，你很可能已经有了。Homeport 不安装 agent。

Swarm 同样只需要引擎。不过，其节点之间必须能互相访问，这是 Homeport 无法替你设置的：请在节点之间开放 2377/tcp（到管理节点）、7946/tcp 和 7946/udp，以及 4789/udp。参见[节点之间的端口](/docs/guides/swarm/#ports-between-nodes)。

### 要管理远程服务器，我的计算机上需要安装 Docker 吗？

不需要。Homeport 自己实现了 Engine API 和 SSH。唯一的例外是部署托管、关联和同步堆栈，这需要使用你计算机上的 `docker compose` 命令；将它们部署到 Swarm 时则使用 `docker` 命令（`docker stack deploy`）。两者都不需要你的计算机上有引擎。

### Homeport 关闭时会保持我的堆栈同步吗？

不会。只有在 Homeport 打开时，才会检查[同步堆栈](/docs/guides/synced-stacks/)是否有新提交，并在你开启自动部署时自动部署。Homeport 不会在你的服务器上安装任何东西（没有 agent，也没有 webhook 接收器），因此它关闭时，没有任何东西在监视你的仓库。再次打开时，它会等待一个间隔，然后检查所有仓库。如果需要全天候跟随每次推送进行部署，请从你的 CI 中运行部署。

要让 Homeport 在没有窗口的情况下保持运行，请在[设置 → 桌面](/docs/guides/settings/#desktop)中打开**关闭时最小化到系统托盘**，以及**登录时启动**和**启动时隐藏到托盘**。

### Homeport 能在容器崩溃时通知我吗？

能，只要它在运行，即使隐藏在系统托盘中也可以。当容器因错误停止、内存耗尽或变为不健康，以及主机变为无法访问或恢复时，它会显示一条系统通知。每种通知都可以在[设置 → 通知](/docs/guides/settings/#notifications)中关闭。这些信息来自它与每台主机已建立的事件流，因此不会在你的服务器上安装任何东西；Homeport 没有运行时，不会通知任何人。

### Homeport 需要 git 吗？

只有[同步堆栈](/docs/guides/synced-stacks/)需要，它们会使用你计算机上安装的 `git`，并使用每个仓库自己的部署密钥或令牌。**设置 → 本机**会显示 Homeport 是否找到了它。你自己的 git 配置、凭据助手和 SSH 密钥从不会被使用。

### 多人可以共享 Homeport 的主机吗？

每个人运行自己的 Homeport，使用自己的主机和密钥。如果每个人在每台服务器上都使用 Homeport 创建的密钥，你就可以在 `authorized_keys` 中查看并撤销每个人的访问权限。

### 我可以把 Homeport 迁移到另一台计算机吗？

可以，有两种方式：

- **全部**：**设置 → 数据 → 备份…**会将每台主机、每个集群、堆栈、镜像仓库登录信息、仓库和设置，连同它们的机密，保存到一个由至少 12 个字符的口令保护的文件中。在新计算机上，同一位置的**恢复…**会打开该文件，列出其中的内容，并替换现有内容。一切都能连接和部署，无需再次向你索要密钥或密码。参见[备份与恢复](/docs/guides/settings/#backing-up-and-restoring)。
- **部分主机**：在**主机**页面上**导出**你选择的主机，然后在另一台计算机上**导入**它们，与那里已有的主机并存。参见[迁移到另一台计算机](/docs/reference/security/#moving-to-another-computer)。

复制 `homeport.db` 对机密无效：它们使用保存在这台计算机钥匙串中的密钥加密，在其他地方会显示为**凭据丢失**。

### 备份包含我的容器和卷吗？

不包含。备份保存的是 Homeport 自己保存的内容（如何访问你的主机、你的堆栈的定义和环境变量、登录信息和设置），而不是主机上运行的内容。你的容器、镜像和卷保留在主机上。要复制卷的数据，请在“卷”页面[将其备份到文件](/docs/guides/volumes-networks/#backing-up-and-restoring)。

### Homeport 能构建镜像吗？

能，在两个地方。**镜像**页面可以从本机上的文件夹构建镜像。[关联堆栈](/docs/guides/stacks/#linked-stacks)（compose 文件位于本机某个文件夹中的堆栈）可以使用 `build:`：Homeport 在该文件夹中运行 `docker compose`，因此构建上下文会从那里发送到主机的引擎，由引擎进行构建。代码变更后，**重新构建并部署**会重新构建。托管堆栈无法构建，因为 Homeport 只保存其 YAML，没有可用于构建的文件夹：请改为关联该文件夹，或将镜像推送到镜像仓库。

### 为什么我的堆栈在另一台计算机上显示为外部堆栈？

在某个文件夹中用 `docker compose up` 启动的堆栈，只有在该文件夹及其 compose 文件存在于相同路径、并且定义了正在运行的服务的计算机上，才会显示为[关联堆栈](/docs/guides/stacks/#how-linked-stacks-are-found)。在另一台计算机上该路径不存在，因此 Homeport 没有它的 compose 文件，会将其显示为外部堆栈。如果也想在那里管理它，请将项目克隆到那台计算机，并通过**新建堆栈 → 使用本机上的文件夹**关联它，或将其接管为托管堆栈。

### Homeport 会收集关于我的数据吗？

不会。没有账户、遥测或分析。Homeport 唯一会主动从你的计算机发出的请求是更新检查，它会从本网站下载一个小文件，不会发送任何关于你的信息；你可以在**设置 → 更新**中关闭它。另外，新版镜像检查会通过每台主机的引擎询问你的镜像所在的镜像仓库，其标签指向哪个版本；对于本地主机，该请求也是从你的计算机发出的。参见[安全与你的数据](/docs/reference/security/#what-homeport-connects-to)。

### Homeport 能使用我的私有镜像仓库吗？

能。在**设置 → 镜像仓库**中为每个镜像仓库添加登录信息：Docker Hub、GHCR 或你自己的镜像仓库。Homeport 会将其交给引擎，用于在任意主机上拉取、推送和构建，并在部署前用它拉取托管、关联或同步堆栈的镜像。密码和令牌使用你的系统钥匙串加密。Homeport 不会读取本机上 `docker login` 的登录信息。

### Homeport 如何知道有更新的镜像？

它通过主机的引擎询问镜像所在的镜像仓库，该标签当前指向哪个摘要，并与主机拉取的摘要进行比较。不会下载任何东西，但 Docker Hub 会将每次询问计入其拉取速率限制，就像一次没有发现新内容的 `docker pull`。因此 Homeport 会将每个答案保留 6 小时，最多同时询问四个镜像仓库，只有在你选择**检查新版镜像**时才会提前再次询问。在**设置 → 镜像仓库**中添加 Docker Hub 登录信息可以提高该限制。如果某个镜像仓库无法访问或正在限速，Homeport 不会为其镜像显示标记，并会在日志中写入一行。

### 如何更新？

在 Windows、macOS 上以及使用 AppImage 时，Homeport 会在有新版本时通知你：选择**安装并重启**。在 Linux 上，`.deb` 和 `.rpm` 软件包会随系统其余部分一起从 Homeport 软件包仓库更新。参见[更新](/docs/reference/updating/)。

### Homeport 在 Homebrew、winget、Flathub 或 AUR 上有吗？

还没有。它们已在计划中；[安装页面](/docs/installation/#coming-later)链接到了各自的 issue。

### Homeport 支持我的语言吗？

Homeport 支持英语、法语、西班牙语、德语、意大利语、巴西葡萄牙语、俄语、简体中文、日语、韩语、印地语和阿拉伯语，并会跟随操作系统的语言，除非你在[设置 → 外观](/docs/guides/settings/#appearance)中选择了其他语言。网站也提供所有这些语言；文档已完整翻译为法语，其他语言在尚无翻译的地方显示英文页面。要添加语言，参见[参与贡献](/docs/project/contributing/#translating-homeport)。

### Homeport 支持无障碍访问吗？

它可以仅用键盘使用，会遵循系统的减弱动态效果设置，满足 WCAG 2.2 AA 对比度标准，并且每个页面都经过自动化无障碍工具检查。目前正在进行屏幕阅读器的实际审查。关于哪些功能可用、已知局限以及如何报告障碍，参见[键盘与无障碍](/docs/reference/accessibility/)。

### 有命令行界面吗？

没有，Homeport 是一个桌面应用。你现有的 `docker` CLI 可以与它并用，当你在终端中更改内容时，Homeport 会实时更新。

### 我能如何帮忙？

在 [GitHub issues](https://github.com/DamienReichhart/Homeport/issues) 中报告 bug 和建议功能，或参见[参与贡献](/docs/project/contributing/)。
