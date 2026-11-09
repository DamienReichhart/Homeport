import type { Translation } from '../../types';
import type en from '../en/changelog';

/** The changelog page, in German. */
export default {
  'changelog.title': 'Änderungen – Homeport',
  'changelog.description':
    'Was in jedem Release von Homeport neu ist: Funktionen, Fehlerbehebungen und andere Änderungen.',
  'changelog.eyebrow': 'Änderungen',
  'changelog.heading': 'Neu in Homeport',
  'changelog.lead':
    'Jedes Release, das neueste zuerst. Die Notizen werden aus den Commits des Projekts erzeugt; die Dateien stehen auf der {link}.',
  'changelog.downloadPage': 'Download-Seite',
  'changelog.notesInEnglish':
    'Die Release-Notizen unten sind auf Englisch, so wie sie auf GitHub veröffentlicht wurden.',
  'changelog.empty': 'Die Release-Historie steht auf {link}.',
  'changelog.github': 'GitHub',
  'changelog.latest': 'Neueste',
  'changelog.prerelease': 'Vorabversion',
  'changelog.onGitHub': 'Release auf GitHub',
  'changelog.download': 'Download',
} satisfies Translation<typeof en>;
