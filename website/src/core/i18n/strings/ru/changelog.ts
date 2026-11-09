import type { Translation } from '../../types';
import type en from '../en/changelog';

/** The changelog page, in Russian. */
export default {
  'changelog.title': 'Список изменений — Homeport',
  'changelog.description':
    'Что нового в каждом выпуске Homeport: возможности, исправления и другие изменения.',
  'changelog.eyebrow': 'Список изменений',
  'changelog.heading': 'Что нового в Homeport',
  'changelog.lead':
    'Все выпуски, сначала новые. Примечания составляются из коммитов проекта; файлы — на {link}.',
  'changelog.downloadPage': 'странице загрузки',
  'changelog.notesInEnglish':
    'Примечания к выпускам ниже — на английском, как они опубликованы на GitHub.',
  'changelog.empty': 'История выпусков — на {link}.',
  'changelog.github': 'GitHub',
  'changelog.latest': 'Последний',
  'changelog.prerelease': 'Предварительный',
  'changelog.onGitHub': 'Выпуск на GitHub',
  'changelog.download': 'Скачать',
} satisfies Translation<typeof en>;
