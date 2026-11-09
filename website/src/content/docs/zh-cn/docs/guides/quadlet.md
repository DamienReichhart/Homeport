---
title: Quadlet 单元
description: 列出 Podman 主机的 Quadlet 单元及其 systemd 状态，阅读它们的文件，安装、编辑和删除它们，并启动或停止它们的服务。
---

**Quadlet** 是 Podman 在 systemd 下运行长期服务的方式。Quadlet 单元是一个文件（`web.container`、`shop.pod`、`cache.volume`、`lan.network`、`app.kube`、`base.image`、`api.build`），systemd 的生成器会将它转换为一个服务 `web.service`，该服务在开机时启动容器，并在容器停止时重启它。rootful Podman 将其单元保存在 `/etc/containers/systemd/` 中；rootless Podman 将每个用户的单元保存在 `~/.config/containers/systemd/` 中。

选择 Podman 主机时，侧边栏中会出现 **Quadlet** 页面。它通过 Podman 自己的 API（在主机已使用的套接字上）工作，并通过 `systemctl` 进行启动和停止，这由 [ADR 0019](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0019-quadlet-units-through-podmans-api-and-systemctl.md) 决定。

## 各 Podman 版本和连接方式支持的操作

Podman 的 API 在两个版本中逐步加入了 Quadlet 支持，并且它从不启动或停止单元的服务。因此 Homeport 能做什么取决于主机的 Podman 以及 Homeport 访问它的方式：

| 操作                 | Podman                  | 连接方式                     |
| -------------------- | ----------------------- | ---------------------------- |
| 列出单元及其状态     | 5.7 及更高版本          | 任意                         |
| 从容器中找出单元     | 4.0 至 5.6              | 任意                         |
| 读取单元文件         | 5.8 及更高版本          | 任意                         |
| 安装、编辑和删除单元 | 5.8 及更高版本          | 任意                         |
| 启动、停止和重启服务 | 任何支持 Quadlet 的版本 | SSH，或 Linux 上的本地套接字 |

在 5.7 之前的 Podman 上，单元是通过 Podman 在它们创建的容器上添加的 `PODMAN_SYSTEMD_UNIT` 标签找到的，因此单元只有在创建了容器之后才会出现，并以其服务命名。页面会在列表上方说明这一点。

启动和停止需要 `systemctl`，Homeport 会在 Podman 运行的地方执行它：对 SSH 主机通过其 SSH 会话执行，对 Linux 上的本地套接字则在本机执行。TCP 或 TLS 主机，或者 macOS 或 Windows 上的本地套接字（Podman 在虚拟机中运行），可以执行其他所有操作；这些按钮会被禁用并说明原因。

## Quadlet 页面

| 列       | 显示内容                                                                                                               |
| -------- | ---------------------------------------------------------------------------------------------------------------------- |
| **单元** | 单元的文件名，以及安装它时所属的应用。                                                                                 |
| **类型** | `container`、`pod`、`volume`、`network`、`kube`、`image`、`build` 或 `artifact`。                                      |
| **服务** | 它生成的 systemd 服务；在 systemd 加载它之前显示为**尚未生成**。                                                       |
| **状态** | **运行中**、**正在启动**、**正在停止**、**已停止**、**失败**、**未被 systemd 加载**或**模板**，以 systemd 报告的为准。 |
| **容器** | 它创建了多少个容器。                                                                                                   |

每一行都有**启动**（已停止的单元）或**停止**（正在运行的单元）。选择一个单元即可打开其页面。列表会随其容器的启停自动更新。

**未被 systemd 加载**的单元已经安装，但 systemd 的生成器跳过了它：通常是文件中有错误。在主机上运行 `systemctl --user daemon-reload`（rootful Podman 则为 `systemctl daemon-reload`），然后用 `journalctl` 查看原因。

在**容器**页面上，由单元创建的容器会在其名称旁显示该单元的服务；选择它即可打开该单元。

## 单元的页面

- 页头有**启动**、**停止**和**重启**、**编辑**以及**删除**。
- **服务**显示 systemd 单元、它是系统单元还是用户单元（`systemctl --user`）、其文件在主机上的位置，以及它的应用和 Pod（如果有）。
- **单元文件**以高亮形式显示 Podman 读取到的文件。
- **容器**列出它创建的容器，每个都可以打开其容器页面。

在“容器”页面停止单元的容器会与 systemd 冲突，systemd 会根据 `Restart=` 再次启动它。请改为停止该单元。

## 安装单元

**新建单元**（或在 Quadlet 页面按 `n`，或在命令面板中选择**新建单元**）会打开一个编辑器，其中包含所选类型的起始模板，也可以用**打开文件…**打开本机上的文件。用扩展名为其命名，例如 `web.container`；文件需要包含其类型对应的节，`.container` 需要 `[Container]`。

**添加文件…**会将其他文件（`.build` 的 `Containerfile`、`.kube` 的 YAML、env 文件）放在单元旁边，位于以该单元命名的应用文件夹中，与 `podman quadlet install` 的做法相同。

Podman 会将文件写入其用户的单元目录并重新加载 systemd，因此服务会立即存在。安装不会启动它：安装完成后选择**启动**。名称已被占用的单元会被拒绝；请改为编辑那个单元。

## 编辑和删除

**编辑**会打开单元的文件。**保存**会原地写入文件并重新加载 systemd；安装在它旁边的文件会保留。正在运行的服务会保留旧设置，直到你**重启**它。

**删除**会删除单元的文件并重新加载 systemd。**先停止其正在运行的服务**默认开启；关闭它时，Podman 会拒绝删除正在运行的单元。单元创建的容器、卷和镜像会保留。

## Homeport 在主机上运行的内容

仅对于**启动**、**停止**和**重启**，Homeport 会通过 SSH 在主机的服务器上（对本地套接字则在本机上）以 SSH 用户的权限运行一条命令：

```bash
systemctl --user start 'web.service'
```

`--user` 用于 rootless Podman 的单元，rootful Podman 的单元则省略；对于相应操作，`start` 会换成 `stop` 或 `restart`。Homeport 会检查服务名称并为其加引号，并且从不使用 `sudo`。因此：

- **系统单元**（rootful Podman）需要一个有权管理它们的 SSH 用户，例如 `root`。
- **用户单元**（rootless Podman）需要该用户的 systemd 实例。为了让它在没有登录会话时也能运行，并让单元开机启动，请在主机上以该用户身份启用一次 lingering：

```bash
loginctl enable-linger "$USER"
```

[安全页面](/docs/reference/security/#what-homeport-runs-on-your-hosts)列出了 Homeport 在主机上运行的每一条命令。
