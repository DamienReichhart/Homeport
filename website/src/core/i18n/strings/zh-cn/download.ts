import type { Translation } from '../../types';
import type en from '../en/download';

/** The download page, and the download button other pages show, in Simplified Chinese. */
export default {
  'download.title': '下载 Homeport',
  'download.description':
    '下载适用于 Windows、macOS 和 Linux 的 Homeport：x64 和 ARM 安装程序以及 Linux 软件包仓库，附带校验和与构建来源证明。',

  // The download button and the line under it; the notes replace the default once the visitor's system is known.
  'download.button': '下载 Homeport',
  'download.button.for': '下载 {os} 版',
  'download.meta.version': '版本 {version}',
  'download.meta.everywhere': 'Windows、macOS 和 Linux',
  'download.meta.all': '所有下载',
  'download.note.windowsArm': 'Windows on ARM 安装程序',
  'download.note.windowsX64': 'Windows x64 安装程序',
  'download.note.macosArm': '适用于 Apple 芯片的 Mac',
  'download.note.macosX64': '适用于 Intel 芯片的 Mac',
  'download.note.macos': 'Apple 芯片和 Intel',
  'download.note.linux': '.deb、.rpm 和 AppImage',

  // The hero.
  'download.hero.eyebrow': '下载',
  'download.hero.title': '获取 Homeport',
  'download.hero.release': '版本 {version}，发布于 {date}。',
  'download.hero.whatsNew': '新内容',
  'download.hero.noRelease': '所有平台的安装程序都在{link}上。',
  'download.hero.releasesPage': 'GitHub 版本发布页面',
  'download.recommend': '你正在使用 {platform}。我们已将它的下载放在最前面。',
  'download.recommend.arch': '{arch} 上的 {os}',

  // The platforms and their installers.
  'download.preview': '{label}{name} 是用于测试的预发布版本。{link}。',
  'download.preview.label': '预览版可用：',
  'download.preview.link': '从 GitHub 下载',
  'download.windows.requirement': 'Windows 10 或 11',
  'download.macos.requirement': 'macOS 10.15 或更高版本',
  'download.linux.requirement': 'Ubuntu 22.04、Debian 12、Fedora 36 或更新版本',
  'download.platform.yours': '你的系统',
  'download.platform.guide': '{platform} 安装指南',
  'download.arch.appleSilicon': 'Apple 芯片',
  'download.arch.intel': 'Intel',
  'download.arch.arm64': 'ARM64',
  'download.arch.x64': 'x64',
  'download.format.nsis.label': '安装程序',
  'download.format.nsis.hint': '推荐。为当前用户安装。',
  'download.format.msi.label': 'MSI 安装包',
  'download.format.msi.hint': '适用于托管安装和静默安装。',
  'download.format.dmg.label': '磁盘映像',
  'download.format.dmg.hint': '将 Homeport 拖入“应用程序”。',
  'download.format.deb.label': 'Debian 软件包',
  'download.format.deb.hint': 'Debian、Ubuntu、Linux Mint、Pop!_OS。',
  'download.format.rpm.label': 'RPM 软件包',
  'download.format.rpm.hint': 'Fedora、RHEL、Rocky、openSUSE。',
  'download.format.appimage.label': 'AppImage',
  'download.format.appimage.hint': '适用于任何发行版，无需安装。',
  'download.files.file': '{format}（{file}）',
  'download.files.checksums': 'SHA-256 校验和',
  'download.files.none': '请参阅{link}。',
  'download.files.latest': 'GitHub 上的最新版本',

  // The Linux package repository.
  'download.repo.title': '快速安装',
  'download.repo.body':
    '添加 Homeport 软件包仓库并安装 Homeport，之后由 apt、dnf 或 zypper 让它保持最新。',
  'download.repo.copy': '复制',
  'download.repo.copied': '已复制',
  'download.repo.script': '如果愿意，可以先{link}，或者自行设置仓库：',
  'download.repo.scriptLink': '阅读脚本',
  'download.repo.manual': '手动设置',
  'download.repo.apt': 'Debian 12、Ubuntu 22.04、Linux Mint 21、Pop!_OS 22.04 或更新版本',
  'download.repo.dnf': 'Fedora 36、RHEL 10、Rocky Linux 和 AlmaLinux 10 或更新版本',
  'download.repo.zypper': 'openSUSE Tumbleweed 和 Leap 16',
  'download.repo.packages':
    '也可以在下方下载软件包。{deb} 和 {rpm} 软件包同样会添加该仓库，因此也能获得更新。',

  // What to know around the install.
  'download.before.title': '安装之前',
  'download.before.body':
    'Homeport 管理 Docker Engine，但本身不包含引擎。可将它与 Docker Engine、Docker Desktop、Colima、OrbStack、Rancher Desktop 或 Podman 一起运行，或让它通过 SSH、TLS 或 TCP 连接远程服务器。',
  'download.before.keychain':
    '{keychain}（macOS 钥匙串、Windows 凭据管理器，或 GNOME Keyring 等 Secret Service），用于存储 SSH 密钥、TLS 证书、堆栈环境变量，以及仓库的部署密钥和令牌。',
  'download.before.keychainName': '系统钥匙串',
  'download.before.compose': '本机上的 {tool}，用于部署托管堆栈、关联堆栈和同步堆栈。',
  'download.before.git': '本机上的 {tool}，用于从 git 仓库部署的同步堆栈。',
  'download.before.optional':
    '以上都是可选的：缺少它们时，Homeport 会告诉你哪些功能不可用。请参阅{link}。',
  'download.before.requirements': '系统要求',
  'download.first.title': '首次启动',
  'download.first.body': '安装程序尚未使用付费证书签名，因此系统可能会请你确认一次：',
  'download.first.windows':
    '{os}：SmartScreen 显示“Windows 已保护你的电脑”。选择{moreInfo}，然后选择{runAnyway}。',
  'download.first.moreInfo': '“更多信息”',
  'download.first.runAnyway': '“仍要运行”',
  'download.first.macos':
    '{os}：如果提示 Homeport“无法打开”，请在{settings}中允许它，然后选择{openAnyway}。',
  'download.first.settings': '“系统设置 → 隐私与安全性”',
  'download.first.openAnyway': '“仍要打开”',
  'download.first.link': '分步说明',
  'download.updates.title': '保持最新',
  'download.updates.body':
    '在 Windows、macOS 上以及使用 AppImage 时，Homeport 会在有新版本时告诉你，并在你选择时安装。每个更新在安装前都会经过签名验证。',
  'download.updates.packages':
    '{deb} 和 {rpm} 软件包会随系统其余部分一起，从 Homeport 软件包仓库更新。',
  'download.updates.link': '更新的工作原理',
  'download.verify.title': '验证你的下载',
  'download.verify.body':
    '每个版本都由 GitHub Actions 从公开源代码构建，每个文件都附带 SHA-256 校验和与已签名的构建来源证明。',
  'download.verify.link': '如何验证',

  // Other ways to get it.
  'download.other.title': '获取 Homeport 的其他方式',
  'download.other.previous.title': '历史版本',
  'download.other.previous.body': '所有版本及其说明和文件，都在 GitHub 上。',
  'download.other.build.title': '从源代码构建',
  'download.other.build.body': 'Bun、Rust 以及 Tauri 的前置依赖。',
  'download.other.source.title': '源代码',
  'download.other.source.body': '依据 AGPL 开源，托管在 GitHub 上。',
  'download.later.title': '即将推出',
  'download.later.homebrew': 'brew install --cask homeport，适用于 macOS',
  'download.later.winget': 'winget install，适用于 Windows',
  'download.later.scoop': 'Homeport 的 bucket，适用于 Windows',
  'download.later.aur': 'homeport-bin，适用于 Arch Linux',
  'download.later.flathub': 'Flatpak，适用于任何 Linux 发行版',
  'download.later.snap': 'Snap Store，适用于 Ubuntu 及其他发行版',
} satisfies Translation<typeof en>;
