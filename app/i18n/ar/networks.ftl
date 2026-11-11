### The `networks` namespace, in العربية (ar).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ar/networks.ts:
### change the text there, then run it again.

networks-nav = الشبكات
networks-title = الشبكات
networks-summary = { $count ->
        [zero] لا شبكات
        [one] شبكة واحدة
        [two] شبكتان
        [few] { NUM($count) } شبكات
        [many] { NUM($count) } شبكة
       *[other] { NUM($count) } شبكة
    }
networks-cleanUp = تنظيف
networks-create = إنشاء شبكة
networks-search = البحث في الشبكات
networks-empty = لا توجد شبكات
networks-column-name = الاسم
networks-column-stack = المكدس
networks-column-driver = برنامج التشغيل
networks-column-subnet = الشبكة الفرعية
networks-column-containers = الحاويات
networks-builtIn = مضمّنة
networks-internal = داخلية
networks-inspectOne = فحص { $name }
networks-removeOne = إزالة { $name }
networks-remove-title = هل تريد إزالة الشبكة { $name }؟
networks-remove-attached = لا تزال { $count ->
        [zero] لا توجد حاويات متصلة بها
        [one] حاوية واحدة متصلة بها
        [two] حاويتان متصلتين بها
        [few] { NUM($count) } حاويات متصلة بها
        [many] { NUM($count) } حاوية متصلة بها
       *[other] { NUM($count) } حاوية متصلة بها
    }؛ افصلها أولًا.
networks-remove-unused = لا تستخدمها أي حاوية. ستفشل الحاويات التي تشير إليها بالاسم في البدء.
networks-prune-title = هل تريد إزالة الشبكات غير المستخدمة؟
networks-prune-description = تُزال كل شبكة مخصصة لا تتصل بها أي حاوية.
networks-connected = تم الاتصال
networks-disconnected = تم قطع الاتصال
networks-removed = تمت إزالة { $name }
networks-pruned = { $count ->
        [zero] لم تُزل أي شبكة
        [one] تمت إزالة شبكة واحدة
        [two] تمت إزالة شبكتين
        [few] تمت إزالة { NUM($count) } شبكات
        [many] تمت إزالة { NUM($count) } شبكة
       *[other] تمت إزالة { NUM($count) } شبكة
    }
networks-createDialog-title = إنشاء شبكة
networks-createDialog-name = الاسم
networks-createDialog-driver = برنامج التشغيل
networks-createDialog-subnet = الشبكة الفرعية
networks-createDialog-gateway = البوابة
networks-createDialog-automatic = تلقائي
networks-createDialog-internal = داخلية — دون وصول إلى الشبكات الخارجية
networks-createDialog-attachable = قابلة للإرفاق
networks-dialog-containers = الحاويات
networks-dialog-raw = JSON الخام
networks-dialog-noContainer = لا توجد حاوية متصلة.
networks-dialog-disconnect = قطع الاتصال
networks-dialog-containerToConnect = الحاوية المراد توصيلها
networks-dialog-connectPlaceholder = توصيل حاوية…
networks-dialog-connect = توصيل
