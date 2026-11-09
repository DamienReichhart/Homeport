/** Spanish text of the `search` namespace. */
import type en from '../en/search';
import type { Translation } from '../types';

export default {
  'search.kind.container': 'Contenedores',
  'search.kind.stack': 'Stacks',
  'search.kind.image': 'Imágenes',
  'search.kind.volume': 'Volúmenes',
  'search.kind.network': 'Redes',
  'search.onHostState': '{host} · {state, select, running {en ejecución} other {detenido}}',
  'search.showAll': 'Mostrar los {count}',
  'search.skipped':
    '{count, plural, one {# host sin buscar (no conectado)} other {# hosts sin buscar (no conectados)}}',
} satisfies Translation<typeof en>;
