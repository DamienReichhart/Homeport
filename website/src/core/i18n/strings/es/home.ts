/** The home page, in Spanish. */
export default {
  // Home: the hero and its map.
  'home.title': 'Homeport — Una sola app para todos los Engines de contenedores',
  'home.hero.title': 'Una sola app para todos los Engines de contenedores.',
  'home.hero.lead':
    'Homeport se ejecuta en tu equipo y no instala nada en tus servidores. Gratuito y de código abierto.',
  'home.hero.docs': 'Leer la documentación',
  'home.map.label':
    'Una flota de ejemplo dibujada como un plano de metro. Este equipo está al final de cinco líneas: el socket local llega a laptop, SSH llega a prod-eu-1, nas y edge-1, TLS llega a staging y vps-fra-1, TCP llega a lan-box, y la API del clúster llega a k8s-prod, jobs y lab. Un stack en vps-fra-1 requiere tu atención.',
  'home.map.thisMachine': 'Este equipo',
  'home.map.needsYou': '1 stack requiere tu atención',
  'home.map.keyNeedsYou': 'Requiere tu atención',
  'home.map.example': 'Una flota de ejemplo',

  // Home: the questions people open it for.
  'home.questions.title': 'Una pregunta, un solo lugar.',
  'home.questions.lead':
    'Abres Homeport para averiguar algo y vuelves a lo tuyo. Cada respuesta está a un paso del Tablero.',
  'home.questions.up.title': '¿Está en marcha?',
  'home.questions.up.body':
    'El Tablero muestra todos los hosts, Swarms y clústeres a la vez, y pone lo que requiere tu atención antes de todo lo que está bien.',
  'home.questions.up.link': 'El Tablero y el Resumen',
  'home.questions.up.screenshot':
    'El Tablero: un mosaico por cada uno de los tres hosts, un contenedor que requiere tu atención y, después, los contenedores de todos los hosts en una sola tabla.',
  'home.questions.died.title': '¿Por qué se cayó?',
  'home.questions.died.body':
    'Abre un contenedor y sus logs ya se están mostrando en directo. Un terminal, sus archivos, gráficos en directo y el documento inspect están a una pestaña de distancia.',
  'home.questions.died.link': 'Trabajar con contenedores',
  'home.questions.died.screenshot':
    'La pestaña Logs de un contenedor, con la salida en directo, búsqueda, marcas de tiempo y filtros de flujo.',
  'home.questions.changed.title': '¿Qué cambió?',
  'home.questions.changed.body':
    'Cada host lleva un historial de lo que hizo su Engine: el contenedor que se reinició, la imagen que se descargó, el volumen que se montó.',
  'home.questions.changed.link': 'Eventos',
  'home.questions.changed.screenshot':
    'La página Eventos de un host: healthchecks, reinicios, descargas y montajes, primero los más recientes, con el stack al que pertenece cada uno.',
  'home.questions.act.title': 'Después, cámbialo y mira antes el cambio.',
  'home.questions.act.body':
    'Un nuevo despliegue muestra qué va a cambiar antes de ejecutarse. Cada despliegue se conserva, así que puedes revertir, y Actualizar descarga imágenes más recientes en un solo paso.',
  'home.questions.act.link': 'Gestionar stacks',
  'home.questions.act.screenshot':
    'La definición de un stack gestionado: el editor de compose.yaml, su entorno y el botón Desplegar.',

  // Home: the lines, and what each needs on the server.
  'home.lines.title': 'Nada se instala.',
  'home.lines.lead':
    'Homeport se queda en tu equipo y visita tus servidores. Ningún agente que instalar, ningún panel web que exponer, ninguna cuenta que crear y nada de telemetría.',
  'home.lines.line': 'Línea',
  'home.lines.reaches': 'Llega a',
  'home.lines.needs': 'Qué necesita en el servidor',
  'home.lines.socket.name': 'Socket local',
  'home.lines.socket.reaches':
    'Docker Desktop, OrbStack, Colima, Rancher Desktop y las máquinas de Podman de este equipo, detectados automáticamente.',
  'home.lines.socket.needs': 'Nada. El Engine ya está en este equipo.',
  'home.lines.ssh.reaches':
    'Docker Engine, Podman, containerd e Incus en un servidor, a través de bastiones si hace falta.',
  'home.lines.ssh.needs':
    'El puerto que ya tienes abierto. Con la clave que crea Homeport, una línea en {file}.',
  'home.lines.tls.reaches':
    'La API de un Engine compatible con Docker, con tu CA y tu certificado de cliente.',
  'home.lines.tls.needs':
    'El puerto de la API del propio Engine (normalmente {port}), que expones tú.',
  'home.lines.tcp.reaches': 'La API de un Engine compatible con Docker, sin cifrar.',
  'home.lines.tcp.needs':
    'El puerto de la API del Engine (normalmente {port}). Solo en una red de confianza.',
  'home.lines.cluster.name': 'API del clúster',
  'home.lines.cluster.reaches':
    'Kubernetes desde tu kubeconfig, Nomad por la dirección de su agente, Incus por HTTPS con un token de confianza.',
  'home.lines.cluster.needs': 'La API que el clúster ya ofrece. No se instala nada en ningún nodo.',
  'home.lines.advice':
    'Cuando puedas, usa SSH: no añade al servidor nada que no tengas ya en ejecución.',
  'home.lines.import':
    '¿Ya tienes descritos tus servidores? Impórtalos desde {file}, bastiones incluidos, o desde tus contextos de Docker.',
  'home.lines.link': 'Cómo funcionan las conexiones',

  // Home: the Engines, as the map's index of stations.
  'home.engines.title': 'Cada Engine, con las mismas palabras.',
  'home.engines.lead':
    'Un host es un host y un stack es un stack, sea cual sea el Engine que lo ejecute. Cuando un Engine tiene algo propio, Homeport también lo muestra.',
  'home.engines.docker.body':
    'Contenedores, imágenes, volúmenes, redes y stacks de Compose: guardados en Homeport, vinculados a una carpeta o sincronizados desde un repositorio git.',
  'home.engines.swarm.body':
    'Nodos, servicios, tareas, secretos y configs, encontrados a través de sus hosts manager. Despliega stacks en el Swarm.',
  'home.engines.podman.body':
    'Pods, y unidades Quadlet instaladas e iniciadas con systemd, a través de la propia API de Podman.',
  'home.engines.containerd.body':
    'Namespaces, tareas, logs, un terminal y las instantáneas que se comen tu disco, sin Docker.',
  'home.engines.incus.body':
    'Contenedores de sistema y máquinas virtuales: su consola, instantáneas, imágenes, perfiles, almacenamiento y redes.',
  'home.engines.kubernetes.body':
    'Cargas de trabajo, Pods, Services y cualquier recurso de la API, en directo. Logs, terminales, reenvíos de puerto, YAML con vista previa, releases de Helm.',
  'home.engines.nomad.body':
    'Jobs ejecutados desde HCL con su plan, sus allocations y sus logs, un shell en una task y el drenaje de nodos.',

  // Home: security.
  'home.security.title': 'Tus claves siguen selladas.',
  'home.security.lead':
    'Homeport guarda las credenciales que abren tus servidores, así que las trata como tales.',
  'home.security.rest.title': 'Cifrado en reposo',
  'home.security.rest.body':
    'Las claves SSH, los certificados TLS, los entornos de los stacks y los tokens de los repositorios se sellan con XChaCha20-Poly1305. La clave vive en el llavero de tu sistema operativo, no junto a los datos.',
  'home.security.disk.title': 'Nunca en disco sin cifrar',
  'home.security.disk.body':
    'Homeport habla SSH por sí mismo, así que las claves descifradas se quedan en memoria. Sin archivos de clave ni cambios en tu {dir}.',
  'home.security.home.title': 'No envía nada a casa',
  'home.security.home.body':
    'Sin cuenta, sin telemetría, sin seguimiento. Cada versión se compila a la vista de todos, con sumas de comprobación y procedencia de compilación.',
  'home.security.link': 'Seguridad y tus datos',

  // Home: the close, where the visitor starts.
  'home.close.title': 'Empieza desde este equipo.',
  'home.close.body':
    'Descarga Homeport en el equipo desde el que trabajas y luego añade tus hosts. Es gratuito, de código abierto bajo la GNU AGPL y se desarrolla a la vista de todos en GitHub.',
  'home.close.github': 'Ver en GitHub',
  'home.desktops.windows.versions': '10 y 11, x64 y ARM64',
  'home.desktops.windows.formats': 'Instalador (.exe) o MSI',
  'home.desktops.macos.versions': 'Apple silicon e Intel',
  'home.desktops.macos.formats': 'Imagen de disco (.dmg)',
  'home.desktops.linux.versions': 'x64 y ARM64',
  'home.desktops.linux.formats': 'apt, dnf, zypper o AppImage',
};
