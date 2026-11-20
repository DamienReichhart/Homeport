/** The home page, in Korean. */
export default {
  // Home: the hero and its map.
  'home.title': 'Homeport — 모든 컨테이너 엔진을 하나의 앱에서',
  'home.hero.title': '모든 컨테이너 엔진을 하나의 앱에서.',
  'home.hero.lead':
    'Homeport는 사용자의 컴퓨터에서 실행되며 서버에는 아무것도 설치하지 않습니다. 무료 오픈 소스입니다.',
  'home.hero.docs': '문서 읽기',
  'home.map.label':
    '노선도로 그린 예시 서버 구성입니다. 이 컴퓨터는 다섯 노선의 종점입니다. 로컬 소켓은 laptop에, SSH는 prod-eu-1, nas, edge-1에, TLS는 staging, vps-fra-1에, TCP는 lan-box에, 클러스터 API는 k8s-prod, jobs, lab에 닿습니다. vps-fra-1의 스택 하나가 확인이 필요합니다.',
  'home.map.thisMachine': '이 컴퓨터',
  'home.map.needsYou': '확인이 필요한 스택 1개',
  'home.map.keyNeedsYou': '확인 필요',
  'home.map.example': '예시 서버 구성',

  // Home: the questions people open it for.
  'home.questions.title': '질문 하나, 찾을 곳도 하나.',
  'home.questions.lead':
    'Homeport는 무언가를 확인하려고 열고, 확인한 뒤에는 다시 일로 돌아가는 앱입니다. 모든 답은 보드에서 한 단계 거리에 있습니다.',
  'home.questions.up.title': '실행 중인가요?',
  'home.questions.up.body':
    '보드는 모든 호스트, Swarm, 클러스터를 한 번에 보여 주며, 정상인 것보다 확인이 필요한 것을 먼저 나열합니다.',
  'home.questions.up.link': '보드와 개요',
  'home.questions.up.screenshot':
    '보드: 호스트 3개의 타일, 확인이 필요한 컨테이너 하나, 그리고 모든 호스트의 컨테이너를 모은 표 하나.',
  'home.questions.died.title': '왜 멈췄나요?',
  'home.questions.died.body':
    '컨테이너를 열면 로그가 이미 스트리밍되고 있습니다. 터미널, 파일, 실시간 차트, 검사 문서는 탭만 바꾸면 볼 수 있습니다.',
  'home.questions.died.link': '컨테이너 다루기',
  'home.questions.died.screenshot':
    '컨테이너의 로그 탭: 검색, 타임스탬프, 스트림 필터와 함께 출력을 스트리밍합니다.',
  'home.questions.changed.title': '무엇이 바뀌었나요?',
  'home.questions.changed.body':
    '각 호스트는 엔진이 한 일을 계속 기록합니다. 다시 시작된 컨테이너, 풀한 이미지, 마운트된 볼륨까지 남습니다.',
  'home.questions.changed.link': '이벤트',
  'home.questions.changed.screenshot':
    '호스트의 이벤트 페이지: 상태 검사, 재시작, 풀, 마운트가 최신순으로, 각각이 속한 스택과 함께 표시됩니다.',
  'home.questions.act.title': '그다음 변경하고, 변경 내용은 먼저 확인하세요.',
  'home.questions.act.body':
    '다시 배포하면 실행하기 전에 무엇이 바뀌는지 보여 줍니다. 배포마다 기록이 남아 롤백할 수 있고, 업데이트는 새 이미지를 한 번에 풀합니다.',
  'home.questions.act.link': '스택 관리하기',
  'home.questions.act.screenshot': '관리형 스택의 정의: compose.yaml 편집기, 환경, 배포 버튼.',

  // Home: the lines, and what each needs on the server.
  'home.lines.title': '서버에 자리 잡는 것은 없습니다.',
  'home.lines.lead':
    'Homeport는 사용자의 컴퓨터에 머물며 서버를 방문합니다. 설치할 에이전트도, 노출할 웹 패널도, 만들 계정도, 원격 분석도 없습니다.',
  'home.lines.line': '노선',
  'home.lines.reaches': '연결 대상',
  'home.lines.needs': '서버에 필요한 것',
  'home.lines.socket.name': '로컬 소켓',
  'home.lines.socket.reaches':
    '이 컴퓨터의 Docker Desktop, OrbStack, Colima, Rancher Desktop, Podman 머신. 자동으로 찾아 줍니다.',
  'home.lines.socket.needs': '없습니다. 엔진이 이미 이 컴퓨터에 있습니다.',
  'home.lines.ssh.reaches':
    '서버의 Docker Engine, Podman, containerd, Incus. 필요하면 배스천을 거칩니다.',
  'home.lines.ssh.needs':
    '이미 열려 있는 포트입니다. Homeport가 만든 키를 사용하면 {file}에 한 줄이 추가됩니다.',
  'home.lines.tls.reaches': 'Docker 호환 엔진의 API. 사용자의 CA와 클라이언트 인증서를 사용합니다.',
  'home.lines.tls.needs': '엔진 자체의 API 포트(보통 {port})이며, 직접 노출해야 합니다.',
  'home.lines.tcp.reaches': 'Docker 호환 엔진의 API. 암호화되지 않습니다.',
  'home.lines.tcp.needs': '엔진의 API 포트(보통 {port}). 신뢰할 수 있는 네트워크에서만 사용하세요.',
  'home.lines.cluster.name': '클러스터 API',
  'home.lines.cluster.reaches':
    'kubeconfig의 Kubernetes, 에이전트 주소로 연결하는 Nomad, 신뢰 토큰을 사용한 HTTPS의 Incus.',
  'home.lines.cluster.needs': '클러스터가 이미 제공하는 API. 노드에는 아무것도 설치하지 않습니다.',
  'home.lines.advice':
    '가능하면 SSH를 사용하세요. 이미 실행 중인 것 외에는 서버에 아무것도 더하지 않습니다.',
  'home.lines.import':
    '서버 설정을 이미 작성해 두셨나요? {file}에서 배스천까지 포함해 가져오거나, Docker 컨텍스트에서 가져오세요.',
  'home.lines.link': '연결 방식 알아보기',

  // Home: the Engines, as the map's index of stations.
  'home.engines.title': '모든 엔진을 같은 말로.',
  'home.engines.lead':
    '어떤 엔진에서 실행되든 호스트는 호스트이고 스택은 스택입니다. 엔진에만 있는 기능이 있으면 Homeport는 그것도 보여 줍니다.',
  'home.engines.docker.body':
    '컨테이너, 이미지, 볼륨, 네트워크, Compose 스택. 스택은 Homeport에 보관하거나, 폴더에 링크하거나, git 저장소에서 동기화합니다.',
  'home.engines.swarm.body':
    '매니저 호스트를 통해 찾은 노드, 서비스, 태스크, 시크릿, config. 스택을 Swarm에 배포할 수 있습니다.',
  'home.engines.podman.body':
    'Pod, 그리고 systemd에서 설치하고 시작하는 Quadlet 유닛. 모두 Podman 자체 API를 통해 이루어집니다.',
  'home.engines.containerd.body':
    '네임스페이스, 태스크, 로그, 터미널, 그리고 디스크를 차지하는 스냅샷. Docker 없이 사용합니다.',
  'home.engines.incus.body':
    '시스템 컨테이너와 가상 머신, 그리고 그 콘솔, 스냅샷, 이미지, 프로필, 스토리지, 네트워크.',
  'home.engines.kubernetes.body':
    '워크로드, Pod, Service와 모든 API 리소스를 실시간으로. 로그, 터미널, 포트 포워딩, 미리 보기가 있는 YAML, Helm 릴리스.',
  'home.engines.nomad.body':
    'HCL로 plan과 함께 실행하는 job, 그 allocation과 로그, 태스크 안의 셸, 노드 드레인.',

  // Home: security.
  'home.security.title': '키는 봉인된 채로 보관됩니다.',
  'home.security.lead': 'Homeport는 서버를 여는 자격 증명을 보관하므로, 그에 걸맞게 취급합니다.',
  'home.security.rest.title': '저장 시 암호화',
  'home.security.rest.body':
    'SSH 키, TLS 인증서, 스택 환경, 저장소 토큰은 XChaCha20-Poly1305로 봉인됩니다. 키는 데이터 옆이 아니라 OS 키체인에 있습니다.',
  'home.security.disk.title': '디스크에 평문으로 저장하지 않음',
  'home.security.disk.body':
    'Homeport가 SSH를 직접 구현하므로 복호화된 키는 메모리에만 있습니다. 키 파일도 만들지 않고, {dir}도 변경하지 않습니다.',
  'home.security.home.title': '외부로 아무것도 보내지 않음',
  'home.security.home.body':
    '계정도, 원격 분석도, 추적도 없습니다. 모든 릴리스는 체크섬과 빌드 출처 증명과 함께 공개적으로 빌드됩니다.',
  'home.security.link': '보안과 데이터',

  // Home: the close, where the visitor starts.
  'home.close.title': '이 컴퓨터에서 시작하세요.',
  'home.close.body':
    '작업하는 컴퓨터에 Homeport를 다운로드한 다음 호스트를 추가하세요. 무료이며 GNU AGPL에 따른 오픈 소스이고, GitHub에서 공개적으로 개발됩니다.',
  'home.close.github': 'GitHub에서 보기',
  'home.desktops.windows.versions': '10 및 11, x64 및 ARM64',
  'home.desktops.windows.formats': '설치 프로그램(.exe) 또는 MSI',
  'home.desktops.macos.versions': 'Apple silicon 및 Intel',
  'home.desktops.macos.formats': '디스크 이미지(.dmg)',
  'home.desktops.linux.versions': 'x64 및 ARM64',
  'home.desktops.linux.formats': 'apt, dnf, zypper 또는 AppImage',
};
