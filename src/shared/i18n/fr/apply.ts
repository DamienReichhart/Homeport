/** French text of the `apply` namespace. */
import type en from '../en/apply';
import type { Translation } from '../types';

export default {
  'apply.nav': 'Appliquer du YAML',
  'apply.title': 'Appliquer du YAML',
  'apply.description':
    'Voyez ce que des manifestes changeraient sur {cluster}, puis appliquez-les.',
  'apply.open': 'Ouvrir un fichier…',
  'apply.openTitle': 'Ouvrir des manifestes',
  'apply.openFailed': 'Homeport n’a pas pu lire ce fichier',
  'apply.manifests': 'Manifestes Kubernetes',
  'apply.editor': 'Manifestes à appliquer',
  'apply.defaultNamespace': 'Les documents sans namespace vont dans {namespace}.',
  'apply.defaultNamespaceCluster':
    'Les documents sans namespace vont dans le namespace par défaut du cluster.',
} satisfies Translation<typeof en>;
