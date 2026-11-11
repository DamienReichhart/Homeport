### The `overview` namespace, in 简体中文 (zh-CN).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/zh-CN/overview.ts:
### change the text there, then run it again.

overview-nav = 概览
overview-title = 概览
overview-addHost = 添加主机
overview-addCluster = 添加集群
overview-cluster-nodes = { $total } 个 Node 中有 { $ready } 个就绪
overview-cluster-nodesNotAllowed = Node：无权限
overview-cluster-pods = { $total } 个 Pod 中有 { $running } 个正在运行
overview-cluster-podsNotAllowed = Pod：无权限
overview-cluster-notAllowed = 无权限。
overview-cluster-versionTitle = 集群
overview-cluster-version = 版本
overview-cluster-platform = 平台
overview-cluster-server = API 服务器
overview-nomad-region = 区域
overview-nomad-address = 地址
overview-cluster-namespace = Namespace
overview-cluster-allNamespaces = 所有 Namespace
overview-cluster-nodesTitle = Node
overview-cluster-nodesReady = 就绪
overview-cluster-allNodes = 所有 Node
overview-cluster-podsTitle = Pod
overview-cluster-running = { $count } 个运行中
overview-cluster-pending = { $count } 个等待中
overview-cluster-crashing = { $count } 个失败中
overview-cluster-done = { $count } 个已完成或其他
overview-cluster-allPods = 所有 Pod
overview-cluster-unreadyTitle = 未就绪的工作负载
overview-cluster-allReady = 所有 Deployment、StatefulSet 和 DaemonSet 的 Pod 均已就绪。
overview-cluster-warningsTitle = 最近的警告
overview-cluster-noWarnings = 没有警告事件。
overview-cluster-allEvents = 所有事件
overview-host-running = <strong>{ $running }</strong> / { $total } 运行中
overview-host-images = { $count ->
       *[other] { NUM($count) } 个镜像
    }
overview-host-containerdContainers = { $count ->
       *[other] { NUM($count) } 个容器
    }，分布在 { $namespaces ->
       *[other] { NUM($namespaces) } 个命名空间
    }中
overview-host-incusInstances = { $count ->
       *[other] { NUM($count) } 个实例
    }，分布在 { $projects ->
       *[other] { NUM($projects) } 个项目
    }中
overview-incus-title = Incus
overview-incus-version = 版本
overview-incus-os = 系统
overview-incus-arch = 架构
overview-incus-projects = 项目
overview-incus-shown = 当前显示
overview-incus-holds = { $instances ->
       *[other] { NUM($instances) } 个实例
    }，{ $images ->
       *[other] { NUM($images) } 个镜像
    }
overview-incus-instances = 实例
overview-incus-images = 镜像
overview-incus-storage = 存储
overview-containerd-title = containerd
overview-containerd-version = 版本
overview-containerd-revision = 修订版本
overview-containerd-platform = 平台
overview-containerd-namespaces = 命名空间
overview-containerd-shown = 当前显示
overview-containerd-holds = { $containers ->
       *[other] { NUM($containers) } 个容器
    }，{ $images ->
       *[other] { NUM($images) } 个镜像
    }
overview-containerd-containers = 容器
overview-containerd-images = 镜像
overview-containerd-snapshots = 快照
overview-host-cpu = { $cpus ->
       *[other] { NUM($cpus) } 个 CPU
    }的 { $percent }
overview-host-cpuTitle = 容器 CPU 使用率，每个 CPU 计 100%
overview-host-memoryTitle = 容器内存使用量
overview-engine-title = 引擎
overview-engine-name = 名称
overview-engine-version = 版本
overview-engine-os = 操作系统
overview-engine-kernel = 内核
overview-engine-architecture = 架构
overview-engine-resources = 资源
overview-engine-resourcesValue = { $cpus ->
       *[other] { NUM($cpus) } 个 CPU
    } · { $memory }
overview-engine-storageDriver = 存储驱动
overview-engine-rootDir = 根目录
overview-disk-title = 磁盘使用情况
overview-disk-cleanUp = 清理…
overview-disk-column-type = 类型
overview-disk-column-total = 总数
overview-disk-column-active = 使用中
overview-disk-column-size = 大小
overview-disk-column-reclaimable = 可回收
overview-disk-images = 镜像
overview-disk-containers = 容器
overview-disk-volumes = 卷
overview-disk-buildCache = 构建缓存
overview-disk-partial = 此 Docker Engine 早于 29 版，报告的信息较少：镜像大小为估算值，卷和构建缓存的大小不可用。
overview-events-title = 最近的事件
overview-events-all = 所有事件
overview-events-empty = 自 Homeport 连接以来未发生任何事件。
overview-prune-title = 清理
overview-prune-description = 选择要从此主机删除的内容。此操作无法撤销。
overview-prune-confirm = 清理
overview-prune-confirmWithVolumes = 删除，包括卷数据
overview-prune-containers = 已停止的容器
overview-prune-containers-detail = 删除所有未在运行的容器。
overview-prune-images = 悬空镜像
overview-prune-images-detail = 构建和拉取遗留下的无标签层。
overview-prune-allImages = 所有未使用的镜像
overview-prune-allImages-detail = 同时删除未被容器使用的有标签镜像。
overview-prune-networks = 未使用的网络
overview-prune-networks-detail = 未连接任何容器的自定义网络。
overview-prune-buildCache = 构建缓存
overview-prune-buildCache-detail = 缓存的构建层。
overview-prune-volumes = 未使用的卷
overview-prune-volumes-detail = 删除所有未被容器使用的卷（包括命名卷）中的数据。
overview-pruned = 已删除 { $containers ->
       *[other] { NUM($containers) } 个容器
    }、{ $images ->
       *[other] { NUM($images) } 个镜像
    }、{ $networks ->
       *[other] { NUM($networks) } 个网络
    }和 { $volumes ->
       *[other] { NUM($volumes) } 个卷
    }——回收了 { $space }。
overview-palette-prune = 清理所选主机
overview-host-onBoard = 在看板上查看其容器
