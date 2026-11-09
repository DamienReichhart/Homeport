/** Japanese text of the `apiresources` namespace. */
import type en from '../en/apiresources';
import type { Translation } from '../types';

export default {
  'apiresources.nav': 'API リソース',
  'apiresources.title': 'API リソース',
  'apiresources.summary': 'このクラスターが提供している {count, plural, other {# 種類}}のリソース',
  'apiresources.refresh': '更新',
  'apiresources.types': 'リソースの種類',
  'apiresources.search': '種類名または短縮名（pvc）',
  'apiresources.searchObjects': '検索',
  'apiresources.noMatch': '一致する種類はありません。',
  'apiresources.coreGroup': 'core',
  'apiresources.pick':
    'オブジェクトを一覧表示する種類を選んでください。組み込みの種類と、CRD で追加された種類があります。',
  'apiresources.typeGone': 'このクラスターはその種類を（もう）提供していません。',
  'apiresources.cantList': '一覧表示できません',
  'apiresources.notAllowed': 'この認証情報では一覧表示できません',
  'apiresources.cantListWhy':
    '{kind} オブジェクトは一覧表示できません。その API では、1 つずつ作成するか読み取ることしかできません。',
  'apiresources.empty': 'ここには何もありません。',
  'apiresources.column.name': '名前',
  'apiresources.column.namespace': 'Namespace',
  'apiresources.column.age': '経過時間',
  'apiresources.tab.overview': '概要',
  'apiresources.tab.conditions': '状態',
  'apiresources.tab.events': 'イベント',
  'apiresources.tab.source': 'YAML',
  'apiresources.detail.namespace': 'Namespace',
  'apiresources.detail.owners': '所有者',
  'apiresources.detail.age': '経過時間',
  'apiresources.detail.labels': 'ラベル',
  'apiresources.detail.annotations': 'アノテーション',
  'apiresources.edit': '編集',
  'apiresources.copyYaml': 'YAML をコピー',
} satisfies Translation<typeof en>;
