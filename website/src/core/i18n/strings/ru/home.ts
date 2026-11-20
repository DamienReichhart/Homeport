/** The home page, in Russian. */
export default {
  // Home: the hero and its map.
  'home.title': 'Homeport — одно приложение для любого движка контейнеров',
  'home.hero.title': 'Одно приложение для любого движка контейнеров.',
  'home.hero.lead':
    'Homeport работает на вашем компьютере и ничего не устанавливает на ваши серверы. Бесплатно и с открытым исходным кодом.',
  'home.hero.docs': 'Читать документацию',
  'home.map.label':
    'Пример парка серверов в виде схемы метро. Этот компьютер — конечная станция пяти линий: локальный сокет ведёт к This Mac, SSH — к prod-eu-1, nas и edge-1, TLS — к staging и vps-fra-1, TCP — к lan-box, а API кластера — к k8s-prod, jobs и lab. Один стек на vps-fra-1 требует внимания.',
  'home.map.thisMachine': 'Этот компьютер',
  'home.map.needsYou': '1 стек требует внимания',
  'home.map.keyNeedsYou': 'Требует внимания',
  'home.map.example': 'Пример парка серверов',

  // Home: the questions people open it for.
  'home.questions.title': 'Один вопрос — одно место.',
  'home.questions.lead':
    'Вы открываете Homeport, чтобы что-то узнать, и возвращаетесь к работе. Каждый ответ — в одном шаге от Доски.',
  'home.questions.up.title': 'Всё ли работает?',
  'home.questions.up.body':
    'Доска показывает все хосты, Swarm и кластеры сразу и выводит то, что требует внимания, раньше всего, что в порядке.',
  'home.questions.up.link': 'Доска и Обзор',
  'home.questions.up.screenshot':
    'Доска: по плитке на каждый из трёх хостов, один контейнер, который требует внимания, и контейнеры всех хостов в одной таблице.',
  'home.questions.died.title': 'Почему он упал?',
  'home.questions.died.body':
    'Откройте контейнер — его журналы уже идут. Терминал, его файлы, графики в реальном времени и документ inspect — на соседних вкладках.',
  'home.questions.died.link': 'Работа с контейнерами',
  'home.questions.died.screenshot':
    'Вкладка «Журналы» контейнера: вывод в реальном времени с поиском, метками времени и фильтрами по потоку.',
  'home.questions.changed.title': 'Что изменилось?',
  'home.questions.changed.body':
    'Каждый хост записывает всё, что делает его движок: какой контейнер перезапустился, какой образ скачан, какой том подключён.',
  'home.questions.changed.link': 'События',
  'home.questions.changed.screenshot':
    'Страница «События» хоста: проверки здоровья, перезапуски, скачивания образов и подключения томов, сначала новые, и у каждого — стек, к которому оно относится.',
  'home.questions.act.title': 'Затем вносите изменения, увидев их заранее.',
  'home.questions.act.body':
    'Повторное развёртывание показывает, что изменится, ещё до запуска. Каждое развёртывание сохраняется, чтобы можно было откатиться, а «Обновить» скачивает новые образы за один шаг.',
  'home.questions.act.link': 'Управление стеками',
  'home.questions.act.screenshot':
    'Определение управляемого стека: редактор compose.yaml, его окружение и кнопка «Развернуть».',
  'home.questions.background.title': 'Наблюдение в фоне',
  'home.questions.background.body':
    'Состояние каждого хоста в системном трее, уведомления, когда контейнер падает или хост отключается, и запуск Homeport при входе в систему — скрытым, пока он не понадобится.',
  'home.questions.keyboard.title': 'Прежде всего клавиатура',
  'home.questions.keyboard.body':
    'Палитра команд, чтобы перейти куда угодно, сменить хост или выполнить команду, а также найти любой контейнер, образ или том на том хосте, где он есть. Сочетания клавиш для каждой страницы, их список открывается по ?.',

  // Home: the lines, and what each needs on the server.
  'home.lines.title': 'На серверах ничего не поселяется.',
  'home.lines.lead':
    'Homeport остаётся на вашем компьютере и приходит к серверам в гости. Не нужно ставить агента, открывать наружу веб-панель или создавать учётную запись, и никакой телеметрии.',
  'home.lines.line': 'Линия',
  'home.lines.reaches': 'Куда ведёт',
  'home.lines.needs': 'Что нужно на сервере',
  'home.lines.socket.name': 'Локальный сокет',
  'home.lines.socket.reaches':
    'Docker Desktop, OrbStack, Colima, Rancher Desktop и машины Podman на этом компьютере — они находятся сами.',
  'home.lines.socket.needs': 'Ничего. Движок уже на этом компьютере.',
  'home.lines.ssh.reaches':
    'Docker Engine, Podman, containerd и Incus на сервере, при необходимости через бастионы.',
  'home.lines.ssh.needs':
    'Порт, который у вас уже открыт. С ключом, который создаёт Homeport, — одна строка в {file}.',
  'home.lines.tls.reaches':
    'API движка, совместимого с Docker, с вашим CA и клиентским сертификатом.',
  'home.lines.tls.needs':
    'Собственный порт API движка (обычно {port}), который вы открываете сами.',
  'home.lines.tcp.reaches': 'API движка, совместимого с Docker, без шифрования.',
  'home.lines.tcp.needs': 'Порт API движка (обычно {port}). Только в сети, которой вы доверяете.',
  'home.lines.cluster.name': 'API кластера',
  'home.lines.cluster.reaches':
    'Kubernetes из вашего kubeconfig, Nomad по адресу его агента, Incus по HTTPS с токеном доверия.',
  'home.lines.cluster.needs':
    'API, который кластер уже предоставляет. На узлы ничего не устанавливается.',
  'home.lines.advice':
    'Когда есть возможность, используйте SSH: он не добавляет на сервер ничего сверх того, что там уже работает.',
  'home.lines.import':
    'Серверы уже описаны? Импортируйте их из {file} вместе с бастионами или из контекстов Docker.',
  'home.lines.link': 'Как устроены подключения',

  // Home: the Engines, as the map's index of stations.
  'home.engines.title': 'Все движки — одними словами.',
  'home.engines.lead':
    'Хост — это хост, а стек — это стек, какой бы движок его ни запускал. Если у движка есть что-то своё, Homeport показывает и это.',
  'home.engines.docker.body':
    'Контейнеры, образы, тома, сети и стеки Compose: хранятся в Homeport, связаны с папкой или синхронизируются из git-репозитория.',
  'home.engines.swarm.body':
    'Узлы, сервисы, задачи, секреты и конфигурации — через его управляющие хосты. Разворачивайте стеки в Swarm.',
  'home.engines.podman.body':
    'Поды и юниты Quadlet, которые устанавливаются и запускаются под systemd, через собственный API Podman.',
  'home.engines.containerd.body':
    'Пространства имён, задачи, журналы, терминал и снимки, которые съедают место на диске, — без Docker.',
  'home.engines.incus.body':
    'Системные контейнеры и виртуальные машины: их консоль, снимки, образы, профили, хранилища и сети.',
  'home.engines.kubernetes.body':
    'Рабочие нагрузки, объекты Pod и Service и любой ресурс API в реальном времени. Журналы, терминалы, пробросы портов, YAML с предпросмотром, релизы Helm.',
  'home.engines.nomad.body':
    'Запуск job из HCL с их планом, allocation и журналы, оболочка в task и освобождение узлов.',

  // Home: security.
  'home.security.title': 'Ваши ключи под замком.',
  'home.security.lead':
    'Homeport работает с учётными данными, которые открывают доступ к вашим серверам, и обращается с ними соответственно.',
  'home.security.rest.title': 'Зашифровано при хранении',
  'home.security.rest.body':
    'Ключи SSH, сертификаты TLS, окружения стеков и токены репозиториев шифруются с помощью XChaCha20-Poly1305. Ключ шифрования хранится в связке ключей ОС, а не рядом с данными.',
  'home.security.disk.title': 'Никогда на диске в открытом виде',
  'home.security.disk.body':
    'Homeport сам реализует SSH, поэтому расшифрованные ключи остаются в памяти. Никаких файлов ключей и никаких изменений в вашем {dir}.',
  'home.security.home.title': 'Ничего не отправляется «домой»',
  'home.security.home.body':
    'Ни учётной записи, ни телеметрии, ни отслеживания. Каждый выпуск собирается открыто, с контрольными суммами и подтверждением происхождения сборки.',
  'home.security.link': 'Безопасность и ваши данные',

  // Home: the close, where the visitor starts.
  'home.close.title': 'Начните с этого компьютера.',
  'home.close.body':
    'Скачайте Homeport на компьютер, за которым работаете, и добавьте свои хосты. Он бесплатный, распространяется с открытым исходным кодом по лицензии GNU AGPL и разрабатывается открыто на GitHub.',
  'home.close.github': 'Открыть на GitHub',
  'home.desktops.windows.versions': '10 и 11, x64 и ARM64',
  'home.desktops.windows.formats': 'Установщик (.exe) или MSI',
  'home.desktops.macos.versions': 'Apple silicon и Intel',
  'home.desktops.macos.formats': 'Образ диска (.dmg)',
  'home.desktops.linux.versions': 'x64 и ARM64',
  'home.desktops.linux.formats': 'apt, dnf, zypper или AppImage',
};
