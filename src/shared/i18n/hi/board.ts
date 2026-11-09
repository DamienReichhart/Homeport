/** Hindi text of the `board` namespace. */
import type en from '../en/board';
import type { Translation } from '../types';

export default {
  'board.nav': 'बोर्ड',
  'board.title': 'बोर्ड',
  'board.summary':
    '{hosts, plural, one {# होस्ट} other {# होस्ट}} पर {count, plural, one {# कंटेनर} other {# कंटेनर}} में से {running} चल रहे हैं',
  'board.column.host': 'होस्ट',
  'board.column.stack': 'स्टैक',
  'board.sortBy': '{column} के अनुसार क्रमबद्ध करें',
  'board.filter.hosts': 'होस्ट',
  'board.filter.allHosts': 'सभी होस्ट',
  'board.filter.someHosts': '{count, plural, one {# होस्ट} other {# होस्ट}}',
  'board.filter.state': 'स्थिति',
  'board.filter.stack': 'स्टैक',
  'board.filter.anyStack': 'कोई भी स्टैक',
  'board.filter.clear': 'फ़िल्टर हटाएँ',
  'board.state.all': 'कोई भी स्थिति',
  'board.state.running': 'चल रहा है',
  'board.state.stopped': 'रुका हुआ',
  'board.state.unhealthy': 'अस्वस्थ',
  'board.search': 'हर होस्ट में खोजें',
  'board.skipped': '{name} ({status}) नहीं दिखाया गया है।',
  'board.openHost': 'होस्ट खोलें',
  'board.listFailed': '{name} के कंटेनर की सूची नहीं बन सकी।',
  'board.noHosts.title': 'कोई होस्ट कनेक्ट नहीं है',
  'board.noHosts.description': 'बोर्ड हर कनेक्ट किए गए होस्ट के कंटेनर दिखाता है।',
  'board.empty.title': 'अभी कोई कंटेनर नहीं',
  'board.empty.description': 'आपके किसी भी कनेक्ट किए गए होस्ट पर कोई कंटेनर नहीं चल रहा।',
  'board.noMatch': 'इन फ़िल्टर से कोई कंटेनर मेल नहीं खाता।',
  'board.selection': 'चुने गए कंटेनर',
  'board.selectAll': 'दिखाए गए हर कंटेनर को चुनें',
  'board.selectOne': '{host} पर {name} चुनें',
  'board.remove.title': '{count, plural, one {# कंटेनर} other {# कंटेनर}} हटाएँ?',
  'board.remove.description':
    'वे अपने अनाम वॉल्यूम के साथ हटा दिए जाते हैं; नाम वाले वॉल्यूम रखे जाते हैं। चल रहे कंटेनर पहले रोके जाते हैं।',
  'board.remove.onHost': '{host} पर: {names}',
} satisfies Translation<typeof en>;
