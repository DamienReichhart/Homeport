---
title: 日志和终端
description: 跟随查看 Pod 或整个工作负载的日志，了解容器崩溃的原因，并在 Pod 的容器中打开终端。
---

Pod 的页面有**日志**和**终端**标签页，工作负载的页面有一个汇集其所有 Pod 的**日志**标签页：即日常使用的 `kubectl logs` 和 `kubectl exec -it`，通过 Kubernetes API 并以集群自身的凭据完成。

## 日志

**日志**会在容器写入日志时实时显示。它与主机上容器所用的查看器相同：

- **历史**：从最后 100、1,000 或 10,000 行开始，或从全部日志开始。
- **起始时间**：只显示最近若干分钟、小时或天内的行。
- **时间戳**：Kubernetes 记录每行的时间。
- **跟随**：让最新的行保持在视图中；关闭它可以在新行不断到来时回看。
- **查找**、**清除**和**下载**（下载为文本文件，内容为目前显示的行）。

有多个容器的 Pod 会提供**容器**选择器，其中标出了 init 容器。它会默认打开 `kubectl` 会选择的那个容器：由 Pod 的 `kubectl.kubernetes.io/default-container` 注解指定的容器，否则为其第一个应用容器。

Kubernetes 会将容器的标准输出和标准错误合并保存，因此不像主机上的容器那样提供 stdout/stderr 筛选。

离开该标签页时日志流会停止，容器停止时也会给出提示。

### 为什么崩溃了？

当容器重启过时，**上一个实例**会显示最近一次重启之前那次运行的日志——对处于 `CrashLoopBackOff` 的 Pod 而言，就是崩溃的那次运行——与 `kubectl logs --previous` 相同。对于从未重启过的容器，它处于关闭状态，因为没有可显示的内容。

当容器尚未启动时（`ContainerCreating`，或在等待再次启动期间处于 `CrashLoopBackOff`），日志会在原处说明这一点及其原因。其 Pod 的页面会说明它在等待什么。

### 工作负载的日志

Deployment、StatefulSet、DaemonSet 或 Job 的**日志**标签页会在其所有 Pod 写入日志时将它们交织显示，每行都以其所属 Pod 的专属颜色标注该 Pod，与 `kubectl logs -l app=api --prefix` 类似。选择一个 Pod 可以单独查看它；当模板中有多个容器时，还可以选择容器。

## 终端

**终端**会在 Pod 的某个正在运行的容器中打开一个 shell：镜像中有 `bash` 时使用 `bash`，否则使用 `sh`。你也可以在输入框中输入一条命令（`psql -U app shop`、`top`），**连接**会像 shell 那样对其加引号后运行。

它是一个真正的 TTY：`vi`、`top`、颜色和方向键都能正常使用，调整窗口大小也会调整容器中终端的大小。进程退出时，终端会显示其退出码；**重新连接**会启动另一个进程。离开该标签页会结束容器中的进程。

### 无法打开时

- **没有 shell**：极简（distroless）镜像根本没有 shell。请输入镜像中存在的命令，或者在终端中用 `kubectl debug -it <pod> --image=busybox --target=<container>` 挂接一个调试容器；Homeport 不会创建调试容器。
- **未运行**：处于等待状态或已结束的容器无法运行任何内容。已完成的 Job 的 Pod 已永久停止。
- **无权限**：打开终端需要对 `pods/exec` 拥有 `create` 权限，读取日志需要对 `pods/log` 拥有 `get` 权限，而只读角色往往缺少这些权限。Homeport 会说明缺少哪一项，并给出用于检查的 `kubectl auth can-i` 命令。

Homeport 不会向 Pod 复制文件或从中复制文件（`kubectl cp`）。
