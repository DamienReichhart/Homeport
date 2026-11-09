/** Japanese text of the `board` namespace. */
import type en from '../en/board';
import type { Translation } from '../types';

export default {
  'board.nav': 'ボード',
  'board.title': 'ボード',
  'board.summary':
    '{hosts, plural, other {# 台のホスト}}で {count, plural, other {# 個のコンテナ}}中 {running} 個が実行中',
  'board.column.host': 'ホスト',
  'board.column.stack': 'スタック',
  'board.sortBy': '{column} で並べ替え',
  'board.filter.hosts': 'ホスト',
  'board.filter.allHosts': 'すべてのホスト',
  'board.filter.someHosts': '{count, plural, other {# 台のホスト}}',
  'board.filter.state': '状態',
  'board.filter.stack': 'スタック',
  'board.filter.anyStack': 'すべてのスタック',
  'board.filter.clear': 'フィルターをクリア',
  'board.state.all': 'すべての状態',
  'board.state.running': '実行中',
  'board.state.stopped': '停止',
  'board.state.unhealthy': '異常',
  'board.search': 'すべてのホストを検索',
  'board.skipped': '{name}（{status}）は表示されていません。',
  'board.openHost': 'ホストを開く',
  'board.listFailed': '{name} のコンテナを一覧表示できませんでした。',
  'board.noHosts.title': '接続しているホストがありません',
  'board.noHosts.description': 'ボードには、接続しているすべてのホストのコンテナが表示されます。',
  'board.empty.title': 'まだコンテナはありません',
  'board.empty.description': '接続しているどのホストでもコンテナは動作していません。',
  'board.noMatch': 'これらのフィルターに一致するコンテナはありません。',
  'board.selection': '選択したコンテナ',
  'board.selectAll': '表示されているすべてのコンテナを選択',
  'board.selectOne': '{host} の {name} を選択',
  'board.remove.title': '{count, plural, other {# 個のコンテナ}}を削除しますか？',
  'board.remove.description':
    '匿名ボリュームと一緒に削除されます。名前付きボリュームは残ります。実行中のものは先に停止されます。',
  'board.remove.onHost': '{host}: {names}',
} satisfies Translation<typeof en>;
