### The `workloads` namespace, in العربية (ar).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ar/workloads.ts:
### change the text there, then run it again.

workloads-nav = أحمال العمل
workloads-title = أحمال العمل
workloads-summary = { $count ->
        [zero] لا أحمال عمل
        [one] حمل عمل واحد
        [two] حملا عمل
        [few] { NUM($count) } أحمال عمل
        [many] { NUM($count) } حمل عمل
       *[other] { NUM($count) } حمل عمل
    }
workloads-search = البحث في أحمال العمل
workloads-empty = لا شيء هنا.
workloads-kind-deployment = Deployments
workloads-kind-statefulSet = StatefulSets
workloads-kind-daemonSet = DaemonSets
workloads-kind-job = Jobs
workloads-kind-cronJob = CronJobs
workloads-one-deployment = Deployment
workloads-one-statefulSet = StatefulSet
workloads-one-daemonSet = DaemonSet
workloads-one-job = Job
workloads-one-cronJob = CronJob
workloads-column-name = الاسم
workloads-column-namespace = Namespace
workloads-column-health = السلامة
workloads-column-ready = جاهز
workloads-column-completions = مرات الاكتمال
workloads-column-active = نشط
workloads-column-upToDate = محدّث
workloads-column-schedule = الجدول الزمني
workloads-column-lastSchedule = آخر تشغيل
workloads-column-images = الصور
workloads-column-age = العمر
workloads-tab-overview = نظرة عامة
workloads-tab-pods = Pods
workloads-tab-jobs = Jobs
workloads-tab-logs = السجلات
workloads-tab-conditions = الشروط
workloads-tab-events = الأحداث
workloads-tab-source = YAML
workloads-detail-namespace = Namespace
workloads-detail-ready = جاهز
workloads-detail-upToDate = محدّث
workloads-detail-schedule = الجدول الزمني
workloads-detail-lastSchedule = آخر تشغيل
workloads-detail-strategy = الاستراتيجية
workloads-detail-selector = يحدد Pods حسب
workloads-detail-owner = يتحكم فيه
workloads-detail-age = العمر
workloads-detail-labels = التسميات
workloads-detail-annotations = التعليقات التوضيحية
workloads-detail-containers = الحاويات
workloads-detail-noJobs = لم يبدأ أي Job لا يزال محفوظًا.
workloads-container-name = الحاوية
workloads-container-image = الصورة
workloads-container-ports = المنافذ
workloads-action-menu = إجراءات { $name }
workloads-action-scale = تغيير عدد النسخ
workloads-action-restart = إعادة التشغيل
workloads-action-trigger = التشغيل الآن
workloads-action-suspend = تعليق
workloads-action-resume = استئناف
workloads-scale-title = تغيير عدد نسخ { $name }
workloads-scale-description = في الـ Namespace { $namespace } على { $cluster }. يشغّل حاليًا { $current ->
        [zero] 0 Pod
        [one] Pod واحدًا
        [two] { NUM($current) } Pods
        [few] { NUM($current) } Pods
        [many] { NUM($current) } Pod
       *[other] { NUM($current) } Pod
    }.
workloads-scale-replicas = Pods
workloads-scale-autoscaled = تدير أداة التحجيم التلقائي { $autoscaler } الـ Pods الخاصة به، لذا ستعيد العدد إلى قيمته في فحصها التالي.
workloads-scale-toZero = مع 0 Pod يتوقف عن الخدمة حتى تزيد عدد نسخه مجددًا.
workloads-scale-submit = تطبيق
workloads-scale-done = تم تغيير عدد نسخ { $name } إلى { $replicas ->
        [zero] 0 Pod
        [one] Pod واحد
        [two] { NUM($replicas) } Pods
        [few] { NUM($replicas) } Pods
        [many] { NUM($replicas) } Pod
       *[other] { NUM($replicas) } Pod
    }
workloads-scale-failed = تعذّر على Homeport تغيير عدد نسخ { $name }
workloads-restart-title = هل تريد إعادة تشغيل { $name }؟
workloads-restart-description = تُستبدل الـ Pods الخاصة به في الـ Namespace { $namespace } على { $cluster } واحدًا تلو الآخر، كما تحدد استراتيجية الطرح الخاصة به، مثل kubectl rollout restart.
workloads-restart-confirm = إعادة التشغيل
workloads-restart-done = جارٍ إعادة تشغيل { $name }
workloads-restart-failed = تعذّر على Homeport إعادة تشغيل { $name }
workloads-suspend-title = هل تريد تعليق { $name }؟
workloads-suspend-description = لن يبدأ أي Jobs أخرى في الـ Namespace { $namespace } على { $cluster } حتى تستأنفه. تستمر الـ Jobs قيد التشغيل بالفعل.
workloads-suspend-confirm = تعليق
workloads-suspend-done = تم تعليق { $name }
workloads-suspend-failed = تعذّر على Homeport تعليق { $name }
workloads-resume-done = تم استئناف { $name }
workloads-resume-failed = تعذّر على Homeport استئناف { $name }
workloads-trigger-done = تم بدء الـ Job { $job }
workloads-trigger-failed = تعذّر على Homeport بدء Job من { $name }
