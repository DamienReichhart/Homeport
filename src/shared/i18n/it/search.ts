/** Italian text of the `search` namespace. */
import type en from '../en/search';
import type { Translation } from '../types';

export default {
  'search.kind.container': 'Container',
  'search.kind.stack': 'Stack',
  'search.kind.image': 'Immagini',
  'search.kind.volume': 'Volumi',
  'search.kind.network': 'Reti',
  'search.onHostState': '{host} · {state, select, running {in esecuzione} other {arrestato}}',
  'search.showAll': 'Mostra tutti ({count})',
  'search.skipped':
    '{count, plural, one {# host non incluso} other {# host non inclusi}} nella ricerca (non connessi)',
} satisfies Translation<typeof en>;
