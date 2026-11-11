### The `apiresources` namespace, in العربية (ar).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ar/apiresources.ts:
### change the text there, then run it again.

apiresources-nav = موارد API
apiresources-title = موارد API
apiresources-summary = { $count ->
        [zero] لا أنواع
        [one] نوع واحد
        [two] نوعان
        [few] { NUM($count) } أنواع
        [many] { NUM($count) } نوعًا
       *[other] { NUM($count) } نوع
    } يقدّمها هذا العنقود
apiresources-refresh = تحديث
apiresources-types = أنواع الموارد
apiresources-search = النوع أو الاسم المختصر (pvc)
apiresources-searchObjects = بحث
apiresources-noMatch = لا يوجد نوع مطابق.
apiresources-coreGroup = core
apiresources-pick = اختر نوعًا لسرد كائناته: الأنواع المضمّنة، والأنواع التي تضيفها CRDs الخاصة بك.
apiresources-typeGone = لا يقدّم هذا العنقود ذلك النوع (بعد الآن).
apiresources-cantList = لا يمكن سرده
apiresources-notAllowed = لا تسمح بيانات اعتمادك بسرده
apiresources-cantListWhy = لا يمكن سرد كائنات { $kind }: لا تسمح واجهة API الخاصة به إلا بإنشائها أو قراءتها واحدًا تلو الآخر.
apiresources-empty = لا شيء هنا.
apiresources-column-name = الاسم
apiresources-column-namespace = Namespace
apiresources-column-age = العمر
apiresources-tab-overview = نظرة عامة
apiresources-tab-conditions = الشروط
apiresources-tab-events = الأحداث
apiresources-tab-source = YAML
apiresources-detail-namespace = Namespace
apiresources-detail-owners = مملوك لـ
apiresources-detail-age = العمر
apiresources-detail-labels = التسميات
apiresources-detail-annotations = التعليقات التوضيحية
apiresources-edit = تحرير
apiresources-copyYaml = نسخ YAML
