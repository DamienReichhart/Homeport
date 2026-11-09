/** Korean text of the `networks` namespace. */
import type en from '../en/networks';
import type { Translation } from '../types';

export default {
  'networks.nav': '네트워크',
  'networks.title': '네트워크',
  'networks.summary': '{count, plural, other {네트워크 #개}}',
  'networks.cleanUp': '정리',
  'networks.create': '네트워크 만들기',
  'networks.search': '네트워크 검색',
  'networks.empty': '네트워크 없음',

  'networks.column.name': '이름',
  'networks.column.stack': '스택',
  'networks.column.driver': '드라이버',
  'networks.column.subnet': '서브넷',
  'networks.column.containers': '컨테이너',
  'networks.builtIn': '기본 제공',
  'networks.internal': '내부',
  'networks.inspectOne': '{name} 검사',
  'networks.removeOne': '{name} 삭제',

  'networks.remove.title': '{name} 네트워크를 삭제하시겠습니까?',
  'networks.remove.attached':
    '아직 {count, plural, other {연결된 컨테이너 #개}}가 있습니다. 먼저 연결을 해제하세요.',
  'networks.remove.unused':
    '이 네트워크를 사용하는 컨테이너가 없습니다. 이름으로 이 네트워크를 참조하는 컨테이너는 시작에 실패합니다.',
  'networks.prune.title': '사용하지 않는 네트워크를 삭제하시겠습니까?',
  'networks.prune.description': '연결된 컨테이너가 없는 모든 사용자 지정 네트워크가 삭제됩니다.',

  'networks.connected': '연결했습니다',
  'networks.disconnected': '연결을 해제했습니다',
  'networks.removed': '{name} 네트워크를 삭제했습니다',
  'networks.pruned': '{count, plural, other {네트워크 #개}}를 삭제했습니다',

  'networks.createDialog.title': '네트워크 만들기',
  'networks.createDialog.name': '이름',
  'networks.createDialog.driver': '드라이버',
  'networks.createDialog.subnet': '서브넷',
  'networks.createDialog.gateway': '게이트웨이',
  'networks.createDialog.automatic': '자동',
  'networks.createDialog.internal': '내부 — 외부 네트워크에 액세스할 수 없음',
  'networks.createDialog.attachable': '연결 가능',

  'networks.dialog.containers': '컨테이너',
  'networks.dialog.raw': '원본 JSON',
  'networks.dialog.noContainer': '연결된 컨테이너가 없습니다.',
  'networks.dialog.disconnect': '연결 해제',
  'networks.dialog.containerToConnect': '연결할 컨테이너',
  'networks.dialog.connectPlaceholder': '컨테이너 연결…',
  'networks.dialog.connect': '연결',
} satisfies Translation<typeof en>;
