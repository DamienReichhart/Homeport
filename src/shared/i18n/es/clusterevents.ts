/** Spanish text of the `clusterevents` namespace. */
import type en from '../en/clusterevents';
import type { Translation } from '../types';

export default {
  'clusterevents.nav': 'Eventos',
  'clusterevents.title': 'Eventos',
  'clusterevents.summary': '{count, plural, one {# evento} other {# eventos}}',
  'clusterevents.search': 'Buscar por motivo, objeto o mensaje',
  'clusterevents.empty': 'No hay eventos. Kubernetes los conserva aproximadamente una hora.',
  'clusterevents.filter': 'Tipo',
  'clusterevents.all': 'Todos',
  'clusterevents.warnings': 'Advertencias',
  'clusterevents.normal': 'Normales',
  'clusterevents.column.type': 'Tipo',
  'clusterevents.column.reason': 'Motivo',
  'clusterevents.column.object': 'Objeto',
  'clusterevents.column.namespace': 'Namespace',
  'clusterevents.column.message': 'Mensaje',
  'clusterevents.column.count': 'Recuento',
  'clusterevents.column.lastSeen': 'Visto por última vez',
} satisfies Translation<typeof en>;
