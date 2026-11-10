---
title: 数据与备份
description: Homeport 在各系统上把数据保存在哪里，如何备份和恢复，以及机密丢失时需要重新输入什么。
---

Homeport 把它所知道的内容保存在你的计算机上：如何访问你的主机和集群、你的堆栈的定义和环境变量、你的镜像仓库登录、仓库和设置。本页说明它们在哪里、要复制什么，以及如何恢复。关于它们如何受到保护，参见[安全与你的数据](/docs/reference/security/)。

在你的主机和集群上运行的内容不是 Homeport 的数据：容器、镜像、卷以及 Kubernetes 或 Nomad 对象都留在原处，Homeport 的任何备份都不包含它们。要复制卷的内容，请在“卷”页面[将其备份到文件](/docs/guides/volumes-networks/#backing-up-and-restoring)。

## 各项内容的位置

### 数据文件夹

| 系统    | 文件夹                                            |
| ------- | ------------------------------------------------- |
| Linux   | `~/.local/share/com.homeport.app/`                |
| macOS   | `~/Library/Application Support/com.homeport.app/` |
| Windows | `%APPDATA%\com.homeport.app\`                     |

在 Linux 上，如果设置了 `$XDG_DATA_HOME`，该文件夹就位于其下。如果 Homeport 无法打开其数据库，窗口会显示该文件的完整路径，并提供**打开文件夹**。

| 文件夹中的内容                       | 说明                                                                                                        |
| ------------------------------------ | ----------------------------------------------------------------------------------------------------------- |
| `homeport.db`                        | 数据库：每台主机、每个集群、堆栈、镜像仓库登录、仓库和设置，以及部署历史和引擎事件。其中的机密是加密的。    |
| `homeport.db-wal`、`homeport.db-shm` | SQLite 的预写日志及其索引，在 Homeport 运行时存在。在 Homeport 退出之前，最新的更改可能只在 `-wal` 文件中。 |
| `homeport.db.before-restore-<date>`  | 每次[恢复](#restoring-a-backup)之前生成的数据库副本。不再需要的可以删除。                                   |
| `repositories/` 和 `synced-stacks/`  | [同步堆栈](/docs/guides/synced-stacks/)的仓库的克隆和工作副本。Homeport 会根据仓库本身重新生成它们。        |

[Homeport 保存的内容](/docs/reference/security/#what-homeport-stores)列出了数据库包含什么，以及哪些部分是加密的。

### 钥匙串条目

加密机密的密钥不在该文件夹中。它是操作系统钥匙串中的一个条目，服务为 `com.homeport.app`，账户为 `secrets-key`：在 macOS 上位于**钥匙串访问**，在 Windows 上位于**凭据管理器 → Windows 凭据**，在 Linux 上位于**密码和密钥**（Seahorse）或 KWallet。它属于你在这台计算机上的用户账户。

数据库和这个条目是一体的。没有密钥的数据库仍然可以打开，所有名称、地址和 compose 文件都在，但其中的机密都无法解密：主机和集群会显示**凭据丢失**。这就是为什么仅复制该文件夹不足以迁移到另一台计算机，也是推荐使用[应用内备份](#backing-up-in-homeport)的原因。

### 日志和窗口位置

| 系统    | 日志                                    | 窗口大小和位置                                                      |
| ------- | --------------------------------------- | ------------------------------------------------------------------- |
| Linux   | `~/.local/share/com.homeport.app/logs/` | `~/.config/com.homeport.app/.window-state.json`                     |
| macOS   | `~/Library/Logs/com.homeport.app/`      | `~/Library/Application Support/com.homeport.app/.window-state.json` |
| Windows | `%LOCALAPPDATA%\com.homeport.app\logs\` | `%APPDATA%\com.homeport.app\.window-state.json`                     |

两者都不值得备份。**设置 → 关于 → 打开日志文件夹**可以打开日志。

### 仍归你自己管理的内容

- **关联堆栈的文件夹。** Homeport 只记录[关联堆栈](/docs/guides/stacks/#linked-stacks)的文件夹和 compose 文件所在的位置。文件夹、其中的 compose 文件和 `.env` 都归你所有：像备份其他项目一样备份它们，通常使用 git。
- **你的 kubeconfig、`~/.ssh` 和 `~/.docker`。** Homeport 只在你从中导入时读取它们，从不更改它们。
- **helm 的 Chart 仓库和登录。** 它们属于 helm 自己，与你的终端共用。

## 在 Homeport 中备份

**设置 → 数据 → 备份…**会把所有内容写入一个 `.homeport` 文件。这是首选的备份方式，因为它携带机密本身，并用口令而不是这台计算机的钥匙串来保护它们：它可以在任何计算机上恢复。

1. 打开**设置**，在**数据**下选择**备份…**。
2. 输入一个至少 12 个字符的口令，输入两次。Homeport 从不保存它，没有它就无法打开该文件。
3. 选择保存文件的位置。

随后 Homeport 会说明它保存了什么，并列出因为机密无法在这台计算机上解密而只能不带机密保存的项目。

| 备份中包含                                                   | 备份中不包含                           |
| ------------------------------------------------------------ | -------------------------------------- |
| 主机，以及它们的密钥、口令、证书、堡垒机和你信任的服务器密钥 | 引擎事件                               |
| 集群，以及它们的证书、令牌和 exec 插件设置                   | 部署历史                               |
| 托管堆栈的 compose 文件和环境变量                            | 关联堆栈的文件夹和文件                 |
| 关联堆栈的文件夹和文件路径                                   | 同步堆栈的克隆（下次同步时会重新生成） |
| 同步堆栈的仓库、分支或标签、同步设置和环境变量               | 日志，以及窗口的大小和位置             |
| 部署到 Swarm 的堆栈                                          | 主机和集群上的任何内容：容器、镜像、卷 |
| 镜像仓库登录，以及它们的密码和令牌                           | helm 的 Chart 仓库                     |
| 仓库，以及它们的部署密钥和令牌                               | 端口转发，它们会在 Homeport 退出时结束 |
| 你的设置                                                     |                                        |

整个文件都用口令加密，因此没有口令，该文件不会泄露任何内容，连主机的名称都不会。它包含 Homeport 拥有的每个机密：请像保管密钥本身一样妥善保管它及其口令。[备份](/docs/reference/security/#backups)介绍了加密方式。

在试用预发布版之前、重装系统之前，以及每当你添加了不想重新设置的主机时，都请备份。

### 恢复备份

1. 在这台或另一台计算机上打开**设置**，在**数据**下选择**恢复…**。
2. 选择 `.homeport` 文件并输入其口令。在更改任何内容之前，Homeport 会列出文件中的内容，以及它由哪个 Homeport 在何时制作。
3. 选择**全部替换**。

恢复会用备份中的数据替换 Homeport 的全部数据，而不是合并。Homeport 会先把当前数据库复制到数据文件夹中的 `homeport.db.before-restore-<date>`，然后一步替换所有内容，因此失败时你的数据保持原样。每个机密都会用这台计算机的钥匙串密钥重新加密。

之后：

- 主机和集群会立即重新连接，不会要求你提供密钥、密码或要信任的服务器密钥。
- 如果关联堆栈的文件夹不在这台计算机的相同路径上，它会显示为外部堆栈，直到该文件夹出现。
- 同步堆栈会在下次同步时重新克隆其仓库。
- “事件”页面和部署历史从空开始。

由旧版 Homeport 制作的备份可以在新版中恢复。由更新版本的 Homeport 制作的备份会被拒绝：请先更新 Homeport。

要回到恢复之前的状态，请退出 Homeport，将 `homeport.db` 重命名为其他名称，将你想要的 `homeport.db.before-restore-<date>` 重命名为 `homeport.db`，删除旁边残留的 `homeport.db-wal` 和 `homeport.db-shm`，然后再次打开 Homeport。

### 只迁移部分主机

“主机”页面上的**导出**和**导入**会把你选择的主机连同其机密，放在同类文件中迁移，并添加到另一台计算机已有的主机中。参见[迁移到另一台计算机](/docs/reference/security/#moving-to-another-computer)。

## 自己复制文件

数据库副本适合用于这台计算机的系统备份，例如 Time Machine、文件历史记录或你自己的脚本。它保留了应用内备份不包含的引擎事件和部署历史，但**不包含钥匙串密钥**：只有在同一台计算机上的同一用户，或钥匙串也随之恢复的计算机上，它才能连同机密一起恢复。在其他任何地方，所有名称、地址和 compose 文件都会恢复，但机密需要[重新输入](#if-the-secrets-are-lost)。

### Homeport 关闭时

退出 Homeport（如果它[关闭到托盘](/docs/guides/settings/#desktop)，请使用托盘中的**退出**），然后复制整个数据文件夹。Homeport 退出后，`homeport.db` 包含所有内容，`-wal` 和 `-shm` 文件也会消失。

### Homeport 运行时

不要在 Homeport 运行时单独复制 `homeport.db`：最新的更改可能仍在 `homeport.db-wal` 中，而在写入过程中复制的副本可能不一致。请改用 `sqlite3` 命令行工具让 SQLite 生成一致的副本（macOS 自带；在 Linux 和 Windows 上，从发行版的 `sqlite3` 软件包或 [sqlite.org](https://sqlite.org/download.html) 安装）：

```sh
# Linux
sqlite3 ~/.local/share/com.homeport.app/homeport.db ".backup '$HOME/homeport-backup.db'"

# macOS
sqlite3 "$HOME/Library/Application Support/com.homeport.app/homeport.db" ".backup '$HOME/homeport-backup.db'"
```

```powershell
# Windows
sqlite3 "$env:APPDATA\com.homeport.app\homeport.db" ".backup '$HOME\homeport-backup.db'"
```

结果是一个完整的文件，旁边没有 `-wal`。要检查它：

```sh
sqlite3 ~/homeport-backup.db "PRAGMA integrity_check;"
```

它会输出 `ok`。

### 恢复副本

1. 退出 Homeport。
2. 在数据文件夹中，把 `homeport.db` 移到别处，如果有 `homeport.db-wal` 和 `homeport.db-shm`，也删除它们：它们属于你要替换的数据库。
3. 把你的副本放到那里，命名为 `homeport.db`。如果你复制的是整个文件夹，请改为把该文件夹放回原处。
4. 打开 Homeport。

由旧版本制作的副本会在 Homeport 打开它时升级。上次由更新版本的 Homeport 使用过的副本会被拒绝且保持不变，并显示[一条指出应安装哪个版本的消息](/docs/reference/troubleshooting/#this-database-was-last-used-by-homeport--or-later)。

## 机密丢失时

在钥匙串重置、使用新的用户账户，或在另一台计算机上打开复制的数据库之后，Homeport 会保留它仍能读取的所有内容，并标记无法读取的内容。不会删除任何东西，非机密的内容（名称、地址、compose 文件、文件夹路径）都完好无损。请重新输入：

| 内容                              | 方法                                                                                                                            |
| --------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| **TLS 主机**                      | 在主机页面上重新加载 CA 证书、客户端证书和客户端密钥，然后选择**保存**。                                                        |
| **使用导入密钥的 SSH 主机**       | 重新粘贴私钥及其口令，然后选择**保存**。                                                                                        |
| **使用 Homeport 密钥的 SSH 主机** | 选择**重新生成密钥**，然后在服务器上[授权新密钥](/docs/hosts/ssh/#authorise-homeports-key)，并删除服务器上旧的 `homeport@` 行。 |
| **通过 HTTPS 连接的 Incus 主机**  | 删除该主机，然后[用新的信任令牌重新添加](/docs/hosts/incus/#over-https-with-a-trust-token)。                                    |
| **Kubernetes 集群**               | 在集群页面上使用**从 kubeconfig 刷新**。                                                                                        |
| **Nomad 集群**                    | 在集群的**设置**中粘贴 ACL 令牌，如果它使用证书，也一并粘贴。                                                                   |
| **堆栈环境变量**                  | 在托管堆栈或同步堆栈的页面上重新输入其环境变量。关联堆栈的环境变量保存在其文件夹的 `.env` 中，Homeport 从未保存过它们。         |
| **镜像仓库登录**                  | 在**设置 → 镜像仓库**中编辑每个登录，并输入其密码或令牌。                                                                       |
| **仓库**                          | 在**设置 → 仓库**中生成**新密钥**并将其添加到 git 托管平台上的仓库，或选择**替换令牌**。                                        |

本地套接字、TCP 和 ssh-agent 主机不保存机密，会继续正常工作。部署历史中记录的环境变量无法重新输入；历史中的 compose 文件仍可读取。

参见故障排除中的[凭据丢失](/docs/reference/troubleshooting/#credentials-lost)。

## 删除你的数据

[删除你的数据](/docs/reference/updating/#removing-your-data)列出了要删除的文件夹和钥匙串条目，以及要从服务器上删除的密钥。
