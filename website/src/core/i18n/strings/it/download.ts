import type { Translation } from '../../types';
import type en from '../en/download';

/** The download page, and the download button other pages show, in Italian. */
export default {
  'download.title': 'Scarica Homeport',
  'download.description':
    'Scarica Homeport per Windows, macOS e Linux — programmi di installazione per x64 e ARM e repository di pacchetti per Linux, con checksum e provenienza della build.',

  // The download button and the line under it; the notes replace the default once the visitor's system is known.
  'download.button': 'Scarica Homeport',
  'download.button.for': 'Scarica per {os}',
  'download.meta.version': 'Versione {version}',
  'download.meta.everywhere': 'Windows, macOS e Linux',
  'download.meta.all': 'Tutti i download',
  'download.note.windowsArm': 'Programma di installazione per Windows su ARM',
  'download.note.windowsX64': 'Programma di installazione per Windows x64',
  'download.note.macosArm': 'Per Mac con Apple silicon',
  'download.note.macosX64': 'Per Mac con Intel',
  'download.note.macos': 'Apple silicon e Intel',
  'download.note.linux': '.deb, .rpm e AppImage',

  // The hero.
  'download.hero.eyebrow': 'Download',
  'download.hero.title': 'Scarica Homeport',
  'download.hero.release': 'Versione {version}, pubblicata il {date}.',
  'download.hero.whatsNew': 'Novità',
  'download.hero.noRelease': 'I programmi di installazione per ogni piattaforma sono nella {link}.',
  'download.hero.releasesPage': 'pagina delle release su GitHub',
  'download.recommend': 'Stai usando {platform}. Abbiamo messo per primi i suoi download.',
  'download.recommend.arch': '{os} su {arch}',

  // The platforms and their installers.
  'download.preview': '{label} {name} è una versione preliminare da provare. {link}.',
  'download.preview.label': 'Anteprima disponibile:',
  'download.preview.link': 'Scaricala da GitHub',
  'download.windows.requirement': 'Windows 10 o 11',
  'download.macos.requirement': 'macOS 10.15 o successivo',
  'download.linux.requirement': 'Ubuntu 22.04, Debian 12, Fedora 36 o più recenti',
  'download.platform.yours': 'Il tuo sistema',
  'download.platform.guide': 'Guida all’installazione per {platform}',
  'download.arch.appleSilicon': 'Apple silicon',
  'download.arch.intel': 'Intel',
  'download.arch.arm64': 'ARM64',
  'download.arch.x64': 'x64',
  'download.format.nsis.label': 'Programma di installazione',
  'download.format.nsis.hint': 'Consigliato. Installa per l’utente corrente.',
  'download.format.msi.label': 'Pacchetto MSI',
  'download.format.msi.hint': 'Per installazioni gestite e silenziose.',
  'download.format.dmg.label': 'Immagine disco',
  'download.format.dmg.hint': 'Trascina Homeport in Applicazioni.',
  'download.format.deb.label': 'Pacchetto Debian',
  'download.format.deb.hint': 'Debian, Ubuntu, Linux Mint, Pop!_OS.',
  'download.format.rpm.label': 'Pacchetto RPM',
  'download.format.rpm.hint': 'Fedora, RHEL, Rocky, openSUSE.',
  'download.format.appimage.label': 'AppImage',
  'download.format.appimage.hint': 'Qualsiasi distribuzione, nessuna installazione.',
  'download.files.file': '{format} ({file})',
  'download.files.checksums': 'Checksum SHA-256',
  'download.files.none': 'Vedi l’{link}.',
  'download.files.latest': 'ultima release su GitHub',

  // The Linux package repository.
  'download.repo.title': 'Installazione rapida',
  'download.repo.body':
    'Aggiunge il repository di pacchetti di Homeport e installa Homeport, così apt, dnf o zypper lo mantengono aggiornato.',
  'download.repo.copy': 'Copia',
  'download.repo.copied': 'Copiato',
  'download.repo.script': '{link} prima, se vuoi, oppure configura tu il repository:',
  'download.repo.scriptLink': 'Leggi lo script',
  'download.repo.manual': 'Configurazione manuale',
  'download.repo.apt': 'Debian 12, Ubuntu 22.04, Linux Mint 21, Pop!_OS 22.04 o più recenti',
  'download.repo.dnf': 'Fedora 36, RHEL 10, Rocky Linux e AlmaLinux 10 o più recenti',
  'download.repo.zypper': 'openSUSE Tumbleweed e Leap 16',
  'download.repo.packages':
    'Oppure scarica un pacchetto qui sotto. Anche i pacchetti {deb} e {rpm} aggiungono il repository, quindi ricevono anch’essi gli aggiornamenti.',

  // What to know around the install.
  'download.before.title': 'Prima di installare',
  'download.before.body':
    'Homeport gestisce i Docker Engine, ma non ne include uno. Usalo accanto a Docker Engine, Docker Desktop, Colima, OrbStack, Rancher Desktop o Podman, oppure puntalo su un server remoto tramite SSH, TLS o TCP.',
  'download.before.keychain':
    '{keychain} (Portachiavi di macOS, Gestione credenziali di Windows o un Secret Service come GNOME Keyring) per archiviare chiavi SSH, certificati TLS, ambienti degli stack e chiavi di deploy e token dei repository.',
  'download.before.keychainName': 'Portachiavi del sistema',
  'download.before.compose':
    '{tool} su questo computer per distribuire stack gestiti, collegati e sincronizzati.',
  'download.before.git':
    '{tool} su questo computer per gli stack sincronizzati, distribuiti da un repository git.',
  'download.before.optional':
    'Sono tutti facoltativi: Homeport ti dice cosa non è disponibile senza di loro. Vedi i {link}.',
  'download.before.requirements': 'requisiti di sistema',
  'download.first.title': 'Primo avvio',
  'download.first.body':
    'I programmi di installazione non sono ancora firmati con un certificato a pagamento, quindi il tuo sistema potrebbe chiedere una conferma la prima volta:',
  'download.first.windows':
    '{os}: SmartScreen mostra «PC protetto da Windows». Scegli {moreInfo}, poi {runAnyway}.',
  'download.first.moreInfo': 'Ulteriori informazioni',
  'download.first.runAnyway': 'Esegui comunque',
  'download.first.macos':
    '{os}: se Homeport «non può essere aperto», consentilo in {settings} e scegli {openAnyway}.',
  'download.first.settings': 'Impostazioni di Sistema → Privacy e sicurezza',
  'download.first.openAnyway': 'Apri comunque',
  'download.first.link': 'Istruzioni passo passo',
  'download.updates.title': 'Restare aggiornati',
  'download.updates.body':
    'Su Windows, su macOS e con l’AppImage, Homeport ti avvisa quando esce una nuova versione e la installa quando lo decidi tu. Ogni aggiornamento è firmato e verificato prima di essere installato.',
  'download.updates.packages':
    'I pacchetti {deb} e {rpm} si aggiornano insieme al resto del sistema, dal repository di pacchetti di Homeport.',
  'download.updates.link': 'Come funzionano gli aggiornamenti',
  'download.verify.title': 'Verifica il download',
  'download.verify.body':
    'Ogni release è compilata da GitHub Actions a partire dal codice sorgente pubblico, con un checksum SHA-256 e un’attestazione firmata della provenienza della build per ogni file.',
  'download.verify.link': 'Come verificare',

  // Other ways to get it.
  'download.other.title': 'Altri modi per ottenere Homeport',
  'download.other.previous.title': 'Versioni precedenti',
  'download.other.previous.body': 'Ogni release, con note e file, su GitHub.',
  'download.other.build.title': 'Compila dal codice sorgente',
  'download.other.build.body': 'Bun, Rust e i prerequisiti di Tauri.',
  'download.other.source.title': 'Codice sorgente',
  'download.other.source.body': 'Open source con licenza AGPL, su GitHub.',
  'download.later.title': 'Più avanti',
  'download.later.homebrew': 'brew install --cask homeport, su macOS',
  'download.later.winget': 'winget install, su Windows',
  'download.later.scoop': 'un bucket di Homeport, su Windows',
  'download.later.aur': 'homeport-bin, su Arch Linux',
  'download.later.flathub': 'un Flatpak, su qualsiasi distribuzione Linux',
  'download.later.snap': 'lo Snap Store, su Ubuntu e altre',
} satisfies Translation<typeof en>;
