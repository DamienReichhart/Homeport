### The `clusters` namespace, in العربية (ar).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ar/clusters.ts:
### change the text there, then run it again.

clusters-title = العناقيد
clusters-summary = { $count ->
        [zero] لا عناقيد
        [one] عنقود واحد
        [two] عنقودان
        [few] { NUM($count) } عناقيد
        [many] { NUM($count) } عنقودًا
       *[other] { NUM($count) } عنقود
    }
clusters-add = إضافة عنقود
clusters-notFound = لم يعد هذا العنقود موجودًا.
clusters-empty-title = لا توجد عناقيد بعد
clusters-empty-description = أضف عناقيد Kubernetes التي يصل إليها ملف kubeconfig. لا يثبّت Homeport أي شيء عليها.
clusters-column-name = الاسم
clusters-column-status = الحالة
clusters-column-kind = النوع
clusters-column-server = API
clusters-column-auth = يسجّل الدخول باستخدام
clusters-palette-add = إضافة عنقود
clusters-palette-manage = إدارة العناقيد
clusters-auth-none = بلا بيانات اعتماد
clusters-auth-clientCertificate = شهادة العميل
clusters-auth-token = رمز
clusters-auth-basic = اسم المستخدم وكلمة المرور
clusters-auth-exec = مكوّن exec الإضافي ({ $command })
clusters-auth-oidc = OpenID Connect
clusters-auth-aclToken = رمز ACL
clusters-addPage-description = يصبح كل سياق Kubernetes تختاره، أو كل وكيل Nomad تسمّيه، عنقودًا. تُخزَّن بيانات اعتماده مشفّرة، ولا تُحفظ أبدًا كنص واضح على القرص.
clusters-addPage-fromMachine = من ملف kubeconfig على هذا الجهاز
clusters-addPage-paste = لصق ملف kubeconfig
clusters-addPage-nomad = Nomad
clusters-paste-description = يُقرأ ملف kubeconfig في الذاكرة ولا يُحفظ كملف أبدًا. يجب أن تكون الملفات التي يشير إليها مسارات كاملة.
clusters-paste-label = Kubeconfig
clusters-paste-read = قراءة السياقات
clusters-import-readFrom = مقروء من { $paths }
clusters-import-noContexts = لا يحتوي ملف kubeconfig هذا على سياقات
clusters-import-pick = استيراد { $context }
clusters-import-name = اسم العنقود المُنشأ من { $context }
clusters-import-current = الحالي
clusters-import-added = مضاف بالفعل باسم { $name }.
clusters-import-refresh = تحديثه
clusters-import-submit = { $count ->
        [0] استيراد
        [zero] استيراد
        [one] استيراد عنقود واحد
        [two] استيراد عنقودين
        [few] استيراد { NUM($count) } عناقيد
        [many] استيراد { NUM($count) } عنقودًا
       *[other] استيراد { NUM($count) } عنقود
    }
clusters-import-done = { $count ->
        [zero] لم يُضف أي عنقود
        [one] تمت إضافة عنقود واحد
        [two] تمت إضافة عنقودين
        [few] تمت إضافة { NUM($count) } عناقيد
        [many] تمت إضافة { NUM($count) } عنقودًا
       *[other] تمت إضافة { NUM($count) } عنقود
    }
clusters-import-problem-empty = أعطِ كل عنقود اسمًا.
clusters-import-problem-duplicate = سيحمل عنقودان الاسم «{ $name }».
clusters-import-problem-taken = يوجد بالفعل عنقود باسم «{ $name }».
clusters-detail-enabled = مفعّل
clusters-detail-reconnect = إعادة الاتصال
clusters-detail-name = الاسم
clusters-detail-rename = إعادة التسمية
clusters-detail-renamed = تمت إعادة تسمية العنقود
clusters-connection-title = الاتصال
clusters-connection-server = خادم API
clusters-connection-address = العنوان
clusters-connection-region = المنطقة
clusters-connection-customCa = شهادة CA خاصة به
clusters-connection-clientCertificate = شهادة العميل
clusters-connection-checkedAs = يُتحقق من الشهادة باسم { $name }
clusters-connection-version = الإصدار
clusters-connection-platform = النظام الأساسي
clusters-connection-namespace = الـ Namespace الافتراضية
clusters-connection-auth = يسجّل الدخول باستخدام
clusters-connection-tlsServerName = اسم الشهادة
clusters-connection-tls = TLS
clusters-connection-insecure = لا يُتحقق من شهادة الخادم (insecure-skip-tls-verify)
clusters-connection-proxy = الوكيل
clusters-connection-source = مستورد من
clusters-source-machine = السياق { $context } من ملف kubeconfig على هذا الجهاز
clusters-source-pasted = السياق { $context } من ملف kubeconfig ملصوق
clusters-nomad-description = يتواصل Homeport مع واجهة HTTP API لوكيل Nomad، خادمًا كان أو عميلًا، من هذا الكمبيوتر. لا يُثبَّت أي شيء على العنقود، ولا حاجة إلى nomad CLI هنا.
clusters-nomad-name = الاسم
clusters-nomad-address = العنوان
clusters-nomad-addressHelp = واجهة HTTP API للوكيل، كما يمررها NOMAD_ADDR إلى nomad CLI. المنفذ 4646 إذا لم يُحدَّد منفذ.
clusters-nomad-region = المنطقة
clusters-nomad-regionHelp = اتركها فارغة لاستخدام منطقة الوكيل نفسه.
clusters-nomad-token = رمز ACL
clusters-nomad-tokenHelp = المعرّف السري للرمز، عندما تكون ACLs مفعّلة في العنقود. تقدّم وثائق Homeport سياسة له.
clusters-nomad-tls = إعدادات TLS
clusters-nomad-tlsHelp = فقط لوكيل يقدّم HTTPS بشهادة CA خاصة به، أو يطلب شهادة عميل. الصق محتويات ملفات PEM.
clusters-nomad-ca = شهادة CA
clusters-nomad-caHelp = جهة CA التي وقّعت شهادة الوكيل، كما يسمّيها NOMAD_CACERT. جهات CA الخاصة بهذا الكمبيوتر موثوقة دون ذلك.
clusters-nomad-clientCertificate = شهادة العميل
clusters-nomad-clientKey = مفتاح العميل
clusters-nomad-tlsServerName = اسم خادم TLS
clusters-nomad-tlsServerNameHelp = الاسم الذي يُتحقق من شهادة الوكيل مقابله، عندما لا يكون اسم العنوان، كما يحدده NOMAD_TLS_SERVER_NAME.
clusters-nomad-keep = محفوظ؛ اتركه فارغًا للاحتفاظ به
clusters-nomad-willBeRemoved = ستتم إزالته عند الحفظ
clusters-nomad-remove = إزالة
clusters-nomad-keepIt = الاحتفاظ به
clusters-nomad-test = اختبار الاتصال
clusters-nomad-testing = جارٍ الاختبار…
clusters-nomad-testedVersion = تم الاتصال بـ Nomad { $version }، المنطقة { $region }.
clusters-nomad-testedNoVersion = تم الاتصال بالمنطقة { $region }. لا يستطيع الرمز قراءة الوكيل، لذا إصداره غير معروف.
clusters-nomad-add = إضافة عنقود
clusters-nomad-added = تمت إضافة { $name }
clusters-nomad-settings = الإعدادات
clusters-nomad-settingsDescription = يعيد الحفظ الاتصال. لا يُعرض الرمز ومفتاح TLS مجددًا أبدًا: اتركهما فارغين للاحتفاظ بهما.
clusters-nomad-save = حفظ التغييرات
clusters-nomad-saved = تم حفظ العنقود
clusters-refresh-title = التحديث من kubeconfig
clusters-refresh-machine = يقرأ السياق { $context } مجددًا من ملف kubeconfig على هذا الجهاز، لحالات إعادة إنشاء العنقود أو تجديد بيانات اعتماده.
clusters-refresh-pasted = الصق ملف kubeconfig مجددًا لقراءة السياق { $context } من جديد، لحالات إعادة إنشاء العنقود أو تجديد بيانات اعتماده.
clusters-refresh-submit = تحديث
clusters-refresh-done = تم تحديث العنقود من ملف kubeconfig الخاص به
clusters-remove-title = إزالة العنقود
clusters-remove-description = ينسى Homeport العنقود وبيانات اعتماده المخزنة. لا يتغير شيء على العنقود أو في ملف kubeconfig.
clusters-remove-confirmTitle = هل تريد إزالة { $name }؟
clusters-remove-confirmDescription = يمكنك إضافته مجددًا من ملف kubeconfig الخاص به.
