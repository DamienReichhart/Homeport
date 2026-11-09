/** Italian text of the `board` namespace. */
import type en from '../en/board';
import type { Translation } from '../types';

export default {
  'board.nav': 'Bacheca',
  'board.title': 'Bacheca',
  'board.summary':
    '{running} di {count, plural, one {# container} other {# container}} in esecuzione su {hosts, plural, one {# host} other {# host}}',
  'board.column.host': 'Host',
  'board.column.stack': 'Stack',
  'board.sortBy': 'Ordina per {column}',
  'board.filter.hosts': 'Host',
  'board.filter.allHosts': 'Tutti gli host',
  'board.filter.someHosts': '{count, plural, one {# host} other {# host}}',
  'board.filter.state': 'Stato',
  'board.filter.stack': 'Stack',
  'board.filter.anyStack': 'Qualsiasi stack',
  'board.filter.clear': 'Cancella i filtri',
  'board.state.all': 'Qualsiasi stato',
  'board.state.running': 'In esecuzione',
  'board.state.stopped': 'Arrestati',
  'board.state.unhealthy': 'Non integri',
  'board.search': 'Cerca in tutti gli host',
  'board.skipped': '{name} ({status}) non è mostrato.',
  'board.openHost': 'Apri l’host',
  'board.listFailed': 'Impossibile elencare i container di {name}.',
  'board.noHosts.title': 'Nessun host connesso',
  'board.noHosts.description': 'La bacheca elenca i container di tutti gli host connessi.',
  'board.empty.title': 'Ancora nessun container',
  'board.empty.description': 'Nessuno dei tuoi host connessi esegue un container.',
  'board.noMatch': 'Nessun container corrisponde a questi filtri.',
  'board.selection': 'Container selezionati',
  'board.selectAll': 'Seleziona tutti i container mostrati',
  'board.selectOne': 'Seleziona {name} su {host}',
  'board.remove.title': 'Rimuovere {count, plural, one {# container} other {# container}}?',
  'board.remove.description':
    'Vengono eliminati con i loro volumi anonimi; i volumi con nome vengono conservati. Quelli in esecuzione vengono prima arrestati.',
  'board.remove.onHost': 'Su {host}: {names}',
} satisfies Translation<typeof en>;
