---
title: 故障排除
description: 按 Homeport 显示的消息，列出你最可能遇到的问题及其解决方法。
---

如果这里没有你遇到的问题，请查看[日志文件](/docs/reference/security/#日志)，并附上**设置**中显示的 Homeport 版本[创建 issue](https://github.com/DamienReichhart/Homeport/issues)。

## 安装与启动

### Windows 提示“Windows 已保护你的电脑”

安装程序目前尚未使用商业证书签名。选择**更多信息**（More info），然后选择**仍要运行**（Run anyway）。你可以先[验证下载](/docs/reference/verifying/)。

### macOS 提示 Homeport“无法打开”

Homeport 目前尚未经过 Apple 公证。打开**系统设置 → 隐私与安全性**（System Settings → Privacy & Security），选择 Homeport 旁边的**仍要打开**（Open Anyway），或运行：

```sh
xattr -dr com.apple.quarantine /Applications/Homeport.app
```

### AppImage 无法启动

如果提示中提到 FUSE，请安装 `libfuse2`（Ubuntu 24.04 及更高版本为 `libfuse2t64`）。检查文件是否可执行：`chmod +x Homeport*.AppImage`。

### Homeport 在错误的位置打开

Homeport 重新打开时，会恢复关闭时窗口的大小、位置和最大化状态。如果它原来所在的显示器已不存在，它会在剩下的某个显示器上居中打开。要忘记保存的位置，请退出 Homeport，然后从其设置文件夹中删除 `.window-state.json`：

| 系统    | 文件                                                                |
| ------- | ------------------------------------------------------------------- |
| Linux   | `~/.config/com.homeport.app/.window-state.json`                     |
| macOS   | `~/Library/Application Support/com.homeport.app/.window-state.json` |
| Windows | `%APPDATA%\com.homeport.app\.window-state.json`                     |

### 再次打开 Homeport 不会打开第二个窗口

同一时间只运行一个 Homeport：在它已经运行时再次打开，会将其窗口带到最前面，即使它隐藏在[系统托盘](/docs/guides/overview/#系统托盘)中。两个副本会对每台主机连接两次，并写入同一份数据。

### 没有托盘图标

在 Linux 上，托盘需要 appindicator 库，并且有些桌面环境会隐藏托盘图标：在 GNOME 上，请安装 AppIndicator Support 扩展。如果 Homeport 找不到 appindicator 库，**设置 → 桌面**会说明这一点，并且关闭窗口即退出应用。参见[系统要求](/docs/requirements/#可选系统托盘)。

### 红色横幅提示没有可用的系统钥匙串

Homeport 无法访问你的钥匙串。在 Linux 上，这通常意味着没有运行 Secret Service，这在精简桌面环境和窗口管理器中很常见。请安装并启动 GNOME Keyring（`gnome-keyring`），或启用 KWallet 的 Secret Service，然后重启 Homeport。在此之前，请使用本地套接字、TCP 或 ssh-agent 主机。

## 更新与软件包仓库

### “无法检查更新”

Homeport 无法从本网站下载更新信息：你可能处于离线状态，或位于阻止 `damienreichhart.github.io` 的代理或防火墙之后。Homeport 会继续正常工作；它稍后会再次检查，你也可以选择**设置 → 更新 → 立即检查**。你随时可以从[下载页面](/download/)下载最新版本。

### 更新安装失败

Homeport 只会安装签名与其更新密钥匹配的更新，如果下载不完整或被篡改则会停止。请稍后重试；如果一直失败，请从[下载页面](/download/)下载新版本并覆盖安装旧版本。

### apt 提示“NO_PUBKEY”或“The repository is not signed”

`/etc/apt/keyrings/homeport.asc` 中缺少 Homeport 密钥。请再次运行[快速安装](/docs/installation/#快速安装)，或[手动设置](/docs/installation/#手动设置)中的前两条命令。

### dnf 或 zypper 询问是否导入密钥

那是 Homeport 软件包密钥，在包管理器首次使用该仓库时出现。请确认它显示的指纹与[验证下载](/docs/reference/verifying/#软件包签名)中的指纹一致，然后接受。

### 发布后 apt 立即提示“Hash Sum mismatch”

apt 读取仓库时，仓库正在更新。几分钟后再次运行 `sudo apt update`。

### 我的包管理器看不到新版本

检查仓库是否仍已设置：`/etc/apt/sources.list.d/homeport.sources`、`/etc/yum.repos.d/homeport.repo` 或 `/etc/zypp/repos.d/homeport.repo`。如果你删除了它，请运行[快速安装](/docs/installation/#快速安装)重新添加。如果你是从下载的文件安装的 0.2.0 或更早版本，仓库从未被添加：参见[从 0.2.0 更新](/docs/reference/updating/#从-020-或更早版本更新)。

## 连接

### 无法访问：Permission denied

你的用户无法打开 Docker 套接字。在 Linux 上，将它加入 `docker` 组并重新登录：

```sh
sudo usermod -aG docker $USER
```

### 无法访问：No such file or directory

套接字路径上没有任何程序在监听。请启动你的引擎（Docker Desktop、Colima、OrbStack、Rancher Desktop），或检查路径。参见[本地引擎](/docs/hosts/local/#手动添加套接字)。

### “Docker Engine 未及时响应。”或“SSH 连接超时”

Homeport 等待连接的时间为 10 秒。请检查地址和端口、服务器是否正常运行，以及是否有防火墙阻挡。Homeport 会自动持续重试。

### 访问被拒绝：“SSH 服务器拒绝了 … 的密钥”

服务器尚未接受 Homeport 的密钥。在服务器上运行主机页面中的**添加公钥**命令，然后选择**重新连接**。参见 [SSH 主机](/docs/hosts/ssh/#授权-homeport-的密钥)。

### “SSH 服务器无法打开 /var/run/docker.sock”

Homeport 已登录，但无法访问 Docker。请检查：

- **远程套接字路径**：rootless Docker 使用 `/run/user/<uid>/docker.sock`；
- 该用户能否使用 Docker：在服务器上运行 `sudo usermod -aG docker <user>`；
- 服务器是否允许套接字转发（`sshd_config` 中的 `AllowStreamLocalForwarding`，默认开启）。

### “没有正在运行的 ssh-agent。”

启动你的 SSH agent，并使用 `ssh-add` 添加你的密钥。如果你是从桌面菜单打开 Homeport 的，它可能看不到你终端使用的 agent；请注销并重新登录，或从 `ssh-add -l` 能列出你密钥的终端中启动 Homeport。

在 Windows 上，Homeport 使用 OpenSSH Authentication Agent 服务或 PuTTY 的 Pageant。打开 Pageant 并加载你的密钥，或在管理员 PowerShell 中启动该服务：

```powershell
Get-Service ssh-agent | Set-Service -StartupType Automatic
Start-Service ssh-agent
ssh-add
```

### 访问被拒绝：“TLS handshake failed”

- CA 证书必须是签发服务器证书的那个。
- 你输入的地址必须与服务器证书中的某个名称或 IP 地址匹配。
- 服务器必须使用 `--tlsverify` 运行，并信任你的客户端证书的 CA。

### 服务器密钥已更改

SSH 服务器提供的密钥与你信任的不同。如果你重装了服务器或更改了其密钥，请选择**信任新密钥**。如果你不知道它为何更改，请不要连接：询问服务器的管理员。参见 [SSH 主机](/docs/hosts/ssh/#服务器密钥更改时)。

### 凭据丢失

用于加密你的机密的密钥已从钥匙串中丢失或已更改：可能发生在重置钥匙串、使用新的用户配置文件，或将 Homeport 的数据复制到另一台计算机之后（后者请使用[导出和导入](/docs/reference/security/#迁移到另一台计算机)）。Homeport 会保留该主机；请在主机页面上重新输入其凭据：

- **TLS**：重新加载三个证书文件，然后选择**保存**。
- **使用导入密钥的 SSH**：重新粘贴私钥和口令，然后选择**保存**。
- **使用 Homeport 密钥的 SSH**：选择**重新生成密钥**，然后在服务器上授权新密钥。

使用旧密钥加密的托管堆栈环境变量也需要重新输入。

## 集群

### 访问被拒绝：“… 拒绝了其 kubeconfig 中的凭据。”

API 服务器返回了 401：证书或令牌已过期或被撤销。请确认 `kubectl --context <context> get namespaces` 仍然可用，然后在集群页面上使用**从 kubeconfig 刷新**。对于 exec 插件，Homeport 已经向它请求过一次新令牌。

### “本机上未安装 …。”

在 `PATH` 或其常见安装位置中找不到该集群的 [exec 插件](/docs/clusters/exec-plugins/)（`aws`、`gke-gcloud-auth-plugin`、`kubelogin`…）。请安装它，确认它能在终端中运行，然后选择**重新连接**。

### “… 需要你登录，但它无法在 Homeport 中完成登录。”

exec 插件需要浏览器或设备代码。在终端中运行一次 `kubectl --context <context> get namespaces`，在那里登录，然后选择**重新连接**。

### “… 无法获取 … 的凭据。”

exec 插件运行了，但失败了。**详细信息**会显示它输出的末尾部分，通常是 profile 过期或区域错误。Homeport 会自动重试。

### “其用户使用 gcp 认证提供程序登录，而 Kubernetes 已不再支持该提供程序。”

`gcp` 和 `azure` 认证提供程序已从 Kubernetes 中移除。安装 `gke-gcloud-auth-plugin` 后重新获取凭据（GKE），或运行 `kubelogin convert-kubeconfig`（AKS），然后重新添加该上下文。

### “你的凭据不允许在 … 中对 … 执行“…”。”

集群的 RBAC 不允许你的 kubeconfig 用户执行该操作。在右上角的选择器中选择一个你有权查看的 Namespace，或询问集群管理员。消息中给出了用于检查的 `kubectl auth can-i` 命令。其他页面仍可正常使用。

### 无法访问：“… 拒绝了连接。”或“… 未及时响应。”

无法从本机访问 API 服务器：集群已停止（本地集群）、位于 VPN 之后，或 kubeconfig 中的服务器地址已更改。Homeport 会自动持续重试；请使用同一上下文运行 `kubectl cluster-info` 检查。

### “… 的证书不是由其 kubeconfig 中的 CA 签发的。”

kind、k3d 和 minikube 每次重新创建集群时都会生成新证书。请在集群页面上使用**从 kubeconfig 刷新**。

### Nomad：“… 不认识 Homeport 发送的 ACL 令牌。”

令牌输入错误、已过期或已被删除。使用 [ACL 策略](/docs/clusters/nomad/#acl-策略)中的策略创建一个新令牌（`nomad acl token create -name=homeport -policy=homeport`），并将其 secret ID 粘贴到集群的**设置**中。在此之前，集群会保持**访问被拒绝**状态。

### Nomad：“… 拒绝了：该 ACL 令牌缺少此操作所需的权限。”

令牌的策略没有授予此页面或操作所需的权限；消息会说明是哪一项（读取 job 或日志、运行或停止 job、扩缩、打开 shell、更改节点）。请将该能力添加到令牌的策略中；[ACL 策略](/docs/clusters/nomad/#acl-策略)列出了每项所需的内容。其他页面仍可正常使用。

### Nomad：“… 没有 leader：其 server 无法就 leader 达成一致。”

集群中正在运行或能互相访问的 server 少于多数，因此 Nomad 无法响应。在某个 server 上使用 `nomad server members` 检查它们，并恢复足够多的 server。Homeport 会持续重试。

### Nomad：“… 没有 job …。”

该 job 已被停止并清除，或已被垃圾回收，或者它位于所选 namespace 之外的另一个 namespace 中。重新打开 Jobs 页面，必要时选择**所有 namespace**。

### Nomad：“… 的 task group … 最多只能运行 … 个。”或“至少需要运行 … 个。”

task group 的 `scaling` 块设置了 `min` 和 `max`，而请求的数量超出了该范围。请在 job 规范中修改上下限并重新运行，才能超出原来的范围。

### Nomad：“在 … 的 deployment 进行期间，Nomad 不会扩缩它。”

运行 service job 的新版本会启动一个 deployment，在新版本的每个 allocation 都健康之前，Nomad 拒绝更改其数量。请等待 job 的 allocation 稳定下来，然后再次扩缩。

### Nomad：“无法解析该 job 规范，位于第 … 行第 … 列。”

Nomad 的解析器在该位置发现了问题；**详细信息**中有其消息。`variable` 块需要默认值，因为 Homeport 不传递变量文件。JSON 规范必须是 Nomad API 格式的 job，与 `nomad job inspect` 输出的格式相同。

### Nomad：“Nomad 无法放置 … 的所有 allocation”

计划没有找到拥有 task group 所需资源或满足其约束条件的合格节点；计划会说明原因。**仍然运行**会将无法放置的部分排队，直到有节点能接收它；或者降低资源、更改约束条件，或添加节点。

### Nomad：“自规划以来，… 已更改，因此未运行。”

在你规划和**运行**之间，其他人运行或停止了该 job，Homeport 不会覆盖他们的更改。请返回，重新规划，并在运行前检查新的差异。

### Nomad：“allocation … 的 task … 未在运行。”

shell 需要一个正在运行的 task：这个 task 尚未启动、已经结束或已失败。在 allocation 被垃圾回收之前，其日志仍可读取。

### Nomad：“… 无法访问 allocation … 所在的节点。”

日志和 shell 经过 server 到达 client node，而 server 无法访问它：节点已宕机、已断开连接，或其 agent 已停止。请在 Nodes 页面上或使用 `nomad node status` 检查它。

### Helm：“本机上未安装 helm CLI。”

[Helm Release](/docs/clusters/helm/) 需要本机上有 3 或更高版本的 `helm`。安装它（`brew install helm`、`winget install Helm.Helm` 或 `snap install helm --classic`），然后重新打开 Helm 页面；**设置 → 本机**会显示 Homeport 找到的 helm。`helm` 2 会被报告为不受支持。

### Helm：“此 Release 的另一次安装、升级或回滚仍在进行中。”

该 Release 处于 `pending-…` 状态：某次安装、升级或回滚没有完成，通常是因为被中断了。在它结束之前，helm 拒绝更改该 Release。请等待它完成，或从其**历史**中将 Release 回滚到上一个已部署的修订版本。

## 使用 Homeport

### 部署显示为灰色：“本机上未安装 docker compose CLI。”

Homeport 需要本机上的 `docker compose` 来部署堆栈。请安装 Docker Desktop 或带 Compose 插件的 Docker CLI，然后重启 Homeport。**设置 → 本机**会显示是否找到了它。

如果已安装但找不到，可能是 Homeport 的 `PATH` 与你终端的不同，这在 macOS 上从桌面菜单启动时很常见。请确保 `docker` 位于标准位置，例如 `/usr/local/bin`。

### 部署失败：“使用了 build:”

托管堆栈没有可用于构建的文件夹。可以改为[关联项目文件夹](/docs/guides/stacks/#创建关联堆栈)（关联堆栈可以构建），或者构建镜像并推送到镜像仓库，然后使用 `image:`。

### 我的堆栈变成了外部堆栈

当[关联堆栈](/docs/guides/stacks/#当文件夹被移动或消失时)的文件夹被移动、重命名或删除，或其中某个 compose 文件被移动、重命名或删除时，它会显示为外部堆栈，并标有 ⚠ **它的 compose 文件已不在本机上。** 什么都不会丢失：把文件夹放回原处，下次刷新时堆栈就会重新成为关联堆栈。如果文件夹已永久移动，请在堆栈的**定义**标签页中选择**忘记**，然后通过**新建堆栈 → 使用本机上的文件夹**从新位置重新关联它。

从另一台计算机上的文件夹启动的堆栈在这里总是外部堆栈，因为本机上没有那个路径。

### “在 Homeport 之外被更改”

自 Homeport 上次读取以来，关联堆栈的 compose 文件在 Homeport 之外（在你的编辑器中、被 `git pull`）被编辑过。选择**重新加载**查看其当前内容。如果你同时也在 Homeport 中编辑，保存会被拒绝，并提示**自你打开以来，… 已在 Homeport 之外被更改。**，这样任何一方的更改都不会被悄悄丢失：重新加载并重做你的编辑，或选择**覆盖**保留你的版本。参见[编辑关联堆栈](/docs/guides/stacks/#编辑关联堆栈)。

### 拉取私有镜像失败

在**设置 → 镜像仓库**中为该镜像的镜像仓库添加登录信息，并使用**检查登录**确认镜像仓库接受它。GHCR 以及启用了双重认证的 Docker Hub 账户需要使用访问令牌，而不是密码。拉取对话框会说明将使用哪个登录信息；如果显示**匿名**，说明该镜像的镜像仓库与保存的登录信息不匹配：`ghcr.io/org/app` 需要 `ghcr.io` 的登录信息，`org/app` 需要 Docker Hub 的登录信息。

### 构建失败

消息会指出失败的步骤，例如**构建在第 3/5 步（RUN make）失败。**；**详细信息**中有构建输出的末尾部分。Homeport 使用引擎的经典构建器，因此需要 BuildKit 的 Dockerfile（`RUN --mount`、`--secret`、heredoc）会失败；请在终端中构建这类镜像。私有的 `FROM` 镜像需要在**设置 → 镜像仓库**中为其镜像仓库添加登录信息。

### 没有出现新版镜像标记

只有在询问了镜像仓库并得到答复时，才会显示该标记。本地构建或按摘要拉取的镜像从不显示。如果某个镜像仓库无法访问或限制请求（Docker Hub 会将这些检查计入其拉取限制），Homeport 会在其[日志](/docs/reference/security/#日志)中写入一行，并且不显示任何内容；答案还会保留 6 小时，因此请选择**检查新版镜像**立即重新询问。在**设置 → 镜像仓库**中添加 Docker Hub 登录信息可以提高该限制。

### 搜索 Docker Hub 时显示错误

搜索在所选主机上运行，因此需要该主机能访问互联网，而标签则由本机从 `hub.docker.com` 读取。两者都可能被限速或被代理阻止。对话框仍然可以拉取你输入的任何完整引用，例如 `postgres:17-alpine`。

### “文件”标签页提示容器中没有 find/stat 工具

精简镜像和 distroless 镜像不包含 `find` 和 `stat`，而 Homeport 需要用它们列出文件。对于这类容器，请使用 `docker cp`。

### 终端无法打开

容器必须正在运行，并且有 `/bin/sh` 或 `/bin/bash`。使用**新建终端**运行其他程序。

### 卷或网络无法删除

只要还有容器（即使是已停止的容器）在使用它，引擎就会拒绝删除。请先删除这些容器；确认框中会列出它们。

### “需要 Podman 5.8 或更高版本”

Podman 的 API 从 Podman 5.8 起可以读取、安装、编辑和删除 [Quadlet 单元](/docs/guides/quadlet/)，从 5.7 起可以列出它们。请在主机上更新 Podman，或在主机的终端中使用 `podman quadlet`。启动和停止单元在所有 Podman 版本上都可用。

### Quadlet：启动和停止显示为灰色

Homeport 使用 `systemctl` 启动和停止单元的服务，它只能通过 SSH 运行，或对本机上的 Podman 运行（Linux）。请通过 SSH 而不是 TCP 或 TLS 访问该主机。服务显示为**尚未生成**的单元没有可启动的东西：systemd 的生成器跳过了它的文件。

### Quadlet：“主机上该用户的 systemd 未在运行”

用户单元（rootless Podman）需要该用户的 systemd 实例，除非开启了 lingering，否则它只在用户登录时运行。在主机上以该用户身份运行 `loginctl enable-linger "$USER"`，然后重试。

### Quadlet：“systemd 不允许此用户管理 …”

系统单元（rootful Podman）只能由有权管理它们的用户（例如 `root`）启动和停止，而 Homeport 从不使用 `sudo`。请以这样的用户连接，或使用 rootless Podman，其单元属于用户自己。

### containerd：“无权使用 /run/containerd/containerd.sock 处的 containerd”

containerd 的套接字属于 root。通过 `/etc/containerd/config.toml` 中 `[grpc]` 下的 `gid` 为某个组授予访问权限，将你的用户（或 SSH 用户）加入该组，重启 containerd 并重新登录；[containerd 页面](/docs/hosts/containerd/#让你的用户使用套接字)中有相关命令。Rootless containerd 完全无法访问：其套接字只存在于 RootlessKit 的命名空间内。

### containerd：“此容器的输出未被记录。”

containerd 不保存日志；启动任务的工具决定其输出去向。使用 `ctr` 创建的容器通常不会把输出发送到任何地方。nerdctl 和 Kubernetes 的容器会将日志写入 Homeport 可以读取的文件；参见[日志](/docs/hosts/containerd/#日志)。

### containerd：日志或终端提示需要 SSH

它们需要 containerd 所在机器上的文件，Homeport 通过 SSH 访问这些文件，或者对于 Linux 上的本地套接字在本机上访问。请通过 SSH 添加该主机。如果日志文件无法读取，它通常属于 root：请以有权读取它的用户连接。

### containerd：“无法为 Homeport 拉取镜像”

拉取通过 containerd 的 Transfer 服务进行，该服务从 containerd 2.0 起稳定。请更新 containerd，或在主机上使用 `ctr images pull` 或 `nerdctl pull` 拉取；之后镜像会显示在“镜像”页面上。

### Incus：“无权使用 /var/lib/incus/unix.socket 处的 Incus”

Incus 的管理套接字供 `incus-admin` 组使用，受限的 `unix.socket.user` 供 `incus` 组使用。使用 `sudo usermod -aG incus-admin "$USER"` 添加你的用户（或 SSH 用户），然后注销并重新登录，或重新通过 SSH 连接。参见 [Incus 的套接字](/docs/hosts/incus/#它的套接字)。

### Incus：使用信任令牌添加时提示“Homeport 无法通过 … 访问 Incus。”

在设置 `core.https_address` 之前，Incus 不会在网络上监听。在服务器上运行 `incus config set core.https_address :8443`，检查防火墙是否放行 8443 端口，然后生成一个新令牌：令牌会列出生成时 Incus 正在监听的地址。

### Incus：“Incus 拒绝了该信任令牌。”

令牌只能使用一次，并且会过期。在服务器上使用 `incus config trust add homeport` 生成一个新令牌，并在其有效期内粘贴。

### Incus：“… 出示的证书指纹为 …，而不是受信任的 …。”

服务器的证书不是 Homeport 信任的那个：Incus 被重装、其证书被更新，或者该地址上有其他东西在响应。在服务器上使用 `incus info` 检查指纹；如果它就是显示的那个，请删除该主机，然后使用新令牌重新添加。参见[固定的证书](/docs/hosts/incus/#服务器证书被固定)。

### Incus：“此主机无法运行虚拟机。”

Incus 运行虚拟机需要 KVM（`/dev/kvm`）和 QEMU。请在服务器的固件中启用虚拟化（对于虚拟服务器，则启用嵌套虚拟化），或改为创建系统容器。

### Incus：“虚拟机的 Incus agent 未在运行”

在虚拟机中执行命令需要经过其中的 `incus-agent`，Incus 自己的镜像会在启动时运行它。请等待启动完成，或使用不需要 agent 的**控制台**标签页。

### “主机 … 未连接”

该操作需要主机，但主机已断开连接。等待它重新连接，或选择**立即重试**。
