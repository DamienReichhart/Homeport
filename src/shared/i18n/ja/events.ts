/** Japanese text of the `events` namespace. */
import type en from '../en/events';
import type { Translation } from '../types';

export default {
  'events.nav': 'イベント',
  'events.title': 'イベント',
  'events.description': 'エンジンが報告した内容です。再起動後も保持されます。',
  'events.descriptionKept':
    'エンジンが報告した内容です。{days, plural, other {# 日間}}、最大 10,000 件まで保持されます。',
  'events.clear': '履歴を消去',
  'events.clear.title': 'イベント履歴を消去しますか？',
  'events.clear.description':
    'このホストについて保持しているイベントはすべて破棄されます。新しいイベントは引き続き表示されます。エンジン自体のイベントには影響しません。',
  'events.clear.confirm': '履歴を消去',
  'events.cleared': 'イベント履歴を消去しました',
  'events.loadOlder': '古いイベントを読み込む',
  'events.search': 'イベントを検索',
  'events.type': '種類',
  'events.allTypes': 'すべての種類',
  'events.empty.title': 'まだイベントはありません',
  'events.empty.description': 'コンテナを起動または停止すると、ここに表示されます。',
  'events.column.time': '時刻',
  'events.column.type': '種類',
  'events.column.action': '操作',
  'events.column.object': '対象',
  'events.column.stack': 'スタック',
} satisfies Translation<typeof en>;
