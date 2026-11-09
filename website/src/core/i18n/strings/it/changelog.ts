import type { Translation } from '../../types';
import type en from '../en/changelog';

/** The changelog page, in Italian. */
export default {
  'changelog.title': 'Novità — Homeport',
  'changelog.description':
    'Le novità di ogni release di Homeport: funzionalità, correzioni e altre modifiche.',
  'changelog.eyebrow': 'Novità',
  'changelog.heading': 'Le novità di Homeport',
  'changelog.lead':
    'Ogni release, dalla più recente. Le note sono generate dai commit del progetto; i file sono nella {link}.',
  'changelog.downloadPage': 'pagina di download',
  'changelog.notesInEnglish':
    'Le note di rilascio qui sotto sono in inglese, come pubblicate su GitHub.',
  'changelog.empty': 'La cronologia delle release è su {link}.',
  'changelog.github': 'GitHub',
  'changelog.latest': 'Più recente',
  'changelog.prerelease': 'Versione preliminare',
  'changelog.onGitHub': 'Release su GitHub',
  'changelog.download': 'Scarica',
} satisfies Translation<typeof en>;
