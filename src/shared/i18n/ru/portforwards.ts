/** Russian text of the `portforwards` namespace. */
import type en from '../en/portforwards';
import type { Translation } from '../types';

export default {
  'portforwards.nav': 'Пробросы портов',
  'portforwards.title': 'Пробросы портов',
  'portforwards.summary':
    '{count, plural, one {# проброс} few {# проброса} many {# пробросов} other {# проброса}}',
  'portforwards.empty.title': 'Нет пробросов портов',
  'portforwards.empty.description':
    'Пробросьте порт со страницы Pod или Service, чтобы обращаться к нему с этого компьютера. Пробросы завершаются, когда Homeport закрывается.',
  'portforwards.column.local': 'Локальный адрес',
  'portforwards.column.target': 'Куда',
  'portforwards.column.namespace': 'Пространство имён',
  'portforwards.column.cluster': 'Кластер',
  'portforwards.column.connections': 'Соединения',
  'portforwards.forward': 'Пробросить…',
  'portforwards.forwardPort': 'Пробросить порт {port}',
  'portforwards.open': 'Открыть',
  'portforwards.stop': 'Остановить',
  'portforwards.stopFailed': 'Homeport не удалось остановить проброс',
  'portforwards.started': '{address} проброшен на {target}:{port}',
  'portforwards.failed': 'Проброс порта завершился ошибкой',
  'portforwards.ended': 'Проброс порта завершён',
  'portforwards.dialog.title': 'Пробросить {target}:{port}',
  'portforwards.dialog.description':
    'В пространстве имён {namespace}. Обращайтесь к нему через локальный порт, пока работает проброс.',
  'portforwards.dialog.localPort': 'Локальный порт',
  'portforwards.dialog.anyPort': 'Любой свободный порт',
  'portforwards.dialog.loopback':
    'Подключиться может только этот компьютер: проброс слушает 127.0.0.1 (и ::1).',
  'portforwards.dialog.submit': 'Пробросить',
} satisfies Translation<typeof en>;
