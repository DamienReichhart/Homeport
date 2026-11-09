import type { Translation } from '../../types';
import type en from '../en/download';

/** The download page, and the download button other pages show, in Korean. */
export default {
  'download.title': 'Homeport 다운로드',
  'download.description':
    'Windows, macOS, Linux용 Homeport를 다운로드하세요. x64 및 ARM 설치 프로그램과 Linux 패키지 저장소를 체크섬, 빌드 출처 증명과 함께 제공합니다.',

  // The download button and the line under it; the notes replace the default once the visitor's system is known.
  'download.button': 'Homeport 다운로드',
  'download.button.for': '{os}용 다운로드',
  'download.meta.version': '버전 {version}',
  'download.meta.everywhere': 'Windows, macOS, Linux',
  'download.meta.all': '모든 다운로드',
  'download.note.windowsArm': 'Windows on ARM 설치 프로그램',
  'download.note.windowsX64': 'Windows x64 설치 프로그램',
  'download.note.macosArm': 'Apple silicon Mac용',
  'download.note.macosX64': 'Intel Mac용',
  'download.note.macos': 'Apple silicon 및 Intel',
  'download.note.linux': '.deb, .rpm, AppImage',

  // The hero.
  'download.hero.eyebrow': '다운로드',
  'download.hero.title': 'Homeport 받기',
  'download.hero.release': '버전 {version}, {date} 출시.',
  'download.hero.whatsNew': '새로운 내용',
  'download.hero.noRelease': '모든 플랫폼용 설치 프로그램은 {link}에 있습니다.',
  'download.hero.releasesPage': 'GitHub 릴리스 페이지',
  'download.recommend': '{platform} 환경을 사용 중입니다. 이 환경용 다운로드를 맨 앞에 두었습니다.',
  'download.recommend.arch': '{os}({arch})',

  // The platforms and their installers.
  'download.preview': '{label} {name} 버전은 테스트용 사전 릴리스입니다. {link}.',
  'download.preview.label': '미리 보기 버전 있음:',
  'download.preview.link': 'GitHub에서 다운로드하세요',
  'download.windows.requirement': 'Windows 10 또는 11',
  'download.macos.requirement': 'macOS 10.15 이상',
  'download.linux.requirement': 'Ubuntu 22.04, Debian 12, Fedora 36 이상',
  'download.platform.yours': '사용 중인 시스템',
  'download.platform.guide': '{platform} 설치 가이드',
  'download.arch.appleSilicon': 'Apple silicon',
  'download.arch.intel': 'Intel',
  'download.arch.arm64': 'ARM64',
  'download.arch.x64': 'x64',
  'download.format.nsis.label': '설치 프로그램',
  'download.format.nsis.hint': '권장. 현재 사용자용으로 설치합니다.',
  'download.format.msi.label': 'MSI 패키지',
  'download.format.msi.hint': '관리형 설치와 자동 설치용입니다.',
  'download.format.dmg.label': '디스크 이미지',
  'download.format.dmg.hint': 'Homeport를 응용 프로그램 폴더로 드래그하세요.',
  'download.format.deb.label': 'Debian 패키지',
  'download.format.deb.hint': 'Debian, Ubuntu, Linux Mint, Pop!_OS.',
  'download.format.rpm.label': 'RPM 패키지',
  'download.format.rpm.hint': 'Fedora, RHEL, Rocky, openSUSE.',
  'download.format.appimage.label': 'AppImage',
  'download.format.appimage.hint': '모든 배포판에서 실행되며 설치 단계가 없습니다.',
  'download.files.file': '{format} ({file})',
  'download.files.checksums': 'SHA-256 체크섬',
  'download.files.none': '{link}에서 확인하세요.',
  'download.files.latest': 'GitHub의 최신 릴리스',

  // The Linux package repository.
  'download.repo.title': '빠른 설치',
  'download.repo.body':
    'Homeport 패키지 저장소를 추가하고 Homeport를 설치하므로, apt, dnf 또는 zypper로 최신 상태를 유지할 수 있습니다.',
  'download.repo.copy': '복사',
  'download.repo.copied': '복사됨',
  'download.repo.script': '원하면 먼저 {link} 내용을 읽어 보거나, 저장소를 직접 설정하세요:',
  'download.repo.scriptLink': '스크립트',
  'download.repo.manual': '수동 설정',
  'download.repo.apt': 'Debian 12, Ubuntu 22.04, Linux Mint 21, Pop!_OS 22.04 이상',
  'download.repo.dnf': 'Fedora 36, RHEL 10, Rocky Linux 및 AlmaLinux 10 이상',
  'download.repo.zypper': 'openSUSE Tumbleweed 및 Leap 16',
  'download.repo.packages':
    '또는 아래에서 패키지를 다운로드하세요. {deb} 및 {rpm} 패키지도 저장소를 추가하므로 업데이트를 받을 수 있습니다.',

  // What to know around the install.
  'download.before.title': '설치하기 전에',
  'download.before.body':
    'Homeport는 Docker Engine을 관리하지만 Docker Engine을 포함하지는 않습니다. Docker Engine, Docker Desktop, Colima, OrbStack, Rancher Desktop 또는 Podman과 함께 실행하거나, SSH, TLS 또는 TCP로 원격 서버에 연결하세요.',
  'download.before.keychain':
    'SSH 키, TLS 인증서, 스택 환경, 저장소의 배포 키와 토큰을 저장할 {keychain}(macOS 키체인, Windows 자격 증명 관리자 또는 GNOME Keyring 같은 Secret Service)',
  'download.before.keychainName': 'OS 키체인',
  'download.before.compose':
    '관리형 스택, 링크된 스택, 동기화 스택을 배포하기 위한 이 컴퓨터의 {tool}',
  'download.before.git': 'git 저장소에서 배포하는 동기화 스택을 위한 이 컴퓨터의 {tool}',
  'download.before.optional':
    '모두 선택 사항입니다. 없을 때 사용할 수 없는 기능은 Homeport가 알려 줍니다. 자세한 내용은 {link}에서 확인하세요.',
  'download.before.requirements': '시스템 요구 사항',
  'download.first.title': '처음 실행할 때',
  'download.first.body':
    '설치 프로그램은 아직 유료 인증서로 서명되지 않았으므로, 시스템에서 한 번 확인을 요청할 수 있습니다.',
  'download.first.windows':
    '{os}: SmartScreen에 “Windows의 PC 보호” 화면이 표시됩니다. {moreInfo} 링크를 선택한 다음 {runAnyway} 버튼을 선택하세요.',
  'download.first.moreInfo': '추가 정보',
  'download.first.runAnyway': '실행',
  'download.first.macos':
    '{os}: Homeport를 “열 수 없다”는 메시지가 표시되면 {settings}에서 허용하고 {openAnyway} 버튼을 선택하세요.',
  'download.first.settings': '시스템 설정 → 개인정보 보호 및 보안',
  'download.first.openAnyway': '그래도 열기',
  'download.first.link': '단계별 안내',
  'download.updates.title': '최신 상태 유지',
  'download.updates.body':
    'Windows, macOS 및 AppImage에서는 새 버전이 나오면 Homeport가 알려 주고, 원할 때 설치합니다. 모든 업데이트는 서명되어 있으며 설치하기 전에 확인합니다.',
  'download.updates.packages':
    '{deb} 및 {rpm} 패키지는 Homeport 패키지 저장소를 통해 시스템의 나머지 부분과 함께 업데이트됩니다.',
  'download.updates.link': '업데이트 방식',
  'download.verify.title': '다운로드 확인',
  'download.verify.body':
    '모든 릴리스는 공개 소스에서 GitHub Actions로 빌드되며, 파일마다 SHA-256 체크섬과 서명된 빌드 출처 증명이 함께 제공됩니다.',
  'download.verify.link': '확인 방법',

  // Other ways to get it.
  'download.other.title': 'Homeport를 받는 다른 방법',
  'download.other.previous.title': '이전 버전',
  'download.other.previous.body': 'GitHub에서 모든 릴리스를 노트, 파일과 함께 볼 수 있습니다.',
  'download.other.build.title': '소스에서 빌드',
  'download.other.build.body': 'Bun, Rust와 Tauri 사전 요구 사항이 필요합니다.',
  'download.other.source.title': '소스 코드',
  'download.other.source.body': 'AGPL에 따른 오픈 소스이며 GitHub에 있습니다.',
  'download.later.title': '추후 지원 예정',
  'download.later.homebrew': 'macOS의 brew install --cask homeport',
  'download.later.winget': 'Windows의 winget install',
  'download.later.scoop': 'Windows의 Homeport 버킷',
  'download.later.aur': 'Arch Linux의 homeport-bin',
  'download.later.flathub': '모든 Linux 배포판의 Flatpak',
  'download.later.snap': 'Ubuntu 등의 Snap Store',
} satisfies Translation<typeof en>;
