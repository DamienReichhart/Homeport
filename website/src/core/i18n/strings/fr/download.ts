import type { Translation } from '../../types';
import type en from '../en/download';

/** The download page, and the download button other pages show, in French. */
export default {
  'download.title': 'Télécharger Homeport',
  'download.description':
    'Téléchargez Homeport pour Windows, macOS et Linux — des installeurs x64 et ARM et des dépôts de paquets Linux, avec des sommes de contrôle et la provenance du build.',

  // The download button and the line under it; the notes replace the default once the visitor's system is known.
  'download.button': 'Télécharger Homeport',
  'download.button.for': 'Télécharger pour {os}',
  'download.meta.version': 'Version {version}',
  'download.meta.everywhere': 'Windows, macOS et Linux',
  'download.meta.all': 'Tous les téléchargements',
  'download.note.windowsArm': 'Installeur pour Windows sur ARM',
  'download.note.windowsX64': 'Installeur pour Windows x64',
  'download.note.macosArm': 'Pour les Mac Apple silicon',
  'download.note.macosX64': 'Pour les Mac Intel',
  'download.note.macos': 'Apple silicon et Intel',
  'download.note.linux': '.deb, .rpm et AppImage',

  // The hero.
  'download.hero.eyebrow': 'Télécharger',
  'download.hero.title': 'Obtenir Homeport',
  'download.hero.release': 'Version {version}, publiée le {date}.',
  'download.hero.whatsNew': 'Nouveautés',
  'download.hero.noRelease': 'Les installeurs pour chaque plateforme se trouvent sur la {link}.',
  'download.hero.releasesPage': 'page des versions sur GitHub',
  'download.recommend': 'Vous êtes sous {platform}. Ses téléchargements sont affichés en premier.',
  'download.recommend.arch': '{os} sur {arch}',

  // The platforms and their installers.
  'download.preview': '{label} {name} est une préversion destinée aux tests. {link}.',
  'download.preview.label': 'Préversion disponible :',
  'download.preview.link': 'Téléchargez-la depuis GitHub',
  'download.windows.requirement': 'Windows 10 ou 11',
  'download.macos.requirement': 'macOS 10.15 ou ultérieur',
  'download.linux.requirement': 'Ubuntu 22.04, Debian 12, Fedora 36 ou plus récent',
  'download.platform.yours': 'Votre système',
  'download.platform.guide': 'Guide d’installation pour {platform}',
  'download.arch.appleSilicon': 'Apple silicon',
  'download.arch.intel': 'Intel',
  'download.arch.arm64': 'ARM64',
  'download.arch.x64': 'x64',
  'download.format.nsis.label': 'Installeur',
  'download.format.nsis.hint': 'Recommandé. Installe pour l’utilisateur actuel.',
  'download.format.msi.label': 'Paquet MSI',
  'download.format.msi.hint': 'Pour les installations gérées et silencieuses.',
  'download.format.dmg.label': 'Image disque',
  'download.format.dmg.hint': 'Faites glisser Homeport dans Applications.',
  'download.format.deb.label': 'Paquet Debian',
  'download.format.deb.hint': 'Debian, Ubuntu, Linux Mint, Pop!_OS.',
  'download.format.rpm.label': 'Paquet RPM',
  'download.format.rpm.hint': 'Fedora, RHEL, Rocky, openSUSE.',
  'download.format.appimage.label': 'AppImage',
  'download.format.appimage.hint': 'Toute distribution, sans étape d’installation.',
  'download.files.file': '{format} ({file})',
  'download.files.checksums': 'Sommes de contrôle SHA-256',
  'download.files.none': 'Consultez la {link}.',
  'download.files.latest': 'dernière version sur GitHub',

  // The Linux package repository.
  'download.repo.title': 'Installation rapide',
  'download.repo.body':
    'Ajoute le dépôt de paquets de Homeport et installe Homeport, pour qu’apt, dnf ou zypper le maintienne à jour.',
  'download.repo.copy': 'Copier',
  'download.repo.copied': 'Copié',
  'download.repo.script': '{link} d’abord si vous le souhaitez, ou configurez le dépôt vous-même :',
  'download.repo.scriptLink': 'Lisez le script',
  'download.repo.manual': 'Configuration manuelle',
  'download.repo.apt': 'Debian 12, Ubuntu 22.04, Linux Mint 21, Pop!_OS 22.04 ou plus récent',
  'download.repo.dnf': 'Fedora 36, RHEL 10, Rocky Linux et AlmaLinux 10 ou plus récent',
  'download.repo.zypper': 'openSUSE Tumbleweed et Leap 16',
  'download.repo.packages':
    'Ou téléchargez un paquet ci-dessous. Les paquets {deb} et {rpm} ajoutent eux aussi le dépôt, et reçoivent donc également les mises à jour.',

  // What to know around the install.
  'download.before.title': 'Avant d’installer',
  'download.before.body':
    'Homeport gère des Docker Engines — il n’en inclut pas. Exécutez-le à côté de Docker Engine, Docker Desktop, Colima, OrbStack, Rancher Desktop ou Podman, ou pointez-le vers un serveur distant via SSH, TLS ou TCP.',
  'download.before.keychain':
    '{keychain} (le trousseau de macOS, le Gestionnaire d’identification de Windows, ou un Secret Service comme GNOME Keyring) pour stocker les clés SSH, les certificats TLS, les environnements de stacks ainsi que les clés de déploiement et les jetons des dépôts.',
  'download.before.keychainName': 'Le trousseau du système',
  'download.before.compose':
    '{tool} sur cette machine pour déployer des stacks gérées, liées et synchronisées.',
  'download.before.git':
    '{tool} sur cette machine pour les stacks synchronisées, déployées depuis un dépôt git.',
  'download.before.optional':
    'Tous sont facultatifs : Homeport vous indique ce qui n’est pas disponible sans eux. Consultez la {link}.',
  'download.before.requirements': 'configuration requise',
  'download.first.title': 'Premier lancement',
  'download.first.body':
    'Les installeurs ne sont pas encore signés avec un certificat payant, votre système peut donc demander une confirmation une fois :',
  'download.first.windows':
    '{os} : SmartScreen affiche « Windows a protégé votre ordinateur ». Choisissez {moreInfo}, puis {runAnyway}.',
  'download.first.moreInfo': 'Informations complémentaires',
  'download.first.runAnyway': 'Exécuter quand même',
  'download.first.macos':
    '{os} : si Homeport « ne peut pas être ouvert », autorisez-le dans {settings} et choisissez {openAnyway}.',
  'download.first.settings': 'Réglages Système → Confidentialité et sécurité',
  'download.first.openAnyway': 'Ouvrir quand même',
  'download.first.link': 'Instructions pas à pas',
  'download.updates.title': 'Rester à jour',
  'download.updates.body':
    'Sous Windows, sous macOS et avec l’AppImage, Homeport vous prévient quand une nouvelle version est disponible et l’installe quand vous le décidez. Chaque mise à jour est signée et vérifiée avant d’être installée.',
  'download.updates.packages':
    'Les paquets {deb} et {rpm} se mettent à jour avec le reste de votre système, depuis le dépôt de paquets de Homeport.',
  'download.updates.link': 'Comment fonctionnent les mises à jour',
  'download.verify.title': 'Vérifier votre téléchargement',
  'download.verify.body':
    'Chaque version est buildée par GitHub Actions à partir du code source public, avec une somme de contrôle SHA-256 et une attestation de provenance du build signée pour chaque fichier.',
  'download.verify.link': 'Comment vérifier',

  // Other ways to get it.
  'download.other.title': 'Autres façons d’obtenir Homeport',
  'download.other.previous.title': 'Versions précédentes',
  'download.other.previous.body': 'Chaque version, avec ses notes et ses fichiers, sur GitHub.',
  'download.other.build.title': 'Compiler depuis le code source',
  'download.other.build.body': 'Bun, Rust et les prérequis de Tauri.',
  'download.other.source.title': 'Code source',
  'download.other.source.body': 'Open source sous licence AGPL, sur GitHub.',
  'download.later.title': 'À venir',
  'download.later.homebrew': 'brew install --cask homeport, sur macOS',
  'download.later.winget': 'winget install, sur Windows',
  'download.later.scoop': 'un bucket Homeport, sur Windows',
  'download.later.aur': 'homeport-bin, sur Arch Linux',
  'download.later.flathub': 'un Flatpak, sur toute distribution Linux',
  'download.later.snap': 'le Snap Store, sur Ubuntu et d’autres',
} satisfies Translation<typeof en>;
