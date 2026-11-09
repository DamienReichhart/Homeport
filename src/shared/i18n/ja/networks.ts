/** Japanese text of the `networks` namespace. */
import type en from '../en/networks';
import type { Translation } from '../types';

export default {
  'networks.nav': 'ネットワーク',
  'networks.title': 'ネットワーク',
  'networks.summary': '{count, plural, other {# 個のネットワーク}}',
  'networks.cleanUp': '未使用を削除',
  'networks.create': 'ネットワークを作成',
  'networks.search': 'ネットワークを検索',
  'networks.empty': 'ネットワークはありません',

  'networks.column.name': '名前',
  'networks.column.stack': 'スタック',
  'networks.column.driver': 'ドライバー',
  'networks.column.subnet': 'サブネット',
  'networks.column.containers': 'コンテナ',
  'networks.builtIn': '組み込み',
  'networks.internal': '内部',
  'networks.inspectOne': '{name} を検査',
  'networks.removeOne': '{name} を削除',

  'networks.remove.title': 'ネットワーク {name} を削除しますか？',
  'networks.remove.attached':
    'まだ {count, plural, other {# 個のコンテナ}}が接続されています。先に切断してください。',
  'networks.remove.unused':
    'このネットワークを使っているコンテナはありません。名前で参照しているコンテナは起動に失敗します。',
  'networks.prune.title': '未使用のネットワークを削除しますか？',
  'networks.prune.description':
    'コンテナが接続されていないカスタムネットワークはすべて削除されます。',

  'networks.connected': '接続しました',
  'networks.disconnected': '切断しました',
  'networks.removed': '{name} を削除しました',
  'networks.pruned': '{count, plural, other {# 個のネットワーク}}を削除しました',

  'networks.createDialog.title': 'ネットワークを作成',
  'networks.createDialog.name': '名前',
  'networks.createDialog.driver': 'ドライバー',
  'networks.createDialog.subnet': 'サブネット',
  'networks.createDialog.gateway': 'ゲートウェイ',
  'networks.createDialog.automatic': '自動',
  'networks.createDialog.internal': '内部 — 外部ネットワークにはアクセスできません',
  'networks.createDialog.attachable': 'アタッチ可能',

  'networks.dialog.containers': 'コンテナ',
  'networks.dialog.raw': '生の JSON',
  'networks.dialog.noContainer': '接続しているコンテナはありません。',
  'networks.dialog.disconnect': '切断',
  'networks.dialog.containerToConnect': '接続するコンテナ',
  'networks.dialog.connectPlaceholder': 'コンテナを接続…',
  'networks.dialog.connect': '接続',
} satisfies Translation<typeof en>;
