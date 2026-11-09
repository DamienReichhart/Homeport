import type { Translation } from '../../types';
import type en from '../en/home';

/** The home page, in Spanish. */
export default {
  // Home: the hero and its board.
  'home.title': 'Homeport — Todos los Docker Engines, en un solo tablero.',
  'home.hero.new': 'Nuevo',
  'home.hero.release': 'Ya está disponible Homeport {version}',
  'home.hero.title': 'Todos los Docker Engines, en un solo tablero.',
  'home.hero.lead':
    'Homeport se ejecuta en tu equipo y se conecta a tus hosts por SSH, TLS o TCP. Ningún agente en el servidor. Gratuito y de código abierto, para Windows, macOS y Linux.',
  'home.hero.source': 'Leer el código fuente',
  'home.board.caption':
    'Un tablero de ejemplo: cinco hosts, sus Engines y cómo llega Homeport a ellos. Uno necesita atención.',
  'home.board.host': 'Host',
  'home.board.engine': 'Engine',
  'home.board.via': 'Vía',
  'home.board.stacks': 'Stacks',
  'home.board.status': 'Estado',
  'home.board.allRunning': 'Todo en ejecución',
  'home.board.needsYou': '1 stack te necesita',
  'home.board.note': 'Lo saludable no destaca. Solo se pone en rojo lo que te necesita.',
  'home.runtimes.label': 'Funciona con',
  'home.runtimes.title': 'Funciona con lo que ya usas',

  // Home: where it lives.
  'home.visits.eyebrow': 'Dónde vive',
  'home.visits.title':
    'Homeport se queda en tu equipo y visita tus servidores. Nunca se queda a vivir en ellos.',
  'home.visits.agent.title': 'Sin agente',
  'home.visits.agent.body':
    'Nada que instalar, actualizar ni proteger en el servidor. Homeport habla con el Engine que ya está ahí.',
  'home.visits.panel.title': 'Sin panel web',
  'home.visits.panel.body':
    'Ningún panel desplegado en tus servidores, ni una página de inicio de sesión esperando a que alguien la encuentre.',
  'home.visits.account.title': 'Sin cuenta',
  'home.visits.account.body':
    'Sin inicio de sesión, sin licencias por puesto, sin suscripción y sin telemetría. Te muestra tus contenedores, y nada más.',
  'home.visits.ports.title': 'Lo que necesita en el servidor, con franqueza',
  'home.visits.ports.ssh':
    'El puerto que ya tienes abierto. Con la clave que crea Homeport, una línea en {file}.',
  'home.visits.ports.tls':
    'El puerto de la API del propio Engine (normalmente {port}), que expones tú.',
  'home.visits.ports.tcp':
    'El puerto de la API del Engine (normalmente {port}), sin cifrar. Solo en una red de confianza.',
  'home.visits.ports.advice':
    'Cuando puedas, usa SSH: no añade al servidor nada que no tengas ya en ejecución.',
  'home.visits.link': 'Cómo funcionan las conexiones',

  // Home: the features.
  'home.features.eyebrow': 'Funciones',
  'home.features.title': 'Todo lo que ejecutan tus Engines, en una sola ventana',
  'home.features.lead':
    'El día a día de Docker — qué se está ejecutando, qué dice, cuánto consume — sin tener que recordar en qué servidor vive.',
  'home.features.hosts.title': 'Hosts, cerca y lejos',
  'home.features.hosts.body':
    'Los Engines locales y las máquinas de Podman se detectan solos. Los remotos se añaden por SSH, a través de bastiones si hace falta, por TLS mutuo o por TCP, o directamente desde tu ~/.ssh/config y tus contextos de Docker. Cada host se vuelve a conectar solo cuando la red falla un momento.',
  'home.features.containers.title': 'Contenedores de un vistazo',
  'home.features.containers.body':
    'Agrupados por stack, con CPU y memoria en directo y el motivo por el que uno no está saludable. Ejecútalos con límites, dispositivos y healthchecks, y luego edita, duplica, haz commit o exporta un contenedor.',
  'home.features.stacks.title': 'Stacks de Compose',
  'home.features.stacks.body':
    'Parte de una plantilla, vincula una carpeta de proyecto de este equipo o sigue un repositorio git, y despliega en cualquier host, compilaciones incluidas. Mira qué cambia antes de cada nuevo despliegue o sincronización, revierte y actualiza a imágenes más recientes en un solo paso.',
  'home.features.swarm.title': 'Docker Swarm',
  'home.features.swarm.body':
    'Los Swarms aparecen a través de sus hosts manager. Gestiona nodos, servicios, tareas, secretos y configs, despliega stacks en un Swarm, y crea un Swarm, únete a uno o sal de él desde un host, sin instalar nada en ningún nodo.',
  'home.features.podman.title': 'Podman, como Podman',
  'home.features.podman.body':
    'Homeport reconoce Podman al conectarse y añade lo que solo Podman tiene: pods, creados con sus puertos y con contenedores ejecutados dentro, y unidades Quadlet, instaladas, editadas e iniciadas con systemd, a través de la propia API de Podman.',
  'home.features.containerd.title': 'containerd, directamente',
  'home.features.containerd.body':
    'Conéctate a containerd por su socket, en local o por SSH, sin Docker: elige un namespace, inicia, detén y envía señales a las tareas de sus contenedores, lee sus logs, abre un terminal, descarga imágenes con tus inicios de sesión del registro y encuentra las instantáneas que se comen tu disco.',
  'home.features.incus.title': 'Instancias de Incus',
  'home.features.incus.body':
    'Añade Incus por su socket, por SSH o por HTTPS con un token de confianza, y ejecuta sus contenedores de sistema y máquinas virtuales: créalos a partir de una imagen remota, inícialos, congélalos y detenlos, abre su consola o un shell, toma y restaura instantáneas, y gestiona imágenes, perfiles, almacenamiento y redes.',
  'home.features.clusters.title': 'Clústeres de Kubernetes y Nomad',
  'home.features.clusters.body':
    'Añade clústeres de Kubernetes desde tu kubeconfig — EKS, GKE y AKS incluidos — o encuentra los locales con un clic. Explora en directo cargas de trabajo, Pods, Services, configuración y cualquier recurso de la API; sigue logs, abre un terminal, reenvía puertos, escala y aplica YAML con una vista previa, e instala, actualiza y revierte charts de Helm. Añade clústeres de Nomad por su dirección para ejecutar jobs desde HCL con su plan, escalarlos y detenerlos, seguir logs, abrir un shell y drenar nodos. No se instala nada en ningún clúster.',
  'home.features.images.title': 'Imágenes',
  'home.features.images.body':
    'Busca en Docker Hub, descarga y sube con tus inicios de sesión del registro, compila desde una carpeta de tu equipo, guarda y carga archivos de imágenes, y mira cuándo sale una imagen más reciente.',
  'home.features.volumes.title': 'Volúmenes y redes',
  'home.features.volumes.body':
    'Mira qué contenedores usan qué. Crea, conecta y desconecta, con una eliminación que te dice exactamente qué vas a perder.',
  'home.features.board.title': 'Un tablero para todos los hosts',
  'home.features.board.body':
    'Todos los contenedores de todos los hosts en una sola tabla, filtrados y con acciones conjuntas. Cada host también en su propia página: detalles del Engine, uso de disco con una limpieza en un solo diálogo y un flujo en directo de lo que hace el Engine.',
  'home.features.keyboard.title': 'Primero el teclado',
  'home.features.keyboard.body':
    'Una paleta de comandos para ir a cualquier sitio, cambiar de host o ejecutar un comando, y para encontrar cualquier contenedor, imagen o volumen en el host que lo tenga. Atajos para cada página, listados con ?.',
  'home.features.background.title': 'Vigilando en segundo plano',
  'home.features.background.body':
    'El estado de cada host en la bandeja del sistema, notificaciones cuando un contenedor falla o un host se desconecta, y Homeport iniciándose al iniciar sesión, oculto hasta que lo necesites.',

  // Home: Hosts.
  'home.hosts.eyebrow': 'Hosts',
  'home.hosts.title': 'Engines remotos sin ceremonias',
  'home.hosts.body':
    'Añade un servidor una vez y se queda en el tablero. Para SSH, Homeport crea una clave solo para ese host y te da el único comando que la autoriza. Comprueba la huella del servidor en el primer contacto y te avisa si alguna vez cambia.',
  'home.hosts.import':
    '¿Ya los tienes descritos? Importa tus servidores desde {file}, bastiones incluidos, o tus contextos de Docker, certificados incluidos. Si cambias de equipo, llévatelos en un solo archivo protegido con una frase de contraseña.',
  'home.hosts.socket.name': 'Socket local',
  'home.hosts.socket.detail': 'Sockets Unix y named pipes de Windows, detectados automáticamente.',
  'home.hosts.ssh.detail':
    'Una clave Ed25519 dedicada por host, tu propia clave, ssh-agent o Pageant; también a través de bastiones.',
  'home.hosts.tls.detail': 'TLS mutuo con tu CA, tu certificado de cliente y tu clave.',
  'home.hosts.tcp.detail': 'TCP sin cifrar, solo para un Engine en una red de confianza.',
  'home.hosts.link': 'Cómo funcionan las conexiones',
  'home.hosts.screenshot':
    'La página Añadir host con SSH seleccionado: servidor, puerto, usuario, ruta del socket remoto y una clave nueva para este host.',

  // Home: containers.
  'home.containers.eyebrow': 'Contenedores',
  'home.containers.title': 'Todo sobre un contenedor, a un clic',
  'home.containers.body':
    'Abre un contenedor y sus logs ya se están mostrando en directo. Cambia de pestaña para un terminal interactivo, sus archivos, gráficos en directo, los procesos y el documento inspect completo, igual de fácil en un host al otro lado del mundo que en tu portátil.',
  'home.containers.logs.title': 'Logs',
  'home.containers.logs.body': 'Sigue, busca, filtra por flujo y por hora, descarga.',
  'home.containers.terminal.title': 'Terminal',
  'home.containers.terminal.body': 'Shells interactivos, tantos como necesites.',
  'home.containers.files.title': 'Archivos',
  'home.containers.files.body': 'Explora, sube y descarga, las carpetas como .tar.',
  'home.containers.stats.title': 'Estadísticas',
  'home.containers.stats.body': 'Gráficos de CPU, memoria, red y E/S de disco.',
  'home.containers.link': 'Trabajar con contenedores',
  'home.containers.screenshot':
    'La pestaña Logs de un contenedor, con la salida en directo, búsqueda, marcas de tiempo y filtros de flujo.',

  // Home: Stacks.
  'home.stacks.eyebrow': 'Stacks',
  'home.stacks.title': 'Stacks de Compose: gestionados, vinculados, sincronizados o encontrados',
  'home.stacks.body':
    'Guarda un archivo compose y su entorno en Homeport y despliégalo en cualquier host: parte de una plantilla, combina varios archivos, elige perfiles; las variables se guardan cifradas y nunca se escriben en disco. O vincula una carpeta de proyecto de este equipo: Homeport edita sus archivos en su sitio y despliega desde ella, compilaciones incluidas. O sigue una rama de un repositorio git, con una clave de despliegue o un token: cada sincronización muestra los commits nuevos y lo que cambian antes de desplegar, y mientras Homeport está abierto los busca y, si se lo pides, despliega por sí solo. Cada nuevo despliegue muestra antes qué cambia, cada despliegue se conserva para que puedas revertir, y Actualizar descarga imágenes más recientes y vuelve a desplegar en un solo paso. Los stacks que alguien desplegó a mano también aparecen: inícialos, detenlos o elimínalos, o adóptalos.',
  'home.stacks.link': 'Gestionar stacks',
  'home.stacks.screenshot':
    'La definición de un stack gestionado: el editor de compose.yaml, su entorno y el botón Guardar y volver a desplegar.',

  // Home: security.
  'home.security.eyebrow': 'Seguridad',
  'home.security.title': 'Tus claves siguen selladas',
  'home.security.lead':
    'Homeport maneja las credenciales que abren tus servidores, así que las trata como tales.',
  'home.security.rest.title': 'Cifrado en reposo',
  'home.security.rest.body':
    'Las claves SSH, los certificados TLS, los entornos de los stacks y los tokens de los repositorios se sellan con XChaCha20-Poly1305. La clave vive en el llavero de tu sistema operativo, no junto a los datos.',
  'home.security.disk.title': 'Nunca en disco sin cifrar',
  'home.security.disk.body':
    'Homeport habla SSH por sí mismo, así que las claves descifradas se quedan en memoria. Sin archivos de clave ni cambios en tu {dir}.',
  'home.security.home.title': 'No envía nada a casa',
  'home.security.home.body':
    'Sin cuenta, sin telemetría, sin seguimiento. Homeport habla con tus Engines y con nada más. Cada versión se compila a la vista de todos, con sumas de comprobación y procedencia de compilación.',
  'home.security.link': 'Seguridad y tus datos',

  // Home: desktops and the closing call.
  'home.desktops.eyebrow': 'Descargar',
  'home.desktops.title': 'Nativo en cada escritorio',
  'home.desktops.lead':
    'Una app pequeña y rápida hecha con Tauri y Rust, no una pestaña del navegador disfrazada. Instaladores para x64 y ARM que mantienen Homeport actualizado.',
  'home.desktops.windows.versions': '10 y 11 · x64 y ARM64',
  'home.desktops.windows.formats': 'Instalador (.exe) · MSI',
  'home.desktops.macos.versions': 'Apple silicon e Intel',
  'home.desktops.macos.formats': 'Imagen de disco (.dmg)',
  'home.desktops.linux.versions': 'x64 y ARM64',
  'home.desktops.linux.formats': 'apt · dnf · zypper · AppImage',
  'home.cta.title': 'Nada que instalar en tus servidores.',
  'home.cta.body':
    'Descarga Homeport en el equipo desde el que trabajas y añade tus hosts. Es gratuito, de código abierto bajo la GNU AGPL y se desarrolla a la vista de todos en GitHub.',
  'home.cta.github': 'Ver en GitHub',
} satisfies Translation<typeof en>;
