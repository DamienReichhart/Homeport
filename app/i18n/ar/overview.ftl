### The `overview` namespace, in العربية (ar).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ar/overview.ts:
### change the text there, then run it again.

overview-nav = نظرة عامة
overview-title = نظرة عامة
overview-addHost = إضافة مضيف
overview-addCluster = إضافة عنقود
overview-cluster-nodes = Nodes الجاهزة: { $ready } من { $total }
overview-cluster-nodesNotAllowed = Nodes: غير مسموح
overview-cluster-pods = Pods قيد التشغيل: { $running } من { $total }
overview-cluster-podsNotAllowed = Pods: غير مسموح
overview-cluster-notAllowed = غير مسموح.
overview-cluster-versionTitle = العنقود
overview-cluster-version = الإصدار
overview-cluster-platform = النظام الأساسي
overview-cluster-server = خادم API
overview-nomad-region = المنطقة
overview-nomad-address = العنوان
overview-cluster-namespace = Namespace
overview-cluster-allNamespaces = كل Namespaces
overview-cluster-nodesTitle = Nodes
overview-cluster-nodesReady = جاهزة
overview-cluster-allNodes = كل Nodes
overview-cluster-podsTitle = Pods
overview-cluster-running = { $count } قيد التشغيل
overview-cluster-pending = { $count } قيد الانتظار
overview-cluster-crashing = { $count } فاشلة
overview-cluster-done = { $count } منتهية أو غير ذلك
overview-cluster-allPods = كل Pods
overview-cluster-unreadyTitle = أحمال عمل غير جاهزة
overview-cluster-allReady = كل Deployment وStatefulSet وDaemonSet جاهزة الـ Pods.
overview-cluster-warningsTitle = التحذيرات الأخيرة
overview-cluster-noWarnings = لا توجد أحداث تحذير.
overview-cluster-allEvents = كل الأحداث
overview-host-running = <strong>{ $running }</strong> / { $total } قيد التشغيل
overview-host-images = { $count ->
        [zero] لا صور
        [one] صورة واحدة
        [two] صورتان
        [few] { NUM($count) } صور
        [many] { NUM($count) } صورة
       *[other] { NUM($count) } صورة
    }
overview-host-containerdContainers = { $count ->
        [zero] لا حاويات
        [one] حاوية واحدة
        [two] حاويتان
        [few] { NUM($count) } حاويات
        [many] { NUM($count) } حاوية
       *[other] { NUM($count) } حاوية
    } في { $namespaces ->
        [zero] 0 مساحة أسماء
        [one] مساحة أسماء واحدة
        [two] مساحتي أسماء
        [few] { NUM($namespaces) } مساحات أسماء
        [many] { NUM($namespaces) } مساحة أسماء
       *[other] { NUM($namespaces) } مساحة أسماء
    }
overview-host-incusInstances = { $count ->
        [zero] لا مثيلات
        [one] مثيل واحد
        [two] مثيلان
        [few] { NUM($count) } مثيلات
        [many] { NUM($count) } مثيلًا
       *[other] { NUM($count) } مثيل
    } في { $projects ->
        [zero] 0 مشروع
        [one] مشروع واحد
        [two] مشروعين
        [few] { NUM($projects) } مشاريع
        [many] { NUM($projects) } مشروعًا
       *[other] { NUM($projects) } مشروع
    }
overview-incus-title = Incus
overview-incus-version = الإصدار
overview-incus-os = النظام
overview-incus-arch = البنية
overview-incus-projects = المشاريع
overview-incus-shown = المعروض
overview-incus-holds = { $instances ->
        [zero] لا مثيلات
        [one] مثيل واحد
        [two] مثيلان
        [few] { NUM($instances) } مثيلات
        [many] { NUM($instances) } مثيلًا
       *[other] { NUM($instances) } مثيل
    }، { $images ->
        [zero] لا صور
        [one] صورة واحدة
        [two] صورتان
        [few] { NUM($images) } صور
        [many] { NUM($images) } صورة
       *[other] { NUM($images) } صورة
    }
overview-incus-instances = المثيلات
overview-incus-images = الصور
overview-incus-storage = التخزين
overview-containerd-title = containerd
overview-containerd-version = الإصدار
overview-containerd-revision = المراجعة
overview-containerd-platform = النظام الأساسي
overview-containerd-namespaces = مساحات الأسماء
overview-containerd-shown = المعروضة
overview-containerd-holds = { $containers ->
        [zero] لا حاويات
        [one] حاوية واحدة
        [two] حاويتان
        [few] { NUM($containers) } حاويات
        [many] { NUM($containers) } حاوية
       *[other] { NUM($containers) } حاوية
    }، { $images ->
        [zero] لا صور
        [one] صورة واحدة
        [two] صورتان
        [few] { NUM($images) } صور
        [many] { NUM($images) } صورة
       *[other] { NUM($images) } صورة
    }
overview-containerd-containers = الحاويات
overview-containerd-images = الصور
overview-containerd-snapshots = اللقطات
overview-host-cpu = { $percent } من { $cpus ->
        [zero] 0 وحدة معالجة
        [one] وحدة معالجة واحدة
        [two] وحدتي معالجة
        [few] { NUM($cpus) } وحدات معالجة
        [many] { NUM($cpus) } وحدة معالجة
       *[other] { NUM($cpus) } وحدة معالجة
    }
overview-host-cpuTitle = استخدام الحاويات للمعالج، 100% لكل وحدة معالجة
overview-host-memoryTitle = استخدام الحاويات للذاكرة
overview-engine-title = المحرك
overview-engine-name = الاسم
overview-engine-version = الإصدار
overview-engine-os = نظام التشغيل
overview-engine-kernel = النواة
overview-engine-architecture = البنية
overview-engine-resources = الموارد
overview-engine-resourcesValue = { $cpus ->
        [zero] 0 وحدة معالجة
        [one] وحدة معالجة واحدة
        [two] وحدتا معالجة
        [few] { NUM($cpus) } وحدات معالجة
        [many] { NUM($cpus) } وحدة معالجة
       *[other] { NUM($cpus) } وحدة معالجة
    } · { $memory }
overview-engine-storageDriver = برنامج تشغيل التخزين
overview-engine-rootDir = الدليل الجذر
overview-disk-title = استخدام القرص
overview-disk-cleanUp = تنظيف…
overview-disk-column-type = النوع
overview-disk-column-total = الإجمالي
overview-disk-column-active = قيد الاستخدام
overview-disk-column-size = الحجم
overview-disk-column-reclaimable = القابل للاسترداد
overview-disk-images = الصور
overview-disk-containers = الحاويات
overview-disk-volumes = وحدات التخزين
overview-disk-buildCache = ذاكرة التخزين المؤقت للبناء
overview-disk-partial = محرك Docker هذا أقدم من الإصدار 29 ويُبلغ عن معلومات أقل: أحجام الصور تقديرية، وأحجام وحدات التخزين وذاكرة التخزين المؤقت للبناء غير متاحة.
overview-events-title = الأحداث الأخيرة
overview-events-all = كل الأحداث
overview-events-empty = لم يحدث شيء منذ اتصال Homeport.
overview-prune-title = تنظيف
overview-prune-description = اختر ما تريد إزالته من هذا المضيف. لا يمكن التراجع عن ذلك.
overview-prune-confirm = تنظيف
overview-prune-confirmWithVolumes = حذف، بما في ذلك بيانات وحدات التخزين
overview-prune-containers = الحاويات المتوقفة
overview-prune-containers-detail = يزيل كل حاوية ليست قيد التشغيل.
overview-prune-images = الصور المعلّقة
overview-prune-images-detail = طبقات بلا وسوم خلّفتها عمليات البناء والسحب.
overview-prune-allImages = كل الصور غير المستخدمة
overview-prune-allImages-detail = يزيل أيضًا الصور ذات الوسوم التي لا تستخدمها أي حاوية.
overview-prune-networks = الشبكات غير المستخدمة
overview-prune-networks-detail = الشبكات المخصصة التي لا تتصل بها أي حاوية.
overview-prune-buildCache = ذاكرة التخزين المؤقت للبناء
overview-prune-buildCache-detail = طبقات البناء المخزنة مؤقتًا.
overview-prune-volumes = وحدات التخزين غير المستخدمة
overview-prune-volumes-detail = يحذف البيانات في كل وحدة تخزين لا تستخدمها أي حاوية، بما في ذلك وحدات التخزين المسماة.
overview-pruned = تمت إزالة { $containers ->
        [zero] 0 حاوية
        [one] حاوية واحدة
        [two] حاويتين
        [few] { NUM($containers) } حاويات
        [many] { NUM($containers) } حاوية
       *[other] { NUM($containers) } حاوية
    }، و{ $images ->
        [zero] 0 صورة
        [one] صورة واحدة
        [two] صورتين
        [few] { NUM($images) } صور
        [many] { NUM($images) } صورة
       *[other] { NUM($images) } صورة
    }، و{ $networks ->
        [zero] 0 شبكة
        [one] شبكة واحدة
        [two] شبكتين
        [few] { NUM($networks) } شبكات
        [many] { NUM($networks) } شبكة
       *[other] { NUM($networks) } شبكة
    }، و{ $volumes ->
        [zero] 0 وحدة تخزين
        [one] وحدة تخزين واحدة
        [two] وحدتي تخزين
        [few] { NUM($volumes) } وحدات تخزين
        [many] { NUM($volumes) } وحدة تخزين
       *[other] { NUM($volumes) } وحدة تخزين
    } — وتم استرداد { $space }.
overview-palette-prune = تنظيف المضيف المحدد
overview-host-onBoard = حاوياته على اللوحة
