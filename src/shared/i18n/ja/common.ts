/** Japanese text of the `common` namespace. */
import type en from '../en/common';
import type { Translation } from '../types';

export default {
  'common.cancel': 'キャンセル',
  'common.close': '閉じる',
  'common.retry': '再試行',
  'common.details': '詳細',
  'common.remove': '削除',
  'common.save': '保存',
  'common.create': '作成',
  'common.refresh': '更新',
  'common.loading': '読み込み中…',
  'common.moreActions': 'その他の操作',
  'common.back': '戻る',
  'common.none': 'なし。',
  'common.optional': '任意',
} satisfies Translation<typeof en>;
