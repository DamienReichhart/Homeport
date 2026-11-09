/** Hindi text of the `apiresources` namespace. */
import type en from '../en/apiresources';
import type { Translation } from '../types';

export default {
  'apiresources.nav': 'API रिसोर्स',
  'apiresources.title': 'API रिसोर्स',
  'apiresources.summary':
    'इस क्लस्टर द्वारा दिए जाने वाले {count, plural, one {# प्रकार} other {# प्रकार}}',
  'apiresources.refresh': 'रीफ़्रेश करें',
  'apiresources.types': 'रिसोर्स के प्रकार',
  'apiresources.search': 'Kind या छोटा नाम (pvc)',
  'apiresources.searchObjects': 'खोजें',
  'apiresources.noMatch': 'कोई प्रकार मेल नहीं खाता।',
  'apiresources.coreGroup': 'core',
  'apiresources.pick':
    'किसी प्रकार के ऑब्जेक्ट की सूची देखने के लिए उसे चुनें: बिल्ट-इन प्रकार, और वे जो आपके CRD जोड़ते हैं।',
  'apiresources.typeGone': 'यह क्लस्टर वह प्रकार (अब) नहीं देता।',
  'apiresources.cantList': 'सूची नहीं बनाई जा सकती',
  'apiresources.notAllowed': 'आपके क्रेडेंशियल इसकी सूची नहीं देख सकते',
  'apiresources.cantListWhy':
    '{kind} ऑब्जेक्ट की सूची नहीं बनाई जा सकती: इसका API उन्हें सिर्फ़ एक-एक करके बनाने या पढ़ने देता है।',
  'apiresources.empty': 'यहाँ कुछ नहीं है।',
  'apiresources.column.name': 'नाम',
  'apiresources.column.namespace': 'Namespace',
  'apiresources.column.age': 'उम्र',
  'apiresources.tab.overview': 'अवलोकन',
  'apiresources.tab.conditions': 'कंडीशन',
  'apiresources.tab.events': 'इवेंट',
  'apiresources.tab.source': 'YAML',
  'apiresources.detail.namespace': 'Namespace',
  'apiresources.detail.owners': 'स्वामी',
  'apiresources.detail.age': 'उम्र',
  'apiresources.detail.labels': 'लेबल',
  'apiresources.detail.annotations': 'एनोटेशन',
  'apiresources.edit': 'संपादित करें',
  'apiresources.copyYaml': 'YAML कॉपी करें',
} satisfies Translation<typeof en>;
