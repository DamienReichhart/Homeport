### The `podmanpods` namespace, in 简体中文 (zh-CN).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/zh-CN/podmanpods.ts:
### change the text there, then run it again.

podmanpods-nav = Pod
podmanpods-title = Pod
podmanpods-summary = { $count ->
       *[other] { NUM($count) } 个 Pod
    }
podmanpods-search = 搜索 Pod
podmanpods-empty = 没有 Pod
podmanpods-emptyDescription = Pod 将共享同一网络、一起启动和停止的容器组合在一起。先创建一个 Pod，然后在其中运行容器。
podmanpods-new = 新建 Pod
podmanpods-column-name = 名称
podmanpods-column-status = 状态
podmanpods-column-containers = 容器
podmanpods-column-networks = 网络
podmanpods-column-created = 创建时间
podmanpods-running = { $count } 个中有 { $running } 个正在运行
podmanpods-status-running = 运行中
podmanpods-status-degraded = 部分运行
podmanpods-status-paused = 已暂停
podmanpods-status-created = 已创建
podmanpods-status-stopped = 已停止
podmanpods-status-failed = 失败
podmanpods-action-start = 启动
podmanpods-action-stop = 停止
podmanpods-action-restart = 重启
podmanpods-action-pause = 暂停
podmanpods-action-unpause = 恢复
podmanpods-actionOne-start = 启动 { $name }
podmanpods-actionOne-stop = 停止 { $name }
podmanpods-actionOne-restart = 重启 { $name }
podmanpods-actionOne-pause = 暂停 { $name }
podmanpods-actionOne-unpause = 恢复 { $name }
podmanpods-removeOne = 删除 { $name }
podmanpods-done-start = { $name } 已启动
podmanpods-done-stop = { $name } 已停止
podmanpods-done-restart = { $name } 已重启
podmanpods-done-pause = { $name } 已暂停
podmanpods-done-unpause = { $name } 已恢复
podmanpods-removed = 已删除 { $name }，{ $count ->
        [0] 不含容器
       *[other] 含 { NUM($count) } 个容器
    }
podmanpods-created = { $name } 已创建
podmanpods-remove-title = 删除 { $name }？
podmanpods-remove-stopped = { $count ->
        [0] 将删除该 Pod 及其 infra 容器。
       *[other] 将删除该 Pod、其 infra 容器及其 { NUM($count) } 个容器。
    }它们的卷会保留。
podmanpods-remove-running = { $count ->
        [0] 该 Pod 正在运行：它会先停止，然后连同其 infra 容器一起删除。
       *[other] 该 Pod 正在运行：它的 { NUM($count) } 个容器会先停止，然后连同该 Pod 及其 infra 容器一起删除。
    }它们的卷会保留。
podmanpods-remove-stopAndRemove = 停止并删除
podmanpods-detail-containers = { $count ->
        [0] 容器
       *[other] { NUM($count) } 个容器
    }
podmanpods-detail-noContainers = 除 infra 容器外，此 Pod 中尚未运行任何容器。
podmanpods-detail-infra = Infra
podmanpods-detail-settings = 设置
podmanpods-detail-ports = 发布的端口
podmanpods-detail-noPorts = 无
podmanpods-detail-shared = 共享的命名空间
podmanpods-detail-networks = 网络
podmanpods-detail-hostname = 主机名
podmanpods-detail-created = 创建时间
podmanpods-detail-labels = 标签
podmanpods-detail-command = 创建命令
podmanpods-detail-runContainer = 运行容器
podmanpods-detail-runContainerIn = 在 { $name } 中运行容器
podmanpods-newDialog-title = 新建 Pod
podmanpods-newDialog-description = 其中的容器共享同一网络，因此端口在 Pod 上为所有容器统一发布。
podmanpods-newDialog-name = 名称
podmanpods-newDialog-portsHint = 主机端口留空则由 Podman 选择一个空闲端口。在 Pod 中运行的容器无法自行发布端口。
podmanpods-newDialog-labels = 标签
podmanpods-newDialog-create = 创建 Pod
podmanpods-form-nameInvalid = 以字母或数字开头，之后可使用字母、数字、_、. 或 -。
podmanpods-form-portInvalid = 端口是 1 到 65535 之间的数字。
podmanpods-form-labelInvalid = 每行写一个 key=value 标签。
