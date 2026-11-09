/** Brazilian Portuguese text of the `search` namespace. */
import type en from '../en/search';
import type { Translation } from '../types';

export default {
  'search.kind.container': 'Contêineres',
  'search.kind.stack': 'Stacks',
  'search.kind.image': 'Imagens',
  'search.kind.volume': 'Volumes',
  'search.kind.network': 'Redes',
  'search.onHostState': '{host} · {state, select, running {em execução} other {parado}}',
  'search.showAll': 'Mostrar todos os {count}',
  'search.skipped':
    '{count, plural, one {# host não pesquisado} other {# hosts não pesquisados}} (não conectado)',
} satisfies Translation<typeof en>;
