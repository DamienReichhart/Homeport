import type { Translation } from '../../types';
import type en from '../en/download';

/** The download page, and the download button other pages show, in German. */
export default {
  'download.title': 'Homeport herunterladen',
  'download.description':
    'Homeport für Windows, macOS und Linux herunterladen – Installationsprogramme für x64 und ARM und Linux-Paketquellen, mit Prüfsummen und Build-Provenienz.',

  // The download button and the line under it; the notes replace the default once the visitor's system is known.
  'download.button': 'Homeport herunterladen',
  'download.button.for': 'Für {os} herunterladen',
  'download.meta.version': 'Version {version}',
  'download.meta.everywhere': 'Windows, macOS und Linux',
  'download.meta.all': 'Alle Downloads',
  'download.note.windowsArm': 'Installationsprogramm für Windows on ARM',
  'download.note.windowsX64': 'Installationsprogramm für Windows x64',
  'download.note.macosArm': 'Für Macs mit Apple Silicon',
  'download.note.macosX64': 'Für Macs mit Intel',
  'download.note.macos': 'Apple Silicon und Intel',
  'download.note.linux': '.deb, .rpm und AppImage',

  // The hero.
  'download.hero.eyebrow': 'Download',
  'download.hero.title': 'Homeport holen',
  'download.hero.release': 'Version {version}, erschienen am {date}.',
  'download.hero.whatsNew': 'Was ist neu',
  'download.hero.noRelease': 'Installationsprogramme für jede Plattform finden Sie auf der {link}.',
  'download.hero.releasesPage': 'Release-Seite auf GitHub',
  'download.recommend': 'Sie verwenden {platform}. Die passenden Downloads stehen ganz oben.',
  'download.recommend.arch': '{os} auf {arch}',

  // The platforms and their installers.
  'download.preview': '{label} {name} ist eine Vorabversion zum Testen. {link}.',
  'download.preview.label': 'Vorschau verfügbar:',
  'download.preview.link': 'Von GitHub herunterladen',
  'download.windows.requirement': 'Windows 10 oder 11',
  'download.macos.requirement': 'macOS 10.15 oder neuer',
  'download.linux.requirement': 'Ubuntu 22.04, Debian 12, Fedora 36 oder neuer',
  'download.platform.yours': 'Ihr System',
  'download.platform.guide': 'Installationsanleitung für {platform}',
  'download.arch.appleSilicon': 'Apple Silicon',
  'download.arch.intel': 'Intel',
  'download.arch.arm64': 'ARM64',
  'download.arch.x64': 'x64',
  'download.format.nsis.label': 'Installationsprogramm',
  'download.format.nsis.hint': 'Empfohlen. Installiert für den aktuellen Benutzer.',
  'download.format.msi.label': 'MSI-Paket',
  'download.format.msi.hint': 'Für verwaltete und unbeaufsichtigte Installationen.',
  'download.format.dmg.label': 'Disk-Image',
  'download.format.dmg.hint': 'Ziehen Sie Homeport in „Programme“.',
  'download.format.deb.label': 'Debian-Paket',
  'download.format.deb.hint': 'Debian, Ubuntu, Linux Mint, Pop!_OS.',
  'download.format.rpm.label': 'RPM-Paket',
  'download.format.rpm.hint': 'Fedora, RHEL, Rocky, openSUSE.',
  'download.format.appimage.label': 'AppImage',
  'download.format.appimage.hint': 'Jede Distribution, ohne Installationsschritt.',
  'download.files.file': '{format} ({file})',
  'download.files.checksums': 'SHA-256-Prüfsummen',
  'download.files.none': 'Siehe das {link}.',
  'download.files.latest': 'neueste Release auf GitHub',

  // The Linux package repository.
  'download.repo.title': 'Schnellinstallation',
  'download.repo.body':
    'Fügt die Paketquelle von Homeport hinzu und installiert Homeport, damit apt, dnf oder zypper es aktuell halten.',
  'download.repo.copy': 'Kopieren',
  'download.repo.copied': 'Kopiert',
  'download.repo.script': '{link}, wenn Sie möchten, oder richten Sie die Paketquelle selbst ein:',
  'download.repo.scriptLink': 'Lesen Sie zuerst das Skript',
  'download.repo.manual': 'Manuelle Einrichtung',
  'download.repo.apt': 'Debian 12, Ubuntu 22.04, Linux Mint 21, Pop!_OS 22.04 oder neuer',
  'download.repo.dnf': 'Fedora 36, RHEL 10, Rocky Linux und AlmaLinux 10 oder neuer',
  'download.repo.zypper': 'openSUSE Tumbleweed und Leap 16',
  'download.repo.packages':
    'Oder laden Sie unten ein Paket herunter. Die Pakete {deb} und {rpm} fügen die Paketquelle ebenfalls hinzu und erhalten so auch Updates.',

  // What to know around the install.
  'download.before.title': 'Vor der Installation',
  'download.before.body':
    'Homeport verwaltet Docker Engines – es enthält keine. Betreiben Sie es neben Docker Engine, Docker Desktop, Colima, OrbStack, Rancher Desktop oder Podman, oder verbinden Sie es über SSH, TLS oder TCP mit einem entfernten Server.',
  'download.before.keychain':
    '{keychain} (macOS-Schlüsselbund, Windows-Anmeldeinformationsverwaltung oder ein Secret Service wie GNOME Keyring), um SSH-Schlüssel, TLS-Zertifikate, Stack-Umgebungen sowie Deploy-Keys und Tokens von Repositorys zu speichern.',
  'download.before.keychainName': 'Schlüsselbund des Betriebssystems',
  'download.before.compose':
    '{tool} auf diesem Computer, um verwaltete, verknüpfte und synchronisierte Stacks bereitzustellen.',
  'download.before.git':
    '{tool} auf diesem Computer für synchronisierte Stacks, die aus einem Git-Repository bereitgestellt werden.',
  'download.before.optional':
    'Alles ist optional: Homeport sagt Ihnen, was ohne sie nicht verfügbar ist. Siehe die {link}.',
  'download.before.requirements': 'Systemvoraussetzungen',
  'download.first.title': 'Erster Start',
  'download.first.body':
    'Die Installationsprogramme sind noch nicht mit einem kostenpflichtigen Zertifikat signiert, daher fragt Ihr System möglicherweise einmal nach einer Bestätigung:',
  'download.first.windows':
    '{os}: SmartScreen zeigt „Der Computer wurde durch Windows geschützt“. Wählen Sie {moreInfo} und dann {runAnyway}.',
  'download.first.moreInfo': 'Weitere Informationen',
  'download.first.runAnyway': 'Trotzdem ausführen',
  'download.first.macos':
    '{os}: Wenn Homeport „nicht geöffnet werden kann“, erlauben Sie es unter {settings} und wählen Sie {openAnyway}.',
  'download.first.settings': 'Systemeinstellungen → Datenschutz & Sicherheit',
  'download.first.openAnyway': 'Dennoch öffnen',
  'download.first.link': 'Schritt-für-Schritt-Anleitung',
  'download.updates.title': 'Auf dem neuesten Stand bleiben',
  'download.updates.body':
    'Unter Windows, macOS und mit dem AppImage meldet Homeport, wenn eine neue Version erschienen ist, und installiert sie, wenn Sie es wählen. Jedes Update ist signiert und wird vor der Installation geprüft.',
  'download.updates.packages':
    'Die Pakete {deb} und {rpm} werden mit dem Rest Ihres Systems aus der Paketquelle von Homeport aktualisiert.',
  'download.updates.link': 'Wie Updates funktionieren',
  'download.verify.title': 'Download prüfen',
  'download.verify.body':
    'Jedes Release wird von GitHub Actions aus dem öffentlichen Quellcode gebaut, mit einer SHA-256-Prüfsumme und einer signierten Build-Provenienz-Attestierung für jede Datei.',
  'download.verify.link': 'Wie Sie prüfen',

  // Other ways to get it.
  'download.other.title': 'Andere Wege zu Homeport',
  'download.other.previous.title': 'Frühere Versionen',
  'download.other.previous.body': 'Jedes Release, mit Notizen und Dateien, auf GitHub.',
  'download.other.build.title': 'Aus dem Quellcode bauen',
  'download.other.build.body': 'Bun, Rust und die Voraussetzungen von Tauri.',
  'download.other.source.title': 'Quellcode',
  'download.other.source.body': 'Open Source unter der AGPL, auf GitHub.',
  'download.later.title': 'Später verfügbar',
  'download.later.homebrew': 'brew install --cask homeport, unter macOS',
  'download.later.winget': 'winget install, unter Windows',
  'download.later.scoop': 'ein Homeport-Bucket, unter Windows',
  'download.later.aur': 'homeport-bin, unter Arch Linux',
  'download.later.flathub': 'ein Flatpak, auf jeder Linux-Distribution',
  'download.later.snap': 'der Snap Store, unter Ubuntu und anderen',
} satisfies Translation<typeof en>;
