---
title: 本地引擎
description: 连接本机上的 Docker Engine、Docker Desktop、Colima、OrbStack、Rancher Desktop 或 Podman。
---

**本地套接字**主机通过 Unix 套接字（在 Windows 上为命名管道）与本机上的引擎通信。它不需要凭据，也不需要钥匙串。

## 自动检测

Homeport 会在常见位置查找正在运行的引擎，并在欢迎屏幕和**添加主机**页面的**在本机上找到**下方列出有响应的引擎。选择其中一个旁边的**添加**，即可创建一台以它命名的主机。

| 运行时                    | macOS 和 Linux                                                                                                     | Windows                           |
| ------------------------- | ------------------------------------------------------------------------------------------------------------------ | --------------------------------- |
| Docker Engine             | `/var/run/docker.sock`                                                                                             | —                                 |
| Docker Engine（rootless） | `$XDG_RUNTIME_DIR/docker.sock`                                                                                     | —                                 |
| Docker Desktop            | `~/.docker/desktop/docker.sock`、`~/.docker/run/docker.sock`                                                       | `//./pipe/docker_engine`          |
| Colima                    | `~/.colima/default/docker.sock`                                                                                    | —                                 |
| OrbStack                  | `~/.orbstack/run/docker.sock`                                                                                      | —                                 |
| Rancher Desktop           | `~/.rd/docker.sock`                                                                                                | —                                 |
| Podman                    | `$XDG_RUNTIME_DIR/podman/podman.sock`、`/run/podman/podman.sock`；[macOS 上的 machine](#macos-上的-podman-machine) | `//./pipe/podman-machine-default` |

每个候选项都会通过向引擎查询版本来检查，因此只会列出正在运行的引擎。当多个路径指向同一个套接字时（在 macOS 上，`/var/run/docker.sock` 通常指向 Docker Desktop、OrbStack 或 Rancher Desktop），该引擎只会以最先找到的名称列出一次。

### macOS 上的 Podman machine

在 macOS 上，Podman 在虚拟机（`podman machine`）中运行其引擎，其套接字随每个 machine 而变化。Homeport 会向 `podman` CLI 查询每个正在运行的 machine 的套接字（`podman machine list` 和 `podman machine inspect`），并将每个 machine 列为 **Podman（_machine 名称_）**，例如 **Podman (podman-machine-default)**。已停止的 machine 不会列出；使用 `podman machine start` 启动一个，然后再次打开**添加主机**。

如果 Homeport 无法运行 `podman`（它不在 PATH 中，也不在 `/opt/homebrew/bin` 或 `/usr/local/bin` 中），则会改为在 Podman 存放套接字的位置查找：Podman 5 为 `$TMPDIR/podman/<machine>-api.sock`，Podman 4 为 `~/.local/share/containers/podman/machine/…/podman.sock`。当 `podman-mac-helper` 已将 `/var/run/docker.sock` 链接到某个 machine 时，该 machine 只会以其 Podman 名称列出一次。

## 手动添加套接字

如果你的引擎没有被检测到（`default` 以外的 Colima profile、Windows 上名称不是默认值的 Podman machine，或自定义路径），请自行添加：

1. 在主机切换器中选择**添加主机…**。
2. 选择**本地套接字**。
3. 输入**名称**和**套接字路径**：Unix 套接字（例如 `/run/user/1000/docker.sock`），或 Windows 上的命名管道（例如 `//./pipe/docker_engine`）。
4. 选择**添加主机**。

要查找 Docker CLI 使用的套接字，请运行：

```sh
docker context inspect --format '{{.Endpoints.docker.Host}}'
```

对于 Podman machine：

```sh
podman machine inspect --format '{{.ConnectionInfo.PodmanSocket.Path}}'
```

## Linux 上的权限

默认套接字 `/var/run/docker.sock` 属于 `docker` 组。如果你的用户不在该组中，主机会显示**无法访问**，并提示 _Homeport 无权使用 /var/run/docker.sock 处的 Docker Engine。_将自己加入该组并重新登录：

```sh
sudo usermod -aG docker $USER
```

属于 `docker` 组相当于拥有该机器的 root 权限；如果你在意这一点，可以使用 rootless Docker 或 Podman 来避免。
