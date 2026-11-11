### The `board` namespace, in العربية (ar).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ar/board.ts:
### change the text there, then run it again.

board-nav = اللوحة
board-title = اللوحة
board-summary = { $running } من { $count ->
        [zero] 0 حاوية
        [one] حاوية واحدة
        [two] حاويتين
        [few] { NUM($count) } حاويات
        [many] { NUM($count) } حاوية
       *[other] { NUM($count) } حاوية
    } قيد التشغيل على { $hosts ->
        [zero] 0 مضيف
        [one] مضيف واحد
        [two] مضيفين
        [few] { NUM($hosts) } مضيفات
        [many] { NUM($hosts) } مضيفًا
       *[other] { NUM($hosts) } مضيف
    }
board-column-host = المضيف
board-column-stack = المكدس
board-sortBy = الفرز حسب { $column }
board-filter-hosts = المضيفات
board-filter-allHosts = كل المضيفات
board-filter-someHosts = { $count ->
        [zero] لا مضيفات
        [one] مضيف واحد
        [two] مضيفان
        [few] { NUM($count) } مضيفات
        [many] { NUM($count) } مضيفًا
       *[other] { NUM($count) } مضيف
    }
board-filter-state = الحالة
board-filter-stack = المكدس
board-filter-anyStack = أي مكدس
board-filter-clear = مسح عوامل التصفية
board-state-all = أي حالة
board-state-running = قيد التشغيل
board-state-stopped = متوقفة
board-state-unhealthy = غير سليمة
board-search = البحث في كل المضيفات
board-skipped = لا يُعرض { $name } ({ $status }).
board-openHost = فتح المضيف
board-listFailed = تعذّر سرد حاويات { $name }.
board-noHosts-title = لا يوجد مضيف متصل
board-noHosts-description = تسرد اللوحة حاويات كل مضيف متصل.
board-empty-title = لا توجد حاويات بعد
board-empty-description = لا يشغّل أي من مضيفاتك المتصلة حاوية.
board-noMatch = لا توجد حاوية تطابق عوامل التصفية هذه.
board-selection = الحاويات المحددة
board-selectAll = تحديد كل الحاويات المعروضة
board-selectOne = تحديد { $name } على { $host }
board-remove-title = هل تريد إزالة { $count ->
        [zero] لا حاويات
        [one] حاوية واحدة
        [two] حاويتين
        [few] { NUM($count) } حاويات
        [many] { NUM($count) } حاوية
       *[other] { NUM($count) } حاوية
    }؟
board-remove-description = تُحذف مع وحدات التخزين المجهولة الخاصة بها؛ وتُحتفظ بوحدات التخزين المسماة. تُوقف الحاويات قيد التشغيل أولًا.
board-remove-onHost = على { $host }: { $names }
