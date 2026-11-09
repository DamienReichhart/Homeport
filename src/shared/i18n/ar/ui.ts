/** Arabic text of the `ui` namespace. */
import type en from '../en/ui';
import type { Translation } from '../types';

export default {
  'ui.copy': 'نسخ',
  'ui.copied': 'تم النسخ',
  'ui.search': 'بحث',
  'ui.shortcuts.focusSearch': 'البحث في هذه الصفحة',
  'ui.shortcuts.thisPage': 'هذه الصفحة',
  'ui.table.selectAll': 'تحديد الكل',
  'ui.table.selectRow': 'تحديد الصف',
  'ui.table.empty': 'لا يوجد شيء هنا.',
  'ui.table.page':
    'الصفحة {page} من {pages} · {rows, plural, zero {لا صفوف} one {صف واحد} two {صفان} few {# صفوف} many {# صفًا} other {# صف}}',
  'ui.table.previous': 'الصفحة السابقة',
  'ui.table.next': 'الصفحة التالية',
  'ui.transfer.done': 'تم.',
  'ui.files.column.name': 'الاسم',
  'ui.files.column.size': 'الحجم',
  'ui.files.column.modified': 'تاريخ التعديل',
  'ui.files.column.mode': 'الأذونات',
  'ui.files.path': 'المسار',
  'ui.files.downloadOne': 'تنزيل {name}',
  'ui.files.empty': 'هذا المجلد فارغ.',
  'ui.chart.time': 'الوقت',
  'ui.errorDetails.title': 'تفاصيل الخطأ',
  'ui.errorDetails.code': 'الرمز',
  'ui.errorDetails.command': 'الأمر',
  'ui.errorDetails.when': 'الوقت',
  'ui.errorDetails.app': 'التطبيق',
  'ui.errorDetails.rawDetail': 'التفاصيل الخام',
  'ui.errorDetails.rawDetailNote': 'كما كتبها النظام أو المحرك أو المكتبة، لذا لم تُترجم.',
  'ui.errorDetails.openLogFolder': 'فتح مجلد السجلات',
  'ui.errorDetails.copyReport': 'نسخ التقرير',
} satisfies Translation<typeof en>;
