### The `search` namespace, in 简体中文 (zh-CN).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/zh-CN/search.ts:
### change the text there, then run it again.

search-kind-container = 容器
search-kind-stack = 堆栈
search-kind-image = 镜像
search-kind-volume = 卷
search-kind-network = 网络
search-onHostState = { $host } · { $state ->
        [running] 运行中
       *[other] 已停止
    }
search-showAll = 显示全部 { $count } 个
search-skipped = { $count ->
       *[other] { NUM($count) } 个主机
    }未搜索（未连接）
