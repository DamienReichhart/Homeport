### The `workloads` namespace, in 简体中文 (zh-CN).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/zh-CN/workloads.ts:
### change the text there, then run it again.

workloads-nav = 工作负载
workloads-title = 工作负载
workloads-summary = { $count ->
       *[other] { NUM($count) } 个工作负载
    }
workloads-search = 搜索工作负载
workloads-empty = 这里没有内容。
workloads-kind-deployment = Deployment
workloads-kind-statefulSet = StatefulSet
workloads-kind-daemonSet = DaemonSet
workloads-kind-job = Job
workloads-kind-cronJob = CronJob
workloads-one-deployment = Deployment
workloads-one-statefulSet = StatefulSet
workloads-one-daemonSet = DaemonSet
workloads-one-job = Job
workloads-one-cronJob = CronJob
workloads-column-name = 名称
workloads-column-namespace = Namespace
workloads-column-health = 健康状况
workloads-column-ready = 就绪
workloads-column-completions = 完成数
workloads-column-active = 活跃
workloads-column-upToDate = 已更新
workloads-column-schedule = 调度计划
workloads-column-lastSchedule = 上次运行
workloads-column-images = 镜像
workloads-column-age = 存在时间
workloads-tab-overview = 概览
workloads-tab-pods = Pod
workloads-tab-jobs = Job
workloads-tab-logs = 日志
workloads-tab-conditions = 条件
workloads-tab-events = 事件
workloads-tab-source = YAML
workloads-detail-namespace = Namespace
workloads-detail-ready = 就绪
workloads-detail-upToDate = 已更新
workloads-detail-schedule = 调度计划
workloads-detail-lastSchedule = 上次运行
workloads-detail-strategy = 策略
workloads-detail-selector = 选择 Pod 的依据
workloads-detail-owner = 控制者
workloads-detail-age = 存在时间
workloads-detail-labels = 标签
workloads-detail-annotations = 注解
workloads-detail-containers = 容器
workloads-detail-noJobs = 它启动的 Job 中没有仍被保留的。
workloads-container-name = 容器
workloads-container-image = 镜像
workloads-container-ports = 端口
workloads-action-menu = { $name } 的操作
workloads-action-scale = 扩缩
workloads-action-restart = 重启
workloads-action-trigger = 立即运行
workloads-action-suspend = 挂起
workloads-action-resume = 恢复
workloads-scale-title = 扩缩 { $name }
workloads-scale-description = 位于 { $cluster } 上的 Namespace { $namespace }。它当前运行 { $current ->
       *[other] { NUM($current) } 个 Pod
    }。
workloads-scale-replicas = Pod
workloads-scale-autoscaled = 自动扩缩器 { $autoscaler } 管理它的 Pod，因此会在下次检查时把数量改回去。
workloads-scale-toZero = Pod 数为 0 时，它将停止提供服务，直到你再次扩容。
workloads-scale-submit = 扩缩
workloads-scale-done = { $name } 已扩缩到 { $replicas ->
       *[other] { NUM($replicas) } 个 Pod
    }
workloads-scale-failed = Homeport 无法扩缩 { $name }
workloads-restart-title = 重启 { $name }？
workloads-restart-description = 它在 { $cluster } 上 Namespace { $namespace } 中的 Pod 会按其滚动更新策略逐个替换，与 kubectl rollout restart 相同。
workloads-restart-confirm = 重启
workloads-restart-done = 正在重启 { $name }
workloads-restart-failed = Homeport 无法重启 { $name }
workloads-suspend-title = 挂起 { $name }？
workloads-suspend-description = 在你恢复它之前，它不会再在 { $cluster } 上的 Namespace { $namespace } 中启动 Job。已在运行的 Job 会继续运行。
workloads-suspend-confirm = 挂起
workloads-suspend-done = { $name } 已挂起
workloads-suspend-failed = Homeport 无法挂起 { $name }
workloads-resume-done = { $name } 已恢复
workloads-resume-failed = Homeport 无法恢复 { $name }
workloads-trigger-done = 已启动 Job { $job }
workloads-trigger-failed = Homeport 无法从 { $name } 启动 Job
