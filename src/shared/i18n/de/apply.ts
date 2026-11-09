/** German text of the `apply` namespace. */
import type en from '../en/apply';
import type { Translation } from '../types';

export default {
  'apply.nav': 'YAML anwenden',
  'apply.title': 'YAML anwenden',
  'apply.description':
    'Fügen Sie Manifeste ein oder öffnen Sie sie, sehen Sie, was sie auf {cluster} ändern würden, und wenden Sie sie dann an, wie kubectl apply.',
  'apply.open': 'Datei öffnen…',
  'apply.openTitle': 'Manifeste öffnen',
  'apply.openFailed': 'Homeport konnte diese Datei nicht lesen',
  'apply.manifests': 'Kubernetes-Manifeste',
  'apply.editor': 'Anzuwendende Manifeste',
  'apply.defaultNamespace': 'Dokumente ohne Namespace landen in {namespace}.',
  'apply.defaultNamespaceCluster':
    'Dokumente ohne Namespace landen im Standard-Namespace des Clusters.',
} satisfies Translation<typeof en>;
