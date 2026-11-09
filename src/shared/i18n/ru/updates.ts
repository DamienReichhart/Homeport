/** Russian text of the `updates` namespace. */
import type en from '../en/updates';
import type { Translation } from '../types';

export default {
  'updates.title': 'Обновления',
  'updates.description':
    'Homeport может искать новую версию при запуске и затем каждые несколько часов.',
  'updates.automatic': 'Проверять обновления автоматически',
  'updates.checking': 'Проверка…',
  'updates.checkNow': 'Проверить сейчас',
  'updates.status.available': 'Доступен Homeport {version} (у вас {current}).',
  'updates.status.latest': 'Homeport {current} — последняя версия.',

  'updates.notice.label': 'Доступно обновление',
  'updates.notice.available': 'Доступен <strong>Homeport {version}</strong>.',
  'updates.notice.availablePackageManager':
    'Доступен <strong>Homeport {version}</strong>. Обновите его через менеджер пакетов:',
  'updates.notice.dismiss': 'Скрыть',

  'updates.copyCommand': 'Скопировать команду',
  'updates.whatsNew': 'Что нового',
  'updates.install': 'Установить и перезапустить',
  'updates.download': 'Скачать',
  'updates.progress.downloading': 'Скачивание…',
  'updates.progress.downloadingPercent': 'Скачивание… {percent}%',
  'updates.progress.installing': 'Установка…',

  'updates.method.inApp':
    'Обновления скачиваются, проверяются ключом подписи Homeport и устанавливаются, когда вы решите.',
  'updates.method.scoop': 'Установлено через Scoop: обновления приходят из bucket Homeport.',
  'updates.method.aur':
    'Установлено из AUR (homeport-bin): обновления устанавливает ваш AUR-помощник.',
  'updates.method.packageManager':
    'Установлено через {manager}: обновления приходят из репозитория пакетов Homeport.',
  'updates.method.manual': 'Это сборка для разработки: скачивайте новые версии с сайта.',
  'updates.palette.check': 'Проверить обновления',
} satisfies Translation<typeof en>;
