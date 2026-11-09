/** Korean text of the `repositories` namespace. */
import type en from '../en/repositories';
import type { Translation } from '../types';

export default {
  'repositories.title': '저장소',
  'repositories.description':
    '모든 호스트에서 동기화 스택을 배포하는 Git 저장소입니다. Homeport는 저장소를 읽기만 합니다.',
  'repositories.add': '저장소 추가',
  'repositories.empty': '아직 저장소가 없습니다. 동기화 스택을 만들려면 저장소를 추가하세요.',
  'repositories.manageOne': '{repository} 액세스 관리',
  'repositories.removeOne': '{repository} 삭제',
  'repositories.remove.title': '{repository} 저장소를 삭제하시겠습니까?',
  'repositories.remove.description':
    'Homeport가 이 저장소의 배포 키나 토큰을 잊고 이 컴퓨터의 클론을 삭제합니다. 저장소 자체는 그대로 남습니다. Git 호스팅 서비스에서도 저장소의 배포 키를 삭제하세요.',
  'repositories.noKeychain':
    '배포 키와 토큰은 OS 키체인에 보관된 키로 암호화되는데, 사용할 수 있는 키체인이 없으므로 공개 저장소만 추가할 수 있습니다.',
  'repositories.public': '공개 저장소: Homeport가 자격 증명 없이 읽습니다.',

  'repositories.added': '{repository} 저장소를 추가했습니다',
  'repositories.removed': '{repository} 저장소를 삭제했습니다',
  'repositories.keyRotated': '새 배포 키를 만들었습니다. 기존 키 대신 이 키를 저장소에 추가하세요.',
  'repositories.tokenReplaced': '토큰을 교체했습니다',
  'repositories.hostKeyReset': 'Homeport는 다음에 서버가 제시하는 키를 신뢰합니다.',

  'repositories.auth.withKey': '배포 키 사용, {host}',
  'repositories.auth.withToken': '{username} 사용자, 토큰 사용, {host}',
  'repositories.auth.public': '공개, {host}',

  'repositories.dialog.addTitle': '저장소 추가',
  'repositories.dialog.description':
    'Homeport는 이 컴퓨터에 저장소를 클론해 동기화 스택을 배포합니다. 커밋하거나 푸시하지 않습니다.',
  'repositories.dialog.url': '저장소 URL',
  'repositories.dialog.auth': '액세스',
  'repositories.dialog.authKey': '배포 키',
  'repositories.dialog.authToken': '토큰',
  'repositories.dialog.authNone': '공개',
  'repositories.dialog.authKeyHelp':
    'Homeport가 이 저장소 전용 키를 만듭니다. 이 키를 읽기 권한으로 저장소에 추가하세요. 사용자의 SSH 키는 절대 사용하지 않습니다.',
  'repositories.dialog.authTokenHelp':
    '저장소 내용을 읽을 수 있는 액세스 토큰으로, HTTPS로 전송됩니다. 이 컴퓨터에서 암호화되며 다시 표시되지 않습니다.',
  'repositories.dialog.authNoneHelp':
    'HTTPS로 접근하는 공개 저장소용입니다. 아무것도 저장하지 않습니다.',
  'repositories.dialog.username': '사용자 이름',
  'repositories.dialog.token': '액세스 토큰',
  'repositories.dialog.add': '추가',
  'repositories.dialog.addWithKey': '추가하고 배포 키 만들기',

  'repositories.missing.url': '저장소 URL을 입력하세요.',
  'repositories.missing.validUrl':
    'git@github.com:owner/repo.git 또는 https://gitlab.com/owner/repo.git 같은 SSH 또는 HTTPS URL을 입력하세요.',
  'repositories.missing.auth': 'Homeport가 이 저장소에 접근할 방법을 선택하세요.',
  'repositories.missing.username': '토큰 소유자의 사용자 이름을 입력하세요.',
  'repositories.missing.token': '액세스 토큰을 입력하세요.',

  'repositories.deployKey.title': '배포 키',
  'repositories.deployKey.github':
    'GitHub: 저장소의 <strong>Settings → Deploy keys → Add deploy key</strong>.',
  'repositories.deployKey.gitlab':
    'GitLab: 프로젝트의 <strong>Settings → Repository → Deploy keys</strong>.',
  'repositories.deployKey.gitea':
    'Gitea 및 Forgejo: 저장소의 <strong>Settings → Deploy keys → Add deploy key</strong>.',
  'repositories.deployKey.readOnly':
    '쓰기 권한은 끈 상태로 두세요. Homeport는 읽기만 합니다. 그런 다음 아래에서 액세스를 테스트하세요.',
  'repositories.deployKey.fingerprint': '지문: <mono>{fingerprint}</mono>',
  'repositories.rotate.button': '새 키',
  'repositories.rotate.title': '새 배포 키를 만드시겠습니까?',
  'repositories.rotate.description':
    '현재 키는 Homeport에서 더 이상 작동하지 않습니다. 새 키를 저장소에 추가하고 기존 키는 삭제하세요.',
  'repositories.rotate.confirm': '새 키 만들기',

  'repositories.token.replace': '토큰 교체',
  'repositories.token.save': '토큰 저장',

  'repositories.hostKey.trusted': '{host}의 신뢰하는 키: <mono>{fingerprint}</mono>',
  'repositories.hostKey.reset': '재설정',
  'repositories.hostKey.resetTitle': '{host}의 신뢰하는 키를 재설정하시겠습니까?',
  'repositories.hostKey.resetDescription':
    '{host}에서 SSH 키를 바꾸었고 웹사이트에서 새 지문을 확인한 경우에만 재설정하세요. Homeport는 다음에 제시되는 키를 무엇이든 신뢰합니다.',

  'repositories.test.button': '액세스 테스트',
  'repositories.test.testing': '테스트하는 중…',
  'repositories.test.success':
    'Homeport가 읽을 수 있습니다: {branches, plural, other {브랜치 #개}}, {tags, plural, other {태그 #개}}.',
} satisfies Translation<typeof en>;
