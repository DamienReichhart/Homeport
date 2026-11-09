---
title: Preguntas frecuentes
description: Preguntas habituales sobre Homeport.
---

### ¿Homeport es gratuito?

Sí. Homeport se puede usar gratis, para trabajo personal y comercial, y es de código abierto bajo la [GNU Affero General Public License, versión 3](https://github.com/DamienReichhart/Homeport/blob/main/LICENSE) (AGPL).

### ¿Qué significa la AGPL para mí?

Si usas Homeport, no cambia nada: instálalo y ejecútalo en casa o en el trabajo, en tantos equipos como quieras. La AGPL solo exige algo a quienes **distribuyen** Homeport, modificado o no, o a quienes ofrecen a otros una versión modificada a través de una red: deben publicar su código fuente completo bajo la misma licencia. Cada copia de Homeport enlaza con el código fuente de su versión exacta en **Ajustes → Licencias**.

Las versiones hasta la 0.11 se publicaron bajo la licencia MIT y siguen disponibles bajo ella.

### ¿Homeport sustituye a Docker Desktop?

No del todo. Homeport es un gestor: no ejecuta ningún Docker Engine por sí mismo. En Linux, instala Docker Engine y usa Homeport como su interfaz. En macOS y Windows sigues necesitando algo que ejecute el Engine —Docker Desktop, Colima, OrbStack, Rancher Desktop o Podman—, y Homeport puede funcionar por encima de cualquiera de ellos, junto con tus servidores remotos.

### ¿Funciona con containerd?

Sí, directamente, sin Docker: el Engine de un host puede ser [containerd](/docs/hosts/containerd/), al que se accede a través de su socket en este equipo o por SSH: el de k3s, el propio de Docker Engine o el servicio de containerd en un servidor. Homeport lista sus namespaces, sus contenedores y sus tareas (iniciar, detener, pausar, enviar señales, eliminar), sus logs donde los escribieron nerdctl o Kubernetes, un terminal en ellos, sus imágenes (descargar, eliminar) y sus instantáneas. No crea contenedores ni despliega stacks ahí; eso lo hace nerdctl.

### ¿Funciona con Incus?

Sí: el Engine de un host puede ser [Incus](/docs/hosts/incus/) 6.0 o posterior, al que se accede a través de su socket en este equipo o por SSH, o por HTTPS con un token de confianza de `incus config trust add`. Homeport lista las [instancias](/docs/guides/instances/) de un proyecto, contenedores de sistema y máquinas virtuales; las inicia, detiene, congela, elimina y crea; abre su consola y un terminal en ellas; toma y restaura instantáneas, y gestiona imágenes, perfiles, pools de almacenamiento y redes. No despliega stacks ahí, y no instala nada en el host.

### ¿Funciona con LXD?

No como tal: Homeport habla con Incus, el fork comunitario de LXD, y rechaza un servidor LXD, cuya API se ha distanciado. Migra el servidor con `lxd-to-incus` y luego añádelo como host de Incus.

### ¿Funciona con Podman?

Sí, a través del socket de la API compatible con Docker de Podman. Homeport detecta los sockets habituales de Podman en Linux, cada [máquina de Podman en macOS](/docs/hosts/local/#máquinas-de-podman-en-macos) en ejecución y la tubería de la máquina de Podman predeterminada en Windows. Cualquier otro se puede [añadir a mano](/docs/hosts/local/#añadir-un-socket-a-mano).

Homeport también reconoce Podman al conectarse, y entonces ofrece lo que solo tiene Podman, a través de la API propia de Podman en el mismo socket: los [pods](/docs/guides/podman-pods/), que puedes listar, iniciar, detener, crear y eliminar, y en los que puedes ejecutar contenedores desde el diálogo de ejecución; y las [unidades Quadlet](/docs/guides/quadlet/), que puedes listar, leer, instalar, editar y eliminar, e iniciar o detener por SSH.

### ¿Funciona con Kubernetes, Nomad o Docker Swarm?

Docker Swarm, sí. Homeport encuentra un Swarm a través de sus hosts manager, en cuanto uno de ellos es un host, y lo gestiona por completo: sus nodos, servicios, tareas, secretos, configs y stacks de Swarm, incluidos los stacks gestionados, vinculados y sincronizados desplegados en él. También puede iniciar un Swarm en un host, y unirse a uno o abandonarlo. Todo pasa por la API del Engine de un manager conectado; no se instala nada en los nodos. Consulta [Swarms](/docs/guides/swarm/).

Kubernetes, sí. Añade un clúster desde tu kubeconfig —el de este equipo o uno que pegues—, y Homeport encuentra por ti los locales (Docker Desktop, OrbStack, Rancher Desktop, kind, k3d, minikube). EKS, GKE y AKS funcionan a través del plugin exec que nombra su kubeconfig. Puedes explorar en directo Namespaces, cargas de trabajo, Pods, Services, Ingresses, ConfigMaps, Secrets, Nodes, eventos y cualquier otro recurso de la API, incluidos los personalizados; seguir logs, abrir un terminal en un Pod, reenviar puertos, escalar, reiniciar y eliminar, y aplicar o editar YAML con vista previa. No se instala nada en el clúster: Homeport solo habla con la API de Kubernetes a la que ya llega tu kubeconfig, con los permisos de sus credenciales. Con la CLI `helm` en este equipo, Homeport también lista, instala, actualiza, revierte y desinstala [releases de Helm](/docs/clusters/helm/), y gestiona los repositorios de charts de helm. Consulta [Clústeres](/docs/clusters/).

Nomad, sí. Añade un clúster por la dirección de su agente, con un token ACL y material TLS cuando los usa. Puedes listar en directo los jobs y sus allocations; ejecutar un job desde HCL o JSON tras ver el plan de Nomad; editar y volver a ejecutar uno, detenerlo o purgarlo, y escalar sus task groups; seguir los logs de una task y abrir un shell en ella; drenar nodos y cambiar su elegibilidad, y elegir un namespace. No se instala nada en el clúster y no hace falta la CLI `nomad`: Homeport habla con la API HTTP de Nomad con los permisos del token. Consulta [Clústeres de Nomad](/docs/clusters/nomad/).

### ¿Necesito kubectl?

No. Homeport lee tu kubeconfig y habla con la API de Kubernetes por sí mismo. Los únicos programas que ejecuta son los [plugins exec](/docs/clusters/exec-plugins/) que nombra un kubeconfig, como la AWS CLI para EKS, `gke-gcloud-auth-plugin` para GKE o `kubelogin` para AKS: los mismos que ejecutaría `kubectl`. Aun así, tener `kubectl` a mano ayuda a comprobar qué pueden hacer tus credenciales (`kubectl auth can-i`).

### ¿Tengo que instalar algo en mis servidores?

No: solo un Docker Engine y un servidor SSH, que lo más probable es que ya tengas. Homeport no instala agentes.

Un Swarm tampoco necesita nada más que el Engine. Eso sí, sus nodos deben poder comunicarse entre sí, algo que Homeport no puede configurar por ti: abre 2377/tcp (hacia los managers), 7946/tcp y 7946/udp, y 4789/udp entre ellos. Consulta [puertos entre nodos](/docs/guides/swarm/#puertos-entre-nodos).

### ¿Necesito Docker instalado en mi equipo para gestionar servidores remotos?

No. Homeport habla por sí mismo la API del Engine y SSH. Las únicas excepciones son desplegar stacks gestionados, vinculados y sincronizados, que usa el comando `docker compose` de tu equipo, y desplegarlos en un Swarm, que usa el comando `docker` (`docker stack deploy`). Ninguno de los dos necesita un Engine en tu equipo.

### ¿Homeport mantiene mis stacks sincronizados cuando está cerrado?

No. En los [stacks sincronizados](/docs/guides/synced-stacks/) se buscan commits nuevos, y se despliegan automáticamente si lo activas, solo mientras Homeport está abierto. Homeport no instala nada en tus servidores —ni agente, ni receptor de webhooks—, así que cuando está cerrado nada vigila tus repositorios. Cuando lo vuelves a abrir, espera un intervalo y luego los comprueba todos. Para despliegues que deban seguir cada push a cualquier hora, ejecútalos desde tu CI.

Para mantener Homeport en ejecución sin su ventana, activa **Cerrar a la bandeja del sistema**, e **Iniciar al iniciar sesión** con **Iniciar oculto en la bandeja**, en [Ajustes → Escritorio](/docs/guides/settings/#escritorio).

### ¿Puede Homeport avisarme cuando un contenedor falla?

Sí, mientras se ejecuta, incluso oculto en la bandeja del sistema. Muestra una notificación del sistema cuando un contenedor se detiene con un error, se queda sin memoria o pasa a no saludable, y cuando un host pasa a inaccesible o vuelve. Cada tipo se puede desactivar en [Ajustes → Notificaciones](/docs/guides/settings/#notificaciones). Lo sabe por el flujo de eventos que ya mantiene con cada host, así que no se instala nada en tus servidores; cuando Homeport no se está ejecutando, no se avisa a nadie.

### ¿Homeport necesita git?

Solo para los [stacks sincronizados](/docs/guides/synced-stacks/), que ejecutan el `git` instalado en tu equipo, con la clave de despliegue o el token propios de cada repositorio. **Ajustes → Este equipo** indica si Homeport lo encontró. Tu propia configuración de git, tus asistentes de credenciales y tus claves SSH nunca se usan.

### ¿Pueden varias personas compartir los hosts de Homeport?

Cada persona ejecuta su propio Homeport con sus propios hosts y claves. Con una clave creada por Homeport por persona y por servidor, puedes ver y revocar el acceso de cada una en `authorized_keys`.

### ¿Puedo pasar Homeport a otro equipo?

Sí, de dos formas:

- **Todo**: **Ajustes → Datos → Hacer copia de seguridad…** guarda todos los hosts, clústeres, stacks, inicios de sesión de registros, repositorios y ajustes, con sus secretos, en un archivo protegido con una frase de contraseña de al menos 12 caracteres. En el equipo nuevo, **Restaurar…**, en el mismo lugar, lo abre, lista lo que contiene y sustituye lo que haya. Todo se conecta y se despliega sin volver a pedirte ninguna clave ni contraseña. Consulta [hacer y restaurar copias de seguridad](/docs/guides/settings/#hacer-y-restaurar-copias-de-seguridad).
- **Algunos hosts**: en la página **Hosts**, **Exporta** los hosts que elijas e **impórtalos** en el otro equipo, junto a los hosts que ya tenga. Consulta [pasar a otro equipo](/docs/reference/security/#pasar-a-otro-equipo).

Copiar `homeport.db` no sirve para los secretos: están cifrados con una clave guardada en el llavero de este equipo, y en otro lugar aparecen como **Credenciales perdidas**.

### ¿Una copia de seguridad incluye mis contenedores y volúmenes?

No. Una copia de seguridad contiene lo que guarda Homeport —cómo acceder a tus hosts, las definiciones y los entornos de tus stacks, los inicios de sesión y los ajustes—, no lo que se ejecuta en tus hosts. Tus contenedores, imágenes y volúmenes se quedan en los hosts. Para copiar los datos de un volumen, [haz una copia de seguridad en un archivo](/docs/guides/volumes-networks/#hacer-y-restaurar-copias-de-seguridad) desde la página Volúmenes.

### ¿Puede Homeport compilar imágenes?

Sí, en dos sitios. La página **Imágenes** compila una imagen a partir de una carpeta de este equipo. Y un [stack vinculado](/docs/guides/stacks/#stacks-vinculados) —un stack cuyos archivos compose están en una carpeta de este equipo— puede usar `build:`: Homeport ejecuta `docker compose` en esa carpeta, así que el contexto de compilación se envía desde ahí al Engine del host, que lo compila. **Recompilar y desplegar** vuelve a compilar después de un cambio en el código. Un stack gestionado no puede compilar, ya que Homeport solo guarda su YAML y no hay ninguna carpeta desde la que compilar: vincula la carpeta en su lugar, o sube la imagen a un registro.

### ¿Por qué mi stack aparece como externo en mi otro equipo?

Un stack iniciado con `docker compose up` en una carpeta aparece como [vinculado](/docs/guides/stacks/#cómo-se-encuentran-los-stacks-vinculados) solo en un equipo donde existan esa carpeta y sus archivos compose, en la misma ruta, y definan los servicios que se ejecutan. En otro equipo la ruta no existe, así que Homeport no tiene ningún archivo compose para él y lo muestra como externo. Para gestionarlo también desde allí, clona el proyecto en ese equipo y vincúlalo con **Nuevo stack → Usar una carpeta de este equipo**, o adóptalo como stack gestionado.

### ¿Homeport recopila datos sobre mí?

No. No hay cuenta, ni telemetría, ni analíticas. La única petición que Homeport hace por su cuenta desde tu equipo es la búsqueda de actualizaciones, que descarga un archivo pequeño de este sitio web y no envía nada sobre ti; puedes desactivarla en **Ajustes → Actualizaciones**. Aparte, la comprobación de imágenes más recientes pregunta a los registros de tus imágenes, a través del Engine de cada host, a qué versión apuntan sus etiquetas; para un host local, esa petición también sale de tu equipo. Consulta [seguridad y tus datos](/docs/reference/security/#a-qué-se-conecta-homeport).

### ¿Puede Homeport usar mis registros privados?

Sí. Añade un inicio de sesión para cada registro en **Ajustes → Registros**: Docker Hub, GHCR o el tuyo propio. Homeport se lo pasa al Engine para descargar, subir y compilar en cualquier host, y descarga con él las imágenes de un stack gestionado, vinculado o sincronizado antes de desplegarlo. Las contraseñas y los tokens se cifran con el llavero de tu sistema. Homeport no lee los inicios de sesión de `docker login` de este equipo.

### ¿Cómo sabe Homeport que existe una imagen más reciente?

Pregunta al registro de la imagen, a través del Engine del host, a qué digest apunta ahora la etiqueta, y lo compara con el que descargó el host. No se descarga nada, pero Docker Hub cuenta cada pregunta en su límite de descargas, como un `docker pull` que no encuentra nada nuevo. Por eso Homeport guarda cada respuesta durante 6 horas, pregunta como mucho a cuatro registros a la vez y solo vuelve a preguntar antes cuando eliges **Buscar imágenes más recientes**. Un inicio de sesión de Docker Hub en **Ajustes → Registros** aumenta el límite. Si no se puede llegar a un registro o está limitando las peticiones, Homeport no muestra ninguna marca para sus imágenes y escribe una línea en su log.

### ¿Cómo actualizo?

En Windows, en macOS y con la AppImage, Homeport te avisa cuando sale una versión nueva: elige **Instalar y reiniciar**. En Linux, los paquetes `.deb` y `.rpm` se actualizan con el resto del sistema desde el repositorio de paquetes de Homeport. Consulta [actualizar](/docs/reference/updating/).

### ¿Está Homeport en Homebrew, winget, Flathub o el AUR?

Todavía no. Están previstos; la [página de instalación](/docs/installation/#próximamente) enlaza con la issue de cada uno.

### ¿Está Homeport disponible en mi idioma?

Homeport está en inglés, francés, español, alemán, italiano, portugués de Brasil, ruso, chino simplificado, japonés, coreano, hindi y árabe, y sigue el idioma de tu sistema operativo salvo que elijas uno en [Ajustes → Apariencia](/docs/guides/settings/#apariencia). El sitio web y esta documentación también están en todos ellos; solo las páginas legales están en inglés. Para añadir un idioma, consulta [Contribuir](/docs/project/contributing/#traducir-homeport).

### ¿Es Homeport accesible?

Se puede usar solo con el teclado, sigue el ajuste de movimiento reducido de tu sistema, cumple el contraste WCAG 2.2 AA, y cada página se comprueba con herramientas automáticas de accesibilidad. Hay en curso una auditoría práctica con lectores de pantalla. Consulta [Teclado y accesibilidad](/docs/reference/accessibility/) para saber qué funciona, los límites conocidos y cómo notificar una barrera.

### ¿Hay una interfaz de línea de comandos?

No: Homeport es una app de escritorio. Tu CLI `docker` de siempre funciona junto a ella, y Homeport se actualiza en directo cuando cambias cosas desde el terminal.

### ¿Cómo puedo ayudar?

Notifica errores y sugiere funciones en las [issues de GitHub](https://github.com/DamienReichhart/Homeport/issues), o consulta [contribuir](/docs/project/contributing/).
