/** Russian text of the `app` namespace. */
import type en from '../en/app';
import type { Translation } from '../types';

export default {
  'app.nav.main': 'Основное',
  'app.keychainMissing':
    'Связка ключей ОС недоступна, поэтому SSH-ключи, TLS-сертификаты и окружения стеков сохранить нельзя. Локальные хосты, хосты TCP и ssh-agent по-прежнему работают.',
  'app.crash.title': 'Что-то пошло не так',
  'app.crash.description':
    'На этой странице в Homeport возникла непредвиденная ошибка. Обычно помогает перезагрузка; если ошибка повторяется, скопируйте отчёт из раздела «Подробности» в issue.',
  'app.crash.reload': 'Перезагрузить',
  'app.crash.backToOverview': 'Вернуться к обзору',
  'app.shortcuts.title': 'Сочетания клавиш',
  'app.shortcuts.description':
    'Сочетания, которые работают здесь. Клавиши вроде N и / не срабатывают, пока вы печатаете в поле.',
  'app.shortcuts.general': 'Общие',
  'app.shortcuts.navigation': 'Переход',
  'app.shortcuts.hosts': 'Хосты',
  'app.shortcuts.help': 'Показать сочетания клавиш',
  'app.shortcuts.settings': 'Открыть настройки',
  'app.shortcuts.goTo': 'Перейти: {page}',
  'app.shortcuts.previousHost': 'Предыдущий хост',
  'app.shortcuts.nextHost': 'Следующий хост',
  'app.shortcuts.refresh': 'Обновить списки этого хоста',
  'app.palette.title': 'Палитра команд',
  'app.palette.description': 'Перейти на страницу, сменить хост или выполнить команду.',
  'app.palette.open': 'Открыть палитру команд',
  'app.palette.placeholder': 'Введите страницу, хост или команду…',
  'app.palette.empty': 'Ничего не найдено.',
  'app.palette.recent': 'Недавние',
  'app.palette.goTo': 'Перейти',
  'app.palette.hosts': 'Хосты',
  'app.palette.actions': 'Действия',
  'app.palette.switchTo': 'Переключиться на {name}',
} satisfies Translation<typeof en>;
