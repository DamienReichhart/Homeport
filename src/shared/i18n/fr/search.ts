/** French text of the `search` namespace. */
import type en from '../en/search';
import type { Translation } from '../types';

export default {
  'search.kind.container': 'Conteneurs',
  'search.kind.stack': 'Stacks',
  'search.kind.image': 'Images',
  'search.kind.volume': 'Volumes',
  'search.kind.network': 'Réseaux',
  'search.onHostState': '{host} · {state, select, running {en cours d’exécution} other {arrêté}}',
  'search.showAll': 'Afficher les {count}',
  'search.skipped':
    '{count, plural, one {# hôte non parcouru} other {# hôtes non parcourus}} (non connecté)',
} satisfies Translation<typeof en>;
