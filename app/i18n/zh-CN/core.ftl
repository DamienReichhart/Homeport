### The `core` namespace, in 简体中文 (zh-CN).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/zh-CN/core.ts:
### change the text there, then run it again.

core-host-engineVersion = 引擎 { $version }
core-host-podmanVersion = Podman { $version }
core-host-containerdVersion = containerd { $version }
core-host-incusVersion = Incus { $version }
core-engine-docker = Docker 兼容
core-engine-containerd = containerd
core-engine-incus = Incus
core-unsupported-title = { $name } 无法显示此页面
core-unsupported-docker = 此页面需要 Docker 兼容引擎，而 { $name } 运行的是 { $engine }。
core-unsupported-podman = 此页面用于 Podman，而 { $name } 运行的是其他 Docker 兼容引擎。请在切换器中选择一个 Podman 主机。
core-unsupported-containerd = 此页面用于 containerd，而 { $name } 运行的是 { $engine }。请在切换器中选择一个 containerd 主机。
core-unsupported-incus = 此页面用于 Incus，而 { $name } 运行的是 { $engine }。请在切换器中选择一个 Incus 主机。
core-unsupported-overview = 转到概览
core-cluster-kind-kubernetes = Kubernetes
core-cluster-kind-nomad = Nomad
core-cluster-unsupported-title = { $name } 无法显示此页面
core-cluster-unsupported-description = 此页面用于 { $kind } 集群，而 { $name } 是 { $current } 集群。请在切换器中选择一个 { $kind } 集群。
core-containerd-namespace = 命名空间
core-containerd-holds = { $containers ->
       *[other] { NUM($containers) } 个容器
    }，{ $images ->
       *[other] { NUM($images) } 个镜像
    }
core-containerd-manage = 管理命名空间
core-containerd-rememberFailed = Homeport 无法记住你选择的命名空间
core-incus-project = 项目
core-incus-holds = { $instances ->
       *[other] { NUM($instances) } 个实例
    }，{ $images ->
       *[other] { NUM($images) } 个镜像
    }
core-incus-rememberFailed = Homeport 无法记住你选择的项目
core-selection-rememberFailed = Homeport 无法记住你的选择
core-connection-local = 本地
core-status-connected = 已连接
core-status-connecting = 正在连接
core-status-disabled = 已停用
core-status-unreachable = 无法访问
core-status-unreachableDetail = { $message } — { $secs } 秒后重试
core-status-unauthorized = 访问被拒绝
core-status-hostKeyUntrusted = 服务器密钥未受信任
core-status-hostKeyUntrustedDetail = 请核对指纹 { $fingerprint }，信任后即可连接。
core-status-hostKeyMismatch = 服务器密钥已更改
core-status-hostKeyMismatchDetail = 应为 { $expected }，实际为 { $actual }。
core-status-credentialsLost = 凭据丢失
core-status-credentialsLostDetail = 无法解密已保存的凭据。请重新输入，或重新生成密钥。
core-status-keychainUnavailable = 钥匙串不可用
core-status-invalid = 配置错误
core-swarm-label = { $host } 的 Swarm
core-swarm-labelUnknown = Swarm { $id }
core-swarm-gone-title = 此 Swarm 已不再可知
core-swarm-gone-description = Homeport 连接的主机中已没有属于它的主机。请将它的某个管理节点添加为主机来管理它。
core-cluster-serverVersion = Kubernetes { $version }
core-cluster-nomadVersion = Nomad { $version }，区域 { $region }
core-cluster-nomadRegion = Nomad，区域 { $region }
core-cluster-credentialsLostDetail = 无法解密已保存的凭据。请从 kubeconfig 刷新该集群。
core-cluster-reconnectFailed = Homeport 无法重新连接到集群
core-cluster-gate-none = 未选择集群
core-cluster-gate-noneDescription = 在侧边栏中选择一个集群，或从 kubeconfig 添加集群。
core-cluster-gate-add = 添加集群
core-cluster-gate-openSettings = 打开集群设置
core-localClusters-title = 本机上的集群
core-localClusters-description = 在本地运行、可通过你的 kubeconfig 访问的 Kubernetes。
core-localClusters-add = 添加
core-localClusters-added = 已添加
core-localClusters-notRunning = 未运行
core-localClusters-addFailed = Homeport 无法添加该集群
core-localClusters-missing = { $tool } 集群 { $name } 正在运行，但你的 kubeconfig 无法访问它。使用以下命令添加：
core-localClusters-tool-dockerDesktop = Docker Desktop
core-localClusters-tool-orbstack = OrbStack
core-localClusters-tool-rancherDesktop = Rancher Desktop
core-localClusters-tool-kind = kind
core-localClusters-tool-k3d = k3d
core-localClusters-tool-minikube = minikube
core-namespace-label = Namespace
core-namespace-all = 所有 Namespace
core-namespace-type = Namespace
core-namespace-cantList = 你的凭据无法列出 Namespace。请输入你有权使用的 Namespace。
core-namespace-rememberFailed = Homeport 无法记住该 Namespace
core-health-ready = 就绪
core-health-progressing = 进行中
core-health-degraded = 降级
core-health-failed = 失败
core-health-paused = 已暂停
core-events-type = 类型
core-events-reason = 原因
core-events-message = 消息
core-events-count = 次数
core-events-lastSeen = 最后出现
core-events-none = 没有事件。Kubernetes 只保留约一小时内的事件。
core-conditions-type = 条件
core-conditions-status = 状态
core-conditions-reason = 原因
core-conditions-message = 消息
core-conditions-since = 起始时间
core-conditions-none = 未报告任何条件。
core-labels-none = 无
core-yaml-label = YAML
core-list-empty = 这里没有内容。
core-yaml-copy = 复制 YAML
core-gate-welcome-title = 欢迎使用 Homeport
core-gate-welcome-description = 添加一个 Docker Engine 即可开始——本机的，或通过 SSH 或 TLS 访问的远程引擎——也可以从 kubeconfig 添加 Kubernetes 集群。
core-gate-addAnother = 添加其他主机
core-gate-addCluster = 添加集群
core-gate-connecting = 正在连接到 { $name }…
core-gate-statusTitle = { $name }：{ $status }
core-gate-retryingOnItsOwn = Homeport 会在 { $secs } 秒后自动重试。
core-gate-retryNow = 立即重试
core-gate-openSettings = 打开主机设置
core-localEngines-title = 在本机上找到
core-localEngines-description = 在本地套接字上响应的引擎。
core-localEngines-add = 添加
core-freshness-check = 检查新版镜像
core-freshness-checking = 正在检查…
core-freshness-allUpToDate = 所有镜像均为最新
core-freshness-newerCount = { $count ->
       *[other] { NUM($count) } 个镜像
    }有新版本
core-freshness-newerOne = 已发布新版 { $reference }
core-freshness-newerMany = 已发布新版镜像：{ $references }
core-delete-title = 删除 { $kind } { $name }？
core-delete-titleMany = 删除 { $count } 个 { $kind } 对象？
core-delete-titleObjects = 删除 { $count } 个对象？
core-delete-one = { $kind } { $name } 将从 { $cluster } 中删除。
core-delete-oneIn = { $kind } { $name } 将从 { $cluster } 上的 Namespace { $namespace } 中删除。
core-delete-many = 以下对象将从 { $cluster } 中删除：{ $names }。
core-delete-manyIn = 以下对象将从 { $cluster } 上的 Namespace { $namespace } 中删除：{ $names }。
core-delete-namespace = 该 Namespace 中的所有内容都会一并删除：工作负载、Pod、Service、ConfigMap 和 Secret。
core-delete-recreated = 它由 { $kind } { $name } 控制，因此会启动一个新 Pod 来替代它。
core-delete-pods = 它的 Pod 会一并删除。
core-delete-cronJob = 它不再启动 Job；它已启动的 Job 会一并删除。
core-delete-node = Homeport 不会先腾空它：它的 Pod 会一直保留，直到该 Node 重新注册或这些 Pod 被重新调度。
core-delete-confirm = 删除
core-delete-failed = Homeport 无法删除 { $name }
core-delete-done = { $count ->
        [1] 已删除 { $name }
       *[other] 已删除 { NUM($count) } 个对象
    }
core-delete-action = 删除
core-delete-selected = 删除 { $count } 个
core-apply-preview = 预览更改
core-apply-apply = 应用
core-apply-force = 仍然应用
core-apply-stale = 自此次预览以来，YAML 已更改。
core-apply-noChanges = 没有更改：集群中已是此内容。
core-apply-confirmTitle = 将 { $count ->
       *[other] { NUM($count) } 项更改
    }应用到 { $cluster }？
core-apply-confirmDescription = Homeport 以 homeport 字段管理器的身份通过服务器端应用来应用它，如预览所示。
core-apply-forceTitle = 接手冲突的字段？
core-apply-forceDescription = 在 { $cluster } 上管理这些字段的工具可能会把它们改回去，或停止管理它们。Homeport 将成为它们的管理者。
core-apply-done = 已应用到 { $cluster }
core-apply-diffOf = 对 { $kind } { $name } 的更改
core-apply-action-created = 已创建
core-apply-action-changed = 已更改
core-apply-action-unchanged = 没有更改
core-apply-action-failed = 无法应用
core-yaml-edit = 编辑
core-yaml-editTitle = 编辑 { $kind } { $name }
core-yaml-editDescription = 在 { $cluster } 上。修改 YAML，预览它的更改，然后应用。
core-metrics-cpu = CPU
core-metrics-memory = 内存
core-metrics-used = 已用
core-metrics-request = 请求
core-metrics-limit = 限制
core-metrics-allocatable = 可分配
core-metrics-ofLimit = 限制的 { $percent }
core-metrics-ofRequest = 请求的 { $percent }
core-metrics-ofAllocatable = 可分配量的 { $percent }
core-metrics-notInstalled-title = 未安装 metrics-server
core-metrics-notInstalled-body = 此集群未提供 Metrics API，而 CPU 和内存数据都来自它。Homeport 不会在集群上安装任何东西：安装 metrics-server 后，这些数据就会显示在这里。
core-metrics-notInstalled-link = 如何安装 metrics-server
core-metrics-notResponding = Metrics API 没有响应：metrics-server 可能正在启动，或没有运行。Homeport 每 15 秒重试一次。
core-metrics-waiting = 正在等待 metrics-server 的第一个样本…
core-metrics-kept = 每 15 秒一个样本，在此页面打开期间保留，最多 15 分钟。
