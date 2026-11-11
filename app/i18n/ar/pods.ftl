### The `pods` namespace, in العربية (ar).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ar/pods.ts:
### change the text there, then run it again.

pods-nav = Pods
pods-title = Pods
pods-summary = { $count ->
        [zero] لا Pods
        [one] Pod واحد
        [two] { NUM($count) } Pods
        [few] { NUM($count) } Pods
        [many] { NUM($count) } Pod
       *[other] { NUM($count) } Pod
    }
pods-search = البحث في Pods
pods-empty = لا توجد Pods هنا.
pods-column-name = الاسم
pods-column-namespace = Namespace
pods-column-status = الحالة
pods-column-ready = جاهز
pods-column-restarts = مرات إعادة التشغيل
pods-column-node = Node
pods-column-age = العمر
pods-tab-overview = نظرة عامة
pods-tab-conditions = الشروط
pods-tab-events = الأحداث
pods-tab-source = YAML
pods-detail-readyOf = الحاويات الجاهزة: { $ready } من { $total }
pods-detail-namespace = Namespace
pods-detail-node = Node
pods-detail-ip = عنوان IP للـ Pod
pods-detail-hostIp = عنوان IP للـ Node
pods-detail-qos = فئة QoS
pods-detail-serviceAccount = حساب الخدمة
pods-detail-owner = يتحكم فيه
pods-detail-age = العمر
pods-detail-labels = التسميات
pods-detail-annotations = التعليقات التوضيحية
pods-container-name = الحاوية
pods-container-init = init
pods-container-image = الصورة
pods-container-state = الحالة
pods-container-ready = جاهزة
pods-container-isReady = نعم
pods-container-notReady = لا
pods-container-restarts = مرات إعادة التشغيل
pods-container-restartsWhy = { $count } (الأخيرة: { $reason })
pods-container-started = وقت البدء
pods-container-ports = المنافذ
pods-state-running = قيد التشغيل
pods-state-waiting = قيد الانتظار
pods-state-terminated = منتهية
pods-tab-logs = السجلات
pods-logs-container = الحاوية
pods-logs-initContainer = { $name } (init)
pods-logs-previous = النسخة السابقة
pods-logs-noRestart = لم يُعَد تشغيلها، لذا لا توجد نسخة سابقة.
pods-logs-pod = Pod
pods-logs-allPods = { $count ->
        [zero] لا Pods
        [one] الـ Pod
        [two] كلا الـ Pods ({ NUM($count) })
        [few] كل الـ Pods ({ NUM($count) })
        [many] كل الـ Pods ({ NUM($count) })
       *[other] كل الـ Pods ({ NUM($count) })
    }
pods-logs-noPods = لا يشغّل أي Pod حاليًا.
pods-tab-terminal = الطرفية
pods-terminal-shell = الصدفة
pods-terminal-command = الأمر
pods-terminal-connect = اتصال
pods-terminal-reconnect = إعادة الاتصال
pods-terminal-notRunning = هذه الحاوية لا تعمل.
pods-terminal-exited = انتهت العملية.
pods-terminal-exitedWithCode = انتهت العملية بالرمز { $code }.
pods-column-cpu = المعالج
pods-column-memory = الذاكرة
pods-tab-metrics = المقاييس
pods-metrics-noSample = ليس لدى metrics-server عيّنة من هذا الـ Pod بعد: ربما بدأ للتو أو لا يعمل.
