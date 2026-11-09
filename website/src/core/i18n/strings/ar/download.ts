import type { Translation } from '../../types';
import type en from '../en/download';

/** The download page, and the download button other pages show, in Arabic. */
export default {
  'download.title': 'تنزيل Homeport',
  'download.description':
    'نزّل Homeport لأنظمة Windows وmacOS وLinux — أدوات تثبيت لمعماريتي x64 وARM ومستودعات حزم Linux، مع مجاميع اختبارية وإثبات مصدر البناء.',

  // The download button and the line under it; the notes replace the default once the visitor's system is known.
  'download.button': 'تنزيل Homeport',
  'download.button.for': 'تنزيل لنظام {os}',
  'download.meta.version': 'الإصدار {version}',
  'download.meta.everywhere': 'Windows وmacOS وLinux',
  'download.meta.all': 'كل التنزيلات',
  'download.note.windowsArm': 'أداة تثبيت Windows على ARM',
  'download.note.windowsX64': 'أداة تثبيت Windows x64',
  'download.note.macosArm': 'لأجهزة Mac بشرائح Apple silicon',
  'download.note.macosX64': 'لأجهزة Mac بمعالجات Intel',
  'download.note.macos': 'Apple silicon وIntel',
  'download.note.linux': '.deb و.rpm وAppImage',

  // The hero.
  'download.hero.eyebrow': 'التنزيل',
  'download.hero.title': 'احصل على Homeport',
  'download.hero.release': 'الإصدار {version}، صدر في {date}.',
  'download.hero.whatsNew': 'ما الجديد',
  'download.hero.noRelease': 'أدوات التثبيت لكل الأنظمة موجودة في {link}.',
  'download.hero.releasesPage': 'صفحة الإصدارات على GitHub',
  'download.recommend': 'أنت تستخدم {platform}. وضعنا تنزيلاته في المقدمة.',
  'download.recommend.arch': '{os} على {arch}',

  // The platforms and their installers.
  'download.preview': '{label} {name} إصدار تجريبي للاختبار. {link}.',
  'download.preview.label': 'تتوفر معاينة:',
  'download.preview.link': 'نزّله من GitHub',
  'download.windows.requirement': 'Windows 10 أو 11',
  'download.macos.requirement': 'macOS 10.15 أو أحدث',
  'download.linux.requirement': 'Ubuntu 22.04 أو Debian 12 أو Fedora 36 أو أحدث',
  'download.platform.yours': 'نظامك',
  'download.platform.guide': 'دليل التثبيت لنظام {platform}',
  'download.arch.appleSilicon': 'Apple silicon',
  'download.arch.intel': 'Intel',
  'download.arch.arm64': 'ARM64',
  'download.arch.x64': 'x64',
  'download.format.nsis.label': 'أداة التثبيت',
  'download.format.nsis.hint': 'مستحسنة. تثبّت للمستخدم الحالي.',
  'download.format.msi.label': 'حزمة MSI',
  'download.format.msi.hint': 'لعمليات التثبيت المُدارة والصامتة.',
  'download.format.dmg.label': 'صورة قرص',
  'download.format.dmg.hint': 'اسحب Homeport إلى «التطبيقات».',
  'download.format.deb.label': 'حزمة Debian',
  'download.format.deb.hint': 'Debian وUbuntu وLinux Mint وPop!_OS.',
  'download.format.rpm.label': 'حزمة RPM',
  'download.format.rpm.hint': 'Fedora وRHEL وRocky وopenSUSE.',
  'download.format.appimage.label': 'AppImage',
  'download.format.appimage.hint': 'أي توزيعة، دون خطوة تثبيت.',
  'download.files.file': '{format} ({file})',
  'download.files.checksums': 'مجاميع SHA-256 الاختبارية',
  'download.files.none': 'راجع {link}.',
  'download.files.latest': 'أحدث إصدار على GitHub',

  // The Linux package repository.
  'download.repo.title': 'تثبيت سريع',
  'download.repo.body':
    'يضيف مستودع حزم Homeport ويثبّت Homeport، ليُبقيه apt أو dnf أو zypper محدّثًا.',
  'download.repo.copy': 'نسخ',
  'download.repo.copied': 'تم النسخ',
  'download.repo.script': '{link} أولًا إن شئت، أو أعدّ المستودع بنفسك:',
  'download.repo.scriptLink': 'اقرأ النص البرمجي',
  'download.repo.manual': 'الإعداد اليدوي',
  'download.repo.apt': 'Debian 12 أو Ubuntu 22.04 أو Linux Mint 21 أو Pop!_OS 22.04 أو أحدث',
  'download.repo.dnf': 'Fedora 36 أو RHEL 10 أو Rocky Linux وAlmaLinux 10 أو أحدث',
  'download.repo.zypper': 'openSUSE Tumbleweed وLeap 16',
  'download.repo.packages':
    'أو نزّل حزمة أدناه. تضيف الحزمتان {deb} و{rpm} المستودع أيضًا، فتحصلان على التحديثات كذلك.',

  // What to know around the install.
  'download.before.title': 'قبل التثبيت',
  'download.before.body':
    'يدير Homeport محركات Docker — ولا يتضمن محركًا. شغّله بجانب Docker Engine أو Docker Desktop أو Colima أو OrbStack أو Rancher Desktop أو Podman، أو وجّهه إلى خادم بعيد عبر SSH أو TLS أو TCP.',
  'download.before.keychain':
    '{keychain} (سلسلة مفاتيح macOS، أو مدير بيانات الاعتماد في Windows، أو خدمة Secret Service مثل GNOME Keyring) لتخزين مفاتيح SSH وشهادات TLS وبيئات المكدسات ومفاتيح النشر والرموز الخاصة بالمستودعات.',
  'download.before.keychainName': 'سلسلة مفاتيح نظام التشغيل',
  'download.before.compose': '{tool} على هذا الجهاز لنشر المكدسات المُدارة والمرتبطة والمتزامنة.',
  'download.before.git': '{tool} على هذا الجهاز للمكدسات المتزامنة، التي تُنشر من مستودع git.',
  'download.before.optional': 'كلها اختيارية: يخبرك Homeport بما لا يتوفر من دونها. راجع {link}.',
  'download.before.requirements': 'متطلبات النظام',
  'download.first.title': 'التشغيل الأول',
  'download.first.body':
    'أدوات التثبيت ليست موقّعة بعد بشهادة مدفوعة، لذا قد يطلب نظامك تأكيدًا مرة واحدة:',
  'download.first.windows':
    '{os}: يعرض SmartScreen الرسالة «قام Windows بحماية الكمبيوتر». اختر {moreInfo}، ثم {runAnyway}.',
  'download.first.moreInfo': 'مزيد من المعلومات',
  'download.first.runAnyway': 'تشغيل على أي حال',
  'download.first.macos':
    '{os}: إذا كان «لا يمكن فتح» Homeport، فاسمح به في {settings} واختر {openAnyway}.',
  'download.first.settings': 'إعدادات النظام ← الخصوصية والأمن',
  'download.first.openAnyway': 'فتح على أي حال',
  'download.first.link': 'تعليمات خطوة بخطوة',
  'download.updates.title': 'البقاء على آخر إصدار',
  'download.updates.body':
    'على Windows وmacOS ومع AppImage، يخبرك Homeport عند صدور إصدار جديد ويثبّته متى اخترت. كل تحديث موقّع ويُتحقق منه قبل تثبيته.',
  'download.updates.packages':
    'تتحدّث الحزمتان {deb} و{rpm} مع بقية نظامك، من مستودع حزم Homeport.',
  'download.updates.link': 'كيف تعمل التحديثات',
  'download.verify.title': 'تحقق من تنزيلك',
  'download.verify.body':
    'يُبنى كل إصدار بواسطة GitHub Actions من الشيفرة المصدرية العامة، مع مجموع اختباري SHA-256 وشهادة موقّعة لإثبات مصدر البناء لكل ملف.',
  'download.verify.link': 'كيفية التحقق',

  // Other ways to get it.
  'download.other.title': 'طرق أخرى للحصول على Homeport',
  'download.other.previous.title': 'الإصدارات السابقة',
  'download.other.previous.body': 'كل إصدار، مع ملاحظاته وملفاته، على GitHub.',
  'download.other.build.title': 'البناء من الشيفرة المصدرية',
  'download.other.build.body': 'Bun وRust والمتطلبات المسبقة لـ Tauri.',
  'download.other.source.title': 'الشيفرة المصدرية',
  'download.other.source.body': 'مفتوحة المصدر بموجب رخصة AGPL، على GitHub.',
  'download.later.title': 'قريبًا',
  'download.later.homebrew': 'brew install --cask homeport، على macOS',
  'download.later.winget': 'winget install، على Windows',
  'download.later.scoop': 'مستودع bucket لـ Homeport، على Windows',
  'download.later.aur': 'homeport-bin، على Arch Linux',
  'download.later.flathub': 'حزمة Flatpak، على أي توزيعة Linux',
  'download.later.snap': 'متجر Snap Store، على Ubuntu وغيره',
} satisfies Translation<typeof en>;
