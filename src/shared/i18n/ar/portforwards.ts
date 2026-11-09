/** Arabic text of the `portforwards` namespace. */
import type en from '../en/portforwards';
import type { Translation } from '../types';

export default {
  'portforwards.nav': 'إعادة توجيه المنافذ',
  'portforwards.title': 'إعادة توجيه المنافذ',
  'portforwards.summary':
    '{count, plural, zero {لا عمليات إعادة توجيه} one {إعادة توجيه واحدة} two {عمليتا إعادة توجيه} few {# عمليات إعادة توجيه} many {# عملية إعادة توجيه} other {# عملية إعادة توجيه}}',
  'portforwards.empty.title': 'لا توجد عمليات إعادة توجيه للمنافذ',
  'portforwards.empty.description':
    'أعد توجيه منفذ من صفحة Pod أو Service للوصول إليه من هذا الكمبيوتر. تنتهي عمليات إعادة التوجيه عند إغلاق Homeport.',
  'portforwards.column.local': 'العنوان المحلي',
  'portforwards.column.target': 'يعيد التوجيه إلى',
  'portforwards.column.namespace': 'Namespace',
  'portforwards.column.cluster': 'العنقود',
  'portforwards.column.connections': 'الاتصالات',
  'portforwards.forward': 'إعادة توجيه…',
  'portforwards.forwardPort': 'إعادة توجيه المنفذ {port}',
  'portforwards.open': 'فتح',
  'portforwards.stop': 'إيقاف',
  'portforwards.stopFailed': 'تعذّر على Homeport إيقاف إعادة التوجيه',
  'portforwards.started': 'تتم إعادة توجيه {address} إلى {target}:{port}',
  'portforwards.failed': 'فشلت إحدى عمليات إعادة توجيه المنافذ',
  'portforwards.ended': 'انتهت إحدى عمليات إعادة توجيه المنافذ',
  'portforwards.dialog.title': 'إعادة توجيه {target}:{port}',
  'portforwards.dialog.description':
    'في الـ Namespace {namespace}. يمكنك الوصول إليه عبر المنفذ المحلي ما دامت إعادة التوجيه تعمل.',
  'portforwards.dialog.localPort': 'المنفذ المحلي',
  'portforwards.dialog.anyPort': 'أي منفذ متاح',
  'portforwards.dialog.loopback':
    'لا يمكن الاتصال إلا من هذا الكمبيوتر: تستمع إعادة التوجيه على 127.0.0.1 (و::1).',
  'portforwards.dialog.submit': 'إعادة توجيه',
} satisfies Translation<typeof en>;
