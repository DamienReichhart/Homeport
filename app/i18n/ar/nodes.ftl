### The `nodes` namespace, in العربية (ar).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ar/nodes.ts:
### change the text there, then run it again.

nodes-nav = Nodes
nodes-title = Nodes
nodes-summary = { $count ->
        [zero] لا Nodes
        [one] Node واحد
        [two] { NUM($count) } Nodes
        [few] { NUM($count) } Nodes
        [many] { NUM($count) } Node
       *[other] { NUM($count) } Node
    }
nodes-search = البحث في Nodes
nodes-ready = جاهز
nodes-notReady = غير جاهز
nodes-cordoned = الجدولة معطّلة
nodes-column-name = الاسم
nodes-column-status = الحالة
nodes-column-roles = الأدوار
nodes-column-version = الإصدار
nodes-column-ip = عنوان IP الداخلي
nodes-column-platform = النظام الأساسي
nodes-column-cpu = وحدات المعالجة
nodes-column-memory = الذاكرة
nodes-column-age = العمر
nodes-tab-overview = نظرة عامة
nodes-tab-pods = Pods
nodes-tab-conditions = الشروط
nodes-tab-events = الأحداث
nodes-tab-source = YAML
nodes-detail-roles = الأدوار
nodes-detail-version = Kubelet
nodes-detail-os = صورة نظام التشغيل
nodes-detail-kernel = النواة
nodes-detail-runtime = وقت تشغيل الحاويات
nodes-detail-addresses = العناوين
nodes-detail-taints = Taints
nodes-detail-noTaints = لا شيء
nodes-detail-age = العمر
nodes-detail-labels = التسميات
nodes-detail-resources = الموارد
nodes-resource-name = المورد
nodes-resource-capacity = السعة
nodes-resource-allocatable = القابل للتخصيص
nodes-column-cpuUsed = المعالج المستخدم
nodes-column-memoryUsed = الذاكرة المستخدمة
nodes-tab-metrics = المقاييس
