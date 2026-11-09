import type { Translation } from '../../types';
import type en from '../en/download';

/** The download page, and the download button other pages show, in Japanese. */
export default {
  'download.title': 'Homeport をダウンロード',
  'download.description':
    'Windows、macOS、Linux 用の Homeport をダウンロードできます。x64 と ARM 用のインストーラーと Linux のパッケージリポジトリを、チェックサムとビルドの来歴付きで提供しています。',

  // The download button and the line under it; the notes replace the default once the visitor's system is known.
  'download.button': 'Homeport をダウンロード',
  'download.button.for': '{os} 版をダウンロード',
  'download.meta.version': 'バージョン {version}',
  'download.meta.everywhere': 'Windows、macOS、Linux',
  'download.meta.all': 'すべてのダウンロード',
  'download.note.windowsArm': 'Windows on ARM 用インストーラー',
  'download.note.windowsX64': 'Windows x64 用インストーラー',
  'download.note.macosArm': 'Apple シリコン搭載の Mac 用',
  'download.note.macosX64': 'Intel 搭載の Mac 用',
  'download.note.macos': 'Apple シリコンおよび Intel',
  'download.note.linux': '.deb、.rpm、AppImage',

  // The hero.
  'download.hero.eyebrow': 'ダウンロード',
  'download.hero.title': 'Homeport を入手',
  'download.hero.release': 'バージョン {version}（{date} リリース）。',
  'download.hero.whatsNew': '新機能',
  'download.hero.noRelease': 'すべてのプラットフォーム用のインストーラーは {link} にあります。',
  'download.hero.releasesPage': 'GitHub のリリースページ',
  'download.recommend':
    'お使いのシステムは {platform} です。そのダウンロードを先頭に表示しています。',
  'download.recommend.arch': '{arch} の {os}',

  // The platforms and their installers.
  'download.preview': '{label} {name} はテスト用のプレリリースです。{link}。',
  'download.preview.label': 'プレビュー版あり：',
  'download.preview.link': 'GitHub からダウンロード',
  'download.windows.requirement': 'Windows 10 または 11',
  'download.macos.requirement': 'macOS 10.15 以降',
  'download.linux.requirement': 'Ubuntu 22.04、Debian 12、Fedora 36 以降',
  'download.platform.yours': 'お使いのシステム',
  'download.platform.guide': '{platform} のインストールガイド',
  'download.arch.appleSilicon': 'Apple シリコン',
  'download.arch.intel': 'Intel',
  'download.arch.arm64': 'ARM64',
  'download.arch.x64': 'x64',
  'download.format.nsis.label': 'インストーラー',
  'download.format.nsis.hint': 'おすすめ。現在のユーザー用にインストールします。',
  'download.format.msi.label': 'MSI パッケージ',
  'download.format.msi.hint': '管理されたインストールやサイレントインストール向け。',
  'download.format.dmg.label': 'ディスクイメージ',
  'download.format.dmg.hint': 'Homeport を「アプリケーション」にドラッグします。',
  'download.format.deb.label': 'Debian パッケージ',
  'download.format.deb.hint': 'Debian、Ubuntu、Linux Mint、Pop!_OS。',
  'download.format.rpm.label': 'RPM パッケージ',
  'download.format.rpm.hint': 'Fedora、RHEL、Rocky、openSUSE。',
  'download.format.appimage.label': 'AppImage',
  'download.format.appimage.hint': 'どのディストリビューションでも、インストール不要。',
  'download.files.file': '{format}（{file}）',
  'download.files.checksums': 'SHA-256 チェックサム',
  'download.files.none': '{link}を参照してください。',
  'download.files.latest': 'GitHub の最新リリース',

  // The Linux package repository.
  'download.repo.title': 'クイックインストール',
  'download.repo.body':
    'Homeport のパッケージリポジトリを追加して Homeport をインストールします。以後は apt、dnf、zypper が最新の状態に保ちます。',
  'download.repo.copy': 'コピー',
  'download.repo.copied': 'コピーしました',
  'download.repo.script': '必要なら先に{link}か、リポジトリを自分で設定してください：',
  'download.repo.scriptLink': 'スクリプトを読む',
  'download.repo.manual': '手動での設定',
  'download.repo.apt': 'Debian 12、Ubuntu 22.04、Linux Mint 21、Pop!_OS 22.04 以降',
  'download.repo.dnf': 'Fedora 36、RHEL 10、Rocky Linux、AlmaLinux 10 以降',
  'download.repo.zypper': 'openSUSE Tumbleweed および Leap 16',
  'download.repo.packages':
    'または、以下からパッケージをダウンロードしてください。{deb} と {rpm} もリポジトリを追加するため、同じようにアップデートを受け取れます。',

  // What to know around the install.
  'download.before.title': 'インストールの前に',
  'download.before.body':
    'Homeport は Docker Engine を管理するアプリで、Docker Engine 自体は含まれていません。Docker Engine、Docker Desktop、Colima、OrbStack、Rancher Desktop、Podman と一緒に使うか、SSH、TLS、TCP でリモートのサーバーに接続してください。',
  'download.before.keychain':
    'SSH キー、TLS 証明書、スタックの環境変数、リポジトリのデプロイキーとトークンを保存するための{keychain}（macOS のキーチェーン、Windows の資格情報マネージャー、または GNOME Keyring などの Secret Service）。',
  'download.before.keychainName': 'OS のキーチェーン',
  'download.before.compose':
    '管理スタック、リンクスタック、同期スタックをデプロイするための、このマシンの {tool}。',
  'download.before.git':
    'git リポジトリからデプロイする同期スタックのための、このマシンの {tool}。',
  'download.before.optional':
    'いずれも必須ではありません。足りないときは、何が使えないかを Homeport が知らせます。{link}を参照してください。',
  'download.before.requirements': 'システム要件',
  'download.first.title': '初回起動',
  'download.first.body':
    'インストーラーはまだ有料の証明書で署名されていないため、システムが一度だけ確認を求めることがあります：',
  'download.first.windows':
    '{os}：SmartScreen に「Windows によって PC が保護されました」と表示されます。「{moreInfo}」を選び、次に「{runAnyway}」を選んでください。',
  'download.first.moreInfo': '詳細情報',
  'download.first.runAnyway': '実行',
  'download.first.macos':
    '{os}：Homeport を「開けません」と表示された場合は、{settings} で許可し、「{openAnyway}」を選んでください。',
  'download.first.settings': '「システム設定」→「プライバシーとセキュリティ」',
  'download.first.openAnyway': 'このまま開く',
  'download.first.link': '詳しい手順',
  'download.updates.title': '最新の状態に保つ',
  'download.updates.body':
    'Windows、macOS、AppImage では、新しいバージョンが出ると Homeport が知らせ、選んだときにインストールします。すべてのアップデートは署名されており、インストール前に検証されます。',
  'download.updates.packages':
    '{deb} と {rpm} のパッケージは、Homeport のパッケージリポジトリから、システムのほかの部分と一緒にアップデートされます。',
  'download.updates.link': 'アップデートのしくみ',
  'download.verify.title': 'ダウンロードを検証',
  'download.verify.body':
    'すべてのリリースは公開されたソースから GitHub Actions でビルドされ、ファイルごとに SHA-256 チェックサムと署名付きのビルド来歴の証明が付いています。',
  'download.verify.link': '検証の方法',

  // Other ways to get it.
  'download.other.title': 'Homeport を入手するほかの方法',
  'download.other.previous.title': '以前のバージョン',
  'download.other.previous.body': 'すべてのリリースを、ノートとファイルとともに GitHub で。',
  'download.other.build.title': 'ソースからビルド',
  'download.other.build.body': 'Bun、Rust、Tauri の前提条件。',
  'download.other.source.title': 'ソースコード',
  'download.other.source.body': 'AGPL のもとでオープンソースとして GitHub で公開。',
  'download.later.title': '今後の予定',
  'download.later.homebrew': 'macOS で brew install --cask homeport',
  'download.later.winget': 'Windows で winget install',
  'download.later.scoop': 'Windows で Homeport の bucket',
  'download.later.aur': 'Arch Linux で homeport-bin',
  'download.later.flathub': 'どの Linux ディストリビューションでも Flatpak',
  'download.later.snap': 'Ubuntu などで Snap Store',
} satisfies Translation<typeof en>;
