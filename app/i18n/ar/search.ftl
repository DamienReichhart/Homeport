### The `search` namespace, in العربية (ar).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ar/search.ts:
### change the text there, then run it again.

search-kind-container = الحاويات
search-kind-stack = المكدسات
search-kind-image = الصور
search-kind-volume = وحدات التخزين
search-kind-network = الشبكات
search-onHostState = { $host } · { $state ->
        [running] قيد التشغيل
       *[other] متوقفة
    }
search-showAll = عرض الكل ({ $count })
search-skipped = { $count ->
        [zero] لا مضيفات
        [one] مضيف واحد
        [two] مضيفان
        [few] { NUM($count) } مضيفات
        [many] { NUM($count) } مضيفًا
       *[other] { NUM($count) } مضيف
    } لم يُبحث فيها (غير متصلة)
