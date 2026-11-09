/** Russian text of the `quadlets` namespace. */
import type en from '../en/quadlets';
import type { Translation } from '../types';

export default {
  'quadlets.nav': 'Quadlet',
  'quadlets.title': 'Юниты Quadlet',
  'quadlets.summary': '{count, plural, one {# юнит} few {# юнита} many {# юнитов} other {# юнита}}',
  'quadlets.search': 'Поиск юнитов',
  'quadlets.empty': 'Нет юнитов Quadlet',
  'quadlets.emptyDescription':
    'Юниты Quadlet — это файлы, которые Podman превращает в сервисы systemd, чтобы контейнеры запускались вместе с хостом, а systemd поддерживал их работу. Установите юнит, чтобы увидеть его здесь.',
  'quadlets.new': 'Новый юнит',
  'quadlets.edit': 'Изменить',
  'quadlets.notGenerated': 'ещё не сгенерирован',
  'quadlets.gone': '{name} больше не установлен',
  'quadlets.column.name': 'Юнит',
  'quadlets.column.kind': 'Вид',
  'quadlets.column.service': 'Сервис',
  'quadlets.column.status': 'Состояние',
  'quadlets.column.containers': 'Контейнеры',
  'quadlets.status.running': 'Работает',
  'quadlets.status.starting': 'Запускается',
  'quadlets.status.stopping': 'Останавливается',
  'quadlets.status.stopped': 'Остановлен',
  'quadlets.status.failed': 'Сбой',
  'quadlets.status.notLoaded': 'Не загружен systemd',
  'quadlets.status.template': 'Шаблон',
  'quadlets.action.start': 'Запустить',
  'quadlets.action.stop': 'Остановить',
  'quadlets.action.restart': 'Перезапустить',
  'quadlets.actionOne.start': 'Запустить {name}',
  'quadlets.actionOne.stop': 'Остановить {name}',
  'quadlets.actionOne.restart': 'Перезапустить {name}',
  'quadlets.done.start': '{name} запущен',
  'quadlets.done.stop': '{name} остановлен',
  'quadlets.done.restart': '{name} перезапущен',
  'quadlets.installed': '{name} установлен',
  'quadlets.saved': '{name} сохранён',
  'quadlets.removed': '{name} удалён',
  'quadlets.blocked.noCommands':
    'Homeport запускает и останавливает юниты через systemctl, а выполнить его он может только по SSH или для Podman на этом компьютере.',
  'quadlets.blocked.notLoaded':
    'systemd не сгенерировал сервис этого юнита, поэтому запускать нечего.',
  'quadlets.blocked.podmanTooOld':
    'Для чтения, установки, изменения и удаления юнитов нужен Podman 5.8 или новее; на этом хосте работает Podman {version}.',
  'quadlets.blocked.noFile':
    'Этот юнит известен только по его контейнерам, поэтому его файл изменить нельзя.',
  'quadlets.note.labels':
    'Podman {version} не умеет перечислять юниты Quadlet, поэтому они взяты из созданных ими контейнеров. Их файлы и юниты, не создавшие ни одного контейнера, видны в Podman 5.7 и новее.',
  'quadlets.note.readOnly':
    'Podman {version} перечисляет юниты, но не умеет читать, устанавливать, изменять или удалять их; для этого нужен Podman 5.8 или новее.',
  'quadlets.note.noCommands':
    'Чтобы запускать и останавливать юниты, к этому хосту нужно подключаться по SSH или использовать Podman на этом компьютере.',
  'quadlets.detail.service': 'Сервис',
  'quadlets.detail.unit': 'Юнит systemd',
  'quadlets.detail.scope': 'Запускается как',
  'quadlets.detail.path': 'Файл',
  'quadlets.detail.app': 'Приложение',
  'quadlets.detail.pod': 'Под',
  'quadlets.detail.containers':
    '{count, plural, =0 {Контейнеры} one {# контейнер} few {# контейнера} many {# контейнеров} other {# контейнера}}',
  'quadlets.detail.noContainers': 'Он не создал ни одного контейнера, или его сервис остановлен.',
  'quadlets.scope.user': 'Пользовательский юнит (systemctl --user)',
  'quadlets.scope.system': 'Системный юнит',
  'quadlets.file.title': 'Файл юнита',
  'quadlets.file.unknown': 'Этот юнит известен только по метке его контейнеров.',
  'quadlets.file.tooOld':
    'Podman {version} не умеет читать файлы юнитов; чтобы их показать, нужен Podman 5.8 или новее.',
  'quadlets.form.newTitle': 'Новый юнит Quadlet',
  'quadlets.form.newDescription':
    'Podman запишет файл туда, где лежат юниты его пользователя, и перезагрузит systemd. Запустите юнит после установки.',
  'quadlets.form.editTitle': 'Изменить {name}',
  'quadlets.form.editDescription':
    'При сохранении файл заменяется, а systemd перезагружается; перезапустите юнит, чтобы работающий сервис использовал новый файл. Файлы, установленные рядом с ним, остаются.',
  'quadlets.form.name': 'Имя файла',
  'quadlets.form.kind': 'Вид',
  'quadlets.form.openFile': 'Открыть файл…',
  'quadlets.form.openTitle': 'Открыть файл юнита Quadlet',
  'quadlets.form.unitFiles': 'Файлы юнитов Quadlet',
  'quadlets.form.content': 'Файл юнита',
  'quadlets.form.extra': 'Файлы рядом с ним',
  'quadlets.form.extraHint':
    'Containerfile, kube YAML или env-файл, которые использует юнит; устанавливаются в папку его приложения.',
  'quadlets.form.extraTitle': 'Выберите файлы для установки рядом с юнитом',
  'quadlets.form.addExtra': 'Добавить файлы…',
  'quadlets.form.removeExtra': 'Удалить {name}',
  'quadlets.form.install': 'Установить',
  'quadlets.form.save': 'Сохранить',
  'quadlets.form.nameInvalid':
    'Назовите его, например, web.container, с окончанием .container, .pod, .volume, .network, .kube, .image, .build или .artifact.',
  'quadlets.form.sectionMissing': 'Чтобы Quadlet мог прочитать файл, в нём нужна секция {section}.',
  'quadlets.form.extraInvalid':
    'Каждому файлу рядом с юнитом нужно собственное простое имя, без папок.',
  'quadlets.remove.title': 'Удалить {name}?',
  'quadlets.remove.description':
    'Podman удалит файл юнита и перезагрузит systemd. Созданные им контейнеры, тома и образы останутся.',
  'quadlets.remove.stop': 'Остановить его сервис, если он работает',
  'quadlets.remove.stopRunning': 'Сначала остановить его работающий сервис',
} satisfies Translation<typeof en>;
