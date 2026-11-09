/** Brazilian Portuguese text of the `clusterevents` namespace. */
import type en from '../en/clusterevents';
import type { Translation } from '../types';

export default {
  'clusterevents.nav': 'Eventos',
  'clusterevents.title': 'Eventos',
  'clusterevents.summary': '{count, plural, one {# evento} other {# eventos}}',
  'clusterevents.search': 'Pesquisar por motivo, objeto ou mensagem',
  'clusterevents.empty': 'Nenhum evento. O Kubernetes os mantém por cerca de uma hora.',
  'clusterevents.filter': 'Tipo',
  'clusterevents.all': 'Todos',
  'clusterevents.warnings': 'Avisos',
  'clusterevents.normal': 'Normal',
  'clusterevents.column.type': 'Tipo',
  'clusterevents.column.reason': 'Motivo',
  'clusterevents.column.object': 'Objeto',
  'clusterevents.column.namespace': 'Namespace',
  'clusterevents.column.message': 'Mensagem',
  'clusterevents.column.count': 'Contagem',
  'clusterevents.column.lastSeen': 'Visto por último',
} satisfies Translation<typeof en>;
