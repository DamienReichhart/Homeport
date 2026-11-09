/** Russian text of the `search` namespace. */
import type en from '../en/search';
import type { Translation } from '../types';

export default {
  'search.kind.container': 'Контейнеры',
  'search.kind.stack': 'Стеки',
  'search.kind.image': 'Образы',
  'search.kind.volume': 'Тома',
  'search.kind.network': 'Сети',
  'search.onHostState': '{host} · {state, select, running {работает} other {остановлен}}',
  'search.showAll': 'Показать все ({count})',
  'search.skipped':
    'Поиск не выполнен на {count, plural, one {# хосте} few {# хостах} many {# хостах} other {# хоста}} (нет подключения)',
} satisfies Translation<typeof en>;
