/** French text of the `clusterevents` namespace. */
import type en from '../en/clusterevents';
import type { Translation } from '../types';

export default {
  'clusterevents.nav': 'Événements',
  'clusterevents.title': 'Événements',
  'clusterevents.summary': '{count, plural, one {# événement} other {# événements}}',
  'clusterevents.search': 'Rechercher par motif, objet ou message',
  'clusterevents.empty': 'Aucun événement. Kubernetes les conserve environ une heure.',
  'clusterevents.filter': 'Type',
  'clusterevents.all': 'Tous',
  'clusterevents.warnings': 'Avertissements',
  'clusterevents.normal': 'Normal',
  'clusterevents.column.type': 'Type',
  'clusterevents.column.reason': 'Motif',
  'clusterevents.column.object': 'Objet',
  'clusterevents.column.namespace': 'Namespace',
  'clusterevents.column.message': 'Message',
  'clusterevents.column.count': 'Nombre',
  'clusterevents.column.lastSeen': 'Vu pour la dernière fois',
} satisfies Translation<typeof en>;
