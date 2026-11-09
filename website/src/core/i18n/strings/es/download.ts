import type { Translation } from '../../types';
import type en from '../en/download';

/** The download page, and the download button other pages show, in Spanish. */
export default {
  'download.title': 'Descargar Homeport',
  'download.description':
    'Descarga Homeport para Windows, macOS y Linux: instaladores para x64 y ARM y repositorios de paquetes para Linux, con sumas de comprobación y procedencia de compilación.',

  // The download button and the line under it; the notes replace the default once the visitor's system is known.
  'download.button': 'Descargar Homeport',
  'download.button.for': 'Descargar para {os}',
  'download.meta.version': 'Versión {version}',
  'download.meta.everywhere': 'Windows, macOS y Linux',
  'download.meta.all': 'Todas las descargas',
  'download.note.windowsArm': 'Instalador para Windows en ARM',
  'download.note.windowsX64': 'Instalador para Windows x64',
  'download.note.macosArm': 'Para Mac con Apple silicon',
  'download.note.macosX64': 'Para Mac con Intel',
  'download.note.macos': 'Apple silicon e Intel',
  'download.note.linux': '.deb, .rpm y AppImage',

  // The hero.
  'download.hero.eyebrow': 'Descargar',
  'download.hero.title': 'Consigue Homeport',
  'download.hero.release': 'Versión {version}, publicada el {date}.',
  'download.hero.whatsNew': 'Novedades',
  'download.hero.noRelease': 'Los instaladores para todas las plataformas están en la {link}.',
  'download.hero.releasesPage': 'página de versiones de GitHub',
  'download.recommend': 'Usas {platform}. Hemos puesto primero sus descargas.',
  'download.recommend.arch': '{os} en {arch}',

  // The platforms and their installers.
  'download.preview': '{label} {name} es una versión preliminar para pruebas. {link}.',
  'download.preview.label': 'Vista previa disponible:',
  'download.preview.link': 'Descárgala de GitHub',
  'download.windows.requirement': 'Windows 10 u 11',
  'download.macos.requirement': 'macOS 10.15 o posterior',
  'download.linux.requirement': 'Ubuntu 22.04, Debian 12, Fedora 36 o posterior',
  'download.platform.yours': 'Tu sistema',
  'download.platform.guide': 'Guía de instalación para {platform}',
  'download.arch.appleSilicon': 'Apple silicon',
  'download.arch.intel': 'Intel',
  'download.arch.arm64': 'ARM64',
  'download.arch.x64': 'x64',
  'download.format.nsis.label': 'Instalador',
  'download.format.nsis.hint': 'Recomendado. Instala para el usuario actual.',
  'download.format.msi.label': 'Paquete MSI',
  'download.format.msi.hint': 'Para instalaciones gestionadas y silenciosas.',
  'download.format.dmg.label': 'Imagen de disco',
  'download.format.dmg.hint': 'Arrastra Homeport a Aplicaciones.',
  'download.format.deb.label': 'Paquete Debian',
  'download.format.deb.hint': 'Debian, Ubuntu, Linux Mint, Pop!_OS.',
  'download.format.rpm.label': 'Paquete RPM',
  'download.format.rpm.hint': 'Fedora, RHEL, Rocky, openSUSE.',
  'download.format.appimage.label': 'AppImage',
  'download.format.appimage.hint': 'Cualquier distribución, sin paso de instalación.',
  'download.files.file': '{format} ({file})',
  'download.files.checksums': 'Sumas de comprobación SHA-256',
  'download.files.none': 'Consulta la {link}.',
  'download.files.latest': 'última versión en GitHub',

  // The Linux package repository.
  'download.repo.title': 'Instalación rápida',
  'download.repo.body':
    'Añade el repositorio de paquetes de Homeport e instala Homeport, para que apt, dnf o zypper lo mantenga actualizado.',
  'download.repo.copy': 'Copiar',
  'download.repo.copied': 'Copiado',
  'download.repo.script': '{link} antes si quieres, o configura el repositorio tú mismo:',
  'download.repo.scriptLink': 'Lee el script',
  'download.repo.manual': 'Configuración manual',
  'download.repo.apt': 'Debian 12, Ubuntu 22.04, Linux Mint 21, Pop!_OS 22.04 o posterior',
  'download.repo.dnf': 'Fedora 36, RHEL 10, Rocky Linux y AlmaLinux 10 o posterior',
  'download.repo.zypper': 'openSUSE Tumbleweed y Leap 16',
  'download.repo.packages':
    'O descarga un paquete abajo. Los paquetes {deb} y {rpm} también añaden el repositorio, así que también reciben actualizaciones.',

  // What to know around the install.
  'download.before.title': 'Antes de instalar',
  'download.before.body':
    'Homeport gestiona Docker Engines, pero no incluye ninguno. Ejecútalo junto a Docker Engine, Docker Desktop, Colima, OrbStack, Rancher Desktop o Podman, o apúntalo a un servidor remoto por SSH, TLS o TCP.',
  'download.before.keychain':
    '{keychain} (el Llavero de macOS, el Administrador de credenciales de Windows o un Secret Service como GNOME Keyring) para guardar claves SSH, certificados TLS, entornos de stacks y las claves de despliegue y los tokens de los repositorios.',
  'download.before.keychainName': 'Llavero del sistema',
  'download.before.compose':
    '{tool} en este equipo para desplegar stacks gestionados, vinculados y sincronizados.',
  'download.before.git':
    '{tool} en este equipo para los stacks sincronizados, desplegados desde un repositorio git.',
  'download.before.optional':
    'Todos son opcionales: Homeport te indica qué no está disponible sin ellos. Consulta los {link}.',
  'download.before.requirements': 'requisitos del sistema',
  'download.first.title': 'Primer inicio',
  'download.first.body':
    'Los instaladores todavía no están firmados con un certificado de pago, así que puede que tu sistema pida confirmación una vez:',
  'download.first.windows':
    '{os}: SmartScreen muestra «Windows protegió su PC». Elige {moreInfo} y luego {runAnyway}.',
  'download.first.moreInfo': 'Más información',
  'download.first.runAnyway': 'Ejecutar de todas formas',
  'download.first.macos':
    '{os}: si Homeport «no se puede abrir», permítelo en {settings} y elige {openAnyway}.',
  'download.first.settings': 'Ajustes del Sistema → Privacidad y seguridad',
  'download.first.openAnyway': 'Abrir igualmente',
  'download.first.link': 'Instrucciones paso a paso',
  'download.updates.title': 'Mantenerse al día',
  'download.updates.body':
    'En Windows, en macOS y con la AppImage, Homeport te avisa cuando sale una versión nueva y la instala cuando tú decidas. Cada actualización está firmada y se comprueba antes de instalarse.',
  'download.updates.packages':
    'Los paquetes {deb} y {rpm} se actualizan con el resto del sistema, desde el repositorio de paquetes de Homeport.',
  'download.updates.link': 'Cómo funcionan las actualizaciones',
  'download.verify.title': 'Verifica tu descarga',
  'download.verify.body':
    'Cada versión la compila GitHub Actions a partir del código fuente público, con una suma de comprobación SHA-256 y una atestación de procedencia de compilación firmada para cada archivo.',
  'download.verify.link': 'Cómo verificar',

  // Other ways to get it.
  'download.other.title': 'Otras formas de conseguir Homeport',
  'download.other.previous.title': 'Versiones anteriores',
  'download.other.previous.body': 'Todas las versiones, con notas y archivos, en GitHub.',
  'download.other.build.title': 'Compilar desde el código fuente',
  'download.other.build.body': 'Bun, Rust y los requisitos previos de Tauri.',
  'download.other.source.title': 'Código fuente',
  'download.other.source.body': 'De código abierto bajo la AGPL, en GitHub.',
  'download.later.title': 'Más adelante',
  'download.later.homebrew': 'brew install --cask homeport, en macOS',
  'download.later.winget': 'winget install, en Windows',
  'download.later.scoop': 'un bucket de Homeport, en Windows',
  'download.later.aur': 'homeport-bin, en Arch Linux',
  'download.later.flathub': 'un Flatpak, en cualquier distribución de Linux',
  'download.later.snap': 'la Snap Store, en Ubuntu y otras',
} satisfies Translation<typeof en>;
