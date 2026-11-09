/** Russian text of the `apiresources` namespace. */
import type en from '../en/apiresources';
import type { Translation } from '../types';

export default {
  'apiresources.nav': 'Ресурсы API',
  'apiresources.title': 'Ресурсы API',
  'apiresources.summary':
    'Этот кластер обслуживает {count, plural, one {# тип} few {# типа} many {# типов} other {# типа}}',
  'apiresources.refresh': 'Обновить',
  'apiresources.types': 'Типы ресурсов',
  'apiresources.search': 'Kind или краткое имя (pvc)',
  'apiresources.searchObjects': 'Поиск',
  'apiresources.noMatch': 'Подходящих типов нет.',
  'apiresources.coreGroup': 'core',
  'apiresources.pick':
    'Выберите тип, чтобы увидеть его объекты: встроенные и добавленные вашими CRD.',
  'apiresources.typeGone': 'Этот кластер не обслуживает (больше) этот тип.',
  'apiresources.cantList': 'Список недоступен',
  'apiresources.notAllowed': 'Ваши учётные данные не позволяют получить список',
  'apiresources.cantListWhy':
    'Список объектов {kind} получить нельзя: их API позволяет только создавать или читать их по одному.',
  'apiresources.empty': 'Здесь ничего нет.',
  'apiresources.column.name': 'Имя',
  'apiresources.column.namespace': 'Пространство имён',
  'apiresources.column.age': 'Возраст',
  'apiresources.tab.overview': 'Обзор',
  'apiresources.tab.conditions': 'Условия',
  'apiresources.tab.events': 'События',
  'apiresources.tab.source': 'YAML',
  'apiresources.detail.namespace': 'Пространство имён',
  'apiresources.detail.owners': 'Владельцы',
  'apiresources.detail.age': 'Возраст',
  'apiresources.detail.labels': 'Метки',
  'apiresources.detail.annotations': 'Аннотации',
  'apiresources.edit': 'Изменить',
  'apiresources.copyYaml': 'Скопировать YAML',
} satisfies Translation<typeof en>;
