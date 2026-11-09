/** Korean text of the `clusters` namespace. */
import type en from '../en/clusters';
import type { Translation } from '../types';

export default {
  'clusters.title': '클러스터',
  'clusters.summary': '{count, plural, other {클러스터 #개}}',
  'clusters.add': '클러스터 추가',
  'clusters.notFound': '이 클러스터는 더 이상 존재하지 않습니다.',
  'clusters.empty.title': '아직 클러스터가 없습니다',
  'clusters.empty.description':
    'kubeconfig로 연결할 수 있는 Kubernetes 클러스터를 추가하세요. Homeport는 클러스터에 아무것도 설치하지 않습니다.',
  'clusters.column.name': '이름',
  'clusters.column.status': '상태',
  'clusters.column.kind': '종류',
  'clusters.column.server': 'API',
  'clusters.column.auth': '로그인 방식',
  'clusters.palette.add': '클러스터 추가',
  'clusters.palette.manage': '클러스터 관리',

  'clusters.auth.none': '자격 증명 없음',
  'clusters.auth.clientCertificate': '클라이언트 인증서',
  'clusters.auth.token': '토큰',
  'clusters.auth.basic': '사용자 이름 및 비밀번호',
  'clusters.auth.exec': 'Exec 플러그인({command})',
  'clusters.auth.oidc': 'OpenID Connect',
  'clusters.auth.aclToken': 'ACL 토큰',

  'clusters.addPage.description':
    '선택한 각 Kubernetes 컨텍스트 또는 지정한 각 Nomad 에이전트가 하나의 클러스터가 됩니다. 자격 증명은 암호화되어 저장되며, 디스크에 평문으로 저장되지 않습니다.',
  'clusters.addPage.fromMachine': '이 컴퓨터의 kubeconfig에서',
  'clusters.addPage.paste': 'kubeconfig 붙여 넣기',
  'clusters.addPage.nomad': 'Nomad',
  'clusters.paste.description':
    'kubeconfig는 메모리에서 읽으며 파일로 저장하지 않습니다. kubeconfig가 참조하는 파일은 전체 경로여야 합니다.',
  'clusters.paste.label': 'Kubeconfig',
  'clusters.paste.read': '컨텍스트 읽기',

  'clusters.import.readFrom': '{paths}에서 읽음',
  'clusters.import.noContexts': '이 kubeconfig에는 컨텍스트가 없습니다',
  'clusters.import.pick': '{context} 가져오기',
  'clusters.import.name': '{context}에서 만들 클러스터의 이름',
  'clusters.import.current': '현재',
  'clusters.import.added': '이미 추가되었습니다(이름: {name}).',
  'clusters.import.refresh': '새로 고치기',
  'clusters.import.submit': '{count, plural, =0 {가져오기} other {클러스터 #개 가져오기}}',
  'clusters.import.done': '{count, plural, other {클러스터 #개}}를 추가했습니다',
  'clusters.import.problem.empty': '모든 클러스터에 이름을 지정하세요.',
  'clusters.import.problem.duplicate': '두 클러스터의 이름이 모두 “{name}” 이름이 됩니다.',
  'clusters.import.problem.taken': '이름이 “{name}”인 클러스터가 이미 있습니다.',

  'clusters.detail.enabled': '사용',
  'clusters.detail.reconnect': '다시 연결',
  'clusters.detail.name': '이름',
  'clusters.detail.rename': '이름 바꾸기',
  'clusters.detail.renamed': '클러스터 이름을 바꿨습니다',
  'clusters.connection.title': '연결',
  'clusters.connection.server': 'API 서버',
  'clusters.connection.address': '주소',
  'clusters.connection.region': '리전',
  'clusters.connection.customCa': '자체 CA 인증서',
  'clusters.connection.clientCertificate': '클라이언트 인증서',
  'clusters.connection.checkedAs': '{name} 이름으로 인증서 확인',
  'clusters.connection.version': '버전',
  'clusters.connection.platform': '플랫폼',
  'clusters.connection.namespace': '기본 Namespace',
  'clusters.connection.auth': '로그인 방식',
  'clusters.connection.tlsServerName': '인증서 이름',
  'clusters.connection.tls': 'TLS',
  'clusters.connection.insecure': '서버 인증서를 확인하지 않음(insecure-skip-tls-verify)',
  'clusters.connection.proxy': '프록시',
  'clusters.connection.source': '가져온 위치',
  'clusters.source.machine': '이 컴퓨터 kubeconfig의 {context} 컨텍스트',
  'clusters.source.pasted': '붙여 넣은 kubeconfig의 {context} 컨텍스트',

  'clusters.nomad.description':
    'Homeport는 이 컴퓨터에서 Nomad 에이전트(서버 또는 클라이언트)의 HTTP API와 통신합니다. 클러스터에는 아무것도 설치하지 않으며, 여기에 nomad CLI도 필요하지 않습니다.',
  'clusters.nomad.name': '이름',
  'clusters.nomad.address': '주소',
  'clusters.nomad.addressHelp':
    'NOMAD_ADDR이 nomad CLI에 전달하는 것과 같은 에이전트의 HTTP API입니다. 포트를 지정하지 않으면 4646을 사용합니다.',
  'clusters.nomad.region': '리전',
  'clusters.nomad.regionHelp': '에이전트 자체의 리전을 사용하려면 비워 두세요.',
  'clusters.nomad.token': 'ACL 토큰',
  'clusters.nomad.tokenHelp':
    '클러스터에 ACL이 사용 설정된 경우 토큰의 시크릿 ID입니다. Homeport 문서에 이 토큰용 정책이 있습니다.',
  'clusters.nomad.tls': 'TLS 설정',
  'clusters.nomad.tlsHelp':
    '자체 CA로 HTTPS를 제공하거나 클라이언트 인증서를 요구하는 에이전트에만 해당합니다. PEM 파일의 내용을 붙여 넣으세요.',
  'clusters.nomad.ca': 'CA 인증서',
  'clusters.nomad.caHelp':
    'NOMAD_CACERT가 가리키는 것처럼, 에이전트의 인증서에 서명한 CA입니다. 이 컴퓨터의 CA는 지정하지 않아도 신뢰합니다.',
  'clusters.nomad.clientCertificate': '클라이언트 인증서',
  'clusters.nomad.clientKey': '클라이언트 키',
  'clusters.nomad.tlsServerName': 'TLS 서버 이름',
  'clusters.nomad.tlsServerNameHelp':
    'NOMAD_TLS_SERVER_NAME처럼, 주소의 이름이 아닌 경우 에이전트의 인증서를 확인할 이름입니다.',
  'clusters.nomad.keep': '저장됨. 유지하려면 비워 두세요',
  'clusters.nomad.willBeRemoved': '저장하면 삭제됩니다',
  'clusters.nomad.remove': '삭제',
  'clusters.nomad.keepIt': '유지',
  'clusters.nomad.test': '연결 테스트',
  'clusters.nomad.testing': '테스트하는 중…',
  'clusters.nomad.testedVersion': 'Nomad {version}, 리전 {region}에 연결했습니다.',
  'clusters.nomad.testedNoVersion':
    '리전 {region}에 연결했습니다. 토큰으로 에이전트를 읽을 수 없어 버전을 알 수 없습니다.',
  'clusters.nomad.add': '클러스터 추가',
  'clusters.nomad.added': '{name} 클러스터를 추가했습니다',
  'clusters.nomad.settings': '설정',
  'clusters.nomad.settingsDescription':
    '저장하면 다시 연결합니다. 토큰과 TLS 키는 다시 표시되지 않습니다. 유지하려면 비워 두세요.',
  'clusters.nomad.save': '변경 사항 저장',
  'clusters.nomad.saved': '클러스터를 저장했습니다',

  'clusters.refresh.title': 'kubeconfig에서 새로 고침',
  'clusters.refresh.machine':
    '클러스터가 다시 만들어졌거나 자격 증명이 갱신된 경우를 위해, 이 컴퓨터의 kubeconfig에서 {context} 컨텍스트를 다시 읽습니다.',
  'clusters.refresh.pasted':
    '클러스터가 다시 만들어졌거나 자격 증명이 갱신된 경우, kubeconfig를 다시 붙여 넣어 {context} 컨텍스트를 새로 읽으세요.',
  'clusters.refresh.submit': '새로 고침',
  'clusters.refresh.done': 'kubeconfig에서 클러스터를 새로 고쳤습니다',

  'clusters.remove.title': '클러스터 삭제',
  'clusters.remove.description':
    'Homeport가 클러스터와 저장된 자격 증명을 잊습니다. 클러스터나 kubeconfig에는 아무것도 변경되지 않습니다.',
  'clusters.remove.confirmTitle': '{name} 클러스터를 삭제하시겠습니까?',
  'clusters.remove.confirmDescription': 'kubeconfig에서 다시 추가할 수 있습니다.',
} satisfies Translation<typeof en>;
