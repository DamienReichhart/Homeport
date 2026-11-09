/** Russian text of the `images` namespace. */
import type en from '../en/images';
import type { Translation } from '../types';

export default {
  'images.nav': 'Образы',
  'images.title': 'Образы',
  'images.summary':
    '{count, plural, one {# образ} few {# образа} many {# образов} other {# образа}} · {size}',
  'images.cleanUp': 'Очистить',
  'images.cleanUp.dangling': 'Висячие образы',
  'images.cleanUp.all': 'Все неиспользуемые образы',
  'images.saveSelected':
    '{count, plural, =1 {Сохранить образ…} one {Сохранить # образ…} few {Сохранить # образа…} many {Сохранить # образов…} other {Сохранить # образа…}}',
  'images.load': 'Загрузить…',
  'images.importTar': 'Импортировать из tar…',
  'images.build': 'Собрать',
  'images.pull': 'Скачать образ',
  'images.search': 'Поиск образов',
  'images.empty.title': 'Образов нет',
  'images.empty.description': 'Для начала скачайте образ из реестра.',

  'images.column.name': 'Имя',
  'images.column.id': 'ID',
  'images.column.created': 'Создан',
  'images.column.size': 'Размер',
  'images.column.status': 'Состояние',
  'images.moreTags':
    '{count, plural, one {ещё # тег} few {ещё # тега} many {ещё # тегов} other {ещё # тега}}',
  'images.inUse': 'Используется · {count}',
  'images.unused': 'Не используется',
  'images.runOne': 'Запустить {name}',
  'images.action.inspect': 'Просмотреть',
  'images.action.tag': 'Добавить тег',
  'images.action.push': 'Отправить',
  'images.action.save': 'Сохранить…',

  'images.remove.title': 'Удалить {name}?',
  'images.remove.everyName':
    'Будут удалены все имена образа ({names}) и его слои, которые больше ничем не используются.',
  'images.remove.description':
    'Образ и слои, которые не использует ни один другой образ, будут удалены. Позже его можно скачать снова.',
  'images.prune.allTitle': 'Удалить все неиспользуемые образы?',
  'images.prune.danglingTitle': 'Удалить висячие образы?',
  'images.prune.allDescription':
    'Удаляются все образы, которые не использует ни один контейнер, включая образы с тегами.',
  'images.prune.danglingDescription':
    'Удаляются образы без тегов, оставшиеся после сборок и скачиваний.',

  'images.removed': '{name} удалён',
  'images.pruned':
    'Удалено: {count, plural, one {# образ} few {# образа} many {# образов} other {# образа}}, освобождено {space}',
  'images.pulled': '{reference} скачан',
  'images.pushed': '{reference} отправлен',
  'images.built': '{name} собран',
  'images.builtUnnamed': 'Образ собран',
  'images.saved': '{path} сохранён',
  'images.loadedOne': '{name} загружен',
  'images.loaded':
    'Загружено: {count, plural, one {# образ} few {# образа} many {# образов} other {# образа}}',
  'images.imported': '{name} импортирован',

  'images.files.save': 'Сохранить образы как',
  'images.files.load': 'Загрузить образы из',
  'images.files.import': 'Импортировать файловую систему из',
  'images.files.imageArchives': 'Архивы образов',
  'images.files.imageArchive': 'Архив образа',
  'images.files.filesystemArchives': 'Архивы файловых систем',

  'images.buildDialog.title': 'Собрать образ',
  'images.buildDialog.description':
    'Из папки на этом компьютере. Homeport отправляет её на хост, без того, что исключает её .dockerignore, а собирает образ движок хоста.',
  'images.buildDialog.built': '✓ Собран',
  'images.buildDialog.folder': 'Папка',
  'images.buildDialog.chooseFolder': 'Выберите папку для сборки',
  'images.buildDialog.choose': 'Выбрать…',
  'images.buildDialog.dockerfile': 'Dockerfile',
  'images.buildDialog.target': 'Целевой этап',
  'images.buildDialog.targetPlaceholder': 'Последний этап',
  'images.buildDialog.names': 'Имена',
  'images.buildDialog.buildArgs': 'Аргументы сборки',
  'images.buildDialog.buildArgsPlaceholder': 'VERSION=1.0, по одному на строку',
  'images.buildDialog.noCache': 'Не использовать кеш сборки',
  'images.buildDialog.pull': 'Скачать новые версии базовых образов',
  'images.buildForm.noFolder': 'Выберите папку для сборки.',
  'images.buildForm.argNeedsName': 'В строке «{line}» нужно имя перед «=».',

  'images.hub.unavailable':
    'Поиск в Docker Hub сейчас недоступен: {error} Полное имя вроде nginx:1.27 по-прежнему скачивается.',
  'images.hub.noMatch': 'В Docker Hub ничего не найдено по запросу «{term}».',
  'images.hub.results': 'Результаты Docker Hub',
  'images.hub.result': '{name}, звёзд: {stars}',
  'images.hub.resultOfficial': '{name}, официальный образ, звёзд: {stars}',
  'images.hub.official': 'Официальный',
  'images.hub.backToSearch': 'Поиск',
  'images.hub.tagCount': '{count, plural, one {# тег} few {# тега} many {# тегов} other {# тега}}',
  'images.hub.filterTags': 'Фильтр тегов',
  'images.hub.tags': 'Теги',
  'images.hub.noTagMatch': 'Нет тегов, подходящих под «{filter}».',
  'images.hub.moreTags': 'Ещё теги',

  'images.importDialog.title': 'Импортировать файловую систему как образ',
  'images.importDialog.description':
    '{path} станет однослойным образом без собственной истории и параметров; чтобы его можно было запустить, задайте ниже команду.',
  'images.importDialog.name': 'Имя образа',
  'images.importDialog.changes': 'Изменения',
  'images.importDialog.changesHint':
    'Инструкции Dockerfile, по одной на строку: CMD, ENTRYPOINT, ENV, EXPOSE, LABEL, ONBUILD, USER, VOLUME или WORKDIR.',
  'images.importDialog.import': 'Импортировать',

  'images.inspect.layers': 'Слои',
  'images.inspect.raw': 'Исходный JSON',
  'images.inspect.created': 'Создан',
  'images.inspect.instruction': 'Инструкция',
  'images.inspect.size': 'Размер',

  'images.pullDialog.title': 'Скачать образ',
  'images.pullDialog.description':
    'Ищите в Docker Hub по имени или введите любую ссылку, доступную хосту, например nginx:1.27 или ghcr.io/org/app:v2.',
  'images.pullDialog.placeholder': 'postgres или nginx:1.27',
  'images.pullDialog.reference': 'Ссылка на образ',
  'images.pullDialog.pull': 'Скачать',
  'images.done': 'Готово.',

  'images.pushDialog.title': 'Отправить {name}',
  'images.pushDialog.description':
    'В реестр, с которого начинается его имя, например ghcr.io/org/app:v2, или в Docker Hub для имён вида org/app:v2.',
  'images.pushDialog.target': 'Имя для отправки',
  'images.pushDialog.push': 'Отправить',
  'images.pushDialog.tagsFirst': 'Сначала образу присваивается это имя, как при добавлении тега.',
  'images.pushDialog.uploading': 'Отправка {current} / {total}',
  'images.push.pushed':
    'Отправлено: {pushed, plural, one {# слой} few {# слоя} many {# слоёв} other {# слоя}}',
  'images.push.pushedAndExisting':
    'Отправлено: {pushed, plural, one {# слой} few {# слоя} many {# слоёв} other {# слоя}}, уже в реестре: {existing}',

  'images.tagDialog.title': 'Добавить тег {name}',
  'images.tagDialog.newName': 'Новое имя',
  'images.tagDialog.add': 'Добавить тег',

  'images.transfer.savingOne': 'Сохранение {name}',
  'images.transfer.saving':
    'Сохранение: {count, plural, one {# образ} few {# образа} many {# образов} other {# образа}}',
  'images.transfer.to': 'В {path}',
  'images.transfer.from': 'Из {path}',
  'images.transfer.written': 'Записано',
  'images.transfer.sent': 'Отправлено',
  'images.transfer.loading': 'Загрузка образов',
  'images.transfer.importing': 'Импорт {reference}',
  'images.transfer.doneWith': 'Готово: {images}',
  'images.palette.pull': 'Скачать образ',
} satisfies Translation<typeof en>;
