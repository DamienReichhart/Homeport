import type { Translation } from '../../types';
import type en from '../en/download';

/** The download page, and the download button other pages show, in Arabic. Still in English: translate each value of ../en/download.ts. */
export default {
  'download.title': 'Download Homeport',
  'download.description':
    'Download Homeport for Windows, macOS and Linux — x64 and ARM installers and Linux package repositories, with checksums and build provenance.',

  // The download button and the line under it; the notes replace the default once the visitor's system is known.
  'download.button': 'Download Homeport',
  'download.button.for': 'Download for {os}',
  'download.meta.version': 'Version {version}',
  'download.meta.everywhere': 'Windows, macOS and Linux',
  'download.meta.all': 'All downloads',
  'download.note.windowsArm': 'Windows on ARM installer',
  'download.note.windowsX64': 'Windows x64 installer',
  'download.note.macosArm': 'For Apple silicon Macs',
  'download.note.macosX64': 'For Intel Macs',
  'download.note.macos': 'Apple silicon and Intel',
  'download.note.linux': '.deb, .rpm and AppImage',

  // The hero.
  'download.hero.eyebrow': 'Download',
  'download.hero.title': 'Get Homeport',
  'download.hero.release': 'Version {version}, released {date}.',
  'download.hero.whatsNew': "What's new",
  'download.hero.noRelease': 'Installers for every platform are on the {link}.',
  'download.hero.releasesPage': 'GitHub releases page',
  'download.recommend': "You're on {platform}. We've put its downloads first.",
  'download.recommend.arch': '{os} on {arch}',

  // The platforms and their installers.
  'download.preview': '{label} {name} is a prerelease for testing. {link}.',
  'download.preview.label': 'Preview available:',
  'download.preview.link': 'Download it from GitHub',
  'download.windows.requirement': 'Windows 10 or 11',
  'download.macos.requirement': 'macOS 10.15 or later',
  'download.linux.requirement': 'Ubuntu 22.04, Debian 12, Fedora 36 or newer',
  'download.platform.yours': 'Your system',
  'download.platform.guide': 'Installation guide for {platform}',
  'download.arch.appleSilicon': 'Apple silicon',
  'download.arch.intel': 'Intel',
  'download.arch.arm64': 'ARM64',
  'download.arch.x64': 'x64',
  'download.format.nsis.label': 'Installer',
  'download.format.nsis.hint': 'Recommended. Installs for the current user.',
  'download.format.msi.label': 'MSI package',
  'download.format.msi.hint': 'For managed and silent installs.',
  'download.format.dmg.label': 'Disk image',
  'download.format.dmg.hint': 'Drag Homeport into Applications.',
  'download.format.deb.label': 'Debian package',
  'download.format.deb.hint': 'Debian, Ubuntu, Linux Mint, Pop!_OS.',
  'download.format.rpm.label': 'RPM package',
  'download.format.rpm.hint': 'Fedora, RHEL, Rocky, openSUSE.',
  'download.format.appimage.label': 'AppImage',
  'download.format.appimage.hint': 'Any distribution, no install step.',
  'download.files.file': '{format} ({file})',
  'download.files.checksums': 'SHA-256 checksums',
  'download.files.none': 'See the {link}.',
  'download.files.latest': 'latest release on GitHub',

  // The Linux package repository.
  'download.repo.title': 'Quick install',
  'download.repo.body':
    'Adds the Homeport package repository and installs Homeport, so apt, dnf or zypper keeps it up to date.',
  'download.repo.copy': 'Copy',
  'download.repo.copied': 'Copied',
  'download.repo.script': '{link} first if you like, or set the repository up yourself:',
  'download.repo.scriptLink': 'Read the script',
  'download.repo.manual': 'Manual setup',
  'download.repo.apt': 'Debian 12, Ubuntu 22.04, Linux Mint 21, Pop!_OS 22.04 or newer',
  'download.repo.dnf': 'Fedora 36, RHEL 10, Rocky Linux and AlmaLinux 10 or newer',
  'download.repo.zypper': 'openSUSE Tumbleweed and Leap 16',
  'download.repo.packages':
    'Or download a package below. The {deb} and {rpm} add the repository too, so they get updates as well.',

  // What to know around the install.
  'download.before.title': 'Before you install',
  'download.before.body':
    "Homeport manages Docker Engines — it doesn't include one. Run it next to Docker Engine, Docker Desktop, Colima, OrbStack, Rancher Desktop or Podman, or point it at a remote server over SSH, TLS or TCP.",
  'download.before.keychain':
    "{keychain} (macOS Keychain, Windows Credential Manager, or a Secret Service such as GNOME Keyring) to store SSH keys, TLS certificates, Stack environments and repositories' deploy keys and tokens.",
  'download.before.keychainName': 'OS keychain',
  'download.before.compose': '{tool} on this machine to deploy Managed, Linked and Synced Stacks.',
  'download.before.git':
    '{tool} on this machine for Synced Stacks, deployed from a git repository.',
  'download.before.optional':
    'All are optional: Homeport tells you what is unavailable without them. See the {link}.',
  'download.before.requirements': 'system requirements',
  'download.first.title': 'First launch',
  'download.first.body':
    'The installers are not yet signed with a paid certificate, so your system may ask for confirmation once:',
  'download.first.windows':
    '{os}: SmartScreen shows “Windows protected your PC”. Choose {moreInfo}, then {runAnyway}.',
  'download.first.moreInfo': 'More info',
  'download.first.runAnyway': 'Run anyway',
  'download.first.macos':
    "{os}: if Homeport “can't be opened”, allow it in {settings} and choose {openAnyway}.",
  'download.first.settings': 'System Settings → Privacy & Security',
  'download.first.openAnyway': 'Open Anyway',
  'download.first.link': 'Step-by-step instructions',
  'download.updates.title': 'Staying up to date',
  'download.updates.body':
    "On Windows, macOS and with the AppImage, Homeport tells you when a new version is out and installs it when you choose. Every update is signed and checked before it's installed.",
  'download.updates.packages':
    'The {deb} and {rpm} packages update with the rest of your system, from the Homeport package repository.',
  'download.updates.link': 'How updates work',
  'download.verify.title': 'Verify your download',
  'download.verify.body':
    'Every release is built by GitHub Actions from the public source, with a SHA-256 checksum and a signed build provenance attestation for each file.',
  'download.verify.link': 'How to verify',

  // Other ways to get it.
  'download.other.title': 'Other ways to get Homeport',
  'download.other.previous.title': 'Previous versions',
  'download.other.previous.body': 'Every release, with notes and files, on GitHub.',
  'download.other.build.title': 'Build from source',
  'download.other.build.body': 'Bun, Rust and the Tauri prerequisites.',
  'download.other.source.title': 'Source code',
  'download.other.source.body': 'Open source under the AGPL, on GitHub.',
  'download.later.title': 'Coming later',
  'download.later.homebrew': 'brew install --cask homeport, on macOS',
  'download.later.winget': 'winget install, on Windows',
  'download.later.scoop': 'a Homeport bucket, on Windows',
  'download.later.aur': 'homeport-bin, on Arch Linux',
  'download.later.flathub': 'a Flatpak, on any Linux distribution',
  'download.later.snap': 'the Snap Store, on Ubuntu and others',
} satisfies Translation<typeof en>;
