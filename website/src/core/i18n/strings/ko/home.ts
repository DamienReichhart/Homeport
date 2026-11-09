import type { Translation } from '../../types';
import type en from '../en/home';

/** The home page, in Korean. */
export default {
  // Home: the hero and its board.
  'home.title': 'Homeport — 모든 Docker Engine을 하나의 보드에서.',
  'home.hero.new': '새 소식',
  'home.hero.release': 'Homeport {version} 출시',
  'home.hero.title': '모든 Docker Engine을 하나의 보드에서.',
  'home.hero.lead':
    'Homeport는 사용자의 컴퓨터에서 실행되며 SSH, TLS 또는 TCP로 호스트에 연결합니다. 서버에 에이전트가 필요 없습니다. 무료, 오픈 소스, Windows, macOS, Linux 지원.',
  'home.hero.source': '소스 코드 보기',
  'home.board.caption':
    '예시 보드: 호스트 5개와 각 호스트의 엔진, Homeport가 연결하는 방식입니다. 하나는 확인이 필요합니다.',
  'home.board.host': '호스트',
  'home.board.engine': '엔진',
  'home.board.via': '연결 방식',
  'home.board.stacks': '스택',
  'home.board.status': '상태',
  'home.board.allRunning': '모두 실행 중',
  'home.board.needsYou': '확인이 필요한 스택 1개',
  'home.board.note': '정상 상태에는 색이 없습니다. 확인이 필요한 것만 빨간색으로 바뀝니다.',
  'home.runtimes.label': '지원 대상',
  'home.runtimes.title': '이미 사용 중인 도구와 함께 작동합니다',

  // Home: where it lives.
  'home.visits.eyebrow': '실행 위치',
  'home.visits.title':
    'Homeport는 사용자의 컴퓨터에 머물며 서버를 방문합니다. 서버에 자리 잡지 않습니다.',
  'home.visits.agent.title': '에이전트 없음',
  'home.visits.agent.body':
    '서버에 설치하거나 업데이트하거나 보안을 관리할 것이 없습니다. Homeport는 이미 서버에 있는 엔진과 통신합니다.',
  'home.visits.panel.title': '웹 패널 없음',
  'home.visits.panel.body':
    '서버에 배포되는 대시보드도, 누군가 찾아내기를 기다리는 로그인 페이지도 없습니다.',
  'home.visits.account.title': '계정 없음',
  'home.visits.account.body':
    '로그인도, 사용자 라이선스도, 구독도, 원격 분석도 없습니다. 컨테이너를 보여 줄 뿐, 그 이상은 하지 않습니다.',
  'home.visits.ports.title': '서버에 필요한 것, 있는 그대로',
  'home.visits.ports.ssh':
    '이미 열려 있는 포트입니다. Homeport가 만든 키를 사용하면 {file}에 한 줄이 추가됩니다.',
  'home.visits.ports.tls': '엔진 자체의 API 포트(보통 {port})이며, 직접 노출해야 합니다.',
  'home.visits.ports.tcp':
    '엔진의 API 포트(보통 {port})이며, 암호화되지 않습니다. 신뢰할 수 있는 네트워크에서만 사용하세요.',
  'home.visits.ports.advice':
    '가능하면 SSH를 사용하세요. 이미 실행 중인 것 외에는 서버에 아무것도 더하지 않습니다.',
  'home.visits.link': '연결 방식 알아보기',

  // Home: the features.
  'home.features.eyebrow': '기능',
  'home.features.title': '엔진에서 실행되는 모든 것을 하나의 창에서',
  'home.features.lead':
    '무엇이 실행 중인지, 무엇을 출력하는지, 리소스를 얼마나 쓰는지 등 Docker의 일상 업무를, 어느 서버에서 실행되는지 기억하지 않고도 처리할 수 있습니다.',
  'home.features.hosts.title': '가까운 호스트도, 먼 호스트도',
  'home.features.hosts.body':
    '로컬 엔진과 Podman 머신은 자동으로 찾아 줍니다. 원격 엔진은 SSH(필요하면 배스천 경유), 상호 TLS 또는 TCP로 추가하거나, ~/.ssh/config와 Docker 컨텍스트에서 바로 가져올 수 있습니다. 네트워크가 잠시 끊겨도 각 호스트가 알아서 다시 연결됩니다.',
  'home.features.containers.title': '컨테이너를 한눈에',
  'home.features.containers.body':
    '스택별로 묶어 CPU와 메모리를 실시간으로 보여 주고, 비정상인 컨테이너는 그 이유도 알려 줍니다. 제한, 장치, 상태 검사를 지정해 실행한 다음 컨테이너를 편집, 복제, 커밋하거나 내보낼 수 있습니다.',
  'home.features.stacks.title': 'Compose 스택',
  'home.features.stacks.body':
    '템플릿에서 시작하거나, 이 컴퓨터의 프로젝트 폴더를 링크하거나, git 저장소를 따라가도록 한 다음 어느 호스트에든 배포하세요. 빌드도 포함됩니다. 다시 배포하거나 동기화하기 전에 매번 무엇이 바뀌는지 확인하고, 롤백하고, 한 번에 새 이미지로 업데이트할 수 있습니다.',
  'home.features.swarm.title': 'Docker Swarm',
  'home.features.swarm.body':
    'Swarm은 매니저 호스트를 통해 표시됩니다. 노드, 서비스, 태스크, 시크릿, config를 관리하고, 스택을 Swarm에 배포하고, 호스트에서 Swarm을 시작하거나 Swarm에 참여하거나 Swarm에서 나올 수 있습니다. 어느 노드에도 아무것도 설치하지 않습니다.',
  'home.features.podman.title': 'Podman은 Podman답게',
  'home.features.podman.body':
    'Homeport는 연결할 때 Podman을 알아보고 Podman에만 있는 기능을 더합니다. 포트를 지정해 Pod를 만들고 그 안에서 컨테이너를 실행할 수 있으며, Quadlet 유닛을 systemd에서 설치, 편집, 시작할 수 있습니다. 모두 Podman 자체 API를 통해 이루어집니다.',
  'home.features.containerd.title': 'containerd에 직접',
  'home.features.containerd.body':
    'Docker 없이 containerd의 소켓에 로컬로 또는 SSH로 연결합니다. 네임스페이스를 선택하고, 컨테이너의 태스크를 시작, 중지하거나 시그널을 보내고, 로그를 읽고, 터미널을 열고, 레지스트리 로그인으로 이미지를 풀하고, 디스크를 차지하는 스냅샷을 찾을 수 있습니다.',
  'home.features.incus.title': 'Incus 인스턴스',
  'home.features.incus.body':
    'Incus를 소켓, SSH 또는 신뢰 토큰을 사용한 HTTPS로 추가하고, 시스템 컨테이너와 가상 머신을 실행하세요. 원격 이미지에서 만들고, 시작, 정지, 중지하고, 콘솔이나 셸을 열고, 스냅샷을 만들고 복원하고, 이미지, 프로필, 스토리지, 네트워크를 관리할 수 있습니다.',
  'home.features.clusters.title': 'Kubernetes와 Nomad 클러스터',
  'home.features.clusters.body':
    'kubeconfig에서 Kubernetes 클러스터(EKS, GKE, AKS 포함)를 추가하거나, 로컬 클러스터를 클릭 한 번으로 찾으세요. 워크로드, Pod, Service, 구성과 모든 API 리소스를 실시간으로 살펴보고, 로그를 따라가고, 터미널을 열고, 포트 포워딩을 하고, 스케일하고, 미리 보기를 확인한 뒤 YAML을 적용하고, Helm 차트를 설치, 업그레이드, 롤백할 수 있습니다. Nomad 클러스터는 주소로 추가해 HCL로 작성한 job을 plan과 함께 실행하고, 스케일하거나 중지하고, 로그를 따라가고, 셸을 열고, 노드를 드레인할 수 있습니다. 클러스터에는 아무것도 설치하지 않습니다.',
  'home.features.images.title': '이미지',
  'home.features.images.body':
    'Docker Hub를 검색하고, 레지스트리 로그인으로 풀하고 푸시하고, 컴퓨터의 폴더에서 빌드하고, 아카이브를 저장하고 불러오고, 새 이미지가 나왔는지 확인하세요.',
  'home.features.volumes.title': '볼륨과 네트워크',
  'home.features.volumes.body':
    '어떤 컨테이너가 무엇을 사용하는지 확인하세요. 만들고, 연결하고, 연결을 해제할 수 있으며, 삭제할 때는 무엇을 잃게 되는지 정확히 알려 줍니다.',
  'home.features.board.title': '모든 호스트를 하나의 보드에',
  'home.features.board.body':
    '모든 호스트의 모든 컨테이너를 하나의 표에 모아 필터링하고 한꺼번에 작업할 수 있습니다. 호스트마다 페이지도 하나씩 있습니다. 엔진 세부 정보, 대화 상자 하나로 정리할 수 있는 디스크 사용량, 엔진이 하는 일을 보여 주는 실시간 스트림이 있습니다.',
  'home.features.keyboard.title': '키보드 우선',
  'home.features.keyboard.body':
    '명령 팔레트로 어디든 이동하고, 호스트를 전환하거나 명령을 실행하고, 어느 호스트에 있든 컨테이너, 이미지, 볼륨을 찾을 수 있습니다. 모든 페이지에 단축키가 있으며, ?를 누르면 목록을 볼 수 있습니다.',
  'home.features.background.title': '백그라운드에서 지켜보기',
  'home.features.background.body':
    '시스템 트레이에 모든 호스트의 상태가 표시되고, 컨테이너가 비정상 종료되거나 호스트 연결이 끊기면 알림이 옵니다. Homeport는 로그인할 때 시작되어 필요할 때까지 숨어 있습니다.',

  // Home: Hosts.
  'home.hosts.eyebrow': '호스트',
  'home.hosts.title': '번거로운 절차 없는 원격 엔진',
  'home.hosts.body':
    '서버를 한 번 추가하면 보드에 계속 남아 있습니다. SSH의 경우 Homeport가 해당 호스트 전용 키를 만들고, 그 키를 승인하는 명령 하나를 알려 줍니다. 처음 연결할 때 서버의 지문을 확인하고, 지문이 바뀌면 경고합니다.',
  'home.hosts.import':
    '이미 설정해 두셨나요? {file}에서 서버를 배스천까지 포함해 가져오거나, Docker 컨텍스트를 인증서까지 포함해 가져오세요. 새 컴퓨터로 옮길 때는 암호 문구로 잠근 파일 하나에 담아 가져갈 수 있습니다.',
  'home.hosts.socket.name': '로컬 소켓',
  'home.hosts.socket.detail': 'Unix 소켓과 Windows 명명된 파이프를 자동으로 감지합니다.',
  'home.hosts.ssh.detail':
    '호스트마다 전용 Ed25519 키, 기존에 쓰던 키, ssh-agent 또는 Pageant를 사용하며, 배스천을 거쳐 연결할 수도 있습니다.',
  'home.hosts.tls.detail': '사용자의 CA, 클라이언트 인증서와 키를 사용하는 상호 TLS.',
  'home.hosts.tcp.detail':
    '암호화되지 않은 일반 TCP. 신뢰할 수 있는 네트워크에 있는 엔진에만 사용하세요.',
  'home.hosts.link': '연결 방식 알아보기',
  'home.hosts.screenshot':
    'SSH가 선택된 호스트 추가 페이지: 서버, 포트, 사용자, 원격 소켓 경로, 이 호스트용 새 키.',

  // Home: containers.
  'home.containers.eyebrow': '컨테이너',
  'home.containers.title': '컨테이너의 모든 것을 클릭 한 번으로',
  'home.containers.body':
    '컨테이너를 열면 로그가 이미 스트리밍되고 있습니다. 탭을 바꾸면 대화형 터미널, 파일, 실시간 차트, 프로세스, 전체 검사 문서를 볼 수 있습니다. 지구 반대편의 호스트도 노트북에서처럼 쉽게 다룰 수 있습니다.',
  'home.containers.logs.title': '로그',
  'home.containers.logs.body': '따라가기, 검색, 스트림과 시간별 필터, 다운로드.',
  'home.containers.terminal.title': '터미널',
  'home.containers.terminal.body': '필요한 만큼 여러 개의 대화형 셸.',
  'home.containers.files.title': '파일',
  'home.containers.files.body': '탐색, 업로드, 다운로드. 폴더는 .tar로.',
  'home.containers.stats.title': '통계',
  'home.containers.stats.body': 'CPU, 메모리, 네트워크, 디스크 I/O 차트.',
  'home.containers.link': '컨테이너 다루기',
  'home.containers.screenshot':
    '컨테이너의 로그 탭: 검색, 타임스탬프, 스트림 필터와 함께 출력을 스트리밍합니다.',

  // Home: Stacks.
  'home.stacks.eyebrow': '스택',
  'home.stacks.title': '관리형, 링크된, 동기화, 찾아낸 Compose 스택',
  'home.stacks.body':
    'compose 파일과 환경을 Homeport에 보관하고 어느 호스트에든 배포하세요. 템플릿에서 시작하고, 여러 파일을 조합하고, 프로필을 선택할 수 있습니다. 변수는 암호화되어 저장되며 디스크에 기록되지 않습니다. 또는 이 컴퓨터의 프로젝트 폴더를 링크하세요. Homeport가 그 폴더의 파일을 그 자리에서 편집하고 그 폴더에서 배포하며, 빌드도 포함됩니다. 또는 배포 키나 토큰으로 git 저장소의 브랜치를 따라가세요. 동기화할 때마다 배포하기 전에 새 커밋과 그 변경 내용을 보여 주고, Homeport가 열려 있는 동안 새 커밋을 확인하며, 원하면 알아서 배포합니다. 다시 배포할 때마다 무엇이 바뀌는지 먼저 보여 주고, 배포마다 기록이 남아 롤백할 수 있으며, 업데이트는 새 이미지를 풀하고 한 번에 다시 배포합니다. 누군가 직접 배포한 스택도 표시됩니다. 시작, 중지, 삭제하거나 관리형으로 전환할 수 있습니다.',
  'home.stacks.link': '스택 관리하기',
  'home.stacks.screenshot':
    '관리형 스택의 정의: compose.yaml 편집기, 환경, 저장 후 다시 배포 버튼.',

  // Home: security.
  'home.security.eyebrow': '보안',
  'home.security.title': '키는 봉인된 채로 보관됩니다',
  'home.security.lead': 'Homeport는 서버를 여는 자격 증명을 다루므로, 그에 걸맞게 취급합니다.',
  'home.security.rest.title': '저장 시 암호화',
  'home.security.rest.body':
    'SSH 키, TLS 인증서, 스택 환경, 저장소 토큰은 XChaCha20-Poly1305로 봉인됩니다. 키는 데이터 옆이 아니라 OS 키체인에 있습니다.',
  'home.security.disk.title': '디스크에 평문으로 저장하지 않음',
  'home.security.disk.body':
    'Homeport가 SSH를 직접 구현하므로 복호화된 키는 메모리에만 있습니다. 키 파일도 만들지 않고, {dir}도 변경하지 않습니다.',
  'home.security.home.title': '외부로 아무것도 보내지 않음',
  'home.security.home.body':
    '계정도, 원격 분석도, 추적도 없습니다. Homeport는 사용자의 엔진하고만 통신합니다. 모든 릴리스는 체크섬과 빌드 출처 증명과 함께 공개적으로 빌드됩니다.',
  'home.security.link': '보안과 데이터',

  // Home: desktops and the closing call.
  'home.desktops.eyebrow': '다운로드',
  'home.desktops.title': '모든 데스크톱에서 네이티브로',
  'home.desktops.lead':
    'Tauri와 Rust로 만든 작고 빠른 앱이며, 브라우저 탭을 감싼 것이 아닙니다. x64와 ARM용 설치 프로그램이 Homeport를 최신 상태로 유지합니다.',
  'home.desktops.windows.versions': '10 및 11 · x64 및 ARM64',
  'home.desktops.windows.formats': '설치 프로그램(.exe) · MSI',
  'home.desktops.macos.versions': 'Apple silicon 및 Intel',
  'home.desktops.macos.formats': '디스크 이미지(.dmg)',
  'home.desktops.linux.versions': 'x64 및 ARM64',
  'home.desktops.linux.formats': 'apt · dnf · zypper · AppImage',
  'home.cta.title': '서버에 설치할 것이 없습니다.',
  'home.cta.body':
    '작업하는 컴퓨터에 Homeport를 다운로드하고 호스트를 추가하세요. 무료이며 GNU AGPL에 따른 오픈 소스이고, GitHub에서 공개적으로 개발됩니다.',
  'home.cta.github': 'GitHub에서 보기',
} satisfies Translation<typeof en>;
