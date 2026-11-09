---
title: Helm Release
description: 通过本机上的 helm CLI 列出、安装、升级、回滚和卸载集群的 Helm Release，并管理 Chart 仓库。
---

集群侧边栏中的 **Helm** 显示其上的 [Helm](https://helm.sh) Release：与在终端中运行 `helm list` 看到的 Release 相同，无论它们是否通过 Homeport 安装。

## Homeport 需要什么

Homeport 在**本**机上运行 3 或更高版本的 `helm` 命令。在 macOS 上用 `brew install helm` 安装，在 Windows 上用 `winget install Helm.Helm`（或 Chocolatey、Scoop）安装，在 Linux 上用 `snap install helm --classic`、发行版的软件包或 helm.sh 提供的脚本安装。**设置 → 本机**显示 Homeport 找到的 helm；没有 helm 时，Helm 页面会说明你的系统需要安装什么，集群的其他所有页面都能正常使用。

Homeport 不会把你的 kubeconfig 交给 helm。对于每条 helm 命令，它会在本机的回环地址上打开一个通往该集群的私有连接，并给 helm 一个指向该连接的临时 kubeconfig，其中只有一次性令牌，没有任何集群凭据。Homeport 会在每个请求经过时为其加上该集群自身的凭据（证书、令牌、exec 插件的令牌），与其他所有页面相同，并在命令结束时删除该 kubeconfig。因此，helm 能访问 Homeport 能访问的每个集群，包括使用 exec 插件的云集群，而且不会有任何凭据写入磁盘。

Helm 将其 Release 以 Secret 的形式保存在每个 Release 所在的 Namespace 中：Homeport 不会在集群上安装任何内容。

## Release

列表显示每个 Release 的名称、Namespace、Chart 及 Chart 版本、应用版本、修订版本、状态以及上次更改时间。在右上角选择一个 Namespace，或选择全部。helm 没有实时更新，因此列表打开时每 30 秒重新查询一次，**刷新**会立即查询。

**失败**的 Release 以红色标记，**等待中**的 Release（正在等待安装、升级或回滚：`pending-install`、`pending-upgrade`、`pending-rollback`）以琥珀色标记：等待中的 Release 在该操作结束前处于锁定状态。

如果你的凭据无法列出每个 Namespace 中的 Secret，Homeport 会改为列出该集群默认 Namespace 中的 Release 并说明这一点；选择另一个 Namespace 即可查看其 Release。

## 安装 Chart

Release 标签页上的**安装 Chart**（或 <kbd>n</kbd>）会在所选集群上安装一个 Chart：

1. **选择 Chart。** 搜索 helm 的[仓库](#chart-仓库)中的 Chart 并点击**使用**，或者在**来自 OCI 镜像仓库**下提供一个引用，例如 `oci://registry-1.docker.io/bitnamicharts/nginx`，这样无需添加仓库即可安装。
2. **选择其版本、名称和 Namespace。** 仓库 Chart 的版本会被列出，最新的在最前；OCI Chart 的版本需手动输入，留空则使用最新版本。Release 起初以 Chart 命名：最多 53 个小写字母、数字、连字符和点。Namespace 为页面显示的那个（或集群的默认 Namespace），如果不存在则会被创建，除非你取消勾选该选项。
3. **编辑其 Values。** 编辑器起初显示 Chart 的默认 Values（`helm show values`）；修改你需要的内容，**重置为 Chart 的默认值**可将其恢复。Values 在 helm 运行之前会作为 YAML 进行检查，并通过标准输入传给 helm——绝不会写入文件。
4. **预览**（可选）：helm 借助集群（`--dry-run=server`）用你的 Values 渲染该 Chart，Homeport 会显示它将应用的清单（包括 hook）以及 Chart 的说明。集群上不会发生任何更改。
5. **安装。** helm 的输出会像 `helm install` 打印的那样实时显示。勾选**等待其资源就绪**后，helm 会在给定的分钟数内（默认 5 分钟）等待该 Release 的 Deployment、Pod 和 Service 就绪，如果未能就绪则将该 Release 标记为失败。随后**打开 Release** 会打开其页面。

本地文件夹或 `.tgz` 文件中的 Chart 无法从 Homeport 安装；需要登录的 OCI 镜像仓库中的 Chart 也不行，除非你已在终端中用 `helm registry login` 登录。

## Release 的页面

点击一个 Release 即可打开它：

- **历史**——每个修订版本，最新的在最前，附有其日期、状态、Chart、应用版本以及 helm 对所发生情况的描述（`Install complete`、`Upgrade "web" failed: …`）。
- **Values**——安装或升级时提供的 Values（**用户提供**），或渲染 Chart 时使用的所有 Values，包括其默认值（**计算结果**）。
- **说明**——Chart 在安装后打印的内容（其 `NOTES.txt`）。
- **清单**——该修订版本渲染出的 Kubernetes 对象，以 YAML 形式显示。
- **资源**——按 kind、名称和 Namespace 列出的这些对象，每个都能在 Homeport 中打开其页面：Deployment、Pod、Service 等有其专门页面，其余的则打开其在 [API 资源](/docs/clusters/api-resources/)下的页面。

Values、说明、清单和资源显示当前修订版本；在它们上方选择一个更早的修订版本，可查看它安装时使用的内容。

## 升级 Release

Release 页面上的**升级**可更改其 Chart 版本、其 Values，或两者都改：

- **Chart**——用于升级的 Chart。helm 不会记录 Chart 的来源，因此 Homeport 会在你的仓库中查找同名的 Chart，包含该 Release 当前版本的仓库排在最前。用**其他 Chart…**可选择另一个，格式为 `repository/chart` 或 `oci://` 引用——从 OCI 镜像仓库安装的 Release 就需要这样做。
- **Chart 版本**——该 Chart 的版本，最新的在最前；OCI Chart 则手动输入。
- **Values**——编辑器起初显示该 Release 的用户提供的 Values。它们会替换该 Release 的 Values：其余部分由 Chart 的默认值补全。**Values 的更改**会将它们与当前 Values 并排显示，**重置为当前 Values** 可将其恢复。
- **等待其资源就绪**，最长等待给定的分钟数。

**预览**会借助集群渲染此次升级而不做任何更改，并显示新的清单以及它与当前清单相比的变化。**升级**会运行 `helm upgrade`，并实时显示 helm 的输出。每次升级都会在**历史**中产生一个新的修订版本。

## 回滚

在**历史**中，任何较早修订版本上的**回滚到修订版本 N** 会先询问你，注明当前和回滚后的 Chart 版本及应用版本，然后运行 `helm rollback`：helm 会以一个新的修订版本恢复该修订版本的 Chart 和 Values，因此回滚本身也可以被回滚。勾选**等待其资源就绪**可让 helm 等待资源就绪。

在安装、升级或回滚中断后卡在**等待中**的 Release 处于锁定状态：在该操作结束前，helm 拒绝更改它。将其回滚到上次部署的修订版本即可解锁。

## 卸载

Release 页面上的**卸载**会在运行 `helm uninstall` 之前注明该 Release、其 Namespace 和集群，并列出将随之删除的对象。Chart 标记为保留的对象（`helm.sh/resource-policy: keep`）以及 StatefulSet 的 PersistentVolumeClaim 会留在集群上。**保留其历史**会让该 Release 以 `uninstalled` 状态留在列表中，以便回滚；不勾选时，该 Release 会从列表中消失。**等待其对象被删除**会让 helm 等待这些对象被删除。

## Chart 仓库

**Chart 仓库**是 Helm 页面的第二个标签页，列出 helm 在本机上知道的仓库：与在终端中运行 `helm repo list` 得到的列表相同，由所有集群共享。Homeport 不保留自己的列表，因此在终端中添加的仓库会出现在这里，在这里添加的仓库也会出现在 `helm repo list` 中。

- **添加仓库**会要求输入名称（字母、数字、点、连字符和下划线）和仓库的 `https://` URL，例如 `https://charts.bitnami.com/bitnami`，并下载其索引。
- **全部更新**，或某个仓库上的**更新**，会像 `helm repo update` 一样下载最新索引，并实时显示 helm 的输出。上次更新之后发布的 Chart 只有在更新后才会显示。
- **删除**会在你确认后忘记该仓库。从它安装的 Release 会继续运行。

**查找 Chart** 会像 `helm search repo` 一样按名称或描述搜索这些仓库中的 Chart：添加 Bitnami 的仓库后输入 `postgres`，即可看到其 PostgreSQL Chart 及其最新的 Chart 版本和应用版本。

Homeport 只添加公开仓库。需要登录的仓库请在终端中用 `helm repo add --username …` 添加，helm 会像往常一样保存其密码；之后它会出现在这里，并与其他仓库一样使用。OCI 镜像仓库中的 Chart（`oci://…`）不需要仓库。

## 无法正常工作时

- **未安装 helm CLI**：按上文安装 helm 3，然后重新打开该页面。
- **Helm 2**：Homeport 只支持 Helm 3 及更高版本。请安装当前版本的 helm。
- **无权限**：helm 从 Secret 中读取 Release；列出它们需要对该 Namespace 中的 `secrets` 拥有 `list` 权限。详细信息会注明被拒绝的内容。
- **另一次安装、升级或回滚正在进行中**：该 Release 被一项等待中的操作锁定。请等待其完成，或回滚到上次部署的修订版本。
- **已存在同名的 Release**：Release 名称在一个 Namespace 中是唯一的。请另选一个名称，或升级现有的 Release。
- **找不到 Chart 或版本**：请检查名称和版本；仓库 Chart 需要先添加其仓库并[更新](#chart-仓库)。要升级其 Chart 不在你任何仓库中的 Release，请先添加该仓库。
- **Values 不是有效的 YAML**：消息会注明行号；修正后重试。
- **放弃了等待**：勾选**等待**时，某个资源未能及时就绪。请查看该 Release 的 Pod 和事件，找出卡住的地方。
- **不是 Chart 仓库，或无法访问**：该 URL 没有提供 `index.yaml`，或者本机无法访问它。详细信息会显示 helm 的输出。
- **未连接**：helm 通过 Homeport 中该集群的连接进行访问；集群的页面会说明它为什么未连接。
