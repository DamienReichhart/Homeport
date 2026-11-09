/** Japanese text of the `search` namespace. */
import type en from '../en/search';
import type { Translation } from '../types';

export default {
  'search.kind.container': 'コンテナ',
  'search.kind.stack': 'スタック',
  'search.kind.image': 'イメージ',
  'search.kind.volume': 'ボリューム',
  'search.kind.network': 'ネットワーク',
  'search.onHostState': '{host} · {state, select, running {実行中} other {停止}}',
  'search.showAll': '{count} 件をすべて表示',
  'search.skipped': '{count, plural, other {# 台のホスト}}は検索されていません（未接続）',
} satisfies Translation<typeof en>;
