/** Korean text of the `settings` namespace. */
import type en from '../en/settings';
import type { Translation } from '../types';

export default {
  'settings.nav': '설정',
  'settings.title': '설정',
  'settings.appearance.title': '모양',
  'settings.appearance.theme': '테마',
  'settings.theme.system': '시스템 설정에 맞춤',
  'settings.theme.light': '라이트',
  'settings.theme.dark': '다크',
  'settings.appearance.language': '언어',
  'settings.language.system': '시스템 설정에 맞춤({language})',
  'settings.desktop.title': '데스크톱',
  'settings.desktop.description': '이 컴퓨터에서 Homeport가 실행되는 방식입니다.',
  'settings.desktop.closeToTray': '닫으면 시스템 트레이로 이동',
  'settings.desktop.closeToTrayHint':
    '창을 닫아도 Homeport와 호스트 연결이 트레이에서 계속 실행됩니다. 종료하려면 트레이 아이콘을 사용하세요. 확장 프로그램 없는 GNOME처럼 트레이 아이콘을 숨기는 데스크톱에서는 아이콘이 보이지 않을 수 있습니다.',
  'settings.desktop.noTray':
    '이 데스크톱에는 Homeport가 사용할 수 있는 시스템 트레이가 없으므로 창을 닫으면 종료됩니다.',
  'settings.desktop.startAtLogin': '로그인 시 시작',
  'settings.desktop.startAtLoginHint':
    '이 컴퓨터에 로그인하면 Homeport가 시작되어 처음부터 호스트를 지켜봅니다.',
  'settings.desktop.startHidden': '트레이에 숨긴 채로 시작',
  'settings.desktop.startHiddenHint':
    '로그인 시 시작될 때 Homeport가 창을 열지 않고 시스템 트레이에 머뭅니다.',
  'settings.desktop.startHiddenNoTray':
    '창을 열 시스템 트레이가 필요한데, 이 데스크톱에는 없습니다.',
  'settings.notifications.title': '알림',
  'settings.notifications.description':
    'Homeport는 창을 닫아 트레이에 있을 때도 다음과 같은 경우 알려 줍니다.',
  'settings.notifications.containerDied': '컨테이너가 오류로 중지되거나 메모리가 부족할 때',
  'settings.notifications.containerUnhealthy': '컨테이너가 비정상 상태가 될 때',
  'settings.notifications.hostUnreachable': '연결된 호스트에 연결할 수 없게 될 때',
  'settings.notifications.hostBack': '연결할 수 없던 호스트가 다시 연결될 때',
  'settings.machine.title': '이 컴퓨터',
  'settings.machine.description': 'Homeport가 여기에서 사용할 수 있는 항목입니다.',
  'settings.machine.keychain': 'OS 키체인',
  'settings.machine.keychainAvailable':
    '사용 가능 — SSH 키, 인증서, 스택 환경, 레지스트리 로그인이 암호화되어 저장됩니다.',
  'settings.machine.keychainUnavailable':
    '사용할 수 없음 — SSH 키, TLS 인증서, 스택 환경, 레지스트리 로그인을 저장할 수 없습니다.',
  'settings.machine.compose': 'Compose CLI',
  'settings.machine.composeMissing': '스택을 시작하고 중지할 수는 있지만 배포할 수는 없습니다.',
  'settings.machine.stackDeploy': 'Docker CLI(Swarm용)',
  'settings.machine.stackDeployMissing':
    'Swarm에 스택을 배포할 수 없습니다. Swarm의 다른 기능은 모두 작동합니다.',
  'settings.machine.git': 'git',
  'settings.machine.helm': 'Helm CLI',
  'settings.sync.title': '동기화 스택',
  'settings.sync.interval': '저장소 확인 주기',
  'settings.sync.minutes': '분',
  'settings.data.title': '데이터',
  'settings.data.description': 'Homeport가 이 컴퓨터에 보관하는 항목입니다.',
  'settings.data.backup': '백업…',
  'settings.data.restore': '복원…',
  'settings.data.backupHint':
    '모든 호스트, 스택, 레지스트리 로그인, 저장소, 설정을 비밀 정보와 함께 암호 문구로 보호되는 파일 하나에 저장하고, 이 컴퓨터나 다른 컴퓨터에서 복원할 수 있습니다.',
  'settings.backup.title': 'Homeport 데이터 백업',
  'settings.backup.description':
    '이 파일에는 모든 호스트, 클러스터, 스택, 레지스트리 로그인, 저장소, 설정과 해당 키, 인증서, 토큰, 비밀번호, 환경이 들어 있습니다. 파일과 암호 문구를 가진 사람은 누구나 이를 사용할 수 있으므로 둘 다 안전하게 보관하세요. 엔진 이벤트와 배포 기록은 포함되지 않습니다.',
  'settings.backup.passphrase': '암호 문구',
  'settings.backup.passphraseHint': '{min}자 이상. 복구할 수 없습니다.',
  'settings.backup.confirmation': '암호 문구 다시 입력',
  'settings.backup.problem.short': '{min}자 이상 사용하세요.',
  'settings.backup.problem.mismatch': '두 암호 문구가 다릅니다.',
  'settings.backup.start': '백업…',
  'settings.backup.saveTitle': '백업을 저장할 위치',
  'settings.backup.fileType': 'Homeport 백업',
  'settings.backup.done':
    '{hosts, plural, other {호스트 #개}}와 {stacks, plural, other {스택 #개}}를 백업했습니다',
  'settings.backup.withoutSecrets':
    '다음 항목은 비밀 정보를 열 수 없어 비밀 정보 없이 백업되었습니다.',
  'settings.restore.pick': '복원할 파일',
  'settings.restore.title': 'Homeport 데이터 복원',
  'settings.restore.from': '원본: {path}',
  'settings.restore.open': '열기',
  'settings.restore.madeBy': '{date}에 Homeport {version} 버전으로 백업했습니다. 포함된 항목:',
  'settings.restore.holds': '백업에 포함된 항목',
  'settings.restore.hosts': '{count, plural, other {호스트 #개}}',
  'settings.restore.clusters': '{count, plural, other {클러스터 #개}}',
  'settings.restore.stacks': '{count, plural, other {스택 #개}}',
  'settings.restore.registries': '{count, plural, other {레지스트리 로그인 #개}}',
  'settings.restore.repositories': '{count, plural, other {저장소 #개}}',
  'settings.restore.settings': '설정',
  'settings.restore.replaces':
    '복원하면 현재 있는 모든 호스트, 스택, 레지스트리 로그인, 저장소, 설정이 교체되고 엔진 이벤트와 배포 기록은 삭제됩니다. 먼저 현재 데이터의 사본이 Homeport 데이터 폴더에 보관됩니다.',
  'settings.restore.confirm': '모두 교체',
  'settings.restore.done':
    '{hosts, plural, other {호스트 #개}}와 {stacks, plural, other {스택 #개}}를 복원했습니다',
  'settings.data.keepEvents': '엔진 이벤트 보관 기간',
  'settings.data.days': '일',
  'settings.data.keepEventsHint':
    '각 호스트는 이벤트 페이지용으로 최대 10,000개의 이벤트를 보관합니다. 오래된 이벤트는 매시간 삭제됩니다.',
  'settings.machine.gitMissing':
    '동기화 스택에 필요합니다. 다른 종류의 스택은 git 없이 작동합니다.',
  'settings.machine.helmMissing':
    'Helm 릴리스를 나열하거나 설치할 수 없습니다. 클러스터의 다른 기능은 모두 작동합니다.',
  'settings.about.tagline': '모든 Docker Engine을 하나의 보드에서.',
  'settings.about.openLogFolder': '로그 폴더 열기',
  'settings.about.licenses': '라이선스',
  'settings.about.exportDiagnostics': '진단 정보 내보내기…',
  'settings.diagnostics.title': '진단 정보 내보내기',
  'settings.diagnostics.description':
    '버그 보고서에 첨부할 zip 파일입니다. 버전과 사용 중인 호스트 및 클러스터 종류의 요약, 그리고 비밀 정보를 제거한 Homeport 로그가 들어 있습니다. 어디로도 전송되지 않으며, 파일을 공유할지는 직접 결정합니다.',
  'settings.diagnostics.includeNames': '이름과 주소 포함',
  'settings.diagnostics.includeNamesHelp':
    '끄면 호스트와 클러스터가 요약과 로그에 host-1, cluster-1…로 표시됩니다.',
  'settings.diagnostics.files': 'zip에 포함된 파일',
  'settings.diagnostics.save': '저장…',
  'settings.diagnostics.saveTitle': '진단 정보 저장',
  'settings.diagnostics.fileType': 'Zip 압축 파일',
  'settings.diagnostics.done': '진단 정보를 {path}에 저장했습니다.',
  'settings.licenses.title': '라이선스',
  'settings.licenses.description':
    'Homeport는 GNU Affero General Public License 버전 3에 따른 자유 소프트웨어이며, 아래의 오픈 소스 패키지를 기반으로 만들어졌습니다.',
  'settings.licenses.source': '이 버전의 소스 코드는 같은 라이선스로 공개되어 있습니다.',
  'settings.licenses.sourceLink': 'v{version} 소스',
  'settings.licenses.thirdParty': '타사 소프트웨어',
  'settings.licenses.missing.title': '이 빌드에 포함되지 않음',
  'settings.licenses.missing.description':
    '릴리스 빌드에는 포함됩니다. 개발 빌드에서 생성하려면 <code>{command}</code> 명령을 실행하세요.',
  'settings.palette.group': '설정',
  'settings.palette.themeLight': '테마: 라이트',
  'settings.palette.themeDark': '테마: 다크',
  'settings.palette.themeSystem': '테마: 시스템 설정에 맞춤',
} satisfies Translation<typeof en>;
