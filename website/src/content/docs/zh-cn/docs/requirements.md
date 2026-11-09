---
title: 系统要求
description: Homeport 运行所需的条件，以及启用 SSH、TLS、堆栈部署、Kubernetes 和 Nomad 集群以及 Helm 的可选组件。
---

## 操作系统

| 系统    | 版本                                                                 | 架构              |
| ------- | -------------------------------------------------------------------- | ----------------- |
| Windows | Windows 10 和 11                                                     | x64、ARM64        |
| macOS   | macOS 10.15 Catalina 或更高版本                                      | Apple 芯片、Intel |
| Linux   | glibc 2.35 或更高版本：Ubuntu 22.04、Debian 12、Fedora 36 及更新版本 | x64、ARM64        |

Homeport 使用系统的 Web 视图绘制窗口：Windows 上为 WebView2（Windows 10 和 11 自带），macOS 上为 WebKit，Linux 上为 WebKitGTK（作为 `.deb` 和 `.rpm` 软件包的依赖安装，AppImage 中已内置）。

## 一个要管理的 Docker Engine

Homeport 不包含容器运行时。它管理已经在某处运行的 Docker Engine：

- **在本机上**：Docker Engine、Docker Desktop、Colima、OrbStack、Rancher Desktop 或 Podman。Homeport 会自动找到它们；参见[本地引擎](/docs/hosts/local/)。
- **在服务器上**：通过 [SSH](/docs/hosts/ssh/) 或通过 [TLS 或 TCP](/docs/hosts/tls-tcp/) 访问。

任何支持 Docker Engine API 的引擎都可以使用，包括 Podman 的 Docker 兼容套接字。

它还能直接管理 [containerd](/docs/hosts/containerd/) 1.7 及更高版本，通过本机上的套接字或通过 SSH：k3s 的、Docker Engine 自带的，或服务器上的 containerd 服务。在其上拉取镜像需要 containerd 2.0 或更高版本；日志和终端需要通过 SSH 访问该主机，或使用本机上的 containerd（Linux）。

它还能管理 [Incus](/docs/hosts/incus/) 6.0 及更高版本，通过本机上的套接字、通过 SSH，或通过 HTTPS 配合信任令牌。虚拟机需要 Incus 服务器支持硬件虚拟化（`/dev/kvm`），在虚拟机中打开终端需要其镜像运行 `incus-agent`。

在 Podman 主机上，Homeport 还会使用 Podman 自身的 API：[Pod](/docs/guides/podman-pods/) 需要 Podman 4.0 或更高版本；[Quadlet 单元](/docs/guides/quadlet/)从 Podman 5.7 起可列出，从 5.8 起可安装、编辑和删除。启动和停止 Quadlet 单元需要通过 SSH 访问该主机，或使用本机上的 Podman（Linux）。

## 可选：Kubernetes 集群

要浏览[集群](/docs/clusters/)，Homeport 需要一个能访问它们的 kubeconfig（`kubectl` 使用的那个，或你粘贴的那个），以及到其 API 服务器的网络访问。Homeport 自己实现了 Kubernetes API 通信，因此不需要 `kubectl`。

- **Kubernetes 版本**：运行 Kubernetes 1.24 或更高版本的集群。Homeport 只使用稳定 API（`v1`、`apps/v1`、`batch/v1`、`networking.k8s.io/v1`），并基于 1.33 API 构建。
- **Exec 插件**：云服务的 kubeconfig 会指定一个在本机上运行以获取凭据的程序：EKS 用 [AWS CLI](https://aws.amazon.com/cli/)（第 2 版），GKE 用 Google Cloud CLI 中的 `gke-gcloud-auth-plugin`，AKS 用 `kubelogin`。安装你的 kubeconfig 指定的那个；参见 [exec 插件](/docs/clusters/exec-plugins/)。

## 可选：Nomad 集群

要管理 [Nomad 集群](/docs/clusters/nomad/)，Homeport 需要到某个 Nomad agent 的 HTTP API（默认端口 4646）的网络访问；如果集群启用了 ACL，还需要一个令牌，参见它所需的 [ACL 策略](/docs/clusters/nomad/#acl-policies)。Homeport 自己实现了 Nomad API 通信，因此不需要 `nomad` CLI，也不会在集群上安装任何东西。

- **Nomad 版本**：Nomad 1.6 或更高版本。**编辑并运行**从 job 提交时的规范开始，Nomad 从 1.6 起会保存该规范。

## 可选：系统钥匙串

Homeport 使用保存在操作系统钥匙串中的一个密钥，加密 SSH 密钥、TLS 证书、集群的 kubeconfig 凭据、堆栈环境变量、镜像仓库登录信息，以及仓库的部署密钥和令牌：

- **macOS**：钥匙串
- **Windows**：凭据管理器
- **Linux**：Secret Service 提供程序，例如 GNOME Keyring 或 KWallet

如果没有钥匙串，Homeport 会显示一条横幅，但仍可使用本地套接字、普通 TCP 以及通过 **ssh-agent** 的 SSH。TLS 主机、保存了密钥的 SSH 主机、集群、堆栈环境变量、镜像仓库登录信息和私有仓库都需要钥匙串；没有钥匙串时，镜像以匿名方式拉取，并且只能添加公开仓库。参见[安全与你的数据](/docs/reference/security/)。

## 可选：Docker Compose

要部署[托管堆栈、关联堆栈和同步堆栈](/docs/guides/stacks/)，Homeport 会在**本机**上（而不是在主机上）运行 `docker compose` 命令。请安装带 Compose 插件的 Docker CLI（Docker Desktop 已包含），或独立的 `docker-compose` v2。

没有它，堆栈（包括关联堆栈）仍会显示，并可启动、停止和删除；只有部署需要 Compose。**设置 → 本机**会显示 Homeport 是否找到了它。

要将堆栈部署到 [Swarm](/docs/guides/swarm/)，Homeport 改为在本机上运行 `docker stack deploy`，因此需要 `docker` 命令本身（不使用 Compose 插件）。Docker Desktop 已包含；在 Linux 上，请安装发行版的 Docker CLI 软件包（`docker-ce-cli`、`docker.io` 或 `docker`）。本机上无需运行任何引擎。

## 可选：git

[同步堆栈](/docs/guides/synced-stacks/)使用**本机**上安装的 `git` 克隆和获取其仓库：Git for Windows、Apple 的命令行工具（`xcode-select --install`），或发行版的 `git` 软件包。在 Windows 上，Homeport 会让它使用系统的 OpenSSH 访问 SSH 仓库。其他所有功能都无需 git；**设置 → 本机**会显示 Homeport 是否找到了它。

## 可选：Helm

要管理集群上的 [Helm Release](/docs/clusters/helm/)，Homeport 会在**本机**上运行 3 或更高版本的 `helm` 命令。它通过私有连接让 helm 指向集群，因此不会有任何集群凭据写入磁盘；helm 使用它自己的 Chart 仓库，与在终端中一样。安装方式：`brew install helm`（macOS），`winget install Helm.Helm`、Chocolatey 或 Scoop（Windows），或者 `snap install helm --classic`、发行版的软件包或 [helm.sh](https://helm.sh/docs/intro/install/) 提供的脚本（Linux）。其他所有集群页面都无需它；**设置 → 本机**会显示 Homeport 是否找到了它。

## 可选：系统托盘

Homeport 会在系统托盘（macOS 上为菜单栏，Windows 上为通知区域）中放置一个图标，列出每台主机及其状态。Windows 和 macOS 总是有托盘。在 Linux 上，它需要 **appindicator** 库：AppImage 已包含该库，`.deb` 和 `.rpm` 软件包将其列为推荐依赖，因此 `apt`、`dnf` 和 `zypper` 会安装它，除非你关闭了推荐软件包。否则请安装 `libayatana-appindicator3-1`（Debian、Ubuntu、openSUSE）或 `libayatana-appindicator-gtk3`（Fedora）。有些桌面环境会隐藏托盘图标，例如 GNOME 只有在安装 AppIndicator Support 等扩展后才会显示。没有托盘时，Homeport 通过其窗口工作，关闭窗口即退出应用。

## 各功能所需条件

| 功能                                    | 钥匙串 | Compose CLI | git | helm |
| --------------------------------------- | :----: | :---------: | :-: | :--: |
| 将堆栈部署到 Swarm                      |   —    | docker CLI  |  —  |  —   |
| 本地套接字主机和 TCP 主机               |   —    |      —      |  —  |  —   |
| 使用 ssh-agent 的 SSH 主机              |   —    |      —      |  —  |  —   |
| 使用 Homeport 密钥或导入密钥的 SSH 主机 |   是   |      —      |  —  |  —   |
| TLS 主机                                |   是   |      —      |  —  |  —   |
| 容器、镜像、卷、网络                    |   —    |      —      |  —  |  —   |
| 启动、停止、删除堆栈                    |   —    |      —      |  —  |  —   |
| 部署托管堆栈和关联堆栈                  |   —    |     是      |  —  |  —   |
| 托管堆栈或同步堆栈的环境变量            |   是   |      —      |  —  |  —   |
| 镜像仓库登录（私有镜像、推送）          |   是   |      —      |  —  |  —   |
| 来自公开仓库的同步堆栈                  |   —    |     是      | 是  |  —   |
| 使用部署密钥或令牌的同步堆栈            |   是   |     是      | 是  |  —   |
| 从文件夹构建镜像                        |   —    |      —      |  —  |  —   |
| Kubernetes 集群                         |   是   |      —      |  —  |  —   |
| Nomad 集群                              |   是   |      —      |  —  |  —   |
| Helm Release                            |   是   |      —      |  —  |  是  |
