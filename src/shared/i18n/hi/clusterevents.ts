/** Hindi text of the `clusterevents` namespace. */
import type en from '../en/clusterevents';
import type { Translation } from '../types';

export default {
  'clusterevents.nav': 'इवेंट',
  'clusterevents.title': 'इवेंट',
  'clusterevents.summary': '{count, plural, one {# इवेंट} other {# इवेंट}}',
  'clusterevents.search': 'कारण, ऑब्जेक्ट या संदेश से खोजें',
  'clusterevents.empty': 'कोई इवेंट नहीं। Kubernetes उन्हें लगभग एक घंटे तक रखता है।',
  'clusterevents.filter': 'प्रकार',
  'clusterevents.all': 'सभी',
  'clusterevents.warnings': 'चेतावनियाँ',
  'clusterevents.normal': 'सामान्य',
  'clusterevents.column.type': 'प्रकार',
  'clusterevents.column.reason': 'कारण',
  'clusterevents.column.object': 'ऑब्जेक्ट',
  'clusterevents.column.namespace': 'Namespace',
  'clusterevents.column.message': 'संदेश',
  'clusterevents.column.count': 'संख्या',
  'clusterevents.column.lastSeen': 'आखिरी बार देखा गया',
} satisfies Translation<typeof en>;
