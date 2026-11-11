### The `containerd` namespace, in العربية (ar).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ar/containerd.ts:
### change the text there, then run it again.

containerd-state-none = لا مهمة
containerd-state-created = تم الإنشاء
containerd-state-running = قيد التشغيل
containerd-state-paused = متوقفة مؤقتًا
containerd-state-pausing = جارٍ الإيقاف المؤقت
containerd-state-stopped = متوقفة
containerd-state-unknown = غير معروفة
containerd-state-exited = انتهت ({ $code })
containerd-createdBy-kubernetes = Kubernetes
containerd-createdBy-nerdctl = nerdctl
containerd-createdBy-docker = Docker
containerd-createdBy-other = ctr أو غيره
containerd-containers-nav = الحاويات
containerd-containers-title = الحاويات
containerd-containers-summary = { $count ->
        [zero] لا حاويات
        [one] حاوية واحدة
        [two] حاويتان
        [few] { NUM($count) } حاويات
        [many] { NUM($count) } حاوية
       *[other] { NUM($count) } حاوية
    }، منها { $running } قيد التشغيل، في مساحة الأسماء { $namespace }.
containerd-containers-search = التصفية حسب الاسم أو المعرّف أو الصورة
containerd-containers-showSandboxes = إظهار حاويات pause الخاصة بـ Pods
containerd-containers-sandbox = pause
containerd-containers-empty = لا توجد حاويات في { $namespace }
containerd-containers-emptyDescription = تظهر هنا الحاويات المُنشأة باستخدام nerdctl أو ctr أو Kubernetes. اختر مساحة أسماء أخرى أسفل أداة تبديل المضيف لرؤية غيرها.
containerd-containers-column-name = الاسم
containerd-containers-column-image = الصورة
containerd-containers-column-state = المهمة
containerd-containers-column-createdBy = أنشأها
containerd-containers-column-created = تاريخ الإنشاء
containerd-containers-action-start = تشغيل
containerd-containers-action-stop = إيقاف
containerd-containers-action-pause = إيقاف مؤقت
containerd-containers-action-resume = استئناف
containerd-containers-action-kill = إرسال إشارة
containerd-containers-actionOne-start = تشغيل { $name }
containerd-containers-actionOne-stop = إيقاف { $name }
containerd-containers-actionOne-pause = إيقاف { $name } مؤقتًا
containerd-containers-actionOne-resume = استئناف { $name }
containerd-containers-removeOne = إزالة { $name }
containerd-containers-done-start = تم تشغيل { $name }
containerd-containers-done-stop = تم إيقاف { $name }
containerd-containers-done-pause = تم إيقاف { $name } مؤقتًا
containerd-containers-done-resume = تم استئناف { $name }
containerd-containers-done-kill = تم إرسال { $signal } إلى { $name }
containerd-containers-removed = تمت إزالة { $name }
containerd-containers-kubelet = يدير Kubernetes هذه الحاوية: يعيد kubelet تشغيلها بعد الإيقاف، ويعيد إنشاءها بعد الإزالة. غيّر الـ Pod الخاص بها على العنقود بدلًا من ذلك.
containerd-containers-remove-title = هل تريد إزالة { $name }؟
containerd-containers-remove-running = مهمتها قيد التشغيل: ينهيها Homeport قسريًا، ثم يزيل المهمة والحاوية.
containerd-containers-remove-stopped = يزيل Homeport مهمتها والحاوية.
containerd-containers-remove-snapshot = إزالة لقطتها أيضًا ({ $key })، مع الملفات التي غيّرتها
containerd-containers-remove-killAndRemove = إنهاء قسري وإزالة
containerd-containers-kill-title = إرسال إشارة إلى { $name }
containerd-containers-kill-description = إلى العملية الرئيسية لمهمتها، كما يفعل ctr task kill -s. اسم مثل SIGHUP، أو رقم.
containerd-containers-kill-signal = الإشارة
containerd-containers-kill-send = إرسال
containerd-containers-backToList = العودة إلى الحاويات
containerd-containers-tab-overview = نظرة عامة
containerd-containers-tab-logs = السجلات
containerd-containers-tab-exec = تنفيذ
containerd-containers-tab-spec = المواصفات
containerd-logs-from = من { $path }
containerd-logs-none-title = لا يُسجَّل إخراج هذه الحاوية
containerd-logs-none-description = لا يحتفظ containerd بأي سجلات: الأداة التي تبدأ المهمة هي التي تحدد أين يذهب إخراجها. الحاويات المُنشأة باستخدام ctr ومعظم العملاء الآخرين لا ترسله إلى أي مكان؛ أما nerdctl وKubernetes فيكتبانه في ملف يقرؤه Homeport.
containerd-logs-driver-title = يرسل nerdctl إخراج هذه الحاوية إلى { $driver }
containerd-logs-driver-description = يقرأ Homeport سجلات json-file الافتراضية لـ nerdctl فقط. اقرأ هذه السجلات حيث يحتفظ بها { $driver }، أو شغّل الحاوية مجددًا باستخدام برنامج التشغيل json-file.
containerd-exec-startFirst = شغّل الحاوية لفتح طرفية فيها.
containerd-exec-command = الأمر
containerd-exec-commandPlaceholder = صدفة، ما لم تحدد أمرًا
containerd-exec-user = معرّف المستخدم
containerd-exec-workingDir = المجلد
containerd-exec-open = الفتح مجددًا
containerd-exec-exited = انتهت العملية.
containerd-exec-exitedWithCode = انتهت العملية بالرمز { $code }.
containerd-containers-field-id = المعرّف
containerd-containers-field-namespace = مساحة الأسماء
containerd-containers-field-image = الصورة
containerd-containers-field-runtime = وقت التشغيل
containerd-containers-field-snapshot = اللقطة
containerd-containers-field-pid = PID
containerd-containers-field-exit = آخر خروج
containerd-containers-field-pod = Pod في Kubernetes
containerd-containers-field-created = تاريخ الإنشاء
containerd-containers-field-updated = تاريخ التحديث
containerd-containers-field-extensions = الامتدادات
containerd-containers-labels = التسميات
containerd-containers-noSpec = لا يحتفظ containerd بمواصفات وقت تشغيل لهذه الحاوية.
containerd-images-nav = الصور
containerd-images-title = الصور
containerd-images-summary = { $count ->
        [zero] لا صور
        [one] صورة واحدة
        [two] صورتان
        [few] { NUM($count) } صور
        [many] { NUM($count) } صورة
       *[other] { NUM($count) } صورة
    } في مساحة الأسماء { $namespace }.
containerd-images-pull = سحب صورة
containerd-images-pulled = تم سحب { $reference } إلى { $namespace }
containerd-images-removed = تمت إزالة { $name }
containerd-images-search = التصفية حسب الاسم
containerd-images-empty = لا توجد صور في { $namespace }
containerd-images-emptyDescription = اسحب صورة: يجلبها containerd، ثم يفك حزمها للنظام الأساسي لهذا المضيف حتى تعمل حاوياتها.
containerd-images-column-name = الاسم
containerd-images-column-platforms = الأنظمة الأساسية
containerd-images-column-size = الحجم
containerd-images-column-containers = الحاويات
containerd-images-column-created = تاريخ الإنشاء
containerd-images-sizeUnknown = محتواها الخاص بالنظام الأساسي لهذا المضيف ليس كله هنا.
containerd-images-removeOne = إزالة { $name }
containerd-images-inUse = أُنشئت منها حاويات في مساحة الأسماء هذه. أزلها أولًا.
containerd-images-remove-title = هل تريد إزالة { $name }؟
containerd-images-remove-description = يزيل containerd الاسم، ويحرر محتواها ولقطاتها عندما لا يحتفظ بها أي شيء آخر.
containerd-snapshots-nav = اللقطات
containerd-snapshots-title = اللقطات
containerd-snapshots-summary = { $count ->
        [zero] لا لقطات
        [one] لقطة واحدة
        [two] لقطتان
        [few] { NUM($count) } لقطات
        [many] { NUM($count) } لقطة
       *[other] { NUM($count) } لقطة
    } في { $namespace } تستخدم { $total }؛ { $unused ->
        [0] لا توجد لقطة غير مستخدمة
        [zero] لا توجد لقطة غير مستخدمة
        [one] واحدة غير مستخدمة
        [two] اثنتان غير مستخدمتين
        [few] { NUM($unused) } غير مستخدمة
        [many] { NUM($unused) } غير مستخدمة
       *[other] { NUM($unused) } غير مستخدمة
    } ({ $unusedSize }).
containerd-snapshots-snapshotter = Snapshotter
containerd-snapshots-unusedOnly = غير المستخدمة فقط
containerd-snapshots-unused = غير مستخدمة
containerd-snapshots-usedByContainer = الحاوية { $id }
containerd-snapshots-usedByImage = الصورة { $name }
containerd-snapshots-empty = لا توجد لقطات في { $snapshotter }
containerd-snapshots-emptyDescription = اللقطات هي طبقات الصور التي فُكّت حزمها والطبقات القابلة للكتابة للحاويات.
containerd-snapshots-column-key = المفتاح
containerd-snapshots-column-kind = النوع
containerd-snapshots-column-size = الحجم
containerd-snapshots-column-usedBy = تستخدمها
containerd-snapshots-kind-committed = طبقة
containerd-snapshots-kind-active = قابلة للكتابة
containerd-snapshots-kind-view = عرض
containerd-snapshots-kind-unknown = غير معروف
containerd-snapshots-removeOne = إزالة { $key }
containerd-snapshots-removed = تمت إزالة اللقطة { $key }
containerd-snapshots-inUse = يستخدمها شيء ما أو مبني عليها. أزل الحاوية أو الصورة أولًا.
containerd-snapshots-remove-title = هل تريد إزالة اللقطة { $key }؟
containerd-snapshots-remove-description = لا يستخدمها شيء. تُحذف ملفاتها، { $size }، من المضيف.
containerd-namespaces-nav = مساحات الأسماء
containerd-namespaces-title = مساحات الأسماء
containerd-namespaces-summary = { $count ->
        [zero] لا مساحات أسماء
        [one] مساحة أسماء واحدة
        [two] مساحتا أسماء
        [few] { NUM($count) } مساحات أسماء
        [many] { NUM($count) } مساحة أسماء
       *[other] { NUM($count) } مساحة أسماء
    }. يعيش كل كائن containerd في واحدة منها: يستخدم nerdctl default، وKubernetes k8s.io، وDocker moby.
containerd-namespaces-new = مساحة أسماء جديدة
containerd-namespaces-created = تم إنشاء مساحة الأسماء { $name }
containerd-namespaces-removed = تمت إزالة مساحة الأسماء { $name }
containerd-namespaces-shown = المعروضة
containerd-namespaces-show = عرض
containerd-namespaces-removeOne = إزالة { $name }
containerd-namespaces-notEmpty = لا يمكن إزالة سوى مساحات الأسماء الفارغة. أزل حاوياتها وصورها أولًا.
containerd-namespaces-remove-title = هل تريد إزالة مساحة الأسماء { $name }؟
containerd-namespaces-remove-description = يرفض containerd ذلك إذا كانت لا تزال تحتوي على لقطات أو محتوى لا تُظهره الأعداد.
containerd-namespaces-column-name = الاسم
containerd-namespaces-column-containers = الحاويات
containerd-namespaces-column-images = الصور
containerd-namespaces-column-labels = التسميات
containerd-namespaces-empty = لا توجد مساحات أسماء
containerd-namespaces-emptyDescription = ينشئ containerd مساحة default في أول مرة يُنشأ فيها شيء ضمنها.
containerd-namespaces-newDialog-title = مساحة أسماء جديدة
containerd-namespaces-newDialog-description = مساحة أسماء فارغة، لإبقاء الحاويات والصور منفصلة عن غيرها.
containerd-namespaces-newDialog-name = الاسم
containerd-namespaces-newDialog-invalid = استخدم حروفًا وأرقامًا، تفصل بينها نقاط أو شرطات أو شرطات سفلية مفردة.
containerd-namespaces-newDialog-create = إنشاء
