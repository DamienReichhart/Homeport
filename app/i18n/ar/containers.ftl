### The `containers` namespace, in العربية (ar).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ar/containers.ts:
### change the text there, then run it again.

containers-nav = الحاويات
containers-title = الحاويات
containers-summary = { $running } قيد التشغيل من { $count }
containers-runContainer = تشغيل حاوية
containers-search = البحث في الحاويات
containers-onlyRunning = قيد التشغيل فقط
containers-empty-title = لا توجد حاويات
containers-empty-description = شغّل حاوية من صورة، أو انشر مكدسًا.
containers-noMatch = لا توجد حاويات مطابقة.
containers-column-name = الاسم
containers-column-image = الصورة
containers-column-status = الحالة
containers-column-ports = المنافذ
containers-column-cpu = المعالج
containers-column-memory = الذاكرة
containers-column-created = تاريخ الإنشاء
containers-selectOne = تحديد { $name }
containers-selectStack = تحديد المكدس { $stack }
containers-stack-running = { $running }/{ $count } قيد التشغيل
containers-stack-swarm = Swarm
containers-stack-open = فتح المكدس
containers-inPod = في Pod الخاص بـ Podman { $pod }
containers-fromUnit = أنشأتها وحدة Quadlet { $unit }
containers-selected = المحدد: { $count }
containers-action-start = تشغيل
containers-action-stop = إيقاف
containers-action-restart = إعادة التشغيل
containers-action-pause = إيقاف مؤقت
containers-action-resume = استئناف
containers-action-kill = إنهاء قسري
containers-action-rename = إعادة التسمية
containers-action-edit = تحرير
containers-action-duplicate = تكرار
containers-action-logs = السجلات
containers-action-terminal = فتح الطرفية
containers-action-commit = حفظ كصورة…
containers-action-export = تصدير نظام الملفات…
containers-kill-title = هل تريد إنهاء { $name } قسريًا؟
containers-kill-description = تتلقى العملية الرئيسية SIGKILL فورًا، دون فرصة لإيقاف تشغيلها بشكل سليم.
containers-done-start = { $count ->
        [zero] لم تُشغَّل أي حاوية
        [one] تم تشغيل الحاوية
        [two] تم تشغيل حاويتين
        [few] تم تشغيل { NUM($count) } حاويات
        [many] تم تشغيل { NUM($count) } حاوية
       *[other] تم تشغيل { NUM($count) } حاوية
    }
containers-done-stop = { $count ->
        [zero] لم تُوقف أي حاوية
        [one] تم إيقاف الحاوية
        [two] تم إيقاف حاويتين
        [few] تم إيقاف { NUM($count) } حاويات
        [many] تم إيقاف { NUM($count) } حاوية
       *[other] تم إيقاف { NUM($count) } حاوية
    }
containers-done-restart = { $count ->
        [zero] لم يُعَد تشغيل أي حاوية
        [one] تمت إعادة تشغيل الحاوية
        [two] تمت إعادة تشغيل حاويتين
        [few] تمت إعادة تشغيل { NUM($count) } حاويات
        [many] تمت إعادة تشغيل { NUM($count) } حاوية
       *[other] تمت إعادة تشغيل { NUM($count) } حاوية
    }
containers-done-pause = { $count ->
        [zero] لم تُوقف أي حاوية مؤقتًا
        [one] تم إيقاف الحاوية مؤقتًا
        [two] تم إيقاف حاويتين مؤقتًا
        [few] تم إيقاف { NUM($count) } حاويات مؤقتًا
        [many] تم إيقاف { NUM($count) } حاوية مؤقتًا
       *[other] تم إيقاف { NUM($count) } حاوية مؤقتًا
    }
containers-done-unpause = { $count ->
        [zero] لم تُستأنف أي حاوية
        [one] تم استئناف الحاوية
        [two] تم استئناف حاويتين
        [few] تم استئناف { NUM($count) } حاويات
        [many] تم استئناف { NUM($count) } حاوية
       *[other] تم استئناف { NUM($count) } حاوية
    }
containers-done-kill = { $count ->
        [zero] لم تُنهَ أي حاوية
        [one] تم إنهاء الحاوية قسريًا
        [two] تم إنهاء حاويتين قسريًا
        [few] تم إنهاء { NUM($count) } حاويات قسريًا
        [many] تم إنهاء { NUM($count) } حاوية قسريًا
       *[other] تم إنهاء { NUM($count) } حاوية قسريًا
    }
containers-done-remove = { $count ->
        [zero] لم تُزل أي حاوية
        [one] تمت إزالة الحاوية
        [two] تمت إزالة حاويتين
        [few] تمت إزالة { NUM($count) } حاويات
        [many] تمت إزالة { NUM($count) } حاوية
       *[other] تمت إزالة { NUM($count) } حاوية
    }
containers-remove-title = هل تريد إزالة { $name }؟
containers-remove-titleMany = هل تريد إزالة { $count ->
        [zero] 0 حاوية
        [one] حاوية واحدة
        [two] حاويتين
        [few] { NUM($count) } حاويات
        [many] { NUM($count) } حاوية
       *[other] { NUM($count) } حاوية
    }؟
containers-remove-deleted = { $count ->
        [zero] تُحذف الحاويات وطبقاتها القابلة للكتابة ووحدات التخزين المجهولة الخاصة بها.
        [one] تُحذف الحاوية وطبقتها القابلة للكتابة ووحدات التخزين المجهولة الخاصة بها.
        [two] تُحذف الحاويتان وطبقتاهما القابلتان للكتابة ووحدات التخزين المجهولة الخاصة بهما.
        [few] تُحذف الحاويات وطبقاتها القابلة للكتابة ووحدات التخزين المجهولة الخاصة بها.
        [many] تُحذف الحاويات وطبقاتها القابلة للكتابة ووحدات التخزين المجهولة الخاصة بها.
       *[other] تُحذف الحاويات وطبقاتها القابلة للكتابة ووحدات التخزين المجهولة الخاصة بها.
    }
containers-remove-running = ستُوقف أولًا الحاويات قيد التشغيل ({ $count }).
containers-remove-namedKept = يُحتفظ بوحدات التخزين المسماة.
containers-detail-gone = لم تعد هذه الحاوية موجودة.
containers-detail-backToList = العودة إلى الحاويات
containers-tab-logs = السجلات
containers-tab-inspect = فحص
containers-tab-health = السلامة
containers-tab-exec = تنفيذ
containers-tab-files = الملفات
containers-tab-stats = الإحصاءات
containers-tab-processes = العمليات
containers-notRunning = الحاوية ليست قيد التشغيل.
containers-logs-tail-100 = آخر 100
containers-logs-tail-1000 = آخر 1 000
containers-logs-tail-10000 = آخر 10 000
containers-logs-tail-all = الكل
containers-logs-since-all = أي وقت
containers-logs-since-300 = آخر 5 دقائق
containers-logs-since-3600 = آخر ساعة
containers-logs-since-86400 = آخر 24 ساعة
containers-logs-history = السجل السابق
containers-logs-sinceLabel = منذ
containers-logs-timestamps = الطوابع الزمنية
containers-logs-follow = متابعة
containers-logs-find = البحث في السجلات
containers-logs-previous = التطابق السابق
containers-logs-next = التطابق التالي
containers-logs-clear = مسح
containers-logs-download = تنزيل السجلات
containers-logs-saveTitle = حفظ السجلات
containers-logs-saved = تم حفظ السجلات
containers-logs-end = — نهاية السجلات —
containers-inspect-raw = JSON الخام
containers-inspect-container = الحاوية
containers-inspect-health = السلامة
containers-inspect-environment = البيئة
containers-inspect-mounts = نقاط الربط
containers-inspect-ports = المنافذ
containers-inspect-networks = الشبكات
containers-inspect-labels = التسميات
containers-inspect-id = المعرّف
containers-inspect-image = الصورة
containers-inspect-created = تاريخ الإنشاء
containers-inspect-state = الحالة
containers-inspect-started = وقت البدء
containers-inspect-entrypoint = نقطة الدخول
containers-inspect-command = الأمر
containers-inspect-workingDir = دليل العمل
containers-inspect-user = المستخدم
containers-inspect-restartPolicy = سياسة إعادة التشغيل
containers-inspect-networkMode = وضع الشبكة
containers-inspect-exited = { $status } (رمز الخروج { $code })
containers-inspect-outOfMemory = { $state }، نفدت الذاكرة
containers-inspect-restarted = { $policy } · أُعيد تشغيلها { $count } مرة
containers-inspect-restartedMax = { $policy } (بحد أقصى { $max }) · أُعيد تشغيلها { $count } مرة
containers-inspect-imageDefault = افتراضي الصورة
containers-inspect-notPublished = غير منشور
containers-inspect-readWrite = قراءة وكتابة
containers-inspect-readOnly = للقراءة فقط
containers-inspect-failingStreak = { $count } إخفاقات متتالية
containers-inspect-noHealthcheck = لا تحدد الصورة أي فحص سلامة.
containers-inspect-column-started = وقت البدء
containers-inspect-column-exit = الخروج
containers-inspect-column-output = الإخراج
containers-inspect-column-variable = المتغير
containers-inspect-column-value = القيمة
containers-inspect-column-type = النوع
containers-inspect-column-source = المصدر
containers-inspect-column-destination = الوجهة
containers-inspect-column-mode = الوضع
containers-inspect-column-container = الحاوية
containers-inspect-column-publishedOn = منشور على
containers-inspect-column-network = الشبكة
containers-inspect-column-ip = IP
containers-inspect-column-gateway = البوابة
containers-inspect-column-aliases = الأسماء المستعارة
containers-inspect-column-label = التسمية
containers-health-none = لا تملك هذه الحاوية فحص سلامة.
containers-health-title = فحص السلامة
containers-health-command = الأمر
containers-health-timingLabel = التوقيت
containers-health-timing = كل { $interval }، المهلة { $timeout }، { $retries ->
        [zero] 0 محاولات إعادة
        [one] محاولة إعادة واحدة
        [two] محاولتا إعادة
        [few] { NUM($retries) } محاولات إعادة
        [many] { NUM($retries) } محاولة إعادة
       *[other] { NUM($retries) } محاولة إعادة
    }، فترة البدء { $start }
containers-health-failingStreak = إخفاقات متتالية
containers-health-lastResults = آخر النتائج
containers-health-noRuns = لم يُجرَ أي فحص بعد.
containers-health-noOutput = لا يوجد إخراج.
containers-health-showLess = عرض أقل
containers-health-showMore = { $count ->
        [zero] عرض 0 سطر إضافي
        [one] عرض سطر إضافي واحد
        [two] عرض سطرين إضافيين
        [few] عرض { NUM($count) } أسطر إضافية
        [many] عرض { NUM($count) } سطرًا إضافيًا
       *[other] عرض { NUM($count) } سطر إضافي
    }
containers-health-exit = رمز الخروج { $code }
containers-health-took = · استغرق { $duration }
containers-exec-startFirst = شغّل الحاوية لفتح طرفية.
containers-exec-shell = الصدفة
containers-exec-shellNumber = الصدفة { $number }
containers-exec-closeOne = إغلاق { $title }
containers-exec-new = طرفية جديدة
containers-exec-command = الأمر
containers-exec-commandPlaceholder = bash، أو sh إذا لم يتوفر
containers-exec-user = المستخدم
containers-exec-directory = الدليل
containers-exec-default = افتراضي
containers-exec-open = فتح
containers-exec-none = لا توجد طرفية مفتوحة.
containers-exec-exited = [انتهت العملية]
containers-exec-exitedWithCode = [انتهت العملية بالرمز { $code }]
containers-files-startFirst = شغّل الحاوية لاستعراض ملفاتها.
containers-files-saveOne = حفظ { $name }
containers-files-uploadTo = الرفع إلى { $path }
containers-files-uploadHere = الرفع هنا
containers-files-saved = تم حفظ { $name }
containers-files-savedArchive = تم حفظ { $name } كأرشيف .tar
containers-files-uploaded = { $count ->
        [zero] لم يُرفع أي ملف
        [one] تم رفع ملف واحد
        [two] تم رفع ملفين
        [few] تم رفع { NUM($count) } ملفات
        [many] تم رفع { NUM($count) } ملفًا
       *[other] تم رفع { NUM($count) } ملف
    } إلى { $path }
containers-stats-startFirst = شغّل الحاوية لرؤية استخدامها للموارد.
containers-stats-cpu = المعالج
containers-stats-memory = الذاكرة
containers-stats-memoryNow = · { $used } من { $limit }
containers-stats-used = المستخدم
containers-stats-network = الشبكة
containers-stats-networkNow = · { $received } وارد، { $sent } صادر
containers-stats-received = المستلم
containers-stats-sent = المُرسَل
containers-stats-disk = إدخال/إخراج القرص
containers-stats-processes = · { $count ->
        [zero] لا عمليات
        [one] عملية واحدة
        [two] عمليتان
        [few] { NUM($count) } عمليات
        [many] { NUM($count) } عملية
       *[other] { NUM($count) } عملية
    }
containers-stats-read = القراءة
containers-stats-written = الكتابة
containers-run-title = تشغيل حاوية
containers-run-description = تُسحب الصورة أولًا إذا لم تكن على هذا المضيف.
containers-run-submit = تشغيل
containers-run-starting = جارٍ البدء…
containers-run-started = بدأت الحاوية
containers-run-image = الصورة
containers-run-name = الاسم
containers-run-namePlaceholder = يُنشأ تلقائيًا إذا تُرك فارغًا
containers-run-nameKept = محفوظ. استخدم «إعادة التسمية» لتغييره.
containers-run-command = الأمر
containers-run-commandPlaceholder = الافتراضي للصورة
containers-run-environment = البيئة
containers-run-network = الشبكة
containers-run-pod = Pod
containers-run-noPod = لا شيء
containers-run-podOwnsNetwork = تتشارك الحاويات في { $pod } شبكته: المنافذ والشبكة واسم المضيف وDNS هي الخاصة بالـ Pod.
containers-run-defaultNetwork = الافتراضية (bridge)
containers-run-restart = إعادة التشغيل
containers-run-restartPolicy = سياسة إعادة التشغيل
containers-run-restart-no = أبدًا
containers-run-restart-onFailure = عند الفشل
containers-run-restart-unlessStopped = ما لم تُوقف
containers-run-restart-always = دائمًا
containers-run-autoRemove = إزالة الحاوية عند انتهائها
containers-run-ports = المنافذ
containers-run-hostPort = منفذ المضيف
containers-run-hostPortPlaceholder = منفذ المضيف (أي منفذ)
containers-run-containerPort = منفذ الحاوية
containers-run-protocol = البروتوكول
containers-run-removePort = إزالة المنفذ
containers-run-addPort = إضافة منفذ
containers-run-volumes = وحدات التخزين
containers-run-source = المصدر
containers-run-sourcePlaceholder = اسم وحدة التخزين أو /host/path
containers-run-containerPath = المسار في الحاوية
containers-run-readOnly = للقراءة فقط
containers-run-removeVolume = إزالة وحدة التخزين
containers-run-addVolume = إضافة وحدة تخزين
containers-run-removeValue = إزالة { $value }
containers-run-error-size = اكتب حجمًا مثل 512m أو 2g.
containers-run-error-cpus = اكتب عدد وحدات المعالجة مثل 0.5 أو 2.
containers-run-error-cpuShares = اكتب عددًا صحيحًا مثل 512.
containers-run-error-memorySwap = اكتب حجمًا مثل 1g، أو -1 لذاكرة تبديل غير محدودة.
containers-run-error-gpuCount = اكتب عدد وحدات GPU، مثل 1.
containers-run-error-duration = اكتب مدة مثل 30s أو 1m30s.
containers-run-error-retries = اكتب عددًا صحيحًا مثل 3.
containers-advanced-title = خيارات متقدمة
containers-advanced-hasErrors = توجد أخطاء
containers-advanced-resources = الموارد
containers-advanced-labels = التسميات
containers-advanced-security = الأمان
containers-advanced-devices = الأجهزة
containers-advanced-healthcheck = فحص السلامة
containers-advanced-network = هوية الشبكة
containers-advanced-logging = التسجيل
containers-advanced-cpus = حد المعالج
containers-advanced-cpusHint = وحدات المعالجة، مثل 0.5 أو 2.
containers-advanced-cpusPlaceholder = كل وحدات المعالجة
containers-advanced-cpuShares = حصص المعالج
containers-advanced-cpuSharesHint = الوزن مقارنةً بالحاويات الأخرى؛ القيمة الافتراضية 1024.
containers-advanced-memory = حد الذاكرة
containers-advanced-memoryHint = مثل 512m أو 2g.
containers-advanced-memoryPlaceholder = بلا حد
containers-advanced-memoryReservation = حجز الذاكرة
containers-advanced-memoryReservationHint = حد مرن، يُلتزم به عندما تنقص ذاكرة المضيف.
containers-advanced-memorySwap = حد الذاكرة وذاكرة التبديل
containers-advanced-memorySwapHint = الذاكرة مع ذاكرة التبديل؛ -1 لذاكرة تبديل غير محدودة.
containers-advanced-memorySwapPlaceholder = ضعف حد الذاكرة
containers-advanced-none = لا شيء
containers-advanced-labelsHint = زوج key=value واحد في كل سطر، كما تقرؤها الوكلاء العكسية وأدوات النسخ الاحتياطي.
containers-advanced-capAdd = إضافة صلاحيات
containers-advanced-capAddHint = مثل NET_ADMIN لشبكة VPN.
containers-advanced-capDrop = إسقاط صلاحيات
containers-advanced-capDropHint = تُسقط ALL كل الصلاحيات.
containers-advanced-privileged = مميزة
containers-advanced-privilegedTitle = تحصل الحاوية على وصول كامل إلى المضيف
containers-advanced-privilegedWarning = يمكنها استخدام كل جهاز وصلاحية، ويمكن لأي شيء يعمل فيها السيطرة على المضيف. فضّل إضافة الصلاحيات والأجهزة التي تحتاج إليها.
containers-advanced-hostDevice = جهاز المضيف
containers-advanced-containerDevice = جهاز الحاوية
containers-advanced-samePath = المسار نفسه
containers-advanced-permissions = الأذونات
containers-advanced-removeDevice = إزالة الجهاز
containers-advanced-addDevice = إضافة جهاز
containers-advanced-gpus = وحدات GPU
containers-advanced-gpusHint = يتطلب تثبيت مجموعة أدوات الحاويات الخاصة بمورّد GPU على المضيف.
containers-advanced-gpus-all = كل وحدات GPU
containers-advanced-gpus-count = عدد من وحدات GPU
containers-advanced-gpus-ids = وحدات GPU محددة
containers-advanced-gpuCount = العدد
containers-advanced-gpuIds = فهارس GPU أو معرّفات UUID
containers-advanced-gpuIdsHint = مفصولة بفواصل.
containers-advanced-health-image = فحص الصورة نفسها، إن وُجد
containers-advanced-health-custom = أمر
containers-advanced-health-disabled = لا شيء (تعطيل فحص الصورة)
containers-advanced-healthCommand = أمر الفحص
containers-advanced-healthCommandHint = يُشغَّل باستخدام /bin/sh -c؛ رمز الخروج 0 يعني سليمة.
containers-advanced-interval = الفاصل الزمني
containers-advanced-timeout = المهلة
containers-advanced-retries = محاولات الإعادة
containers-advanced-startPeriod = فترة البدء
containers-advanced-hostname = اسم المضيف
containers-advanced-hostnamePlaceholder = معرّف الحاوية
containers-advanced-domainname = اسم النطاق
containers-advanced-dns = خوادم DNS
containers-advanced-dnsHint = عنوان واحد في كل سطر.
containers-advanced-dnsSearch = نطاقات بحث DNS
containers-advanced-dnsSearchHint = واحد في كل سطر.
containers-advanced-extraHosts = أسماء مضيفين إضافية
containers-advanced-extraHostsHint = زوج name:address واحد في كل سطر؛ وتشير host-gateway إلى عنوان المضيف.
containers-advanced-logDriver = برنامج تشغيل السجلات
containers-advanced-logDriverHint = مع none، تبقى علامة التبويب «السجلات» فارغة.
containers-advanced-logDriverPlaceholder = الافتراضي للمحرك
containers-advanced-logOptions = خيارات برنامج التشغيل
containers-advanced-logOptionsHint = زوج key=value واحد في كل سطر؛ يدوّر max-size وmax-file سجلات json-file.
containers-rename-title = إعادة تسمية { $name }
containers-rename-newName = الاسم الجديد
containers-duplicate-title = تكرار { $name }
containers-duplicate-sharedPorts = ينشر { $name } بالفعل { $count ->
        [zero] منافذ المضيف
        [one] منفذ المضيف
        [two] منفذي المضيف
        [few] منافذ المضيف
        [many] منافذ المضيف
       *[other] منافذ المضيف
    } { $ports }.
containers-duplicate-sharedPortsFix = { $count ->
        [zero] لا يمكن أن تبدأ النسخة ما دامت الحاويتان تستخدمانها
        [one] لا يمكن أن تبدأ النسخة ما دامت الحاويتان تستخدمانه
        [two] لا يمكن أن تبدأ النسخة ما دامت الحاويتان تستخدمانهما
        [few] لا يمكن أن تبدأ النسخة ما دامت الحاويتان تستخدمانها
        [many] لا يمكن أن تبدأ النسخة ما دامت الحاويتان تستخدمانها
       *[other] لا يمكن أن تبدأ النسخة ما دامت الحاويتان تستخدمانها
    }: غيّر منفذ المضيف، أو اتركه فارغًا ليختار المحرك منفذًا.
containers-edit-title = تحرير { $name }
containers-edit-description = يعيد التطبيق إنشاء الحاوية بهذه الإعدادات، مع الاحتفاظ باسمها وشبكاتها ووحدات التخزين المسماة.
containers-edit-apply = تطبيق
containers-edit-recreating = جارٍ إعادة الإنشاء…
containers-edit-recreated = تمت إعادة إنشاء الحاوية
containers-edit-confirmTitle = هل تريد إعادة إنشاء { $name }؟
containers-edit-nothingLost = لا يُفقد شيء آخر: يُحتفظ باسمها وشبكاتها ووحدات التخزين المسماة.
containers-edit-keptBut = يُحتفظ باسمها وشبكاتها ووحدات التخزين المسماة، ولكن ليس بما يلي:
containers-edit-commitFirst = الحفظ كصورة أولًا…
containers-edit-recreate = إعادة الإنشاء
containers-loss-anonymousVolume = وحدة التخزين المجهولة في { $path }: تبدأ الحاوية الجديدة بوحدة فارغة. يُحتفظ بالقديمة كوحدة تخزين غير مستخدمة.
containers-loss-filesystemChanges = { $count ->
        [zero] لم يتغير أي ملف
        [one] تغيّر ملف واحد
        [two] تغيّر ملفان
        [few] تغيّرت { NUM($count) } ملفات
        [many] تغيّر { NUM($count) } ملفًا
       *[other] تغيّر { NUM($count) } ملف
    } داخل الحاوية منذ إنشائها. احفظها كصورة أولًا للاحتفاظ بها.
containers-loss-downtime = تتوقف أثناء التبديل بين الحاويتين، لبضع ثوانٍ عادةً.
containers-loss-stackRedeploy = تنتمي إلى المكدس { $stack }: يعيد النشر التالي للمكدس إنشاءها من ملف compose الخاص به، دون هذا التعديل.
containers-commit-title = حفظ { $name } كصورة
containers-commit-description = تحتوي الصورة الجديدة على ملفات الحاوية كما هي الآن، بما في ذلك التغييرات التي أُجريت داخلها، وعلى إعداداتها. لا تُضمَّن وحدات التخزين.
containers-commit-imageName = اسم الصورة
containers-commit-imageNameHint = مثل repo:tag.
containers-commit-nameTaken = توجد بالفعل صورة باسم { $name }. ينتقل الاسم إلى الصورة الجديدة؛ وتحتفظ القديمة بأسمائها الأخرى، أو تصبح بلا وسم.
containers-commit-message = الرسالة
containers-commit-messagePlaceholder = قبل الترقية
containers-commit-author = المؤلف
containers-commit-changes = التغييرات
containers-commit-changesHint = تعليمات Dockerfile، واحدة في كل سطر: CMD أو ENTRYPOINT أو ENV أو EXPOSE أو LABEL أو ONBUILD أو USER أو VOLUME أو WORKDIR.
containers-commit-pause = إيقاف الحاوية مؤقتًا أثناء الحفظ
containers-commit-pauseOn = تُجمَّد عملياتها خلال الثواني القليلة التي يستغرقها الحفظ، لتكون الملفات قيد الكتابة متسقة.
containers-commit-pauseOff = تستمر في العمل، لذا قد تُلتقط الملفات قيد الكتابة في منتصفها.
containers-commit-submit = حفظ كصورة
containers-commit-committing = جارٍ الحفظ…
containers-commit-done = تم الحفظ في { $reference }
containers-commit-openImages = فتح الصور
containers-export-tarArchive = أرشيف Tar
containers-export-pick = تصدير نظام ملفات { $name }
containers-export-title = جارٍ تصدير { $name }
containers-export-to = إلى { $path }
containers-export-written = المكتوب
containers-export-finished = تم. استورده في صفحة «الصور»، أو باستخدام docker import.
containers-export-done = تم التصدير إلى { $destination }
containers-palette-run = تشغيل حاوية
