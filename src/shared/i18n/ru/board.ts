/** Russian text of the `board` namespace. */
import type en from '../en/board';
import type { Translation } from '../types';

export default {
  'board.nav': 'Доска',
  'board.title': 'Доска',
  'board.summary':
    'Работает {running} из {count, plural, one {# контейнера} few {# контейнеров} many {# контейнеров} other {# контейнера}} на {hosts, plural, one {# хосте} few {# хостах} many {# хостах} other {# хоста}}',
  'board.column.host': 'Хост',
  'board.column.stack': 'Стек',
  'board.sortBy': 'Сортировать по столбцу «{column}»',
  'board.filter.hosts': 'Хосты',
  'board.filter.allHosts': 'Все хосты',
  'board.filter.someHosts':
    '{count, plural, one {# хост} few {# хоста} many {# хостов} other {# хоста}}',
  'board.filter.state': 'Состояние',
  'board.filter.stack': 'Стек',
  'board.filter.anyStack': 'Любой стек',
  'board.filter.clear': 'Сбросить фильтры',
  'board.state.all': 'Любое состояние',
  'board.state.running': 'Работает',
  'board.state.stopped': 'Остановлен',
  'board.state.unhealthy': 'Нездоров',
  'board.search': 'Поиск на всех хостах',
  'board.skipped': '{name} ({status}) не показан.',
  'board.openHost': 'Открыть хост',
  'board.listFailed': 'Не удалось получить список контейнеров {name}.',
  'board.noHosts.title': 'Нет подключённых хостов',
  'board.noHosts.description': 'На доске перечислены контейнеры всех подключённых хостов.',
  'board.empty.title': 'Контейнеров пока нет',
  'board.empty.description': 'Ни на одном из подключённых хостов не работает ни один контейнер.',
  'board.noMatch': 'Нет контейнеров, подходящих под эти фильтры.',
  'board.selection': 'Выбранные контейнеры',
  'board.selectAll': 'Выбрать все показанные контейнеры',
  'board.selectOne': 'Выбрать {name} на {host}',
  'board.remove.title':
    'Удалить {count, plural, one {# контейнер} few {# контейнера} many {# контейнеров} other {# контейнера}}?',
  'board.remove.description':
    'Они удаляются вместе со своими анонимными томами; именованные тома сохраняются. Работающие сначала останавливаются.',
  'board.remove.onHost': 'На {host}: {names}',
} satisfies Translation<typeof en>;
