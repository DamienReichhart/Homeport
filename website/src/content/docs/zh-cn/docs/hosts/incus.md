---
title: Incus
description: 连接本机上的 Incus，或通过 SSH、通过带信任令牌的 HTTPS 连接，并选择其页面显示的项目。
---

**Incus** 运行系统容器和虚拟机，即[指南](/docs/guides/instances/)中的**实例**。Homeport 通过 Incus 自身的 REST API 连接它，这是 [ADR 0021](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0021-incus-over-its-rest-api.md) 的决定。Incus 主机在侧边栏中有自己的页面：**实例**、**镜像**、**配置文件**、**存储**和**网络**。Docker 的页面、堆栈和看板在其上不可用。

## 添加 Incus 主机

在表单的**引擎**字段中选择 **Incus**，然后选择访问方式：

| 连接             | 适用情况                             | 所需条件                                             |
| ---------------- | ------------------------------------ | ---------------------------------------------------- |
| **本地套接字**   | Incus 在本机上运行（Linux）          | 你的用户属于 `incus-admin` 组或 `incus` 组（见下文） |
| **SSH**          | Incus 在你可以登录的服务器上运行     | SSH 用户属于 `incus-admin` 组或 `incus` 组           |
| **TLS**（HTTPS） | Incus 在网络上监听，或不允许使用 SSH | 信任令牌，或 Incus 已信任的客户端证书                |

普通 **TCP** 显示为灰色：Incus 不会在没有 TLS 的情况下提供其 API。Homeport 可连接 **Incus 6.0**（长期支持版本）及更高版本。

### 它的套接字

| 套接字                            | 谁可以使用                 | 能访问的内容                     |
| --------------------------------- | -------------------------- | -------------------------------- |
| `/var/lib/incus/unix.socket`      | `incus-admin` 组           | 全部：每个项目、存储池和网络     |
| `/var/lib/incus/unix.socket.user` | `incus` 组（`incus-user`） | 仅用户自己的项目，首次使用时创建 |

在 Linux 上，两者都有响应时，**在本机上找到**会同时列出它们，受限的那个显示为 **Incus（限于你的项目）**。你暂时还不能使用的套接字会连同修复命令一起列出：

```bash
sudo usermod -aG incus-admin "$USER"
```

组的变更对新的登录生效：添加主机之前，请注销并重新登录（或重新通过 SSH 连接）。已停止的 Incus 会说明哪些单元可以启动它：

```bash
sudo systemctl enable --now incus.socket incus.service
```

### 通过 HTTPS 使用信任令牌

设置 `core.https_address` 后，Incus 会在网络上监听。在 Incus 服务器上：

```bash
incus config set core.https_address :8443
incus config trust add homeport
```

第二条命令会输出一个**信任令牌**：一行字母和数字。在 Homeport 中，选择 **Incus** 和 **TLS**，保持选中**信任令牌**，然后粘贴它。Homeport 会显示它读取到的内容（令牌对应的名称、服务器的地址和证书指纹，以及过期时间），然后在选择**添加**时：

1. 创建自己的客户端证书，使用在内存中生成的 ECDSA P-384 密钥；
2. 按顺序尝试令牌中的地址，保留第一个证书指纹与令牌中指纹一致的地址；其他地址会被拒绝，并显示两个指纹；
3. 连同证书一起发送令牌，并检查 Incus 现在是否信任它。

之后，该主机会以令牌的名称出现在 `incus config trust list` 中。令牌只能使用一次，并且会过期；已使用、已撤销或已过期的令牌会被拒绝，并显示生成新令牌的命令。之后如果要撤销 Homeport 的访问权限，请在那里删除其证书：

```bash
incus config trust list
incus config trust remove <fingerprint>
```

**现有证书**使用 Incus 已信任的客户端证书和密钥（`incus config trust add-certificate`），并需要提供要信任的 Incus 自身证书（`/var/lib/incus/server.crt`，集群中为 `cluster.crt`）。

### 服务器证书被固定

Incus 的证书是自签名的，因此没有可用于检查它的 CA。Homeport 会将它信任的证书与主机一起保存，之后只接受完全相同的那个证书（按其 SHA-256 指纹比较），无论通过哪个地址访问。如果 Incus 被重装或其证书被更新，主机会被拒绝，并显示预期的指纹和实际得到的指纹：在服务器上使用 `incus info` 检查新指纹，然后使用新令牌重新添加主机。客户端密钥和两个证书都像其他所有机密一样加密保存；参见[安全](/docs/reference/security/#incus-证书)。

## 项目

Incus 将实例、镜像、配置文件、自定义卷和网络保存在**项目**中。页面一次显示一个项目，在主机切换器下方选择，那里会列出每个项目及其包含的实例和镜像数量。Homeport 会在每次启动之间记住每台主机的选择。在你选择之前，主机显示 `default`，或受限套接字上它唯一的项目。存储池属于整个服务器；其中的卷按所选项目显示。

## 事件

页面会根据 Incus 在所有项目中的生命周期事件刷新：实例启动或停止、创建快照、添加镜像、配置文件或网络变更。这些事件也会保留在**事件**页面上。Incus 不保存事件历史，因此 Homeport 关闭期间发生的事情不会被补上。

连接期间，Homeport 每 15 秒向 Incus 请求一次 `GET /1.0`。Incus 停止时，主机会变为**无法访问**，当它再次响应时，Homeport 会自动重新连接。

## Homeport 在主机上做了什么

除了 Incus 本身，Incus 主机上不运行任何东西：控制台和终端通过 Incus 自己的 websocket 进行，远程镜像由 Incus 下载。Homeport 会从本机读取远程镜像服务器的镜像列表，用于镜像选择器。
