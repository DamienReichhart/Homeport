import type { Translation } from '../../types';
import type en from '../en/download';

/** The download page, and the download button other pages show, in Russian. */
export default {
  'download.title': 'Скачать Homeport',
  'download.description':
    'Скачайте Homeport для Windows, macOS и Linux: установщики для x64 и ARM и репозитории пакетов для Linux, с контрольными суммами и подтверждением происхождения сборки.',

  // The download button and the line under it; the notes replace the default once the visitor's system is known.
  'download.button': 'Скачать Homeport',
  'download.button.for': 'Скачать для {os}',
  'download.meta.version': 'Версия {version}',
  'download.meta.everywhere': 'Windows, macOS и Linux',
  'download.meta.all': 'Все загрузки',
  'download.note.windowsArm': 'Установщик для Windows на ARM',
  'download.note.windowsX64': 'Установщик для Windows x64',
  'download.note.macosArm': 'Для Mac с Apple silicon',
  'download.note.macosX64': 'Для Mac с Intel',
  'download.note.macos': 'Apple silicon и Intel',
  'download.note.linux': '.deb, .rpm и AppImage',

  // The hero.
  'download.hero.eyebrow': 'Скачать',
  'download.hero.title': 'Скачайте Homeport',
  'download.hero.release': 'Версия {version}, выпущена {date}.',
  'download.hero.whatsNew': 'Что нового',
  'download.hero.noRelease': 'Установщики для всех платформ — на {link}.',
  'download.hero.releasesPage': 'странице выпусков на GitHub',
  'download.recommend': 'У вас {platform}. Мы показали загрузки для этой системы первыми.',
  'download.recommend.arch': '{os} на {arch}',

  // The platforms and their installers.
  'download.preview': '{label} {name} — предварительная версия для тестирования. {link}.',
  'download.preview.label': 'Доступна предварительная версия:',
  'download.preview.link': 'Скачайте её с GitHub',
  'download.windows.requirement': 'Windows 10 или 11',
  'download.macos.requirement': 'macOS 10.15 или новее',
  'download.linux.requirement': 'Ubuntu 22.04, Debian 12, Fedora 36 или новее',
  'download.platform.yours': 'Ваша система',
  'download.platform.guide': 'Руководство по установке для {platform}',
  'download.arch.appleSilicon': 'Apple silicon',
  'download.arch.intel': 'Intel',
  'download.arch.arm64': 'ARM64',
  'download.arch.x64': 'x64',
  'download.format.nsis.label': 'Установщик',
  'download.format.nsis.hint': 'Рекомендуется. Устанавливает для текущего пользователя.',
  'download.format.msi.label': 'Пакет MSI',
  'download.format.msi.hint': 'Для централизованной и тихой установки.',
  'download.format.dmg.label': 'Образ диска',
  'download.format.dmg.hint': 'Перетащите Homeport в «Программы».',
  'download.format.deb.label': 'Пакет Debian',
  'download.format.deb.hint': 'Debian, Ubuntu, Linux Mint, Pop!_OS.',
  'download.format.rpm.label': 'Пакет RPM',
  'download.format.rpm.hint': 'Fedora, RHEL, Rocky, openSUSE.',
  'download.format.appimage.label': 'AppImage',
  'download.format.appimage.hint': 'Любой дистрибутив, без установки.',
  'download.files.file': '{format} ({file})',
  'download.files.checksums': 'Контрольные суммы SHA-256',
  'download.files.none': 'См. {link}.',
  'download.files.latest': 'последний выпуск на GitHub',

  // The Linux package repository.
  'download.repo.title': 'Быстрая установка',
  'download.repo.body':
    'Добавляет репозиторий пакетов Homeport и устанавливает Homeport, чтобы apt, dnf или zypper поддерживали его в актуальном состоянии.',
  'download.repo.copy': 'Копировать',
  'download.repo.copied': 'Скопировано',
  'download.repo.script': '{link}, если хотите, или настройте репозиторий сами:',
  'download.repo.scriptLink': 'Сначала прочитайте скрипт',
  'download.repo.manual': 'Ручная настройка',
  'download.repo.apt': 'Debian 12, Ubuntu 22.04, Linux Mint 21, Pop!_OS 22.04 или новее',
  'download.repo.dnf': 'Fedora 36, RHEL 10, Rocky Linux и AlmaLinux 10 или новее',
  'download.repo.zypper': 'openSUSE Tumbleweed и Leap 16',
  'download.repo.packages':
    'Или скачайте пакет ниже. Пакеты {deb} и {rpm} тоже добавляют репозиторий, поэтому они тоже получают обновления.',

  // What to know around the install.
  'download.before.title': 'Перед установкой',
  'download.before.body':
    'Homeport управляет Docker Engine, но не содержит его. Запускайте Homeport рядом с Docker Engine, Docker Desktop, Colima, OrbStack, Rancher Desktop или Podman либо подключайте его к удалённому серверу по SSH, TLS или TCP.',
  'download.before.keychain':
    '{keychain} (Связка ключей macOS, Диспетчер учётных данных Windows или Secret Service, например GNOME Keyring) для хранения ключей SSH, сертификатов TLS, окружений стеков, а также ключей развёртывания и токенов репозиториев.',
  'download.before.keychainName': 'Связка ключей ОС',
  'download.before.compose':
    '{tool} на этом компьютере для развёртывания управляемых, связанных и синхронизируемых стеков.',
  'download.before.git':
    '{tool} на этом компьютере для синхронизируемых стеков, которые разворачиваются из git-репозитория.',
  'download.before.optional':
    'Всё это необязательно: Homeport сообщит, что без них недоступно. См. {link}.',
  'download.before.requirements': 'системные требования',
  'download.first.title': 'Первый запуск',
  'download.first.body':
    'Установщики пока не подписаны платным сертификатом, поэтому система может один раз попросить подтверждение:',
  'download.first.windows':
    '{os}: SmartScreen показывает «Система Windows защитила ваш компьютер». Выберите {moreInfo}, затем {runAnyway}.',
  'download.first.moreInfo': 'Подробнее',
  'download.first.runAnyway': 'Выполнить в любом случае',
  'download.first.macos':
    '{os}: если Homeport «не удаётся открыть», разрешите его в разделе {settings} и выберите {openAnyway}.',
  'download.first.settings': 'Системные настройки → Конфиденциальность и безопасность',
  'download.first.openAnyway': 'Всё равно открыть',
  'download.first.link': 'Пошаговые инструкции',
  'download.updates.title': 'Обновления',
  'download.updates.body':
    'В Windows, macOS и с AppImage Homeport сообщает о выходе новой версии и устанавливает её, когда вы решите. Каждое обновление подписано и проверяется перед установкой.',
  'download.updates.packages':
    'Пакеты {deb} и {rpm} обновляются вместе с остальной системой из репозитория пакетов Homeport.',
  'download.updates.link': 'Как работают обновления',
  'download.verify.title': 'Проверьте загруженный файл',
  'download.verify.body':
    'Каждый выпуск собирается в GitHub Actions из открытого исходного кода, с контрольной суммой SHA-256 и подписанным подтверждением происхождения сборки для каждого файла.',
  'download.verify.link': 'Как проверить',

  // Other ways to get it.
  'download.other.title': 'Другие способы получить Homeport',
  'download.other.previous.title': 'Предыдущие версии',
  'download.other.previous.body': 'Все выпуски с примечаниями и файлами — на GitHub.',
  'download.other.build.title': 'Сборка из исходного кода',
  'download.other.build.body': 'Bun, Rust и зависимости Tauri.',
  'download.other.source.title': 'Исходный код',
  'download.other.source.body': 'Открытый исходный код под лицензией AGPL, на GitHub.',
  'download.later.title': 'Появится позже',
  'download.later.homebrew': 'brew install --cask homeport в macOS',
  'download.later.winget': 'winget install в Windows',
  'download.later.scoop': 'bucket Homeport в Windows',
  'download.later.aur': 'homeport-bin в Arch Linux',
  'download.later.flathub': 'Flatpak для любого дистрибутива Linux',
  'download.later.snap': 'Snap Store в Ubuntu и других дистрибутивах',
} satisfies Translation<typeof en>;
