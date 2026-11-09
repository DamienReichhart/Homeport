/** French text of the `apply` namespace. */
import type en from '../en/apply';
import type { Translation } from '../types';

export default {
  'apply.nav': 'Appliquer du YAML',
  'apply.title': 'Appliquer du YAML',
  'apply.description':
    'Collez ou ouvrez des manifestes, voyez ce qu’ils changeraient sur {cluster}, puis appliquez-les, comme kubectl apply.',
  'apply.open': 'Ouvrir un fichier…',
  'apply.openTitle': 'Ouvrir des manifestes',
  'apply.openFailed': 'Homeport n’a pas pu lire ce fichier',
  'apply.manifests': 'Manifestes Kubernetes',
  'apply.editor': 'Manifestes à appliquer',
  'apply.defaultNamespace': 'Les documents sans namespace vont dans {namespace}.',
  'apply.defaultNamespaceCluster':
    'Les documents sans namespace vont dans le namespace par défaut du cluster.',
} satisfies Translation<typeof en>;
