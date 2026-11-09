/** Russian text of the `clusters` namespace. */
import type en from '../en/clusters';
import type { Translation } from '../types';

export default {
  'clusters.title': 'Кластеры',
  'clusters.summary':
    '{count, plural, one {# кластер} few {# кластера} many {# кластеров} other {# кластера}}',
  'clusters.add': 'Добавить кластер',
  'clusters.notFound': 'Этого кластера больше не существует.',
  'clusters.empty.title': 'Кластеров пока нет',
  'clusters.empty.description':
    'Добавьте кластеры Kubernetes, до которых доходит ваш kubeconfig. Homeport ничего на них не устанавливает.',
  'clusters.column.name': 'Имя',
  'clusters.column.status': 'Состояние',
  'clusters.column.kind': 'Вид',
  'clusters.column.server': 'API',
  'clusters.column.auth': 'Вход через',
  'clusters.palette.add': 'Добавить кластер',
  'clusters.palette.manage': 'Управление кластерами',

  'clusters.auth.none': 'Без учётных данных',
  'clusters.auth.clientCertificate': 'Клиентский сертификат',
  'clusters.auth.token': 'Токен',
  'clusters.auth.basic': 'Имя пользователя и пароль',
  'clusters.auth.exec': 'Плагин exec ({command})',
  'clusters.auth.oidc': 'OpenID Connect',
  'clusters.auth.aclToken': 'Токен ACL',

  'clusters.addPage.description':
    'Каждый выбранный вами контекст Kubernetes или каждый указанный агент Nomad становится кластером. Его учётные данные хранятся в зашифрованном виде и никогда не лежат на диске открытым текстом.',
  'clusters.addPage.fromMachine': 'Из kubeconfig этого компьютера',
  'clusters.addPage.paste': 'Вставить kubeconfig',
  'clusters.addPage.nomad': 'Nomad',
  'clusters.paste.description':
    'Kubeconfig читается в памяти и никогда не сохраняется в файл. Пути к файлам, на которые он ссылается, должны быть полными.',
  'clusters.paste.label': 'Kubeconfig',
  'clusters.paste.read': 'Прочитать контексты',

  'clusters.import.readFrom': 'Прочитано из {paths}',
  'clusters.import.noContexts': 'В этом kubeconfig нет контекстов',
  'clusters.import.pick': 'Импортировать {context}',
  'clusters.import.name': 'Имя кластера, созданного из {context}',
  'clusters.import.current': 'текущий',
  'clusters.import.added': 'Уже добавлен как {name}.',
  'clusters.import.refresh': 'Обновить его',
  'clusters.import.submit':
    '{count, plural, =0 {Импортировать} one {Импортировать # кластер} few {Импортировать # кластера} many {Импортировать # кластеров} other {Импортировать # кластера}}',
  'clusters.import.done':
    '{count, plural, one {Добавлен # кластер} few {Добавлено # кластера} many {Добавлено # кластеров} other {Добавлено # кластера}}',
  'clusters.import.problem.empty': 'Дайте имя каждому кластеру.',
  'clusters.import.problem.duplicate': 'Два кластера получили бы имя «{name}».',
  'clusters.import.problem.taken': 'Кластер с именем «{name}» уже существует.',

  'clusters.detail.enabled': 'Включён',
  'clusters.detail.reconnect': 'Переподключиться',
  'clusters.detail.name': 'Имя',
  'clusters.detail.rename': 'Переименовать',
  'clusters.detail.renamed': 'Кластер переименован',
  'clusters.connection.title': 'Подключение',
  'clusters.connection.server': 'Сервер API',
  'clusters.connection.address': 'Адрес',
  'clusters.connection.region': 'Регион',
  'clusters.connection.customCa': 'Собственный сертификат ЦС',
  'clusters.connection.clientCertificate': 'Клиентский сертификат',
  'clusters.connection.checkedAs': 'Сертификат проверяется как {name}',
  'clusters.connection.version': 'Версия',
  'clusters.connection.platform': 'Платформа',
  'clusters.connection.namespace': 'Пространство имён по умолчанию',
  'clusters.connection.auth': 'Вход через',
  'clusters.connection.tlsServerName': 'Имя в сертификате',
  'clusters.connection.tls': 'TLS',
  'clusters.connection.insecure': 'Сертификат сервера не проверяется (insecure-skip-tls-verify)',
  'clusters.connection.proxy': 'Прокси',
  'clusters.connection.source': 'Импортирован из',
  'clusters.source.machine': 'Контекст {context} из kubeconfig этого компьютера',
  'clusters.source.pasted': 'Контекст {context} из вставленного kubeconfig',

  'clusters.nomad.description':
    'Homeport обращается к HTTP API агента Nomad, сервера или клиента, с этого компьютера. На кластер ничего не устанавливается, и CLI nomad здесь не нужен.',
  'clusters.nomad.name': 'Имя',
  'clusters.nomad.address': 'Адрес',
  'clusters.nomad.addressHelp':
    'HTTP API агента, как его передаёт CLI nomad переменная NOMAD_ADDR. Если порт не указан, используется 4646.',
  'clusters.nomad.region': 'Регион',
  'clusters.nomad.regionHelp': 'Оставьте пустым, чтобы использовать собственный регион агента.',
  'clusters.nomad.token': 'Токен ACL',
  'clusters.nomad.tokenHelp':
    'Secret ID токена, если в кластере включены ACL. В документации Homeport есть политика для него.',
  'clusters.nomad.tls': 'Параметры TLS',
  'clusters.nomad.tlsHelp':
    'Только для агента, который отдаёт HTTPS со своим ЦС или запрашивает клиентский сертификат. Вставьте содержимое PEM-файлов.',
  'clusters.nomad.ca': 'Сертификат ЦС',
  'clusters.nomad.caHelp':
    'ЦС, подписавший сертификат агента, как его указывает NOMAD_CACERT. ЦС этого компьютера считаются доверенными и без него.',
  'clusters.nomad.clientCertificate': 'Клиентский сертификат',
  'clusters.nomad.clientKey': 'Клиентский ключ',
  'clusters.nomad.tlsServerName': 'Имя сервера TLS',
  'clusters.nomad.tlsServerNameHelp':
    'Имя, с которым сверяется сертификат агента, если оно отличается от имени в адресе, как его задаёт NOMAD_TLS_SERVER_NAME.',
  'clusters.nomad.keep': 'Сохранено; оставьте пустым, чтобы не менять',
  'clusters.nomad.willBeRemoved': 'Будет удалено при сохранении',
  'clusters.nomad.remove': 'Удалить',
  'clusters.nomad.keepIt': 'Оставить',
  'clusters.nomad.test': 'Проверить подключение',
  'clusters.nomad.testing': 'Проверка…',
  'clusters.nomad.testedVersion': 'Подключено к Nomad {version}, регион {region}.',
  'clusters.nomad.testedNoVersion':
    'Подключено к региону {region}. Токен не может читать агент, поэтому его версия неизвестна.',
  'clusters.nomad.add': 'Добавить кластер',
  'clusters.nomad.added': '{name} добавлен',
  'clusters.nomad.settings': 'Параметры',
  'clusters.nomad.settingsDescription':
    'При сохранении подключение переустанавливается. Токен и ключ TLS больше не показываются: оставьте поля пустыми, чтобы их не менять.',
  'clusters.nomad.save': 'Сохранить изменения',
  'clusters.nomad.saved': 'Кластер сохранён',

  'clusters.refresh.title': 'Обновить из kubeconfig',
  'clusters.refresh.machine':
    'Заново читает контекст {context} из kubeconfig этого компьютера — на случай, если кластер был пересоздан или его учётные данные обновлены.',
  'clusters.refresh.pasted':
    'Вставьте kubeconfig ещё раз, чтобы заново прочитать контекст {context}, — на случай, если кластер был пересоздан или его учётные данные обновлены.',
  'clusters.refresh.submit': 'Обновить',
  'clusters.refresh.done': 'Кластер обновлён из своего kubeconfig',

  'clusters.remove.title': 'Удалить кластер',
  'clusters.remove.description':
    'Homeport забудет кластер и его сохранённые учётные данные. Ни в кластере, ни в вашем kubeconfig ничего не изменится.',
  'clusters.remove.confirmTitle': 'Удалить {name}?',
  'clusters.remove.confirmDescription': 'Его можно снова добавить из его kubeconfig.',
} satisfies Translation<typeof en>;
