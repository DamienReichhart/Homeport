### The `volumes` namespace, in العربية (ar).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ar/volumes.ts:
### change the text there, then run it again.

volumes-nav = وحدات التخزين
volumes-title = وحدات التخزين
volumes-summary = { $count ->
        [zero] لا وحدات تخزين
        [one] وحدة تخزين واحدة
        [two] وحدتا تخزين
        [few] { NUM($count) } وحدات تخزين
        [many] { NUM($count) } وحدة تخزين
       *[other] { NUM($count) } وحدة تخزين
    } · { $unused } غير مستخدمة
volumes-cleanUp = تنظيف
volumes-cleanUp-anonymous = وحدات التخزين المجهولة غير المستخدمة
volumes-cleanUp-all = كل وحدات التخزين غير المستخدمة
volumes-create = إنشاء وحدة تخزين
volumes-search = البحث في وحدات التخزين
volumes-empty-title = لا توجد وحدات تخزين
volumes-empty-description = تحتفظ وحدات التخزين ببيانات الحاويات عبر عمليات إعادة التشغيل وإعادة الإنشاء.
volumes-column-name = الاسم
volumes-column-stack = المكدس
volumes-column-driver = برنامج التشغيل
volumes-column-size = الحجم
volumes-size-label = الحجم:
volumes-size-measuring = جارٍ القياس
volumes-size-unknown = لا يُبلغ برنامج تشغيل وحدة التخزين هذه عن حجمها.
volumes-size-refresh = تحديث الأحجام
volumes-column-usedBy = تستخدمها
volumes-column-created = تاريخ الإنشاء
volumes-unused = غير مستخدمة
volumes-inspectOne = فحص { $name }
volumes-deleteOne = حذف { $name }
volumes-browseOne = استعراض ملفات { $name }
volumes-browse-title = ملفات { $name }
volumes-browse-description = للقراءة فقط: لا شيء هنا يمكنه تغيير وحدة التخزين، حتى أثناء استخدام الحاويات لها.
volumes-browse-saveOne = حفظ { $name }
volumes-browse-saved = تم حفظ { $name }
volumes-browse-savedArchive = تم حفظ { $name } كأرشيف .tar
volumes-helper-needed = يفتح Homeport وحدات التخزين بحاوية مساعدة قصيرة العمر، تحتاج إلى الصورة { $image } (حوالي { $size }). لا تتوفر هذه الصورة على هذا المضيف.
volumes-helper-removed = لا يسحبها Homeport إلا بموافقتك، ويزيلها مجددًا عند الانتهاء، ما لم يستخدمها شيء آخر.
volumes-helper-pull = سحب { $image }
volumes-helper-pulling = جارٍ سحب { $image }…
volumes-helper-progress = تقدم السحب
volumes-moreActions = مزيد من الإجراءات لـ { $name }
volumes-backupMenu = نسخ احتياطي إلى ملف…
volumes-restoreMenu = استعادة من ملف…
volumes-cloneMenu = استنساخ…
volumes-clone-title = استنساخ { $volume }
volumes-clone-description = ينسخ ملفاتها، مع مالكيها وأذوناتها، إلى وحدة تخزين جديدة على هذا المضيف. لا تنتمي النسخة إلى أي مكدس.
volumes-clone-name = اسم النسخة
volumes-clone-start = استنساخ
volumes-clone-into = إلى { $target }
volumes-clone-copying = جارٍ النسخ…
volumes-clone-done = تم استنساخ { $source } إلى { $target }
volumes-restoreFromFile = الاستعادة من ملف
volumes-backup-title = نسخ { $volume } احتياطيًا
volumes-backup-saveAs = نسخ { $volume } احتياطيًا إلى
volumes-backup-to = إلى { $path }
volumes-backup-done = تم نسخ { $volume } احتياطيًا إلى { $path }
volumes-restore-pick = استعادة وحدة تخزين من
volumes-restore-title = استعادة وحدة تخزين
volumes-restore-titleInto = استعادة { $volume }
volumes-restore-from = من { $path }
volumes-restore-into = الاستعادة إلى
volumes-restore-newVolume = وحدة تخزين جديدة
volumes-restore-existingVolume = وحدة تخزين موجودة
volumes-restore-volume = وحدة التخزين
volumes-restore-chooseVolume = اختر وحدة تخزين
volumes-restore-nameTaken = توجد بالفعل وحدة تخزين باسم { $volume }.
volumes-restore-start = استعادة
volumes-restore-done = تمت استعادة { $volume }
volumes-restore-cancelled = أُلغيت استعادة { $volume }؛ وقد تكون وحدة التخزين مكتوبة جزئيًا.
volumes-transfer-written = المكتوب
volumes-transfer-sent = المُرسَل
volumes-transfer-archives = أرشيفات Tar
volumes-flow-running-backup = { $containers } { $count ->
        [zero] قيد التشغيل وتستخدم
        [one] قيد التشغيل وتستخدم
        [two] قيد التشغيل وتستخدمان
        [few] قيد التشغيل وتستخدم
        [many] قيد التشغيل وتستخدم
       *[other] قيد التشغيل وتستخدم
    } { $volume }. قد تكون النسخة الاحتياطية المأخوذة أثناء الكتابة إليها غير متسقة، مثل قاعدة بيانات أُخذت في منتصف عملية كتابة.
volumes-flow-running-restore = { $containers } { $count ->
        [zero] قيد التشغيل وتستخدم
        [one] قيد التشغيل وتستخدم
        [two] قيد التشغيل وتستخدمان
        [few] قيد التشغيل وتستخدم
        [many] قيد التشغيل وتستخدم
       *[other] قيد التشغيل وتستخدم
    } { $volume }. { $count ->
        [zero] يوقفها Homeport أثناء الاستعادة ثم يعيد تشغيلها بعد ذلك.
        [one] يوقفها Homeport أثناء الاستعادة ثم يعيد تشغيلها بعد ذلك.
        [two] يوقفهما Homeport أثناء الاستعادة ثم يعيد تشغيلهما بعد ذلك.
        [few] يوقفها Homeport أثناء الاستعادة ثم يعيد تشغيلها بعد ذلك.
        [many] يوقفها Homeport أثناء الاستعادة ثم يعيد تشغيلها بعد ذلك.
       *[other] يوقفها Homeport أثناء الاستعادة ثم يعيد تشغيلها بعد ذلك.
    }
volumes-flow-running-clone = { $containers } { $count ->
        [zero] قيد التشغيل وتستخدم
        [one] قيد التشغيل وتستخدم
        [two] قيد التشغيل وتستخدمان
        [few] قيد التشغيل وتستخدم
        [many] قيد التشغيل وتستخدم
       *[other] قيد التشغيل وتستخدم
    } { $volume }. قد تكون النسخة المأخوذة أثناء الكتابة إليها غير متسقة، مثل قاعدة بيانات أُخذت في منتصف عملية كتابة.
volumes-flow-anyway-backup = النسخ الاحتياطي على أي حال
volumes-flow-anyway-clone = النسخ على أي حال
volumes-flow-stopDuring-backup = { $count ->
        [zero] إيقافها
        [one] إيقافها
        [two] إيقافهما
        [few] إيقافها
        [many] إيقافها
       *[other] إيقافها
    } أثناء النسخ الاحتياطي
volumes-flow-stopDuring-restore = إيقاف واستعادة
volumes-flow-stopDuring-clone = { $count ->
        [zero] إيقافها
        [one] إيقافها
        [two] إيقافهما
        [few] إيقافها
        [many] إيقافها
       *[other] إيقافها
    } أثناء النسخ
volumes-flow-replace = تحتوي { $volume } على ملفات بالفعل. تحذفها الاستعادة وتضع ملفات النسخة الاحتياطية مكانها.
volumes-flow-replaceConfirm = استبدال محتوياتها
volumes-usedBy = تستخدمها { $containers }.
volumes-notUsed = لا تستخدم أي حاوية وحدة التخزين هذه.
volumes-createDialog-title = إنشاء وحدة تخزين
volumes-createDialog-name = الاسم
volumes-createDialog-driver = برنامج التشغيل
volumes-deleted = تم حذف { $name }
volumes-pruned = { $count ->
        [zero] لم تُحذف أي وحدة تخزين
        [one] تم حذف وحدة تخزين واحدة
        [two] تم حذف وحدتي تخزين
        [few] تم حذف { NUM($count) } وحدات تخزين
        [many] تم حذف { NUM($count) } وحدة تخزين
       *[other] تم حذف { NUM($count) } وحدة تخزين
    }، وتم استرداد { $space }
volumes-remove-title = هل تريد حذف وحدة التخزين { $name }؟
volumes-remove-warning = <strong>تُحذف كل البيانات في وحدة التخزين هذه نهائيًا.</strong>
volumes-remove-mounted = {" "}إنها مربوطة بـ { $containers }؛ أزل هذه الحاويات أولًا.
volumes-remove-confirm = حذف وحدة التخزين والبيانات
volumes-prune-allTitle = هل تريد حذف كل وحدات التخزين غير المستخدمة؟
volumes-prune-anonymousTitle = هل تريد حذف وحدات التخزين المجهولة غير المستخدمة؟
volumes-prune-allWarning = تُحذف نهائيًا البيانات في كل وحدة تخزين لا تستخدمها أي حاوية، بما في ذلك وحدات التخزين المسماة.
volumes-prune-anonymousWarning = تُحذف نهائيًا البيانات في وحدات التخزين غير المسماة التي لا تستخدمها أي حاوية.
volumes-prune-confirm = حذف وحدات التخزين والبيانات
