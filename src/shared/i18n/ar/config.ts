/** Arabic text of the `config` namespace. */
import type en from '../en/config';
import type { Translation } from '../types';

export default {
  'config.nav': 'التكوين',
  'config.title': 'التكوين',
  'config.configMaps': 'ConfigMaps',
  'config.secrets': 'Secrets',
  'config.configMap': 'ConfigMap',
  'config.secret': 'Secret',
  'config.configMapsSummary':
    '{count, plural, zero {لا ConfigMaps} one {ConfigMap واحد} two {# ConfigMaps} few {# ConfigMaps} many {# ConfigMap} other {# ConfigMap}}',
  'config.secretsSummary':
    '{count, plural, zero {لا Secrets} one {Secret واحد} two {# Secrets} few {# Secrets} many {# Secret} other {# Secret}}',
  'config.searchConfigMaps': 'البحث في ConfigMaps',
  'config.searchSecrets': 'البحث في Secrets',
  'config.column.name': 'الاسم',
  'config.column.namespace': 'Namespace',
  'config.column.type': 'النوع',
  'config.column.keys': 'المفاتيح',
  'config.column.age': 'العمر',
  'config.tab.data': 'البيانات',
  'config.tab.overview': 'نظرة عامة',
  'config.tab.events': 'الأحداث',
  'config.tab.source': 'YAML',
  'config.detail.namespace': 'Namespace',
  'config.detail.type': 'النوع',
  'config.detail.labels': 'التسميات',
  'config.detail.annotations': 'التعليقات التوضيحية',
  'config.noKeys': 'لا توجد مفاتيح.',
  'config.binary': '(بيانات ثنائية)',
  'config.base64': 'base64',
  'config.reveal': 'إظهار',
  'config.revealOne': 'إظهار {key}',
  'config.hide': 'إخفاء',
  'config.copy': 'نسخ {key}',
  'config.reveal.failed': 'تعذّر على Homeport إظهار {key}',
} satisfies Translation<typeof en>;
