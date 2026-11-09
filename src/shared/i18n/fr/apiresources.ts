/** French text of the `apiresources` namespace. */
import type en from '../en/apiresources';
import type { Translation } from '../types';

export default {
  'apiresources.nav': 'Ressources de l’API',
  'apiresources.title': 'Ressources de l’API',
  'apiresources.summary':
    '{count, plural, one {# type servi} other {# types servis}} par ce cluster',
  'apiresources.refresh': 'Actualiser',
  'apiresources.types': 'Types de ressources',
  'apiresources.search': 'Kind ou nom court (pvc)',
  'apiresources.searchObjects': 'Rechercher',
  'apiresources.noMatch': 'Aucun type ne correspond.',
  'apiresources.coreGroup': 'core',
  'apiresources.pick':
    'Choisissez un type pour lister ses objets : ceux intégrés, et ceux qu’ajoutent vos CRD.',
  'apiresources.typeGone': 'Ce cluster ne sert pas (ou plus) ce type.',
  'apiresources.cantList': 'Impossible à lister',
  'apiresources.notAllowed': 'Vos identifiants ne permettent pas de le lister',
  'apiresources.cantListWhy':
    'Les objets {kind} ne peuvent pas être listés : son API permet seulement de les créer ou de les lire un par un.',
  'apiresources.empty': 'Rien ici.',
  'apiresources.column.name': 'Nom',
  'apiresources.column.namespace': 'Namespace',
  'apiresources.column.age': 'Âge',
  'apiresources.tab.overview': 'Vue d’ensemble',
  'apiresources.tab.conditions': 'Conditions',
  'apiresources.tab.events': 'Événements',
  'apiresources.tab.source': 'YAML',
  'apiresources.detail.namespace': 'Namespace',
  'apiresources.detail.owners': 'Appartient à',
  'apiresources.detail.age': 'Âge',
  'apiresources.detail.labels': 'Labels',
  'apiresources.detail.annotations': 'Annotations',
  'apiresources.edit': 'Modifier',
  'apiresources.copyYaml': 'Copier le YAML',
} satisfies Translation<typeof en>;
