/** Arabic text of the `settings` namespace. */
import type en from '../en/settings';
import type { Translation } from '../types';

export default {
  'settings.nav': 'الإعدادات',
  'settings.title': 'الإعدادات',
  'settings.appearance.title': 'المظهر',
  'settings.appearance.theme': 'السمة',
  'settings.theme.system': 'مطابقة النظام',
  'settings.theme.light': 'فاتحة',
  'settings.theme.dark': 'داكنة',
  'settings.appearance.language': 'اللغة',
  'settings.language.system': 'مطابقة النظام ({language})',
  'settings.desktop.title': 'سطح المكتب',
  'settings.desktop.description': 'طريقة عمل Homeport على هذا الكمبيوتر.',
  'settings.desktop.closeToTray': 'الإغلاق إلى علبة النظام',
  'settings.desktop.closeToTrayHint':
    'يُبقي إغلاق النافذة Homeport واتصالاته بالمضيفات قيد التشغيل في علبة النظام. اخرج من أيقونة العلبة. في بيئات سطح المكتب التي تُخفي أيقونات العلبة، مثل GNOME دون إضافة، قد لا تظهر الأيقونة.',
  'settings.desktop.noTray':
    'لا تحتوي بيئة سطح المكتب هذه على علبة نظام يمكن لـ Homeport استخدامها، لذا يؤدي إغلاق النافذة إلى إنهائه.',
  'settings.desktop.startAtLogin': 'البدء عند تسجيل الدخول',
  'settings.desktop.startAtLoginHint':
    'يبدأ Homeport عند تسجيل دخولك إلى هذا الكمبيوتر، فيراقب مضيفاتك منذ البداية.',
  'settings.desktop.startHidden': 'البدء مخفيًا في علبة النظام',
  'settings.desktop.startHiddenHint':
    'عند بدئه مع تسجيل الدخول، يبقى Homeport في علبة النظام دون فتح نافذته.',
  'settings.desktop.startHiddenNoTray':
    'يتطلب علبة نظام لفتح النافذة منها، وهي غير متوفرة في بيئة سطح المكتب هذه.',
  'settings.notifications.title': 'الإشعارات',
  'settings.notifications.description':
    'يُعلمك Homeport، حتى عندما تكون نافذته مغلقة إلى علبة النظام، عندما:',
  'settings.notifications.containerDied': 'تتوقف حاوية بسبب خطأ أو نفاد الذاكرة',
  'settings.notifications.containerUnhealthy': 'تصبح حاوية غير سليمة',
  'settings.notifications.hostUnreachable': 'يصبح الوصول إلى مضيف متصل متعذرًا',
  'settings.notifications.hostBack': 'يعود مضيف كان الوصول إليه متعذرًا',
  'settings.machine.title': 'هذا الجهاز',
  'settings.machine.description': 'ما يمكن لـ Homeport استخدامه هنا.',
  'settings.machine.keychain': 'سلسلة مفاتيح نظام التشغيل',
  'settings.machine.keychainAvailable':
    'متاحة — تُخزَّن مفاتيح SSH والشهادات وبيئات المكدسات وعمليات تسجيل الدخول إلى السجلات مشفّرة.',
  'settings.machine.keychainUnavailable':
    'غير متاحة — لا يمكن تخزين مفاتيح SSH وشهادات TLS وبيئات المكدسات وعمليات تسجيل الدخول إلى السجلات.',
  'settings.machine.compose': 'واجهة سطر أوامر Compose',
  'settings.machine.composeMissing': 'يمكن تشغيل المكدسات وإيقافها، لكن لا يمكن نشرها.',
  'settings.machine.stackDeploy': 'Docker CLI، لـ Swarm',
  'settings.machine.stackDeployMissing':
    'لا يمكن نشر المكدسات على Swarm؛ ويعمل كل ما عدا ذلك في Swarm.',
  'settings.machine.git': 'git',
  'settings.machine.helm': 'Helm CLI',
  'settings.sync.title': 'المكدسات المتزامنة',
  'settings.sync.interval': 'التحقق من المستودعات كل',
  'settings.sync.minutes': 'دقيقة',
  'settings.data.title': 'البيانات',
  'settings.data.description': 'ما يحتفظ به Homeport على هذا الكمبيوتر.',
  'settings.data.backup': 'نسخ احتياطي…',
  'settings.data.restore': 'استعادة…',
  'settings.data.backupHint':
    'احفظ كل مضيف ومكدس وتسجيل دخول إلى سجل ومستودع وإعداد، مع أسرارها، في ملف واحد محمي بعبارة مرور، واستعده على هذا الكمبيوتر أو على كمبيوتر آخر.',
  'settings.backup.title': 'نسخ بيانات Homeport احتياطيًا',
  'settings.backup.description':
    'يحتوي الملف على كل مضيف وعنقود ومكدس وتسجيل دخول إلى سجل ومستودع وإعداد، ومفاتيحها وشهاداتها ورموزها وكلمات مرورها وبيئاتها. يمكن لأي شخص يملكه ويعرف عبارة مروره استخدامها، لذا احفظ الاثنين في مكان آمن. لا تُضمَّن أحداث المحرك وسجل النشر.',
  'settings.backup.passphrase': 'عبارة المرور',
  'settings.backup.passphraseHint': '{min} أحرف على الأقل. لا يمكن استردادها.',
  'settings.backup.confirmation': 'عبارة المرور مرة أخرى',
  'settings.backup.problem.short': 'استخدم {min} أحرف على الأقل.',
  'settings.backup.problem.mismatch': 'عبارتا المرور مختلفتان.',
  'settings.backup.start': 'نسخ احتياطي…',
  'settings.backup.saveTitle': 'حفظ النسخة الاحتياطية باسم',
  'settings.backup.fileType': 'نسخة Homeport الاحتياطية',
  'settings.backup.done':
    'تم نسخ {hosts, plural, zero {0 مضيف} one {مضيف واحد} two {مضيفين} few {# مضيفات} many {# مضيفًا} other {# مضيف}} و{stacks, plural, zero {0 مكدس} one {مكدس واحد} two {مكدسين} few {# مكدسات} many {# مكدسًا} other {# مكدس}} احتياطيًا',
  'settings.backup.withoutSecrets': 'نُسخت هذه احتياطيًا دون أسرارها، التي تعذّر فتحها:',
  'settings.restore.pick': 'الاستعادة من',
  'settings.restore.title': 'استعادة بيانات Homeport',
  'settings.restore.from': 'من {path}',
  'settings.restore.open': 'فتح',
  'settings.restore.madeBy': 'نسخها Homeport {version} احتياطيًا بتاريخ {date}. تحتوي على:',
  'settings.restore.holds': 'محتوى النسخة الاحتياطية',
  'settings.restore.hosts':
    '{count, plural, zero {لا مضيفات} one {مضيف واحد} two {مضيفان} few {# مضيفات} many {# مضيفًا} other {# مضيف}}',
  'settings.restore.clusters':
    '{count, plural, zero {لا عناقيد} one {عنقود واحد} two {عنقودان} few {# عناقيد} many {# عنقودًا} other {# عنقود}}',
  'settings.restore.stacks':
    '{count, plural, zero {لا مكدسات} one {مكدس واحد} two {مكدسان} few {# مكدسات} many {# مكدسًا} other {# مكدس}}',
  'settings.restore.registries':
    '{count, plural, zero {لا عمليات تسجيل دخول إلى سجلات} one {تسجيل دخول واحد إلى سجل} two {تسجيلا دخول إلى سجلات} few {# عمليات تسجيل دخول إلى سجلات} many {# عملية تسجيل دخول إلى سجلات} other {# عملية تسجيل دخول إلى سجلات}}',
  'settings.restore.repositories':
    '{count, plural, zero {لا مستودعات} one {مستودع واحد} two {مستودعان} few {# مستودعات} many {# مستودعًا} other {# مستودع}}',
  'settings.restore.settings': 'إعداداتك',
  'settings.restore.replaces':
    'تستبدل الاستعادة كل مضيف ومكدس وتسجيل دخول إلى سجل ومستودع وإعداد لديك الآن، وتنسى أحداث المحرك وسجل النشر. يُحتفظ أولًا بنسخة من البيانات الحالية في مجلد بيانات Homeport.',
  'settings.restore.confirm': 'استبدال كل شيء',
  'settings.restore.done':
    'تمت استعادة {hosts, plural, zero {0 مضيف} one {مضيف واحد} two {مضيفين} few {# مضيفات} many {# مضيفًا} other {# مضيف}} و{stacks, plural, zero {0 مكدس} one {مكدس واحد} two {مكدسين} few {# مكدسات} many {# مكدسًا} other {# مكدس}}',
  'settings.data.keepEvents': 'الاحتفاظ بأحداث المحرك لمدة',
  'settings.data.days': 'يوم',
  'settings.data.keepEventsHint':
    'يحتفظ كل مضيف بـ 10,000 حدث على الأكثر لصفحة الأحداث. تُزال الأحداث الأقدم كل ساعة.',
  'settings.machine.gitMissing':
    'تحتاج إليه المكدسات المتزامنة؛ ويعمل كل نوع آخر من المكدسات بدونه.',
  'settings.machine.helmMissing':
    'لا يمكن سرد إصدارات Helm أو تثبيتها؛ ويعمل كل ما عدا ذلك في العناقيد.',
  'settings.about.tagline': 'كل محركات Docker، على لوحة واحدة.',
  'settings.about.openLogFolder': 'فتح مجلد السجلات',
  'settings.about.licenses': 'التراخيص',
  'settings.about.exportDiagnostics': 'تصدير التشخيص…',
  'settings.diagnostics.title': 'تصدير التشخيص',
  'settings.diagnostics.description':
    'ملف zip لإرفاقه ببلاغ عن خطأ: ملخّص للإصدارات ولأنواع المضيفات والعناقيد التي تستخدمها، وسجلات Homeport بعد إزالة الأسرار منها. لا يُرسَل شيء إلى أي مكان: أنت من يقرّر مشاركة الملف.',
  'settings.diagnostics.includeNames': 'تضمين الأسماء والعناوين',
  'settings.diagnostics.includeNamesHelp':
    'عند الإيقاف، تظهر المضيفات والعناقيد بالشكل host-1 و cluster-1… في الملخّص وفي السجلات.',
  'settings.diagnostics.files': 'الملفات في ملف zip',
  'settings.diagnostics.save': 'حفظ…',
  'settings.diagnostics.saveTitle': 'حفظ التشخيص',
  'settings.diagnostics.fileType': 'أرشيف zip',
  'settings.diagnostics.done': 'حُفظ التشخيص في {path}.',
  'settings.licenses.title': 'التراخيص',
  'settings.licenses.description':
    'Homeport برنامج حر بموجب رخصة GNU Affero العمومية العامة، الإصدار 3، مبني على الحزم مفتوحة المصدر أدناه.',
  'settings.licenses.source': 'الشيفرة المصدرية لهذا الإصدار منشورة بموجب الترخيص نفسه.',
  'settings.licenses.sourceLink': 'مصدر v{version}',
  'settings.licenses.thirdParty': 'برامج الجهات الخارجية',
  'settings.licenses.missing.title': 'غير مضمّنة في هذا الإصدار',
  'settings.licenses.missing.description':
    'تتضمنها إصدارات الإطلاق؛ شغّل <code>{command}</code> لإنشائها لإصدار تطويري.',
  'settings.palette.group': 'الإعدادات',
  'settings.palette.themeLight': 'السمة: فاتحة',
  'settings.palette.themeDark': 'السمة: داكنة',
  'settings.palette.themeSystem': 'السمة: مطابقة النظام',
} satisfies Translation<typeof en>;
