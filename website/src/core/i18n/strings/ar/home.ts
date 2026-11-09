import type { Translation } from '../../types';
import type en from '../en/home';

/** The home page, in Arabic. */
export default {
  // Home: the hero and its board.
  'home.title': 'Homeport — كل محركات Docker، على لوحة واحدة.',
  'home.hero.new': 'جديد',
  'home.hero.release': 'صدر Homeport {version}',
  'home.hero.title': 'كل محركات Docker، على لوحة واحدة.',
  'home.hero.lead':
    'يعمل Homeport على جهازك ويصل إلى مضيفاتك عبر SSH أو TLS أو TCP. لا وكيل على الخادم. مجاني، مفتوح المصدر، Windows وmacOS وLinux.',
  'home.hero.source': 'اقرأ الشيفرة المصدرية',
  'home.board.caption':
    'لوحة على سبيل المثال: خمسة مضيفات، ومحركاتها، وطريقة وصول Homeport إليها. أحدها يحتاج إلى انتباهك.',
  'home.board.host': 'المضيف',
  'home.board.engine': 'المحرك',
  'home.board.via': 'عبر',
  'home.board.stacks': 'المكدسات',
  'home.board.status': 'الحالة',
  'home.board.allRunning': 'الكل قيد التشغيل',
  'home.board.needsYou': 'مكدس واحد يحتاج إليك',
  'home.board.note': 'ما هو سليم يبقى عاديًا. وحده ما يحتاج إليك يتحول إلى الأحمر.',
  'home.runtimes.label': 'يعمل مع',
  'home.runtimes.title': 'يعمل مع ما تشغّله بالفعل',

  // Home: where it lives.
  'home.visits.eyebrow': 'أين يقيم',
  'home.visits.title': 'يبقى Homeport على جهازك ويزور خوادمك. ولا يستقر فيها أبدًا.',
  'home.visits.agent.title': 'لا وكيل',
  'home.visits.agent.body':
    'لا شيء يُثبَّت أو يُحدَّث أو يُؤمَّن على الخادم. يتحدث Homeport إلى المحرك الموجود هناك بالفعل.',
  'home.visits.panel.title': 'لا لوحة ويب',
  'home.visits.panel.body':
    'لا لوحة معلومات منشورة على خوادمك، ولا صفحة تسجيل دخول تنتظر أن يعثر عليها أحد.',
  'home.visits.account.title': 'لا حساب',
  'home.visits.account.body':
    'لا تسجيل دخول، ولا مقعد، ولا اشتراك، ولا قياس عن بُعد. يعرض لك حاوياتك، وهذا كل شيء.',
  'home.visits.ports.title': 'ما يحتاج إليه على الخادم، بصراحة',
  'home.visits.ports.ssh':
    'المنفذ المفتوح لديك بالفعل. ومع المفتاح الذي ينشئه Homeport، سطر واحد في {file}.',
  'home.visits.ports.tls': 'منفذ API الخاص بالمحرك (عادةً {port})، الذي تكشفه بنفسك.',
  'home.visits.ports.tcp':
    'منفذ API الخاص بالمحرك (عادةً {port})، بلا تشفير. فقط على شبكة تثق بها.',
  'home.visits.ports.advice':
    'استخدم SSH متى استطعت: فهو لا يضيف إلى الخادم شيئًا لا تشغّله بالفعل.',
  'home.visits.link': 'كيف تعمل الاتصالات',

  // Home: the features.
  'home.features.eyebrow': 'الميزات',
  'home.features.title': 'كل ما تشغّله محركاتك، في نافذة واحدة',
  'home.features.lead':
    'عمل Docker اليومي — ما الذي يعمل، وماذا يقول، وكم يستهلك — دون أن تتذكر على أي خادم يقيم.',
  'home.features.hosts.title': 'مضيفات قريبة وبعيدة',
  'home.features.hosts.body':
    'يعثر Homeport من أجلك على المحركات المحلية وأجهزة Podman. أما البعيدة فتنضم عبر SSH، ومن خلال خوادم Bastion عند الحاجة، أو عبر TLS المتبادل أو TCP، أو مباشرةً من ~/.ssh/config وسياقات Docker لديك. ويعيد كل مضيف الاتصال من تلقاء نفسه عندما تنقطع الشبكة للحظة.',
  'home.features.containers.title': 'الحاويات بنظرة واحدة',
  'home.features.containers.body':
    'مجمّعة حسب المكدس، مع استخدام المعالج والذاكرة مباشرةً وسبب كون إحداها غير سليمة. شغّل حاوية مع حدود وأجهزة وفحوص سلامة، ثم حرّرها أو كرّرها أو احفظها كصورة أو صدّرها.',
  'home.features.stacks.title': 'مكدسات Compose',
  'home.features.stacks.body':
    'ابدأ من قالب، أو اربط مجلد مشروع على هذا الكمبيوتر، أو تابع مستودع git، ثم انشر على أي مضيف — بما في ذلك عمليات البناء. اطّلع على ما سيتغير قبل كل إعادة نشر أو مزامنة، وتراجع، وحدّث إلى صور أحدث في خطوة واحدة.',
  'home.features.swarm.title': 'Docker Swarm',
  'home.features.swarm.body':
    'يظهر كل Swarm من خلال مضيفاته المديرة. أدر العقد والخدمات والمهام والأسرار والتكوينات، وانشر المكدسات على Swarm، وابدأ Swarm أو انضم إليه أو غادره من أحد المضيفات، دون تثبيت أي شيء على أي عقدة.',
  'home.features.podman.title': 'Podman، كما هو Podman',
  'home.features.podman.body':
    'يتعرّف Homeport على Podman عند الاتصال ويضيف ما لا يملكه إلا Podman: الـ Pods، تُنشأ بمنافذها وتُشغَّل فيها الحاويات، ووحدات Quadlet، تُثبَّت وتُحرَّر وتُشغَّل تحت systemd، عبر API الخاص بـ Podman نفسه.',
  'home.features.containerd.title': 'containerd، مباشرةً',
  'home.features.containerd.body':
    'اتصل بـ containerd عبر مقبسه، محليًا أو عبر SSH، دون Docker: اختر مساحة أسماء، وشغّل مهام حاوياتها وأوقفها وأرسل إليها الإشارات، واقرأ سجلاتها، وافتح طرفية، واسحب الصور بتسجيلات دخولك إلى السجلات، واعثر على اللقطات التي تستهلك قرصك.',
  'home.features.incus.title': 'مثيلات Incus',
  'home.features.incus.body':
    'أضف Incus عبر مقبسه أو SSH أو HTTPS برمز الثقة، وشغّل حاويات النظام والأجهزة الافتراضية الخاصة به: أنشئها من صورة بعيدة، وشغّلها وجمّدها وأوقفها، وافتح وحدة التحكم الخاصة بها أو صدفة، والتقط اللقطات واستعدها، وأدر الصور وملفات التعريف والتخزين والشبكات.',
  'home.features.clusters.title': 'عناقيد Kubernetes وNomad',
  'home.features.clusters.body':
    'أضف عناقيد Kubernetes من ملف kubeconfig لديك — بما فيها EKS وGKE وAKS — أو اعثر على العناقيد المحلية بنقرة واحدة. تصفّح أحمال العمل والـ Pods والـ Services والتكوين وأي مورد API مباشرةً؛ تابع السجلات، وافتح طرفية، وأعد توجيه المنافذ، وغيّر عدد النسخ وطبّق YAML مع معاينة، وثبّت مخططات Helm ورقّها وتراجع عنها. أضف عناقيد Nomad بعنوانها لتشغيل jobs من HCL مع خطتها، وتغيير عدد نسخها وإيقافها، ومتابعة السجلات، وفتح صدفة، وتفريغ العقد. لا يُثبَّت أي شيء على العنقود.',
  'home.features.images.title': 'الصور',
  'home.features.images.body':
    'ابحث في Docker Hub، واسحب وادفع بتسجيلات دخولك إلى السجلات، وابنِ من مجلد على الكمبيوتر، واحفظ الأرشيفات وحمّلها، واعرف متى تصدر صورة أحدث.',
  'home.features.volumes.title': 'وحدات التخزين والشبكات',
  'home.features.volumes.body':
    'اعرف أي حاويات تستخدم ماذا. أنشئ واتصل وافصل، مع حذف يخبرك بالضبط بما أنت على وشك أن تفقده.',
  'home.features.board.title': 'لوحة واحدة لكل المضيفات',
  'home.features.board.body':
    'كل حاوية من كل مضيف في جدول واحد، تُصفّى ويُنفَّذ عليها الإجراء معًا. ولكل مضيف صفحة واحدة أيضًا: تفاصيل المحرك، واستخدام القرص مع تنظيف في مربع حوار واحد، وبث مباشر لما يفعله المحرك.',
  'home.features.keyboard.title': 'لوحة المفاتيح أولًا',
  'home.features.keyboard.body':
    'لوحة أوامر للانتقال إلى أي مكان، أو تبديل المضيف، أو تشغيل أمر، وللعثور على أي حاوية أو صورة أو وحدة تخزين على أي مضيف توجد عليه. اختصارات لكل صفحة، تعرضها ?.',
  'home.features.background.title': 'مراقبة في الخلفية',
  'home.features.background.body':
    'حالة كل مضيف في علبة النظام، وإشعارات عندما تتعطل حاوية أو ينقطع مضيف، وHomeport يبدأ عند تسجيل الدخول، مخفيًا حتى تحتاج إليه.',

  // Home: Hosts.
  'home.hosts.eyebrow': 'المضيفات',
  'home.hosts.title': 'محركات بعيدة دون مراسم',
  'home.hosts.body':
    'أضف خادمًا مرة واحدة فيبقى على اللوحة. ولـ SSH، ينشئ Homeport مفتاحًا لهذا المضيف وحده ويعطيك الأمر الوحيد الذي يأذن له. ويتحقق من بصمة الخادم عند أول اتصال، وينبهك إذا تغيرت يومًا.',
  'home.hosts.import':
    'هل وصفتها من قبل؟ استورد خوادمك من {file}، بما فيها خوادم القفز، أو سياقات Docker لديك، بما فيها الشهادات. وعند الانتقال إلى كمبيوتر جديد، خذها معك في ملف واحد مقفل بعبارة مرور.',
  'home.hosts.socket.name': 'مقبس محلي',
  'home.hosts.socket.detail': 'مقابس Unix والأنابيب المسمّاة في Windows، تُكتشف تلقائيًا.',
  'home.hosts.ssh.detail':
    'مفتاح Ed25519 مخصص لكل مضيف، أو مفتاحك الخاص، أو ssh-agent أو Pageant؛ ومن خلال خوادم Bastion أيضًا.',
  'home.hosts.tls.detail': 'TLS متبادل مع جهة الإصدار CA وشهادة العميل ومفتاحه.',
  'home.hosts.tcp.detail': 'TCP عادي غير مشفّر، فقط لمحرك على شبكة تثق بها.',
  'home.hosts.link': 'كيف تعمل الاتصالات',
  'home.hosts.screenshot':
    'صفحة «إضافة مضيف» مع تحديد SSH: الخادم، والمنفذ، والمستخدم، ومسار المقبس البعيد، ومفتاح جديد لهذا المضيف.',

  // Home: containers.
  'home.containers.eyebrow': 'الحاويات',
  'home.containers.title': 'كل ما يخص الحاوية، على بُعد نقرة',
  'home.containers.body':
    'افتح حاوية تجد سجلاتها تتدفق بالفعل. بدّل علامات التبويب للحصول على طرفية تفاعلية، وملفاتها، ومخططات مباشرة، وعملياتها، ومستند الفحص الكامل — على مضيف في الطرف الآخر من العالم بسهولة ما على حاسوبك المحمول.',
  'home.containers.logs.title': 'السجلات',
  'home.containers.logs.body': 'تابع وابحث وصفِّ حسب التدفق والوقت ونزّل.',
  'home.containers.terminal.title': 'الطرفية',
  'home.containers.terminal.body': 'صدفات تفاعلية، بالعدد الذي تحتاج إليه.',
  'home.containers.files.title': 'الملفات',
  'home.containers.files.body': 'تصفّح وارفع ونزّل، والمجلدات بصيغة .tar.',
  'home.containers.stats.title': 'الإحصاءات',
  'home.containers.stats.body': 'مخططات المعالج والذاكرة والشبكة وإدخال القرص وإخراجه.',
  'home.containers.link': 'العمل مع الحاويات',
  'home.containers.screenshot':
    'علامة التبويب «السجلات» لحاوية، تبث المخرجات مع البحث والطوابع الزمنية وعوامل تصفية التدفق.',

  // Home: Stacks.
  'home.stacks.eyebrow': 'المكدسات',
  'home.stacks.title': 'مكدسات Compose، مُدارة أو مرتبطة أو متزامنة أو مكتشفة',
  'home.stacks.body':
    'احتفظ بملف compose وبيئته في Homeport وانشره على أي مضيف — ابدأ من قالب، واجمع عدة ملفات، واختر ملفات التعريف؛ تُخزَّن المتغيرات مشفّرة، ولا تُكتب على القرص أبدًا. أو اربط مجلد مشروع على هذا الكمبيوتر: يحرّر Homeport ملفاته في مكانها وينشر منه، بما في ذلك عمليات البناء. أو تابع فرعًا من مستودع git، بمفتاح نشر أو برمز: تعرض كل مزامنة الإيداعات الجديدة وما تغيّره قبل النشر، ويتحقق Homeport منها ما دام مفتوحًا، وينشر من تلقاء نفسه إذا طلبت ذلك. كل إعادة نشر تعرض ما سيتغير أولًا، وكل نشر يُحتفظ به لتتمكن من التراجع، و«تحديث» يسحب صورًا أحدث ويعيد النشر في خطوة واحدة. والمكدسات التي نشرها أحدهم يدويًا تظهر أيضًا: شغّلها أو أوقفها أو أزلها، أو تبنَّها.',
  'home.stacks.link': 'إدارة المكدسات',
  'home.stacks.screenshot': 'تعريف مكدس مُدار: محرر compose.yaml، وبيئته، وزر «حفظ وإعادة النشر».',

  // Home: security.
  'home.security.eyebrow': 'الأمان',
  'home.security.title': 'مفاتيحك تبقى مختومة',
  'home.security.lead':
    'يتعامل Homeport مع بيانات الاعتماد التي تفتح خوادمك، لذا يعاملها على هذا الأساس.',
  'home.security.rest.title': 'مشفّرة أثناء التخزين',
  'home.security.rest.body':
    'تُختم مفاتيح SSH وشهادات TLS وبيئات المكدسات ورموز المستودعات باستخدام XChaCha20-Poly1305. ويقيم المفتاح في سلسلة مفاتيح نظام التشغيل، لا بجوار البيانات.',
  'home.security.disk.title': 'لا تُكتب على القرص بلا تشفير أبدًا',
  'home.security.disk.body':
    'يتحدث Homeport بروتوكول SSH بنفسه، لذا تبقى المفاتيح المفكوك تشفيرها في الذاكرة. لا ملفات مفاتيح، ولا تغييرات على {dir}.',
  'home.security.home.title': 'لا شيء يتصل بالخارج',
  'home.security.home.body':
    'لا حساب، ولا قياس عن بُعد، ولا تتبّع. يتحدث Homeport إلى محركاتك ولا شيء غيرها. كل إصدار يُبنى علنًا، مع مجاميع اختبارية وإثبات مصدر البناء.',
  'home.security.link': 'الأمان وبياناتك',

  // Home: desktops and the closing call.
  'home.desktops.eyebrow': 'التنزيل',
  'home.desktops.title': 'أصلي على كل سطح مكتب',
  'home.desktops.lead':
    'تطبيق صغير وسريع مبني بـ Tauri وRust — لا علامة تبويب متصفح متنكرة. أدوات تثبيت لمعماريتي x64 وARM تُبقي Homeport محدّثًا.',
  'home.desktops.windows.versions': '10 و11 · x64 وARM64',
  'home.desktops.windows.formats': 'أداة التثبيت (.exe) · MSI',
  'home.desktops.macos.versions': 'Apple silicon وIntel',
  'home.desktops.macos.formats': 'صورة قرص (.dmg)',
  'home.desktops.linux.versions': 'x64 وARM64',
  'home.desktops.linux.formats': 'apt · dnf · zypper · AppImage',
  'home.cta.title': 'لا شيء يُثبَّت على خوادمك.',
  'home.cta.body':
    'نزّل Homeport على الجهاز الذي تعمل منه وأضف مضيفاتك. إنه مجاني، ومفتوح المصدر بموجب رخصة GNU AGPL، ويُبنى علنًا على GitHub.',
  'home.cta.github': 'عرض على GitHub',
} satisfies Translation<typeof en>;
