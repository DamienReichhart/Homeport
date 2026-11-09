---
title: 扩缩、重启和删除
description: 扩缩 Deployment 和 StatefulSet、重启 rollout、挂起和运行 CronJob，以及删除集群中的任何对象，每项操作都会先进行注明对象名称的确认。
---

对集群的日常更改只需在其页面上点击一下即可完成，每项更改都像 `kubectl` 一样，以该集群自身的凭据发送到 Kubernetes API。列表会通过其 watch 实时跟进更改。

每项会删除或替换内容的操作都会先询问你，确认信息会注明集群、namespace、kind 和名称，因此更改绝不会意外落到错误的集群或 namespace 上。

## 工作负载

工作负载的页面在页头中提供其操作；在**工作负载**页面上，每一行的 **⋯** 菜单中都有这些操作。

| 操作         | 适用于                             | 作用，相当于                                             |
| ------------ | ---------------------------------- | -------------------------------------------------------- |
| **扩缩**     | Deployment、StatefulSet            | `kubectl scale --replicas=N`                             |
| **重启**     | Deployment、StatefulSet、DaemonSet | `kubectl rollout restart`                                |
| **挂起**     | CronJob                            | `kubectl patch cronjob … -p '{"spec":{"suspend":true}}'` |
| **恢复**     | 已挂起的 CronJob                   | 相同命令，改为 `false`                                   |
| **立即运行** | CronJob                            | `kubectl create job --from=cronjob/…`                    |
| **删除**     | 以上全部                           | `kubectl delete`（其 Pod 和 Job 会在后台随之删除）       |

### 扩缩

**扩缩**会询问要运行多少个 Pod，并显示当前运行的数量。扩缩到 `0` 会停止该工作负载，直到你再次扩容。如果某个 HorizontalPodAutoscaler 管理着该工作负载，对话框会说明这一点：自动扩缩器会在下次检查时把数量改回去，因此请改为调整自动扩缩器的最小值和最大值。

### 重启

**重启**会在 Pod 模板上添加注解 `kubectl.kubernetes.io/restartedAt`，与 `kubectl rollout restart` 的做法完全相同：工作负载会按照其策略推出一组新的 Pod（对 Deployment 而言是一个新的 ReplicaSet），旧的 Pod 会在新 Pod 就绪后删除。

### CronJob

**挂起**会阻止 CronJob 启动新的 Job（已在运行的 Job 会继续运行），**恢复**则让它重新开始启动 Job；已挂起的 CronJob 显示为**已暂停**。**立即运行**会立即根据其模板启动一个 Job，命名为 `<cronjob>-manual-<id>`，归该 CronJob 所有，并像 kubectl 一样标记为 `cronjob.kubernetes.io/instantiate: manual`。它会显示在该 CronJob 的 **Job** 标签页下。

## 删除

每个对象的页面——工作负载、Pod、Service、Ingress、ConfigMap、Secret、Node，或 **API 资源**下的任何对象——都在页头中提供**删除**。在列表中，勾选若干行后，**删除 N 个**会在一次列出所有这些对象的确认之后将它们全部删除。

对象以 `Background` 传播策略删除，与 `kubectl delete` 相同：对象会立即删除，其拥有的内容（Deployment 的 ReplicaSet 和 Pod、CronJob 的 Job）由垃圾回收器删除。

确认信息会说明接下来会发生什么：

- **由控制器拥有的 Pod**——控制器为 ReplicaSet、StatefulSet、DaemonSet 或 Job——会重新出现：控制器会启动一个新的 Pod 取而代之。要彻底去掉它，请删除或扩缩该工作负载。
- **Namespace** 会带走其中的一切：其工作负载、Pod、Service、ConfigMap 和 Secret。
- **Node** 会从集群的列表中删除，事先不会被腾空。

## 无法生效时

- **无权限**：你的凭据不允许执行的更改会注明其动词和资源——“你的凭据不允许在 shop 中对 deployments 执行‘patch’。”——并给出用于检查的 `kubectl auth can-i` 命令。
- **期间已被更改**：如果在你查看之后、执行操作之前有其他人更改了该对象，Homeport 会说明这一点；请重新查看后重试。
- **已不存在**：删除已被删除的内容时，会提示它已不存在。
