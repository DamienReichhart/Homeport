/** Hindi text of the `search` namespace. */
import type en from '../en/search';
import type { Translation } from '../types';

export default {
  'search.kind.container': 'कंटेनर',
  'search.kind.stack': 'स्टैक',
  'search.kind.image': 'इमेज',
  'search.kind.volume': 'वॉल्यूम',
  'search.kind.network': 'नेटवर्क',
  'search.onHostState': '{host} · {state, select, running {चल रहा है} other {रुका हुआ}}',
  'search.showAll': 'सभी {count} दिखाएँ',
  'search.skipped':
    '{count, plural, one {# होस्ट में} other {# होस्ट में}} नहीं खोजा गया (कनेक्ट नहीं है)',
} satisfies Translation<typeof en>;
