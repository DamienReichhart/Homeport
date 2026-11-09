import type { Translation } from '../../types';
import type en from '../en/download';

/** The download page, and the download button other pages show, in Brazilian Portuguese. */
export default {
  'download.title': 'Baixar o Homeport',
  'download.description':
    'Baixe o Homeport para Windows, macOS e Linux — instaladores para x64 e ARM e repositórios de pacotes para Linux, com checksums e procedência da build.',

  // The download button and the line under it; the notes replace the default once the visitor's system is known.
  'download.button': 'Baixar o Homeport',
  'download.button.for': 'Baixar para {os}',
  'download.meta.version': 'Versão {version}',
  'download.meta.everywhere': 'Windows, macOS e Linux',
  'download.meta.all': 'Todos os downloads',
  'download.note.windowsArm': 'Instalador para Windows em ARM',
  'download.note.windowsX64': 'Instalador para Windows x64',
  'download.note.macosArm': 'Para Macs com Apple silicon',
  'download.note.macosX64': 'Para Macs com Intel',
  'download.note.macos': 'Apple silicon e Intel',
  'download.note.linux': '.deb, .rpm e AppImage',

  // The hero.
  'download.hero.eyebrow': 'Download',
  'download.hero.title': 'Obtenha o Homeport',
  'download.hero.release': 'Versão {version}, lançada em {date}.',
  'download.hero.whatsNew': 'Novidades',
  'download.hero.noRelease': 'Os instaladores para todas as plataformas estão na {link}.',
  'download.hero.releasesPage': 'página de versões do GitHub',
  'download.recommend':
    'Você está usando {platform}. Os downloads correspondentes aparecem primeiro.',
  'download.recommend.arch': '{os} em {arch}',

  // The platforms and their installers.
  'download.preview': '{label} {name} é uma versão de pré-lançamento para testes. {link}.',
  'download.preview.label': 'Prévia disponível:',
  'download.preview.link': 'Baixe-a no GitHub',
  'download.windows.requirement': 'Windows 10 ou 11',
  'download.macos.requirement': 'macOS 10.15 ou posterior',
  'download.linux.requirement': 'Ubuntu 22.04, Debian 12, Fedora 36 ou mais recente',
  'download.platform.yours': 'Seu sistema',
  'download.platform.guide': 'Guia de instalação para {platform}',
  'download.arch.appleSilicon': 'Apple silicon',
  'download.arch.intel': 'Intel',
  'download.arch.arm64': 'ARM64',
  'download.arch.x64': 'x64',
  'download.format.nsis.label': 'Instalador',
  'download.format.nsis.hint': 'Recomendado. Instala para o usuário atual.',
  'download.format.msi.label': 'Pacote MSI',
  'download.format.msi.hint': 'Para instalações gerenciadas e silenciosas.',
  'download.format.dmg.label': 'Imagem de disco',
  'download.format.dmg.hint': 'Arraste o Homeport para Aplicativos.',
  'download.format.deb.label': 'Pacote Debian',
  'download.format.deb.hint': 'Debian, Ubuntu, Linux Mint, Pop!_OS.',
  'download.format.rpm.label': 'Pacote RPM',
  'download.format.rpm.hint': 'Fedora, RHEL, Rocky, openSUSE.',
  'download.format.appimage.label': 'AppImage',
  'download.format.appimage.hint': 'Qualquer distribuição, sem etapa de instalação.',
  'download.files.file': '{format} ({file})',
  'download.files.checksums': 'Checksums SHA-256',
  'download.files.none': 'Veja a {link}.',
  'download.files.latest': 'versão mais recente no GitHub',

  // The Linux package repository.
  'download.repo.title': 'Instalação rápida',
  'download.repo.body':
    'Adiciona o repositório de pacotes do Homeport e instala o Homeport, para que o apt, o dnf ou o zypper o mantenha atualizado.',
  'download.repo.copy': 'Copiar',
  'download.repo.copied': 'Copiado',
  'download.repo.script': '{link} antes, se quiser, ou configure o repositório você mesmo:',
  'download.repo.scriptLink': 'Leia o script',
  'download.repo.manual': 'Configuração manual',
  'download.repo.apt': 'Debian 12, Ubuntu 22.04, Linux Mint 21, Pop!_OS 22.04 ou mais recente',
  'download.repo.dnf': 'Fedora 36, RHEL 10, Rocky Linux e AlmaLinux 10 ou mais recente',
  'download.repo.zypper': 'openSUSE Tumbleweed e Leap 16',
  'download.repo.packages':
    'Ou baixe um pacote abaixo. Os pacotes {deb} e {rpm} também adicionam o repositório, então recebem atualizações da mesma forma.',

  // What to know around the install.
  'download.before.title': 'Antes de instalar',
  'download.before.body':
    'O Homeport gerencia Docker Engines — ele não inclui um. Execute-o ao lado do Docker Engine, Docker Desktop, Colima, OrbStack, Rancher Desktop ou Podman, ou aponte-o para um servidor remoto por SSH, TLS ou TCP.',
  'download.before.keychain':
    '{keychain} (as Chaves do macOS, o Gerenciador de Credenciais do Windows ou um Secret Service como o GNOME Keyring) para armazenar chaves SSH, certificados TLS, ambientes de stacks e as chaves de deploy e tokens dos repositórios.',
  'download.before.keychainName': 'Chaveiro do sistema',
  'download.before.compose':
    '{tool} neste computador para implantar stacks gerenciadas, vinculadas e sincronizadas.',
  'download.before.git':
    '{tool} neste computador para stacks sincronizadas, implantadas a partir de um repositório git.',
  'download.before.optional':
    'Todos são opcionais: o Homeport avisa o que não fica disponível sem eles. Veja os {link}.',
  'download.before.requirements': 'requisitos do sistema',
  'download.first.title': 'Primeira execução',
  'download.first.body':
    'Os instaladores ainda não são assinados com um certificado pago, então seu sistema pode pedir uma confirmação uma vez:',
  'download.first.windows':
    '{os}: o SmartScreen mostra “O Windows protegeu o computador”. Escolha {moreInfo} e depois {runAnyway}.',
  'download.first.moreInfo': 'Mais informações',
  'download.first.runAnyway': 'Executar assim mesmo',
  'download.first.macos':
    '{os}: se o Homeport “não puder ser aberto”, permita-o em {settings} e escolha {openAnyway}.',
  'download.first.settings': 'Ajustes do Sistema → Privacidade e Segurança',
  'download.first.openAnyway': 'Abrir Mesmo Assim',
  'download.first.link': 'Instruções passo a passo',
  'download.updates.title': 'Mantendo-se atualizado',
  'download.updates.body':
    'No Windows, no macOS e com o AppImage, o Homeport avisa quando sai uma nova versão e a instala quando você escolher. Cada atualização é assinada e verificada antes de ser instalada.',
  'download.updates.packages':
    'Os pacotes {deb} e {rpm} são atualizados junto com o resto do seu sistema, a partir do repositório de pacotes do Homeport.',
  'download.updates.link': 'Como as atualizações funcionam',
  'download.verify.title': 'Verifique seu download',
  'download.verify.body':
    'Cada versão é gerada pelo GitHub Actions a partir do código-fonte público, com um checksum SHA-256 e um atestado assinado de procedência da build para cada arquivo.',
  'download.verify.link': 'Como verificar',

  // Other ways to get it.
  'download.other.title': 'Outras formas de obter o Homeport',
  'download.other.previous.title': 'Versões anteriores',
  'download.other.previous.body': 'Todas as versões, com notas e arquivos, no GitHub.',
  'download.other.build.title': 'Fazer build do código-fonte',
  'download.other.build.body': 'Bun, Rust e os pré-requisitos do Tauri.',
  'download.other.source.title': 'Código-fonte',
  'download.other.source.body': 'De código aberto sob a AGPL, no GitHub.',
  'download.later.title': 'Em breve',
  'download.later.homebrew': 'brew install --cask homeport, no macOS',
  'download.later.winget': 'winget install, no Windows',
  'download.later.scoop': 'um bucket do Homeport, no Windows',
  'download.later.aur': 'homeport-bin, no Arch Linux',
  'download.later.flathub': 'um Flatpak, em qualquer distribuição Linux',
  'download.later.snap': 'a Snap Store, no Ubuntu e em outras',
} satisfies Translation<typeof en>;
