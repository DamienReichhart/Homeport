/** Arabic text of the `clusterevents` namespace. */
import type en from '../en/clusterevents';
import type { Translation } from '../types';

export default {
  'clusterevents.nav': 'الأحداث',
  'clusterevents.title': 'الأحداث',
  'clusterevents.summary':
    '{count, plural, zero {لا أحداث} one {حدث واحد} two {حدثان} few {# أحداث} many {# حدثًا} other {# حدث}}',
  'clusterevents.search': 'البحث حسب السبب أو الكائن أو الرسالة',
  'clusterevents.empty': 'لا توجد أحداث. يحتفظ بها Kubernetes لمدة ساعة تقريبًا.',
  'clusterevents.filter': 'النوع',
  'clusterevents.all': 'الكل',
  'clusterevents.warnings': 'التحذيرات',
  'clusterevents.normal': 'عادي',
  'clusterevents.column.type': 'النوع',
  'clusterevents.column.reason': 'السبب',
  'clusterevents.column.object': 'الكائن',
  'clusterevents.column.namespace': 'Namespace',
  'clusterevents.column.message': 'الرسالة',
  'clusterevents.column.count': 'العدد',
  'clusterevents.column.lastSeen': 'آخر ظهور',
} satisfies Translation<typeof en>;
