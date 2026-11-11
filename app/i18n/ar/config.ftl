### The `config` namespace, in العربية (ar).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ar/config.ts:
### change the text there, then run it again.

config-nav = التكوين
config-title = التكوين
config-configMaps = ConfigMaps
config-secrets = Secrets
config-configMap = ConfigMap
config-secret = Secret
config-configMapsSummary = { $count ->
        [zero] لا ConfigMaps
        [one] ConfigMap واحد
        [two] { NUM($count) } ConfigMaps
        [few] { NUM($count) } ConfigMaps
        [many] { NUM($count) } ConfigMap
       *[other] { NUM($count) } ConfigMap
    }
config-secretsSummary = { $count ->
        [zero] لا Secrets
        [one] Secret واحد
        [two] { NUM($count) } Secrets
        [few] { NUM($count) } Secrets
        [many] { NUM($count) } Secret
       *[other] { NUM($count) } Secret
    }
config-searchConfigMaps = البحث في ConfigMaps
config-searchSecrets = البحث في Secrets
config-column-name = الاسم
config-column-namespace = Namespace
config-column-type = النوع
config-column-keys = المفاتيح
config-column-age = العمر
config-tab-data = البيانات
config-tab-overview = نظرة عامة
config-tab-events = الأحداث
config-tab-source = YAML
config-detail-namespace = Namespace
config-detail-type = النوع
config-detail-labels = التسميات
config-detail-annotations = التعليقات التوضيحية
config-noKeys = لا توجد مفاتيح.
config-binary = (بيانات ثنائية)
config-base64 = base64
config-reveal = إظهار
config-revealOne = إظهار { $key }
config-hide = إخفاء
config-copy = نسخ { $key }
config-reveal-failed = تعذّر على Homeport إظهار { $key }
