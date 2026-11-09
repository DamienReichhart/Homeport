/** Korean text of the `clusterevents` namespace. */
import type en from '../en/clusterevents';
import type { Translation } from '../types';

export default {
  'clusterevents.nav': '이벤트',
  'clusterevents.title': '이벤트',
  'clusterevents.summary': '{count, plural, other {이벤트 #개}}',
  'clusterevents.search': '사유, 객체 또는 메시지로 검색',
  'clusterevents.empty': '이벤트가 없습니다. Kubernetes는 이벤트를 약 1시간 동안 보관합니다.',
  'clusterevents.filter': '유형',
  'clusterevents.all': '전체',
  'clusterevents.warnings': '경고',
  'clusterevents.normal': '정상',
  'clusterevents.column.type': '유형',
  'clusterevents.column.reason': '사유',
  'clusterevents.column.object': '객체',
  'clusterevents.column.namespace': 'Namespace',
  'clusterevents.column.message': '메시지',
  'clusterevents.column.count': '횟수',
  'clusterevents.column.lastSeen': '마지막 발생',
} satisfies Translation<typeof en>;
