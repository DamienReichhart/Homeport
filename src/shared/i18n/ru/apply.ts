/** Russian text of the `apply` namespace. */
import type en from '../en/apply';
import type { Translation } from '../types';

export default {
  'apply.nav': 'Применить YAML',
  'apply.title': 'Применить YAML',
  'apply.description':
    'Вставьте или откройте манифесты, посмотрите, что они изменят в {cluster}, и примените их, как kubectl apply.',
  'apply.open': 'Открыть файл…',
  'apply.openTitle': 'Открыть манифесты',
  'apply.openFailed': 'Homeport не удалось прочитать этот файл',
  'apply.manifests': 'Манифесты Kubernetes',
  'apply.editor': 'Манифесты для применения',
  'apply.defaultNamespace': 'Документы без пространства имён попадут в {namespace}.',
  'apply.defaultNamespaceCluster':
    'Документы без пространства имён попадут в пространство имён кластера по умолчанию.',
} satisfies Translation<typeof en>;
