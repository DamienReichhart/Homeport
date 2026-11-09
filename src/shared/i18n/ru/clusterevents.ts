/** Russian text of the `clusterevents` namespace. */
import type en from '../en/clusterevents';
import type { Translation } from '../types';

export default {
  'clusterevents.nav': 'События',
  'clusterevents.title': 'События',
  'clusterevents.summary':
    '{count, plural, one {# событие} few {# события} many {# событий} other {# события}}',
  'clusterevents.search': 'Поиск по причине, объекту или сообщению',
  'clusterevents.empty': 'Событий нет. Kubernetes хранит их около часа.',
  'clusterevents.filter': 'Тип',
  'clusterevents.all': 'Все',
  'clusterevents.warnings': 'Предупреждения',
  'clusterevents.normal': 'Обычные',
  'clusterevents.column.type': 'Тип',
  'clusterevents.column.reason': 'Причина',
  'clusterevents.column.object': 'Объект',
  'clusterevents.column.namespace': 'Пространство имён',
  'clusterevents.column.message': 'Сообщение',
  'clusterevents.column.count': 'Количество',
  'clusterevents.column.lastSeen': 'Последнее',
} satisfies Translation<typeof en>;
