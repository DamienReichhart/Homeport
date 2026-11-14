### The `ui` namespace, in العربية (ar).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ar/ui.ts:
### change the text there, then run it again.

ui-copy = نسخ
ui-copied = تم النسخ
ui-search = بحث
ui-search-clear = مسح البحث
ui-shortcuts-focusSearch = البحث في هذه الصفحة
ui-shortcuts-thisPage = هذه الصفحة
ui-table-selectAll = تحديد الكل
ui-table-selectRow = تحديد الصف
ui-table-empty = لا يوجد شيء هنا.
ui-table-page = الصفحة { $page } من { $pages } · { $rows ->
        [zero] لا صفوف
        [one] صف واحد
        [two] صفان
        [few] { NUM($rows) } صفوف
        [many] { NUM($rows) } صفًا
       *[other] { NUM($rows) } صف
    }
ui-table-previous = الصفحة السابقة
ui-table-next = الصفحة التالية
ui-transfer-done = تم.
ui-files-column-name = الاسم
ui-files-column-size = الحجم
ui-files-column-modified = تاريخ التعديل
ui-files-column-mode = الأذونات
ui-files-path = المسار
ui-files-downloadOne = تنزيل { $name }
ui-files-empty = هذا المجلد فارغ.
ui-files-upload = رفع
ui-json-key = المفتاح
ui-json-value = القيمة
ui-json-copyPath = نسخ المسار
ui-json-copyValue = نسخ القيمة
ui-chart-time = الوقت
ui-chart-hide = إخفاء { $name }
ui-chart-show = إظهار { $name }
ui-chart-more = +{ $count }
ui-errorDetails-title = تفاصيل الخطأ
ui-errorDetails-code = الرمز
ui-errorDetails-command = الأمر
ui-errorDetails-when = الوقت
ui-errorDetails-app = التطبيق
ui-errorDetails-rawDetail = التفاصيل الخام
ui-errorDetails-rawDetailNote = كما كتبها النظام أو المحرك أو المكتبة، لذا لم تُترجم.
ui-errorDetails-openLogFolder = فتح مجلد السجلات
ui-errorDetails-copyReport = نسخ التقرير
ui-confirm-typeName = اكتب { $name } للتأكيد
ui-select-noMatch = لا توجد نتائج مطابقة.
ui-password-show = إظهار كلمة المرور
ui-password-hide = إخفاء كلمة المرور
ui-log-wrap = التفاف الأسطر
ui-log-latest = الانتقال إلى الأحدث
ui-log-matchCase = مطابقة حالة الأحرف
ui-log-regex = تعبير نمطي
ui-log-filter = تصفية
ui-log-filterHint = إظهار الأسطر المطابقة فقط
ui-log-matches = { $current } من { $count }
ui-log-noMatches = لا توجد نتائج مطابقة
ui-log-invalidRegex = تعبير نمطي غير صالح
ui-log-skipped = { $count ->
        [zero] — لم يُتخطَّ أي سطر —
        [one] — تم تخطي سطر واحد —
        [two] — تم تخطي سطرين —
        [few] — تم تخطي { NUM($count) } أسطر —
        [many] — تم تخطي { NUM($count) } سطرًا —
       *[other] — تم تخطي { NUM($count) } سطر —
    }
ui-log-searchFailed = توقف البحث قبل قراءة كل الأسطر
ui-terminal-find = البحث في الطرفية
ui-terminal-closeSearch = إغلاق البحث
ui-terminal-atLeast = { $count }+
ui-terminal-noneNear = لا توجد نتائج قريبة
