/** Spanish text of the `apply` namespace. */
import type en from '../en/apply';
import type { Translation } from '../types';

export default {
  'apply.nav': 'Aplicar YAML',
  'apply.title': 'Aplicar YAML',
  'apply.description':
    'Pega o abre manifiestos, mira qué cambiarían en {cluster} y luego aplícalos, como kubectl apply.',
  'apply.open': 'Abrir archivo…',
  'apply.openTitle': 'Abrir manifiestos',
  'apply.openFailed': 'Homeport no pudo leer ese archivo',
  'apply.manifests': 'Manifiestos de Kubernetes',
  'apply.editor': 'Manifiestos que aplicar',
  'apply.defaultNamespace': 'Los documentos sin namespace van a {namespace}.',
  'apply.defaultNamespaceCluster':
    'Los documentos sin namespace van al namespace predeterminado del clúster.',
} satisfies Translation<typeof en>;
