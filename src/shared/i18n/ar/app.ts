/** Arabic text of the `app` namespace. */
import type en from '../en/app';
import type { Translation } from '../types';

export default {
  'app.nav.main': 'الرئيسية',
  'app.nav.docs': 'الوثائق',
  'app.nav.github': 'GitHub',
  'app.nav.opensInBrowser': '{label} (يُفتح في المتصفح)',
  'app.keychainMissing':
    'لا تتوفر سلسلة مفاتيح في نظام التشغيل، لذا لا يمكن تخزين مفاتيح SSH وشهادات TLS وبيئات المكدسات. تظل المضيفات المحلية ومضيفات TCP وssh-agent تعمل.',
  'app.startup.title': 'يتعذّر على Homeport فتح قاعدة بياناته',
  'app.startup.description': 'لم يُشغَّل أي شيء آخر. قاعدة البيانات موجودة في هذا الملف:',
  'app.startup.openFolder': 'فتح المجلد',
  'app.crash.title': 'حدث خطأ ما',
  'app.crash.description':
    'واجه Homeport مشكلة غير متوقعة في هذه الصفحة. عادةً ما تساعد إعادة التحميل؛ وإذا تكرر الأمر، فانسخ التقرير من «التفاصيل» إلى بلاغ (issue).',
  'app.crash.reload': 'إعادة التحميل',
  'app.crash.backToOverview': 'العودة إلى النظرة العامة',
  'app.shortcuts.title': 'اختصارات لوحة المفاتيح',
  'app.shortcuts.description':
    'الاختصارات التي تعمل هنا. لا تعمل مفاتيح مثل N و/ أثناء الكتابة في حقل.',
  'app.shortcuts.general': 'عام',
  'app.shortcuts.navigation': 'انتقال إلى',
  'app.shortcuts.hosts': 'المضيفات',
  'app.shortcuts.help': 'عرض اختصارات لوحة المفاتيح',
  'app.shortcuts.settings': 'فتح الإعدادات',
  'app.shortcuts.goTo': 'الانتقال إلى {page}',
  'app.shortcuts.previousHost': 'المضيف السابق',
  'app.shortcuts.nextHost': 'المضيف التالي',
  'app.shortcuts.refresh': 'تحديث قوائم هذا المضيف',
  'app.palette.title': 'لوحة الأوامر',
  'app.palette.description': 'انتقل إلى صفحة، أو بدّل المضيف، أو شغّل أمرًا.',
  'app.palette.open': 'فتح لوحة الأوامر',
  'app.palette.placeholder': 'اكتب اسم صفحة أو مضيف أو أمر…',
  'app.palette.empty': 'لا توجد نتائج مطابقة.',
  'app.palette.recent': 'الأخيرة',
  'app.palette.goTo': 'انتقال إلى',
  'app.palette.hosts': 'المضيفات',
  'app.palette.actions': 'الإجراءات',
  'app.palette.switchTo': 'التبديل إلى {name}',
} satisfies Translation<typeof en>;
