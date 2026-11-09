/** Korean text of the `ui` namespace. */
import type en from '../en/ui';
import type { Translation } from '../types';

export default {
  'ui.copy': '복사',
  'ui.copied': '복사됨',
  'ui.search': '검색',
  'ui.shortcuts.focusSearch': '이 페이지 검색',
  'ui.shortcuts.thisPage': '이 페이지',
  'ui.table.selectAll': '모두 선택',
  'ui.table.selectRow': '행 선택',
  'ui.table.empty': '항목이 없습니다.',
  'ui.table.page': '{page}/{pages} 페이지 · {rows, plural, other {#개 행}}',
  'ui.table.previous': '이전 페이지',
  'ui.table.next': '다음 페이지',
  'ui.transfer.done': '완료되었습니다.',
  'ui.files.column.name': '이름',
  'ui.files.column.size': '크기',
  'ui.files.column.modified': '수정한 날짜',
  'ui.files.column.mode': '모드',
  'ui.files.path': '경로',
  'ui.files.downloadOne': '{name} 다운로드',
  'ui.files.empty': '이 폴더는 비어 있습니다.',
  'ui.chart.time': '시간',
  'ui.errorDetails.title': '오류 세부 정보',
  'ui.errorDetails.code': '코드',
  'ui.errorDetails.command': '명령',
  'ui.errorDetails.when': '시각',
  'ui.errorDetails.app': '앱',
  'ui.errorDetails.rawDetail': '원본 세부 정보',
  'ui.errorDetails.rawDetailNote':
    '시스템, 엔진 또는 라이브러리가 작성한 그대로이므로 번역되지 않습니다.',
  'ui.errorDetails.openLogFolder': '로그 폴더 열기',
  'ui.errorDetails.copyReport': '보고서 복사',
} satisfies Translation<typeof en>;
