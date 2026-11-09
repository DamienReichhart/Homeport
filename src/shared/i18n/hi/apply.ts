/** Hindi text of the `apply` namespace. */
import type en from '../en/apply';
import type { Translation } from '../types';

export default {
  'apply.nav': 'YAML लागू करें',
  'apply.title': 'YAML लागू करें',
  'apply.description':
    'मैनिफ़ेस्ट पेस्ट करें या खोलें, देखें कि वे {cluster} पर क्या बदलेंगे, फिर उन्हें लागू करें, kubectl apply की तरह।',
  'apply.open': 'फ़ाइल खोलें…',
  'apply.openTitle': 'मैनिफ़ेस्ट खोलें',
  'apply.openFailed': 'Homeport वह फ़ाइल नहीं पढ़ सका',
  'apply.manifests': 'Kubernetes मैनिफ़ेस्ट',
  'apply.editor': 'लागू करने के लिए मैनिफ़ेस्ट',
  'apply.defaultNamespace': 'जिन दस्तावेज़ों में Namespace नहीं है, वे {namespace} में जाते हैं।',
  'apply.defaultNamespaceCluster':
    'जिन दस्तावेज़ों में Namespace नहीं है, वे क्लस्टर के डिफ़ॉल्ट Namespace में जाते हैं।',
} satisfies Translation<typeof en>;
