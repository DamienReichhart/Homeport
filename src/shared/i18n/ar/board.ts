/** Arabic text of the `board` namespace. */
import type en from '../en/board';
import type { Translation } from '../types';

export default {
  'board.nav': 'اللوحة',
  'board.title': 'اللوحة',
  'board.summary':
    '{running} من {count, plural, zero {0 حاوية} one {حاوية واحدة} two {حاويتين} few {# حاويات} many {# حاوية} other {# حاوية}} قيد التشغيل على {hosts, plural, zero {0 مضيف} one {مضيف واحد} two {مضيفين} few {# مضيفات} many {# مضيفًا} other {# مضيف}}',
  'board.column.host': 'المضيف',
  'board.column.stack': 'المكدس',
  'board.sortBy': 'الفرز حسب {column}',
  'board.filter.hosts': 'المضيفات',
  'board.filter.allHosts': 'كل المضيفات',
  'board.filter.someHosts':
    '{count, plural, zero {لا مضيفات} one {مضيف واحد} two {مضيفان} few {# مضيفات} many {# مضيفًا} other {# مضيف}}',
  'board.filter.state': 'الحالة',
  'board.filter.stack': 'المكدس',
  'board.filter.anyStack': 'أي مكدس',
  'board.filter.clear': 'مسح عوامل التصفية',
  'board.state.all': 'أي حالة',
  'board.state.running': 'قيد التشغيل',
  'board.state.stopped': 'متوقفة',
  'board.state.unhealthy': 'غير سليمة',
  'board.search': 'البحث في كل المضيفات',
  'board.skipped': 'لا يُعرض {name} ({status}).',
  'board.openHost': 'فتح المضيف',
  'board.listFailed': 'تعذّر سرد حاويات {name}.',
  'board.noHosts.title': 'لا يوجد مضيف متصل',
  'board.noHosts.description': 'تسرد اللوحة حاويات كل مضيف متصل.',
  'board.empty.title': 'لا توجد حاويات بعد',
  'board.empty.description': 'لا يشغّل أي من مضيفاتك المتصلة حاوية.',
  'board.noMatch': 'لا توجد حاوية تطابق عوامل التصفية هذه.',
  'board.selection': 'الحاويات المحددة',
  'board.selectAll': 'تحديد كل الحاويات المعروضة',
  'board.selectOne': 'تحديد {name} على {host}',
  'board.remove.title':
    'هل تريد إزالة {count, plural, zero {لا حاويات} one {حاوية واحدة} two {حاويتين} few {# حاويات} many {# حاوية} other {# حاوية}}؟',
  'board.remove.description':
    'تُحذف مع وحدات التخزين المجهولة الخاصة بها؛ وتُحتفظ بوحدات التخزين المسماة. تُوقف الحاويات قيد التشغيل أولًا.',
  'board.remove.onHost': 'على {host}: {names}',
} satisfies Translation<typeof en>;
