import type { Translation } from '../../types';
import type en from '../en/changelog';

/** The changelog page, in French. */
export default {
  'changelog.title': 'Nouveautés — Homeport',
  'changelog.description':
    'Les nouveautés de chaque version de Homeport : fonctionnalités, corrections et autres changements.',
  'changelog.eyebrow': 'Nouveautés',
  'changelog.heading': 'Les nouveautés de Homeport',
  'changelog.lead':
    'Chaque version, de la plus récente à la plus ancienne. Les notes sont générées à partir des commits du projet ; les fichiers se trouvent sur la {link}.',
  'changelog.downloadPage': 'page de téléchargement',
  'changelog.notesInEnglish':
    'Les notes de version ci-dessous sont en anglais, telles que publiées sur GitHub.',
  'changelog.empty': 'L’historique des versions se trouve sur {link}.',
  'changelog.github': 'GitHub',
  'changelog.latest': 'Dernière version',
  'changelog.prerelease': 'Préversion',
  'changelog.onGitHub': 'Version sur GitHub',
  'changelog.download': 'Télécharger',
} satisfies Translation<typeof en>;
