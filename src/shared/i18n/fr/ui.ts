/** French text of the `ui` namespace. */
import type en from '../en/ui';
import type { Translation } from '../types';

export default {
  'ui.copy': 'Copier',
  'ui.copied': 'Copié',
  'ui.search': 'Rechercher',
  'ui.shortcuts.focusSearch': 'Rechercher dans cette page',
  'ui.shortcuts.thisPage': 'Cette page',
  'ui.table.selectAll': 'Tout sélectionner',
  'ui.table.selectRow': 'Sélectionner la ligne',
  'ui.table.empty': 'Rien ici.',
  'ui.table.page': 'Page {page} sur {pages} · {rows, plural, one {# ligne} other {# lignes}}',
  'ui.table.previous': 'Page précédente',
  'ui.table.next': 'Page suivante',
  'ui.transfer.done': 'Terminé.',
  'ui.files.column.name': 'Nom',
  'ui.files.column.size': 'Taille',
  'ui.files.column.modified': 'Modifié',
  'ui.files.column.mode': 'Mode',
  'ui.files.path': 'Chemin',
  'ui.files.downloadOne': 'Télécharger {name}',
  'ui.files.empty': 'Ce dossier est vide.',
  'ui.chart.time': 'Heure',
  'ui.errorDetails.title': 'Détails de l’erreur',
  'ui.errorDetails.code': 'Code',
  'ui.errorDetails.command': 'Commande',
  'ui.errorDetails.when': 'Quand',
  'ui.errorDetails.app': 'Application',
  'ui.errorDetails.rawDetail': 'Détail brut',
  'ui.errorDetails.rawDetailNote':
    'Tel que le système, le moteur ou la bibliothèque l’a écrit ; il n’est donc pas traduit.',
  'ui.errorDetails.openLogFolder': 'Ouvrir le dossier des journaux',
  'ui.errorDetails.copyReport': 'Copier le rapport',
} satisfies Translation<typeof en>;
