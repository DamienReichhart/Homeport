/** Arabic text of the `apply` namespace. */
import type en from '../en/apply';
import type { Translation } from '../types';

export default {
  'apply.nav': 'تطبيق YAML',
  'apply.title': 'تطبيق YAML',
  'apply.description': 'اطّلع على ما ستغيّره ملفات البيان في {cluster}، ثم طبّقها.',
  'apply.open': 'فتح ملف…',
  'apply.openTitle': 'فتح ملفات البيان',
  'apply.openFailed': 'تعذّر على Homeport قراءة ذلك الملف',
  'apply.manifests': 'ملفات بيان Kubernetes',
  'apply.editor': 'ملفات البيان المراد تطبيقها',
  'apply.defaultNamespace': 'تذهب المستندات التي لا تحدد Namespace إلى {namespace}.',
  'apply.defaultNamespaceCluster':
    'تذهب المستندات التي لا تحدد Namespace إلى Namespace الافتراضية للعنقود.',
} satisfies Translation<typeof en>;
