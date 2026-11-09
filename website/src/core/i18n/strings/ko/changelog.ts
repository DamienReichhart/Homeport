import type { Translation } from '../../types';
import type en from '../en/changelog';

/** The changelog page, in Korean. */
export default {
  'changelog.title': '변경 내역 — Homeport',
  'changelog.description':
    'Homeport 릴리스마다 새로 추가된 기능, 수정 사항, 그 밖의 변경 사항입니다.',
  'changelog.eyebrow': '변경 내역',
  'changelog.heading': 'Homeport의 새로운 내용',
  'changelog.lead':
    '모든 릴리스를 최신순으로 보여 줍니다. 릴리스 노트는 프로젝트의 커밋에서 생성되며, 파일은 {link}에 있습니다.',
  'changelog.downloadPage': '다운로드 페이지',
  'changelog.notesInEnglish': '아래 릴리스 노트는 GitHub에 게시된 그대로 영어로 되어 있습니다.',
  'changelog.empty': '릴리스 기록은 {link}에 있습니다.',
  'changelog.github': 'GitHub',
  'changelog.latest': '최신',
  'changelog.prerelease': '사전 릴리스',
  'changelog.onGitHub': 'GitHub의 릴리스',
  'changelog.download': '다운로드',
} satisfies Translation<typeof en>;
