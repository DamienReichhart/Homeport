/** French text of the `board` namespace. */
import type en from '../en/board';
import type { Translation } from '../types';

export default {
  'board.nav': 'Tableau',
  'board.title': 'Tableau',
  'board.summary':
    '{running} sur {count, plural, one {# conteneur} other {# conteneurs}} en cours d’exécution sur {hosts, plural, one {# hôte} other {# hôtes}}',
  'board.column.host': 'Hôte',
  'board.column.stack': 'Stack',
  'board.sortBy': 'Trier par {column}',
  'board.filter.hosts': 'Hôtes',
  'board.filter.allHosts': 'Tous les hôtes',
  'board.filter.someHosts': '{count, plural, one {# hôte} other {# hôtes}}',
  'board.filter.state': 'État',
  'board.filter.stack': 'Stack',
  'board.filter.anyStack': 'Toutes les stacks',
  'board.filter.clear': 'Effacer les filtres',
  'board.state.all': 'Tous les états',
  'board.state.running': 'En cours d’exécution',
  'board.state.stopped': 'Arrêté',
  'board.state.unhealthy': 'En mauvaise santé',
  'board.search': 'Rechercher sur tous les hôtes',
  'board.skipped': '{name} ({status}) n’est pas affiché.',
  'board.openHost': 'Ouvrir l’hôte',
  'board.listFailed': 'Impossible de lister les conteneurs de {name}.',
  'board.noHosts.title': 'Aucun hôte connecté',
  'board.noHosts.description': 'Le tableau liste les conteneurs de tous les hôtes connectés.',
  'board.empty.title': 'Aucun conteneur pour l’instant',
  'board.empty.description': 'Aucun de vos hôtes connectés n’exécute de conteneur.',
  'board.noMatch': 'Aucun conteneur ne correspond à ces filtres.',
  'board.selection': 'Conteneurs sélectionnés',
  'board.selectAll': 'Sélectionner tous les conteneurs affichés',
  'board.selectOne': 'Sélectionner {name} sur {host}',
  'board.remove.title': 'Supprimer {count, plural, one {# conteneur} other {# conteneurs}} ?',
  'board.remove.description':
    'Ils sont supprimés avec leurs volumes anonymes ; les volumes nommés sont conservés. Ceux en cours d’exécution sont d’abord arrêtés.',
  'board.remove.onHost': 'Sur {host} : {names}',
} satisfies Translation<typeof en>;
