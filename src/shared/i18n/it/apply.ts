/** Italian text of the `apply` namespace. */
import type en from '../en/apply';
import type { Translation } from '../types';

export default {
  'apply.nav': 'Applica YAML',
  'apply.title': 'Applica YAML',
  'apply.description':
    'Incolla o apri dei manifest, guarda cosa cambierebbero su {cluster}, poi applicali, come con kubectl apply.',
  'apply.open': 'Apri file…',
  'apply.openTitle': 'Apri manifest',
  'apply.openFailed': 'Homeport non è riuscito a leggere il file',
  'apply.manifests': 'Manifest Kubernetes',
  'apply.editor': 'Manifest da applicare',
  'apply.defaultNamespace': 'I documenti senza namespace vanno in {namespace}.',
  'apply.defaultNamespaceCluster':
    'I documenti senza namespace vanno nel namespace predefinito del cluster.',
} satisfies Translation<typeof en>;
