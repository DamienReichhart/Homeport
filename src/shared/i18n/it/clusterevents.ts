/** Italian text of the `clusterevents` namespace. */
import type en from '../en/clusterevents';
import type { Translation } from '../types';

export default {
  'clusterevents.nav': 'Eventi',
  'clusterevents.title': 'Eventi',
  'clusterevents.summary': '{count, plural, one {# evento} other {# eventi}}',
  'clusterevents.search': 'Cerca per motivo, oggetto o messaggio',
  'clusterevents.empty': 'Nessun evento. Kubernetes li conserva per circa un’ora.',
  'clusterevents.filter': 'Tipo',
  'clusterevents.all': 'Tutti',
  'clusterevents.warnings': 'Avvisi',
  'clusterevents.normal': 'Normali',
  'clusterevents.column.type': 'Tipo',
  'clusterevents.column.reason': 'Motivo',
  'clusterevents.column.object': 'Oggetto',
  'clusterevents.column.namespace': 'Namespace',
  'clusterevents.column.message': 'Messaggio',
  'clusterevents.column.count': 'Conteggio',
  'clusterevents.column.lastSeen': 'Ultima occorrenza',
} satisfies Translation<typeof en>;
