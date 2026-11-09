/** Korean text of the `events` namespace. */
import type en from '../en/events';
import type { Translation } from '../types';

export default {
  'events.nav': '이벤트',
  'events.title': '이벤트',
  'events.description': '엔진이 보고한 내용으로, 다시 시작해도 유지됩니다.',
  'events.descriptionKept':
    '엔진이 보고한 내용으로, 최대 10,000개까지 {days, plural, other {#일}} 동안 보관됩니다.',
  'events.clear': '기록 지우기',
  'events.clear.title': '이벤트 기록을 지우시겠습니까?',
  'events.clear.description':
    '이 호스트에 보관된 모든 이벤트가 삭제됩니다. 새 이벤트는 계속 표시되며, 엔진 자체의 이벤트에는 영향이 없습니다.',
  'events.clear.confirm': '기록 지우기',
  'events.cleared': '이벤트 기록을 지웠습니다',
  'events.loadOlder': '이전 이벤트 불러오기',
  'events.search': '이벤트 검색',
  'events.type': '유형',
  'events.allTypes': '모든 유형',
  'events.empty.title': '아직 이벤트가 없습니다',
  'events.empty.description': '컨테이너를 시작하거나 중지하면 여기에 표시됩니다.',
  'events.column.time': '시각',
  'events.column.type': '유형',
  'events.column.action': '동작',
  'events.column.object': '객체',
  'events.column.stack': '스택',
} satisfies Translation<typeof en>;
