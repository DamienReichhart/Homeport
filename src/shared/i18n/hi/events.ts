/** Hindi text of the `events` namespace. */
import type en from '../en/events';
import type { Translation } from '../types';

export default {
  'events.nav': 'इवेंट',
  'events.title': 'इवेंट',
  'events.description': 'इंजन ने जो रिपोर्ट किया, रीस्टार्ट के बाद भी रखा गया।',
  'events.descriptionKept':
    'इंजन ने जो रिपोर्ट किया, {days, plural, one {# दिन} other {# दिन}} तक रखा जाता है, अधिकतम 10,000।',
  'events.clear': 'इतिहास साफ़ करें',
  'events.clear.title': 'इवेंट का इतिहास साफ़ करें?',
  'events.clear.description':
    'इस होस्ट के लिए रखा गया हर इवेंट भुला दिया जाएगा। नए इवेंट आते रहेंगे; इंजन के अपने इवेंट पर कोई असर नहीं पड़ता।',
  'events.clear.confirm': 'इतिहास साफ़ करें',
  'events.cleared': 'इवेंट का इतिहास साफ़ कर दिया गया',
  'events.loadOlder': 'पुराने इवेंट लोड करें',
  'events.search': 'इवेंट खोजें',
  'events.type': 'प्रकार',
  'events.allTypes': 'सभी प्रकार',
  'events.empty.title': 'अभी कोई इवेंट नहीं',
  'events.empty.description': 'कोई कंटेनर शुरू करें या रोकें, वह यहाँ दिखेगा।',
  'events.column.time': 'समय',
  'events.column.type': 'प्रकार',
  'events.column.action': 'कार्रवाई',
  'events.column.object': 'ऑब्जेक्ट',
  'events.column.stack': 'स्टैक',
} satisfies Translation<typeof en>;
