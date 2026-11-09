/** Simplified Chinese text of the `search` namespace. */
import type en from '../en/search';
import type { Translation } from '../types';

export default {
  'search.kind.container': '容器',
  'search.kind.stack': '堆栈',
  'search.kind.image': '镜像',
  'search.kind.volume': '卷',
  'search.kind.network': '网络',
  'search.onHostState': '{host} · {state, select, running {运行中} other {已停止}}',
  'search.showAll': '显示全部 {count} 个',
  'search.skipped': '{count, plural, other {# 个主机}}未搜索（未连接）',
} satisfies Translation<typeof en>;
