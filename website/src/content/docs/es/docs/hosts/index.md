---
title: Hosts y conexiones
description: Cómo se conecta Homeport a Docker Engines, containerd e Incus, qué significa cada estado de un host y cómo funciona la reconexión.
---

Un **host** es un Docker Engine, containerd o Incus al que se conecta Homeport. Tiene un nombre y una forma de acceder a él. Cada contenedor, instancia, imagen, volumen, red y stack que ves pertenece exactamente a un host.

## Tipos de conexión

| Tipo             | Accede a un Engine…                                     | Credenciales                                                                         | Guía                                                           |
| ---------------- | ------------------------------------------------------- | ------------------------------------------------------------------------------------ | -------------------------------------------------------------- |
| **Socket local** | en este equipo, a través de un socket o una tubería     | ninguna                                                                              | [Engines locales](/docs/hosts/local/)                          |
| **SSH**          | en un servidor en el que puedes iniciar sesión          | una clave de Homeport, tu propia clave o ssh-agent                                   | [SSH](/docs/hosts/ssh/)                                        |
| **TLS**          | expuesto con `dockerd --tlsverify`, o el HTTPS de Incus | certificado de CA, certificado y clave de cliente; para Incus, un token de confianza | [TLS y TCP](/docs/hosts/tls-tcp/), [Incus](/docs/hosts/incus/) |
| **TCP**          | expuesto sin cifrado, en una red de confianza           | ninguna                                                                              | [TLS y TCP](/docs/hosts/tls-tcp/)                              |

Para un servidor remoto, es preferible **SSH**: no necesita exponer nada más allá del puerto SSH y es lo más sencillo de configurar de forma segura.

## Tipos de Engine

Además de cómo acceder a él, cada host indica a qué tipo de Engine apunta, en el campo **Engine** del formulario. Eso decide qué páginas muestra la barra lateral para ese host.

| Engine                    | Qué abarca                                                                  | Páginas                                                                                                         |
| ------------------------- | --------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| **Compatible con Docker** | Docker Engine, Docker Desktop, Colima, OrbStack, Rancher Desktop, Podman    | Contenedores, Imágenes, Volúmenes, Redes, Stacks, Eventos; y en Podman, también sus propias páginas             |
| **containerd**            | El servicio propio de containerd, el de k3s, el containerd de Docker Engine | Contenedores, Imágenes, Instantáneas y Namespaces de containerd; consulta [containerd](/docs/hosts/containerd/) |
| **Incus**                 | Incus 6.0 y posteriores: contenedores de sistema y máquinas virtuales       | Instancias, Imágenes, Perfiles, Almacenamiento y Redes; consulta [Incus](/docs/hosts/incus/)                    |

El formulario elige el Engine a partir de la ruta del socket cuando nombra el socket de containerd o el de Incus. A containerd solo se accede a través de su socket, en este equipo o por SSH, así que TCP y TLS aparecen atenuados para él; a Incus, a través de su socket o por HTTPS (TLS), así que solo TCP aparece atenuado. Los hosts añadidos antes de Homeport 0.18 son compatibles con Docker, sin ningún otro cambio.

**Podman** es un Engine compatible con Docker que Homeport reconoce cada vez que se conecta, porque el mismo socket puede cambiar de producto: el estado del host muestra entonces **Podman** y su versión en lugar de la del Engine, y la barra lateral añade las páginas que solo tiene Podman. Una página que el host seleccionado no puede mostrar no aparece en la barra lateral, y si la abres desde un enlace te dice por qué.

Un host que pertenece a un Docker Swarm muestra **Manager de** o **Worker de** su Swarm, y basta con añadir un manager como host para que el Swarm aparezca en la barra lateral. La página de un host también tiene una tarjeta **Swarm** para iniciar un Swarm, unirse a uno o abandonarlo. Consulta [Swarms](/docs/guides/swarm/).

## Añadir, editar y quitar hosts

- **Añade** un host con **Añadir host…** en el selector de hosts, en la parte superior de la barra lateral, o con **Añadir host** en el Resumen.
- **Gestionar hosts**, en el selector de hosts, lista todos los hosts con su tipo, dirección y estado. Selecciona uno para abrir su página.
- En la página de un host puedes cambiar sus datos de conexión y elegir **Guardar**; Homeport vuelve a conectarse con los nuevos ajustes. El tipo de conexión de un host no se puede cambiar: quítalo y vuelve a añadirlo.
- El interruptor **Activado** desconecta un host sin olvidarlo. Un host desactivado no vuelve a intentarlo hasta que lo reactivas.
- **Quitar host** olvida el host, sus credenciales guardadas, las definiciones compose de sus stacks gestionados, sus stacks vinculados y el historial de despliegues de sus stacks. No se toca nada de lo que se ejecuta en el host: los contenedores, las imágenes y los volúmenes siguen como estaban, y las carpetas de los stacks vinculados también. Los stacks de un Swarm que gestiona el host pertenecen al Swarm y se conservan.

Los nombres de host deben ser únicos, sin distinguir mayúsculas de minúsculas.

## Importar hosts

**Añadir host** ofrece dos formas de añadir muchos hosts a la vez, a partir de lo que este equipo ya describe. Cada una abre una vista previa: elige las entradas que quieres importar, cámbiales el nombre y elige **Importar**.

- **Importar desde ~/.ssh/config** añade los servidores que nombra tu configuración de SSH, con su usuario, puerto, clave y bastiones `ProxyJump`. Consulta [Importar desde ~/.ssh/config](/docs/hosts/ssh/#importar-desde-sshconfig).
- **Importar contextos de Docker** añade los Engines de los contextos de la CLI de Docker (`docker context ls`), con sus certificados TLS.

Las claves y los certificados se leen una sola vez, al importar, y se guardan cifrados como cualquier otro; ni `~/.ssh` ni `~/.docker` se modifican nunca.

### Contextos de Docker

Homeport lee todos los contextos de `~/.docker/contexts` (o de `$DOCKER_CONFIG` si está definido) y asigna su endpoint de Docker:

| Endpoint                                               | Se convierte en                                               |
| ------------------------------------------------------ | ------------------------------------------------------------- |
| `unix:///…`, `npipe:////./pipe/…`                      | un host de **Socket local**                                   |
| `tcp://host:port` sin archivos TLS                     | un host **TCP**                                               |
| `tcp://host:port` con `ca.pem`, `cert.pem` y `key.pem` | un host **TLS**, con sus certificados                         |
| `ssh://user@host:port`                                 | un host **SSH** que usa ssh-agent, con `/var/run/docker.sock` |

El contexto integrado `default` no se ofrece: la [detección local](/docs/hosts/local/) ya encuentra el Engine de este equipo. Un contexto se muestra pero no se puede elegir cuando:

- define `SkipTLSVerify`, ya que Homeport siempre comprueba el certificado del Engine;
- su carpeta TLS contiene solo algunos de los tres archivos;
- su endpoint es de otro tipo, como uno de Kubernetes;
- un host ya accede al mismo Engine. Su nombre indica cuál.

Un contexto cuyo nombre ya es el de un host se ofrece como `name (2)`.

## Pasar a otro equipo

**Exportar**, en la página Hosts, guarda los hosts que elijas en un archivo protegido con una frase de contraseña; **Importar**, en la página Hosts del otro equipo, lo abre y añade los hosts que elijas, listos para conectarse. Un host cuyo nombre ya está en uso allí se propone como `name (2)`, y puedes cambiarle el nombre u omitirlo. Consulta [pasar a otro equipo](/docs/reference/security/#pasar-a-otro-equipo) para ver cómo se protege el archivo. Para moverlo todo a la vez, con clústeres, stacks, inicios de sesión de registros, repositorios y ajustes incluidos, usa en su lugar una [copia de seguridad](/docs/reference/data-and-backups/#backing-up-in-homeport).

## Seleccionar un host

Cada página de recursos —Contenedores, Imágenes, Volúmenes, Redes, Stacks y Eventos— muestra el host seleccionado en el **selector de hosts**. Las páginas de un host de containerd también muestran un namespace a la vez, elegido [bajo el selector](/docs/hosts/containerd/#namespaces), y las de un host de Incus, un [proyecto](/docs/hosts/incus/#proyectos). Cambia de host ahí, o selecciona la tarjeta de un host en el [Resumen](/docs/guides/overview/). Homeport recuerda tu elección entre sesiones.

## Estados

Cada host muestra un estado junto a su nombre. Cuando el host seleccionado no está conectado, las páginas de recursos muestran su estado con **Reintentar ahora** y **Abrir ajustes del host**.

| Estado                              | Significado                                                                         | Qué pasa después                                                                              |
| ----------------------------------- | ----------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------- |
| **Conectado**                       | El Engine respondió. Se muestra su versión, o la de Podman, containerd o Incus.     | Homeport sigue vigilando la conexión.                                                         |
| **Conectando**                      | Homeport está accediendo al Engine.                                                 | —                                                                                             |
| **Desactivado**                     | El interruptor **Activado** del host está apagado.                                  | Nada, hasta que lo actives.                                                                   |
| **Inaccesible**                     | La red, el socket o el Engine no respondieron.                                      | Lo vuelve a intentar solo, esperando cada vez más.                                            |
| **Acceso denegado**                 | El servidor SSH rechazó la clave, ssh-agent falló o falló el handshake TLS.         | Corrige la causa y luego **Volver a conectar**.                                               |
| **Clave del servidor no confiable** | Primera conexión SSH: la clave del servidor o de un bastión necesita tu aprobación. | [Confía en la clave](/docs/hosts/ssh/#confía-en-la-clave-del-servidor) en la página del host. |
| **La clave del servidor cambió**    | El servidor SSH presentó una clave distinta de la que aceptaste.                    | [Investiga primero](/docs/hosts/ssh/#cuando-cambia-la-clave-del-servidor).                    |
| **Credenciales perdidas**           | Las credenciales guardadas no se pueden descifrar.                                  | [Vuelve a introducirlas](/docs/reference/troubleshooting/#credenciales-perdidas).             |
| **Llavero no disponible**           | El host necesita secretos guardados, pero no hay llavero del sistema.               | Consulta los [requisitos](/docs/requirements/#opcional-un-llavero-del-sistema).               |
| **Mal configurado**                 | Falta algo en los ajustes del host o no es válido.                                  | El mensaje dice qué; corrígelo y **Guardar**.                                                 |

## Reconexión

Homeport se conecta a todos los hosts activados al arrancar, y cada host tiene su propia conexión, así que un servidor lento nunca retrasa a otro.

Mientras está conectado, Homeport escucha el flujo de eventos del Engine —así es como las páginas se actualizan solas— y comprueba el Engine cada 15 segundos. Si la conexión se cae, el host pasa a **Inaccesible** y Homeport lo vuelve a intentar al cabo de 1 segundo, luego de 2, 4, 8 y así sucesivamente, hasta una vez por minuto. Una conexión correcta reinicia la espera.

Solo **Inaccesible** se reintenta automáticamente. Cualquier otro problema necesita algo por tu parte, y luego **Volver a conectar** (en la página del host) o **Reintentar ahora** lo intenta de nuevo al momento.

## Eventos y actualizaciones en directo

Homeport no hace sondeos. Cuando el Engine notifica un cambio —un contenedor que se inicia, una imagen que se descarga—, las páginas afectadas se actualizan en una fracción de segundo. La página [Eventos](/docs/guides/overview/#eventos) muestra el flujo sin procesar.
