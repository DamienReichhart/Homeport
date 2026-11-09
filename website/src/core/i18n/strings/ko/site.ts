import type { Translation } from '../../types';
import type en from '../en/site';

/** The layout, the header and the footer, in Korean. */
export default {
  // The layout and the facts every page repeats.
  'site.tagline': '모든 Docker Engine을 하나의 보드에서.',
  'site.description':
    'Homeport는 어디에 있든 Docker Engine을 관리하는 무료 오픈 소스 데스크톱 앱입니다. 이 컴퓨터의 엔진은 물론, SSH, TLS 또는 TCP로 원격 엔진도 관리할 수 있으며 Windows, macOS, Linux를 지원합니다.',
  'site.ogImageAlt': 'Homeport — 모든 Docker Engine을 하나의 보드에서.',
  'site.skipToContent': '본문으로 건너뛰기',

  // The header, the language menu and the theme switch.
  'header.home': 'Homeport 홈',
  'header.logo': 'Homeport',
  'header.nav': '기본',
  'header.features': '기능',
  'header.docs': '문서',
  'header.changelog': '변경 내역',
  'header.download': '다운로드',
  'header.github': 'GitHub의 Homeport',
  'header.githubShort': 'GitHub',
  'header.menu': '메뉴',
  'language.label': '언어',
  'theme.toLight': '밝은 테마로 전환',
  'theme.toDark': '어두운 테마로 전환',

  // The footer.
  'footer.product': '제품',
  'footer.features': '기능',
  'footer.download': '다운로드',
  'footer.changelog': '변경 내역',
  'footer.releases': '모든 릴리스',
  'footer.documentation': '문서',
  'footer.introduction': '소개',
  'footer.installation': '설치',
  'footer.hosts': '호스트 연결',
  'footer.troubleshooting': '문제 해결',
  'footer.project': '프로젝트',
  'footer.source': '소스 코드',
  'footer.issue': '문제 신고',
  'footer.contributing': '기여하기',
  'footer.security': '보안',
  'footer.legal': '법률',
  'footer.legalNotice': '법적 고지',
  'footer.privacy': '개인정보 보호',
  'footer.terms': '이용 약관',
  'footer.cookies': '쿠키',
  'footer.licence': '© {year} Damien Reichhart. Homeport는 {licence}에 따른 오픈 소스입니다.',
  'footer.licenceName': 'GNU AGPL 버전 3',
  'footer.trademark':
    'Docker는 Docker, Inc.의 상표입니다. Homeport는 독립 프로젝트이며 Docker, Inc.와 제휴하거나 Docker, Inc.의 보증을 받지 않았습니다.',
} satisfies Translation<typeof en>;
