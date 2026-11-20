/** The home page, in Arabic. */
export default {
  // Home: the hero and its map.
  'home.title': 'Homeport — تطبيق واحد لكل محركات الحاويات',
  'home.hero.title': 'تطبيق واحد لكل محركات الحاويات.',
  'home.hero.lead': 'يعمل Homeport على جهازك ولا يثبّت شيئًا على خوادمك. مجاني ومفتوح المصدر.',
  'home.hero.docs': 'اقرأ التوثيق',
  'home.map.label':
    'أسطول على سبيل المثال مرسوم كخريطة خطوط. يقع هذا الجهاز في طرف خمسة خطوط: يصل المقبس المحلي إلى laptop، ويصل SSH إلى prod-eu-1 وnas وedge-1، ويصل TLS إلى staging وvps-fra-1، ويصل TCP إلى lan-box، ويصل API العنقود إلى k8s-prod وjobs وlab. مكدس واحد على vps-fra-1 يحتاج إليك.',
  'home.map.thisMachine': 'هذا الجهاز',
  'home.map.needsYou': 'مكدس واحد يحتاج إليك',
  'home.map.keyNeedsYou': 'يحتاج إليك',
  'home.map.example': 'أسطول على سبيل المثال',

  // Home: the questions people open it for.
  'home.questions.title': 'سؤال واحد، مكان واحد.',
  'home.questions.lead':
    'تفتح Homeport لتعرف شيئًا، ثم تعود إلى عملك. كل إجابة على بُعد خطوة من اللوحة.',
  'home.questions.up.title': 'هل يعمل؟',
  'home.questions.up.body':
    'تعرض اللوحة كل مضيف وSwarm وعنقود معًا، وتضع ما يحتاج إليك قبل كل ما هو سليم.',
  'home.questions.up.link': 'اللوحة والنظرة العامة',
  'home.questions.up.screenshot':
    'اللوحة: مربع لكل واحد من ثلاثة مضيفات، وحاوية واحدة تحتاج إليك، ثم حاويات كل المضيفات في جدول واحد.',
  'home.questions.died.title': 'لماذا تعطّل؟',
  'home.questions.died.body':
    'افتح حاوية تجد سجلاتها تتدفق بالفعل. والطرفية وملفاتها والمخططات المباشرة ومستند الفحص على بُعد علامة تبويب.',
  'home.questions.died.link': 'العمل مع الحاويات',
  'home.questions.died.screenshot':
    'علامة التبويب «السجلات» لحاوية، تبث المخرجات مع البحث والطوابع الزمنية وعوامل تصفية التدفق.',
  'home.questions.changed.title': 'ما الذي تغيّر؟',
  'home.questions.changed.body':
    'يدوّن كل مضيف أولًا بأول ما فعله محركه: الحاوية التي أُعيد تشغيلها، والصورة التي سُحبت، ووحدة التخزين التي رُبطت.',
  'home.questions.changed.link': 'الأحداث',
  'home.questions.changed.screenshot':
    'صفحة «الأحداث» لمضيف: فحوص السلامة وإعادات التشغيل وعمليات السحب والربط، الأحدث أولًا، مع المكدس الذي ينتمي إليه كل منها.',
  'home.questions.act.title': 'ثم غيّره، واطّلع على التغيير أولًا.',
  'home.questions.act.body':
    'تعرض إعادة النشر ما سيتغير قبل تنفيذها. ويُحتفظ بكل نشر لتتمكن من التراجع، و«تحديث» يسحب صورًا أحدث في خطوة واحدة.',
  'home.questions.act.link': 'إدارة المكدسات',
  'home.questions.act.screenshot': 'تعريف مكدس مُدار: محرر compose.yaml، وبيئته، وزر «نشر».',

  // Home: the lines, and what each needs on the server.
  'home.lines.title': 'لا شيء يستقر على خوادمك.',
  'home.lines.lead':
    'يبقى Homeport على جهازك ويزور خوادمك. لا وكيل لتثبيته، ولا لوحة ويب لكشفها، ولا حساب لإنشائه، ولا قياس عن بُعد.',
  'home.lines.line': 'الخط',
  'home.lines.reaches': 'يصل إلى',
  'home.lines.needs': 'ما يحتاج إليه على الخادم',
  'home.lines.socket.name': 'مقبس محلي',
  'home.lines.socket.reaches':
    'Docker Desktop وOrbStack وColima وRancher Desktop وأجهزة Podman على هذا الكمبيوتر، يعثر عليها Homeport من أجلك.',
  'home.lines.socket.needs': 'لا شيء. المحرك موجود بالفعل على هذا الجهاز.',
  'home.lines.ssh.reaches':
    'Docker Engine وPodman وcontainerd وIncus على خادم، ومن خلال خوادم Bastion عند الحاجة.',
  'home.lines.ssh.needs':
    'المنفذ المفتوح لديك بالفعل. ومع المفتاح الذي ينشئه Homeport، سطر واحد في {file}.',
  'home.lines.tls.reaches': 'API محرك متوافق مع Docker، بجهة الإصدار CA وشهادة العميل الخاصتين بك.',
  'home.lines.tls.needs': 'منفذ API الخاص بالمحرك (عادةً {port})، الذي تكشفه بنفسك.',
  'home.lines.tcp.reaches': 'API محرك متوافق مع Docker، بلا تشفير.',
  'home.lines.tcp.needs': 'منفذ API الخاص بالمحرك (عادةً {port}). فقط على شبكة تثق بها.',
  'home.lines.cluster.name': 'API العنقود',
  'home.lines.cluster.reaches':
    'Kubernetes من ملف kubeconfig لديك، وNomad بعنوان الوكيل الخاص به، وIncus عبر HTTPS برمز ثقة.',
  'home.lines.cluster.needs': 'API الذي يقدّمه العنقود بالفعل. لا يُثبَّت أي شيء على أي عقدة.',
  'home.lines.advice': 'استخدم SSH متى استطعت: فهو لا يضيف إلى الخادم شيئًا لا تشغّله بالفعل.',
  'home.lines.import':
    'هل وصفت خوادمك من قبل؟ استوردها من {file}، بما فيها خوادم القفز، أو من سياقات Docker لديك.',
  'home.lines.link': 'كيف تعمل الاتصالات',

  // Home: the Engines, as the map's index of stations.
  'home.engines.title': 'كل محرك، بالكلمات نفسها.',
  'home.engines.lead':
    'المضيف مضيف والمكدس مكدس، أيًّا كان المحرك الذي يشغّله. وحيث يملك محرك شيئًا خاصًا به، يعرضه Homeport أيضًا.',
  'home.engines.docker.body':
    'الحاويات والصور ووحدات التخزين والشبكات ومكدسات Compose: محفوظة في Homeport، أو مرتبطة بمجلد، أو متزامنة من مستودع git.',
  'home.engines.swarm.body':
    'العقد والخدمات والمهام والأسرار والتكوينات، يُعثر عليها من خلال مضيفاته المديرة. انشر المكدسات على Swarm.',
  'home.engines.podman.body':
    'الـ Pods، ووحدات Quadlet تُثبَّت وتُشغَّل تحت systemd، عبر API الخاص بـ Podman نفسه.',
  'home.engines.containerd.body':
    'مساحات الأسماء والمهام والسجلات وطرفية واللقطات التي تستهلك قرصك، دون Docker.',
  'home.engines.incus.body':
    'حاويات النظام والأجهزة الافتراضية: وحدة التحكم الخاصة بها، ولقطاتها، وصورها، وملفات تعريفها، وتخزينها، وشبكاتها.',
  'home.engines.kubernetes.body':
    'أحمال العمل والـ Pods والـ Services وأي مورد API، مباشرةً. السجلات والطرفيات وإعادة توجيه المنافذ وYAML مع معاينة وإصدارات Helm.',
  'home.engines.nomad.body':
    'jobs تُشغَّل من HCL مع خطتها، وallocations الخاصة بها وسجلاتها، وصدفة داخل مهمة، وتفريغ العقد.',

  // Home: security.
  'home.security.title': 'مفاتيحك تبقى مختومة.',
  'home.security.lead':
    'يحتفظ Homeport ببيانات الاعتماد التي تفتح خوادمك، لذا يعاملها على هذا الأساس.',
  'home.security.rest.title': 'مشفّرة أثناء التخزين',
  'home.security.rest.body':
    'تُختم مفاتيح SSH وشهادات TLS وبيئات المكدسات ورموز المستودعات باستخدام XChaCha20-Poly1305. ويقيم المفتاح في سلسلة مفاتيح نظام التشغيل، لا بجوار البيانات.',
  'home.security.disk.title': 'لا تُكتب على القرص بلا تشفير أبدًا',
  'home.security.disk.body':
    'يتحدث Homeport بروتوكول SSH بنفسه، لذا تبقى المفاتيح المفكوك تشفيرها في الذاكرة. لا ملفات مفاتيح، ولا تغييرات على {dir}.',
  'home.security.home.title': 'لا شيء يتصل بالخارج',
  'home.security.home.body':
    'لا حساب، ولا قياس عن بُعد، ولا تتبّع. كل إصدار يُبنى علنًا، مع مجاميع اختبارية وإثبات مصدر البناء.',
  'home.security.link': 'الأمان وبياناتك',

  // Home: the close, where the visitor starts.
  'home.close.title': 'ابدأ من هذا الجهاز.',
  'home.close.body':
    'نزّل Homeport على الكمبيوتر الذي تعمل منه، ثم أضف مضيفاتك. إنه مجاني، ومفتوح المصدر بموجب رخصة GNU AGPL، ويُبنى علنًا على GitHub.',
  'home.close.github': 'عرض على GitHub',
  'home.desktops.windows.versions': '10 و11، x64 وARM64',
  'home.desktops.windows.formats': 'أداة التثبيت (.exe) أو MSI',
  'home.desktops.macos.versions': 'Apple silicon وIntel',
  'home.desktops.macos.formats': 'صورة قرص (.dmg)',
  'home.desktops.linux.versions': 'x64 وARM64',
  'home.desktops.linux.formats': 'apt أو dnf أو zypper أو AppImage',
};
