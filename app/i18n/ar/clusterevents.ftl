### The `clusterevents` namespace, in العربية (ar).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ar/clusterevents.ts:
### change the text there, then run it again.

clusterevents-nav = الأحداث
clusterevents-title = الأحداث
clusterevents-summary = { $count ->
        [zero] لا أحداث
        [one] حدث واحد
        [two] حدثان
        [few] { NUM($count) } أحداث
        [many] { NUM($count) } حدثًا
       *[other] { NUM($count) } حدث
    }
clusterevents-search = البحث حسب السبب أو الكائن أو الرسالة
clusterevents-empty = لا توجد أحداث. يحتفظ بها Kubernetes لمدة ساعة تقريبًا.
clusterevents-filter = النوع
clusterevents-all = الكل
clusterevents-warnings = التحذيرات
clusterevents-normal = عادي
clusterevents-column-type = النوع
clusterevents-column-reason = السبب
clusterevents-column-object = الكائن
clusterevents-column-namespace = Namespace
clusterevents-column-message = الرسالة
clusterevents-column-count = العدد
clusterevents-column-lastSeen = آخر ظهور
