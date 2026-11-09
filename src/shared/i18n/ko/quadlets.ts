/** Korean text of the `quadlets` namespace. */
import type en from '../en/quadlets';
import type { Translation } from '../types';

export default {
  'quadlets.nav': 'Quadlet',
  'quadlets.title': 'Quadlet 유닛',
  'quadlets.summary': '{count, plural, other {유닛 #개}}',
  'quadlets.search': '유닛 검색',
  'quadlets.empty': 'Quadlet 유닛 없음',
  'quadlets.emptyDescription':
    'Quadlet 유닛은 Podman이 systemd 서비스로 바꾸는 파일로, 컨테이너가 호스트와 함께 시작되고 systemd가 계속 실행되도록 유지합니다. 유닛을 설치하면 여기에 표시됩니다.',
  'quadlets.new': '새 유닛',
  'quadlets.edit': '편집',
  'quadlets.notGenerated': '아직 생성되지 않음',
  'quadlets.gone': '{name} 유닛은 더 이상 설치되어 있지 않습니다',
  'quadlets.column.name': '유닛',
  'quadlets.column.kind': '종류',
  'quadlets.column.service': '서비스',
  'quadlets.column.status': '상태',
  'quadlets.column.containers': '컨테이너',
  'quadlets.status.running': '실행 중',
  'quadlets.status.starting': '시작하는 중',
  'quadlets.status.stopping': '중지하는 중',
  'quadlets.status.stopped': '중지됨',
  'quadlets.status.failed': '실패',
  'quadlets.status.notLoaded': 'systemd에 로드되지 않음',
  'quadlets.status.template': '템플릿',
  'quadlets.action.start': '시작',
  'quadlets.action.stop': '중지',
  'quadlets.action.restart': '다시 시작',
  'quadlets.actionOne.start': '{name} 시작',
  'quadlets.actionOne.stop': '{name} 중지',
  'quadlets.actionOne.restart': '{name} 다시 시작',
  'quadlets.done.start': '{name} 시작됨',
  'quadlets.done.stop': '{name} 중지됨',
  'quadlets.done.restart': '{name} 다시 시작됨',
  'quadlets.installed': '{name} 설치됨',
  'quadlets.saved': '{name} 저장됨',
  'quadlets.removed': '{name} 삭제됨',
  'quadlets.blocked.noCommands':
    'Homeport는 systemctl로 유닛을 시작하고 중지하는데, systemctl은 SSH를 통해서나 이 컴퓨터의 Podman에서만 실행할 수 있습니다.',
  'quadlets.blocked.notLoaded': 'systemd가 이 유닛의 서비스를 생성하지 않아 시작할 것이 없습니다.',
  'quadlets.blocked.podmanTooOld':
    '유닛을 읽고, 설치하고, 편집하고, 삭제하려면 Podman 5.8 이상이 필요합니다. 이 호스트의 Podman 버전은 {version}입니다.',
  'quadlets.blocked.noFile':
    '이 유닛은 컨테이너를 통해서만 알려져 있으므로 파일을 변경할 수 없습니다.',
  'quadlets.note.labels':
    'Podman {version}에서는 Quadlet 유닛 목록을 볼 수 없으므로 이 항목은 유닛이 만든 컨테이너에서 가져온 것입니다. 유닛 파일과 컨테이너를 만들지 않은 유닛은 Podman 5.7 이상에서 표시됩니다.',
  'quadlets.note.readOnly':
    'Podman {version}에서는 유닛 목록은 볼 수 있지만 읽기, 설치, 편집, 삭제는 할 수 없습니다. 이 작업에는 Podman 5.8 이상이 필요합니다.',
  'quadlets.note.noCommands':
    '유닛을 시작하고 중지하려면 이 호스트에 SSH로 연결하거나 이 컴퓨터의 Podman이어야 합니다.',
  'quadlets.detail.service': '서비스',
  'quadlets.detail.unit': 'systemd 유닛',
  'quadlets.detail.scope': '실행 주체',
  'quadlets.detail.path': '파일',
  'quadlets.detail.app': '애플리케이션',
  'quadlets.detail.pod': 'Pod',
  'quadlets.detail.containers': '{count, plural, =0 {컨테이너} other {컨테이너 #개}}',
  'quadlets.detail.noContainers': '컨테이너를 만들지 않았거나 서비스가 중지되어 있습니다.',
  'quadlets.scope.user': '사용자 유닛(systemctl --user)',
  'quadlets.scope.system': '시스템 유닛',
  'quadlets.file.title': '유닛 파일',
  'quadlets.file.unknown': '이 유닛은 컨테이너의 레이블을 통해서만 알려져 있습니다.',
  'quadlets.file.tooOld':
    'Podman {version}에서는 유닛 파일을 읽을 수 없습니다. 표시하려면 Podman 5.8 이상이 필요합니다.',
  'quadlets.form.newTitle': '새 Quadlet 유닛',
  'quadlets.form.newDescription':
    'Podman이 해당 사용자의 유닛 위치에 파일을 쓰고 systemd를 다시 로드합니다. 설치되면 유닛을 시작하세요.',
  'quadlets.form.editTitle': '{name} 편집',
  'quadlets.form.editDescription':
    '저장하면 파일이 교체되고 systemd가 다시 로드됩니다. 실행 중인 서비스에 적용하려면 유닛을 다시 시작하세요. 함께 설치된 파일은 그대로 유지됩니다.',
  'quadlets.form.name': '파일 이름',
  'quadlets.form.kind': '종류',
  'quadlets.form.openFile': '파일 열기…',
  'quadlets.form.openTitle': 'Quadlet 유닛 파일 열기',
  'quadlets.form.unitFiles': 'Quadlet 유닛 파일',
  'quadlets.form.content': '유닛 파일',
  'quadlets.form.extra': '함께 설치할 파일',
  'quadlets.form.extraHint':
    '유닛이 사용하는 Containerfile, kube YAML 또는 env 파일로, 유닛의 애플리케이션 폴더에 설치됩니다.',
  'quadlets.form.extraTitle': '유닛과 함께 설치할 파일 선택',
  'quadlets.form.addExtra': '파일 추가…',
  'quadlets.form.removeExtra': '{name} 삭제',
  'quadlets.form.install': '설치',
  'quadlets.form.save': '저장',
  'quadlets.form.nameInvalid':
    'web.container처럼 .container, .pod, .volume, .network, .kube, .image, .build 또는 .artifact로 끝나는 이름을 지정하세요.',
  'quadlets.form.sectionMissing': 'Quadlet이 파일을 읽으려면 {section} 섹션이 있어야 합니다.',
  'quadlets.form.extraInvalid':
    '유닛과 함께 설치하는 각 파일에는 폴더 없이 고유한 이름만 있어야 합니다.',
  'quadlets.remove.title': '{name} 유닛을 삭제하시겠습니까?',
  'quadlets.remove.description':
    'Podman이 유닛 파일을 삭제하고 systemd를 다시 로드합니다. 유닛이 만든 컨테이너, 볼륨, 이미지는 그대로 유지됩니다.',
  'quadlets.remove.stop': '서비스가 실행 중이면 중지',
  'quadlets.remove.stopRunning': '먼저 실행 중인 서비스 중지',
} satisfies Translation<typeof en>;
