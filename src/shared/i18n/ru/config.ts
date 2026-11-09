/** Russian text of the `config` namespace. */
import type en from '../en/config';
import type { Translation } from '../types';

export default {
  'config.nav': 'Конфигурация',
  'config.title': 'Конфигурация',
  'config.configMaps': 'ConfigMap',
  'config.secrets': 'Secret',
  'config.configMap': 'ConfigMap',
  'config.secret': 'Secret',
  'config.configMapsSummary':
    '{count, plural, one {# ConfigMap} few {# ConfigMap} many {# ConfigMap} other {# ConfigMap}}',
  'config.secretsSummary':
    '{count, plural, one {# Secret} few {# Secret} many {# Secret} other {# Secret}}',
  'config.searchConfigMaps': 'Поиск ConfigMap',
  'config.searchSecrets': 'Поиск Secret',
  'config.column.name': 'Имя',
  'config.column.namespace': 'Пространство имён',
  'config.column.type': 'Тип',
  'config.column.keys': 'Ключи',
  'config.column.age': 'Возраст',
  'config.tab.data': 'Данные',
  'config.tab.overview': 'Обзор',
  'config.tab.events': 'События',
  'config.tab.source': 'YAML',
  'config.detail.namespace': 'Пространство имён',
  'config.detail.type': 'Тип',
  'config.detail.labels': 'Метки',
  'config.detail.annotations': 'Аннотации',
  'config.noKeys': 'Ключей нет.',
  'config.binary': '(двоичные данные)',
  'config.base64': 'base64',
  'config.reveal': 'Показать',
  'config.revealOne': 'Показать {key}',
  'config.hide': 'Скрыть',
  'config.copy': 'Скопировать {key}',
  'config.reveal.failed': 'Homeport не удалось показать {key}',
} satisfies Translation<typeof en>;
