/** Arabic text of the `search` namespace. */
import type en from '../en/search';
import type { Translation } from '../types';

export default {
  'search.kind.container': 'الحاويات',
  'search.kind.stack': 'المكدسات',
  'search.kind.image': 'الصور',
  'search.kind.volume': 'وحدات التخزين',
  'search.kind.network': 'الشبكات',
  'search.onHostState': '{host} · {state, select, running {قيد التشغيل} other {متوقفة}}',
  'search.showAll': 'عرض الكل ({count})',
  'search.skipped':
    '{count, plural, zero {لا مضيفات} one {مضيف واحد} two {مضيفان} few {# مضيفات} many {# مضيفًا} other {# مضيف}} لم يُبحث فيها (غير متصلة)',
} satisfies Translation<typeof en>;
