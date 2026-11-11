### The `config` namespace, in 简体中文 (zh-CN).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/zh-CN/config.ts:
### change the text there, then run it again.

config-nav = 配置
config-title = 配置
config-configMaps = ConfigMap
config-secrets = Secret
config-configMap = ConfigMap
config-secret = Secret
config-configMapsSummary = { $count ->
       *[other] { NUM($count) } 个 ConfigMap
    }
config-secretsSummary = { $count ->
       *[other] { NUM($count) } 个 Secret
    }
config-searchConfigMaps = 搜索 ConfigMap
config-searchSecrets = 搜索 Secret
config-column-name = 名称
config-column-namespace = Namespace
config-column-type = 类型
config-column-keys = 键
config-column-age = 存在时间
config-tab-data = 数据
config-tab-overview = 概览
config-tab-events = 事件
config-tab-source = YAML
config-detail-namespace = Namespace
config-detail-type = 类型
config-detail-labels = 标签
config-detail-annotations = 注解
config-noKeys = 没有键。
config-binary = （二进制数据）
config-base64 = base64
config-reveal = 显示
config-revealOne = 显示 { $key }
config-hide = 隐藏
config-copy = 复制 { $key }
config-reveal-failed = Homeport 无法显示 { $key }
