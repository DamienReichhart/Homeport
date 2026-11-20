import type { Translation } from '../../types';
import type en from '../en/site';

/** The layout, the header and the footer, in Russian. */
export default {
  // The layout and the facts every page repeats.
  'site.tagline': 'Одно приложение для любого движка контейнеров.',
  'site.description':
    'Homeport — бесплатное настольное приложение с открытым исходным кодом для Docker, Podman, containerd, Incus, Kubernetes и Nomad. Оно работает на вашем компьютере и подключается к серверам по SSH, TLS или через ваш kubeconfig, ничего на них не устанавливая. Для Windows, macOS и Linux.',
  'site.ogImageAlt': 'Homeport — одно приложение для любого движка контейнеров.',
  'site.skipToContent': 'Перейти к содержимому',

  // The header, the language menu and the theme switch.
  'header.home': 'Главная страница Homeport',
  'header.logo': 'Homeport',
  'header.nav': 'Основное',
  'header.engines': 'Движки',
  'header.docs': 'Документация',
  'header.changelog': 'Изменения',
  'header.download': 'Скачать',
  'header.github': 'Homeport на GitHub',
  'header.githubShort': 'GitHub',
  'header.menu': 'Меню',
  'language.label': 'Язык',
  'theme.toLight': 'Включить светлую тему',
  'theme.toDark': 'Включить тёмную тему',

  // The footer.
  'footer.product': 'Продукт',
  'footer.engines': 'Движки',
  'footer.download': 'Скачать',
  'footer.changelog': 'Список изменений',
  'footer.releases': 'Все выпуски',
  'footer.documentation': 'Документация',
  'footer.introduction': 'Введение',
  'footer.installation': 'Установка',
  'footer.hosts': 'Подключение хостов',
  'footer.troubleshooting': 'Устранение неполадок',
  'footer.project': 'Проект',
  'footer.source': 'Исходный код',
  'footer.issue': 'Сообщить о проблеме',
  'footer.contributing': 'Участие в разработке',
  'footer.security': 'Безопасность',
  'footer.legal': 'Правовая информация',
  'footer.legalNotice': 'Правовое уведомление',
  'footer.privacy': 'Конфиденциальность',
  'footer.terms': 'Условия использования',
  'footer.cookies': 'Файлы cookie',
  'footer.licence':
    '© {year} Damien Reichhart. Homeport распространяется с открытым исходным кодом по лицензии {licence}.',
  'footer.licenceName': 'GNU AGPL версии 3',
  'footer.trademark':
    'Docker — товарный знак Docker, Inc. Homeport — независимый проект, не связанный с Docker, Inc. и не одобренный ею.',
} satisfies Translation<typeof en>;
