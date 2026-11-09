/** Korean text of the `search` namespace. */
import type en from '../en/search';
import type { Translation } from '../types';

export default {
  'search.kind.container': '컨테이너',
  'search.kind.stack': '스택',
  'search.kind.image': '이미지',
  'search.kind.volume': '볼륨',
  'search.kind.network': '네트워크',
  'search.onHostState': '{host} · {state, select, running {실행 중} other {중지됨}}',
  'search.showAll': '{count}개 모두 보기',
  'search.skipped': '{count, plural, other {호스트 #개}}는 검색하지 않음(연결되지 않음)',
} satisfies Translation<typeof en>;
