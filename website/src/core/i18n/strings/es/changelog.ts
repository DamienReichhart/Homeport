import type { Translation } from '../../types';
import type en from '../en/changelog';

/** The changelog page, in Spanish. */
export default {
  'changelog.title': 'Novedades — Homeport',
  'changelog.description':
    'Las novedades de cada versión de Homeport: funciones, correcciones y otros cambios.',
  'changelog.eyebrow': 'Novedades',
  'changelog.heading': 'Novedades de Homeport',
  'changelog.lead':
    'Todas las versiones, de la más reciente a la más antigua. Las notas se generan a partir de los commits del proyecto; los archivos están en la {link}.',
  'changelog.downloadPage': 'página de descarga',
  'changelog.notesInEnglish':
    'Las notas de versión de abajo están en inglés, tal como se publican en GitHub.',
  'changelog.empty': 'El historial de versiones está en {link}.',
  'changelog.github': 'GitHub',
  'changelog.latest': 'Más reciente',
  'changelog.prerelease': 'Versión preliminar',
  'changelog.onGitHub': 'Versión en GitHub',
  'changelog.download': 'Descargar',
} satisfies Translation<typeof en>;
