/** Arabic text of the `events` namespace. */
import type en from '../en/events';
import type { Translation } from '../types';

export default {
  'events.nav': 'الأحداث',
  'events.title': 'الأحداث',
  'events.description': 'ما أبلغ عنه المحرك، محفوظًا عبر عمليات إعادة التشغيل.',
  'events.descriptionKept':
    'ما أبلغ عنه المحرك، محفوظًا لمدة {days, plural, zero {# يوم} one {يوم واحد} two {يومين} few {# أيام} many {# يومًا} other {# يوم}}، بحد أقصى 10,000 حدث.',
  'events.clear': 'مسح السجل',
  'events.clear.title': 'هل تريد مسح سجل الأحداث؟',
  'events.clear.description':
    'تُنسى كل الأحداث المحفوظة لهذا المضيف. تستمر الأحداث الجديدة في الظهور؛ ولا تتأثر أحداث المحرك نفسه.',
  'events.clear.confirm': 'مسح السجل',
  'events.cleared': 'تم مسح سجل الأحداث',
  'events.loadOlder': 'تحميل الأحداث الأقدم',
  'events.search': 'البحث في الأحداث',
  'events.type': 'النوع',
  'events.allTypes': 'كل الأنواع',
  'events.empty.title': 'لا توجد أحداث بعد',
  'events.empty.description': 'شغّل حاوية أو أوقفها وستظهر هنا.',
  'events.column.time': 'الوقت',
  'events.column.type': 'النوع',
  'events.column.action': 'الإجراء',
  'events.column.object': 'الكائن',
  'events.column.stack': 'المكدس',
} satisfies Translation<typeof en>;
