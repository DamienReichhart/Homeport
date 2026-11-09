---
title: Glosario
description: Las palabras que usa Homeport, y qué significan.
---

Homeport nombra las cosas con precisión. Estos términos aparecen en toda la app y en esta documentación. Cada entrada lleva el término en inglés del producto, seguido de la palabra que usa la app en español.

### Host (es. _host_)

Un Docker Engine al que se conecta Homeport, definido por un nombre y por cómo acceder a él: un socket local, TCP, TLS o SSH. Cada contenedor, imagen, volumen, red y stack pertenece exactamente a un host. Consulta [Hosts y conexiones](/docs/hosts/).

### Engine kind (es. _tipo de Engine_)

El tipo de Engine al que apunta un host: compatible con Docker (Docker Engine, o Podman a través de su API compatible), containerd o Incus. Decide qué páginas muestra un host: los hosts de containerd y de Incus tienen las suyas. Podman no es un tipo aparte: Homeport lo reconoce cada vez que se conecta. Consulta [Tipos de Engine](/docs/hosts/#engine-kinds).

### Namespace (containerd's) (es. _namespace de containerd_)

En un host de containerd, el espacio en el que vive cada contenedor, imagen e instantánea: nerdctl usa `default`, Kubernetes `k8s.io`, Docker `moby`. Las páginas de un host de containerd muestran uno a la vez. Los Namespaces de un clúster son los de Kubernetes. Consulta [namespaces de containerd](/docs/hosts/containerd/#namespaces).

### Snapshot (es. _instantánea_)

En un host de containerd, una capa de archivos en el disco: una capa de imagen desempaquetada, o la capa modificable de un contenedor. Consulta [instantáneas](/docs/hosts/containerd/#snapshots). En un host de Incus, un estado guardado de una instancia al que se puede restaurar; consulta [instantáneas de instancia](/docs/guides/instances/#snapshots).

### Instance (es. _instancia_)

Lo que ejecuta un host de Incus: un contenedor de sistema, que arranca una distribución Linux completa sobre el kernel del host, o una máquina virtual con su propio kernel. No es un contenedor, que ejecuta una sola aplicación. Consulta [Instancias](/docs/guides/instances/).

### Project (Incus's) (es. _proyecto_)

En un host de Incus, el espacio en el que viven sus instancias, imágenes, perfiles, volúmenes personalizados y redes. Las páginas de un host de Incus muestran uno a la vez, elegido bajo el selector de hosts. Consulta [proyectos](/docs/hosts/incus/#projects).

### Profile (es. _perfil_)

En un host de Incus, configuración y dispositivos compartidos por las instancias que lo listan. Consulta [perfiles](/docs/guides/instances/#profiles).

### Trust token (es. _token de confianza_)

Lo que muestra `incus config trust add` en un servidor de Incus, y que se pega una vez en Homeport para añadir ese servidor por HTTPS. Consulta [tokens de confianza](/docs/hosts/incus/#over-https-with-a-trust-token).

### Pod (es. _pod de Podman_)

En un host de Podman, un grupo de contenedores que comparten una red y se inician y se detienen juntos, en torno a un contenedor infra que mantiene los puertos que publica el pod. Los Pods de un clúster son Pods de Kubernetes. Consulta [Pods de Podman](/docs/guides/podman-pods/).

### Quadlet unit (es. _unidad Quadlet_)

Un archivo en un host de Podman, como `web.container`, que systemd convierte en un servicio, `web.service`, para que los contenedores de Podman se inicien con el arranque y systemd los mantenga en ejecución. Consulta [Unidades Quadlet](/docs/guides/quadlet/).

### Cluster (es. _clúster_)

Un clúster de Kubernetes o de Nomad al que se conecta Homeport a través de su API. Uno de Kubernetes se añade desde un contexto de un kubeconfig: un nombre, la dirección del servidor de API y las credenciales que da el kubeconfig, guardadas cifradas. Uno de Nomad se añade por la dirección de uno de sus agentes, con un token ACL y material TLS, guardados cifrados. Homeport no instala nada en ninguno de los dos y solo llega a lo que permiten esas credenciales. No es un Swarm, que se encuentra a través de sus hosts y nunca se añade. Los objetos propios de Kubernetes conservan sus nombres de Kubernetes: Namespace, Pod, Deployment, Service, Node… Los de Nomad conservan los de Nomad: job, task group, task, allocation, nodo cliente, namespace. Consulta [Clústeres](/docs/clusters/) y [Clústeres de Nomad](/docs/clusters/nomad/).

### Allocation (es. _allocation_)

En un clúster de Nomad, una colocación del task group de un job en un nodo cliente, que ejecuta sus tasks. Sus logs y un shell en ella pasan por los servidores de Nomad hasta el nodo. Consulta [logs y shell](/docs/clusters/nomad/#logs-and-shell).

### Swarm (es. _Swarm_)

Un clúster de Docker Swarm, que se encuentra a través de los hosts que pertenecen a él y nunca se añade por sí solo. Homeport accede a él a través de cualquier host manager conectado, distingue los Swarms por su ID de swarm y no guarda nada sobre ellos. Se llama como su primer host manager, ya que Docker da el mismo nombre a todos los Swarms. Consulta [Swarms](/docs/guides/swarm/).

### Port forward (es. _reenvío de puerto_)

Un puerto del loopback de este equipo que Homeport conecta, a través de la API de Kubernetes, con un puerto de un Pod, o de un Service a través de uno de sus Pods listos, como `kubectl port-forward`. Termina cuando Homeport se cierra. Consulta [Reenvío de puertos](/docs/clusters/port-forward/).

### Release (es. _release_)

Un release de Helm: una instalación de un chart en un clúster, con nombre en su Namespace, con una revisión numerada por cada instalación, actualización y reversión. Helm lo guarda en el clúster; un release creado en un terminal y uno creado en Homeport son lo mismo. Consulta [Releases de Helm](/docs/clusters/helm/).

### Chart repository (es. _repositorio de charts_)

Un servidor del que helm descarga charts, en la lista propia de helm en este equipo: la que muestra `helm repo list`. Los charts de un registro OCI se instalan en cambio por su referencia `oci://`. Consulta [repositorios de charts](/docs/clusters/helm/#chart-repositories).

### Node (es. _nodo_)

Un Engine de un Swarm: un manager o un worker. Los Nodes de un clúster son Nodes de Kubernetes, las máquinas en las que ejecuta Pods. Un nodo solo es un host cuando Homeport se conecta a él, y la página de un host enlaza con su nodo.

### Service (es. _servicio_)

Lo que ejecuta un Swarm: una imagen, sus ajustes y un número de réplicas, o una tarea en cada nodo para un servicio global.

### Task (es. _tarea_)

Un hueco de un servicio en un nodo, que se ejecuta allí como un contenedor. Cuando un servicio no converge, sus tareas dicen por qué. En un host de containerd, una tarea es lo que se ejecuta para un contenedor, que son solo metadatos: iniciar un contenedor crea su tarea.

### Stack (es. _stack_)

Un proyecto de Compose que se ejecuta en un host, o un stack de Swarm que se ejecuta en un Swarm, identificado por su nombre de proyecto. Cada stack es gestionado, vinculado, sincronizado o externo. Consulta [Stacks](/docs/guides/stacks/) y [stacks de Swarm](/docs/guides/swarm/#swarm-stacks).

### Managed Stack (es. _stack gestionado_)

Un stack cuya definición compose y cuyo entorno guarda Homeport, de modo que se puede desplegar, volver a desplegar y editar desde Homeport.

### Linked Stack (es. _stack vinculado_)

Un stack cuyos archivos compose son una carpeta de este equipo. Homeport guarda dónde están la carpeta y sus archivos compose, lee y escribe los archivos en su sitio, y despliega con esa carpeta como directorio del proyecto, así que `build:`, `env_file:` y las rutas relativas funcionan. Se encuentra a partir de las etiquetas compose de los contenedores en ejecución, o se crea a partir de una carpeta. Pasa a ser externo mientras faltan sus archivos. Consulta [Stacks vinculados](/docs/guides/stacks/#linked-stacks).

### Synced Stack (es. _stack sincronizado_)

Un stack cuya definición compose viene de un repositorio git. Homeport guarda un clon en este equipo, obtiene una rama o una etiqueta bajo demanda o a intervalos mientras Homeport está abierto, muestra lo que ha cambiado y despliega un commit desde la carpeta del archivo compose, así que `build:` y las rutas relativas funcionan. Nunca se edita en Homeport: sus archivos cambian a través de git; solo su entorno y sus ajustes de sincronización son de Homeport. Consulta [Stacks sincronizados](/docs/guides/synced-stacks/).

### Repository (es. _repositorio_)

Un repositorio git que Homeport puede leer para los stacks sincronizados: su URL y cómo se autentica Homeport en él: una clave de despliegue que generó, un nombre de usuario y un token, o nada para uno público. Se guarda una sola vez para todos los hosts, y lo comparten todos los stacks sincronizados creados a partir de él.

### Deploy key (es. _clave de despliegue_)

La clave Ed25519 que Homeport genera para un repositorio, y que añades a él en tu forja de git con acceso de solo lectura. Nunca es tu propia clave SSH.

### External Stack (es. _stack externo_)

Un stack que se encuentra en ejecución en un host y cuya definición compose Homeport no tiene, ni guardada, ni en una carpeta de este equipo, ni en un repositorio que sigue. Se puede iniciar, detener y eliminar, pero no volver a desplegar ni editar.

### Adopt (es. _adoptar_)

Convertir un stack externo en un stack gestionado aportando su definición compose.

### Registry login (es. _inicio de sesión del registro_)

El nombre de usuario y la contraseña o el token de acceso que Homeport guarda para un registro de contenedores, como Docker Hub o GHCR, y que usa en todos los hosts para descargar, subir y compilar. Consulta [Ajustes](/docs/guides/settings/#registries).

### Helper container (es. _contenedor auxiliar_)

Un contenedor `busybox` de corta duración que Homeport crea en un host para abrir un volumen —para explorarlo, hacer una copia de seguridad, restaurarlo o clonarlo—, ya que la API del Engine no puede leerlo. No tiene red, monta el volumen en solo lectura salvo que tenga que escribir en él, y se elimina cuando termina la acción. Nunca aparece en la lista de contenedores ni en la página Eventos. Consulta [Explorar los archivos de un volumen](/docs/guides/volumes-networks/#the-helper-container).

### Board (es. _Tablero_)

La página que lista los contenedores de todos los hosts conectados en una sola tabla, cada uno etiquetado con su host. Consulta [Tablero](/docs/guides/overview/#board).

### Command palette (es. _paleta de comandos_)

El cuadro de búsqueda que se abre con <kbd>Mod</kbd> <kbd>K</kbd>: ir a una página, cambiar de host, ejecutar un comando o encontrar un contenedor, una imagen, un volumen, una red o un stack en cualquier host conectado. Consulta [Teclado](/docs/guides/keyboard/#command-palette).

### Engine (es. _Engine_)

El Docker Engine (`dockerd`), o uno compatible como Podman, que ejecuta contenedores en una máquina. Homeport habla con él a través de la API de Docker Engine.

### Socket (es. _socket_)

La dirección en la que escucha un Engine: un archivo de socket Unix como `/var/run/docker.sock`, o una tubería con nombre como `//./pipe/docker_engine` en Windows.

### Keychain (es. _llavero del sistema_)

El almacén seguro de contraseñas y claves de tu sistema operativo —el Llavero de macOS, el Administrador de credenciales de Windows o un Secret Service de Linux—, donde Homeport guarda la clave que cifra sus secretos.

### Trust on first use (es. _confianza en el primer uso_)

Cómo comprueba Homeport los servidores SSH: confirmas la huella de la clave de un servidor en la primera conexión, y Homeport se niega a conectarse si la clave cambia después, hasta que vuelvas a confirmarla.

## En español

La app, la documentación y el sitio web en español usan estos términos, para que una pantalla y una página digan siempre lo mismo.

| Inglés         | Español                  |
| -------------- | ------------------------ |
| Host           | host (m.)                |
| Engine         | Engine (m.)              |
| Instance       | instancia (f.)           |
| Swarm          | Swarm (m.)               |
| Cluster        | clúster (m.)             |
| Stack          | stack (m.)               |
| Managed Stack  | stack gestionado         |
| Linked Stack   | stack vinculado          |
| Synced Stack   | stack sincronizado       |
| External Stack | stack externo            |
| Adopt          | adoptar                  |
| Board          | Tablero                  |
| Bastion        | bastión (m.)             |
| Deploy key     | clave de despliegue (f.) |
| Port forward   | reenvío de puerto (m.)   |
| Release (Helm) | release (m.)             |
