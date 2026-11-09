/** German text of the `search` namespace. */
import type en from '../en/search';
import type { Translation } from '../types';

export default {
  'search.kind.container': 'Container',
  'search.kind.stack': 'Stacks',
  'search.kind.image': 'Images',
  'search.kind.volume': 'Volumes',
  'search.kind.network': 'Netzwerke',
  'search.onHostState': '{host} · {state, select, running {läuft} other {gestoppt}}',
  'search.showAll': 'Alle {count} anzeigen',
  'search.skipped':
    '{count, plural, one {# Host} other {# Hosts}} nicht durchsucht (nicht verbunden)',
} satisfies Translation<typeof en>;
