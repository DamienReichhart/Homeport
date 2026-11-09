/** Korean text of the `app` namespace. */
import type en from '../en/app';
import type { Translation } from '../types';

export default {
  'app.nav.main': '기본',
  'app.keychainMissing':
    'OS 키체인을 사용할 수 없어 SSH 키, TLS 인증서, 스택 환경을 저장할 수 없습니다. 로컬, TCP, ssh-agent 호스트는 계속 작동합니다.',
  'app.crash.title': '문제가 발생했습니다',
  'app.crash.description':
    'Homeport가 이 페이지에서 예기치 않은 문제를 만났습니다. 대개 다시 불러오면 해결됩니다. 계속 발생하면 세부 정보에서 보고서를 복사해 이슈에 붙여 넣으세요.',
  'app.crash.reload': '다시 불러오기',
  'app.crash.backToOverview': '개요로 돌아가기',
  'app.shortcuts.title': '키보드 단축키',
  'app.shortcuts.description':
    '여기에서 작동하는 단축키입니다. N, / 같은 키는 입력란에 입력하는 동안에는 작동하지 않습니다.',
  'app.shortcuts.general': '일반',
  'app.shortcuts.navigation': '이동',
  'app.shortcuts.hosts': '호스트',
  'app.shortcuts.help': '키보드 단축키 보기',
  'app.shortcuts.settings': '설정 열기',
  'app.shortcuts.goTo': '{page} 페이지로 이동',
  'app.shortcuts.previousHost': '이전 호스트',
  'app.shortcuts.nextHost': '다음 호스트',
  'app.shortcuts.refresh': '이 호스트의 목록 새로 고침',
  'app.palette.title': '명령 팔레트',
  'app.palette.description': '페이지로 이동하거나, 호스트를 전환하거나, 명령을 실행합니다.',
  'app.palette.open': '명령 팔레트 열기',
  'app.palette.placeholder': '페이지, 호스트 또는 명령 입력…',
  'app.palette.empty': '일치하는 항목이 없습니다.',
  'app.palette.recent': '최근 항목',
  'app.palette.goTo': '이동',
  'app.palette.hosts': '호스트',
  'app.palette.actions': '작업',
  'app.palette.switchTo': '{name} 호스트로 전환',
} satisfies Translation<typeof en>;
