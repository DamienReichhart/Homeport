---
title: Solución de problemas
description: Soluciones para los problemas que más probablemente encontrarás, según el mensaje que muestra Homeport.
---

Si tu problema no está aquí, [abre una issue](https://github.com/DamienReichhart/Homeport/issues/new/choose) con el [archivo de diagnóstico](#exportar-el-diagnóstico-para-un-informe-de-error) de **Ajustes → Acerca de**.

## Instalar e iniciar

### Windows dice «Windows protegió su PC»

El instalador todavía no está firmado con un certificado comercial. Elige **Más información** y luego **Ejecutar de todas formas**. Puedes [verificar la descarga](/docs/reference/verifying/) antes.

### macOS dice que Homeport «no se puede abrir»

Homeport todavía no está notarizado por Apple. Abre **Ajustes del Sistema → Privacidad y seguridad** y elige **Abrir igualmente** junto a Homeport, o ejecuta:

```sh
xattr -dr com.apple.quarantine /Applications/Homeport.app
```

### La AppImage no arranca

Si menciona FUSE, instala `libfuse2` (`libfuse2t64` en Ubuntu 24.04 y posteriores). Comprueba que el archivo es ejecutable: `chmod +x Homeport*.AppImage`.

### Homeport se abre en el lugar equivocado

Homeport se vuelve a abrir con el tamaño, la posición y el estado maximizado que tenía su ventana al cerrarla. Si la pantalla en la que estaba ya no existe, se abre centrado en una de las que quedan. Para olvidar la posición guardada, cierra Homeport y borra `.window-state.json` de su carpeta de ajustes:

| Sistema | Archivo                                                             |
| ------- | ------------------------------------------------------------------- |
| Linux   | `~/.config/com.homeport.app/.window-state.json`                     |
| macOS   | `~/Library/Application Support/com.homeport.app/.window-state.json` |
| Windows | `%APPDATA%\com.homeport.app\.window-state.json`                     |

### Volver a abrir Homeport no abre una segunda ventana

Solo se ejecuta un Homeport a la vez: abrirlo cuando ya está en ejecución trae su ventana al frente, incluso cuando está oculto en la [bandeja del sistema](/docs/guides/overview/#bandeja-del-sistema). Dos copias se conectarían dos veces a cada host y escribirían en los mismos datos.

### No hay icono en la bandeja

En Linux, la bandeja necesita la biblioteca appindicator, y algunos escritorios ocultan los iconos de la bandeja: en GNOME, instala la extensión AppIndicator Support. Si Homeport no encuentra ninguna biblioteca appindicator, **Ajustes → Escritorio** lo indica y al cerrar la ventana se sale de la app. Consulta los [requisitos](/docs/requirements/#opcional-una-bandeja-del-sistema).

### Un aviso rojo dice que no hay ningún llavero del sistema disponible

Homeport no pudo acceder a tu llavero. En Linux, esto suele significar que no hay ningún Secret Service en ejecución, algo habitual en escritorios mínimos y gestores de ventanas. Instala e inicia GNOME Keyring (`gnome-keyring`) o activa el Secret Service de KWallet, y luego reinicia Homeport. Mientras tanto, usa hosts de socket local, TCP o ssh-agent.

### «Esta base de datos la usó por última vez Homeport … o una versión posterior»

Instalaste un Homeport más antiguo que el que abrió tus datos por última vez, por ejemplo al reinstalar una versión anterior. Cada versión actualiza la base de datos a su propio formato al arrancar, y una versión más antigua no puede leer un formato más reciente, así que Homeport se detiene antes de cambiar nada: la ventana muestra este mensaje, la ruta de la base de datos y **Abrir carpeta**, y no se inicia nada más.

Tus hosts, stacks y secretos están intactos. Instala la versión que indica el mensaje, o cualquiera más reciente, desde la [página de descarga](/download/), y Homeport los abre como antes. No borres ni muevas `homeport.db`: es la única forma de perder lo que contiene. Volver a una versión anterior no se admite; consulta [Tus datos entre versiones](/docs/reference/updating/#tus-datos-entre-versiones).

La misma ventana aparece ante cualquier otro fallo al abrir la base de datos (archivo dañado, disco lleno, otro Homeport que la tiene abierta), con el mensaje propio de ese fallo y lo que hay que hacer.

## Actualizaciones y el repositorio de paquetes

### «Homeport no pudo descargar la actualización»

Cuando aparece al buscar actualizaciones, Homeport no pudo descargar la información de actualización de este sitio web: puede que no tengas conexión, o que estés detrás de un proxy o un cortafuegos que bloquea `damienreichhart.github.io`. Homeport sigue funcionando; vuelve a comprobarlo más tarde, o elige **Ajustes → Actualizaciones → Comprobar ahora**. Siempre puedes descargar la última versión desde la [página de descarga](/download/).

### Una actualización no se instala

Homeport solo instala una actualización cuya firma coincide con su clave de actualización, y se detiene si la descarga está incompleta o alterada. Vuelve a intentarlo más tarde; si sigue fallando, descarga la nueva versión desde la [página de descarga](/download/) e instálala encima de la antigua.

### apt dice «NO_PUBKEY» o «The repository is not signed»

Falta la clave de Homeport en `/etc/apt/keyrings/homeport.asc`. Vuelve a ejecutar la [instalación rápida](/docs/installation/#instalación-rápida), o los dos primeros comandos de la [configuración manual](/docs/installation/#configuración-manual).

### dnf o zypper pregunta si importar una clave

Es la clave de paquetes de Homeport, la primera vez que tu gestor de paquetes usa el repositorio. Comprueba que la huella que muestra coincide con la de [verificar descargas](/docs/reference/verifying/#firmas-de-los-paquetes), y luego acepta.

### apt dice «Hash Sum mismatch» justo después de una release

El repositorio se estaba actualizando mientras apt lo leía. Vuelve a ejecutar `sudo apt update` unos minutos más tarde.

### Mi gestor de paquetes no ve la nueva versión

Comprueba que el repositorio sigue configurado: `/etc/apt/sources.list.d/homeport.sources`, `/etc/yum.repos.d/homeport.repo` o `/etc/zypp/repos.d/homeport.repo`. Si lo borraste, ejecuta la [instalación rápida](/docs/installation/#instalación-rápida) para volver a añadirlo. Si instalaste la 0.2.0 o una anterior desde un archivo descargado, el repositorio nunca se añadió: consulta [actualizar desde la 0.2.0](/docs/reference/updating/#actualizar-desde-la-020-o-anteriores).

## Conectarse

### «Homeport no tiene permiso para usar el Docker Engine en …»

Tu usuario no puede abrir el socket de Docker. En Linux, añádelo al grupo `docker` y vuelve a iniciar sesión:

```sh
sudo usermod -aG docker $USER
```

### «No hay ningún Docker Engine en ejecución en …»

No hay nada escuchando en la ruta del socket. Cuando no existe nada en esa ruta, el mensaje es en cambio _Parece que el Docker Engine no está instalado_. Inicia tu Engine —Docker Desktop, Colima, OrbStack, Rancher Desktop— o comprueba la ruta. Consulta [Engines locales](/docs/hosts/local/#añadir-un-socket-a-mano).

### «El Docker Engine no respondió a tiempo» o «… no respondió a tiempo»

Homeport espera 10 segundos a una conexión. Comprueba la dirección y el puerto, que el servidor está en marcha y que ningún cortafuegos se interpone. Homeport sigue reintentándolo solo.

### Acceso denegado: «El servidor SSH no permitió que … iniciara sesión»

El servidor todavía no acepta la clave de Homeport. Ejecuta en el servidor el comando **Añade la clave pública** de la página del host y luego elige **Volver a conectar**. Consulta [Hosts SSH](/docs/hosts/ssh/#autoriza-la-clave-de-homeport).

### «… no puede abrir el socket del Docker Engine en /var/run/docker.sock en …»

Homeport inició sesión, pero no pudo acceder a Docker. Cuando puede saber por qué, el mensaje lo dice en su lugar: el Engine no está instalado o no se está ejecutando en el servidor, el usuario no tiene permiso para usarlo, o el servidor SSH no permite el reenvío a su socket. Comprueba:

- la **Ruta del socket remoto**: Docker sin root usa `/run/user/<uid>/docker.sock`;
- que el usuario puede usar Docker: `sudo usermod -aG docker <user>` en el servidor;
- que el servidor permite el reenvío de sockets (`AllowStreamLocalForwarding` en `sshd_config`, activado por defecto).

### «No hay ningún ssh-agent en ejecución»

Inicia tu agente SSH y añade tu clave con `ssh-add`. Si abriste Homeport desde un menú del escritorio, puede que no vea el agente que usa tu terminal; cierra la sesión y vuelve a iniciarla, o inicia Homeport desde un terminal en el que `ssh-add -l` liste tu clave.

En Windows, Homeport usa el servicio OpenSSH Authentication Agent o el Pageant de PuTTY. Abre Pageant y carga tu clave, o inicia el servicio en un PowerShell de administrador:

```powershell
Get-Service ssh-agent | Set-Service -StartupType Automatic
Start-Service ssh-agent
ssh-add
```

### Acceso denegado: «Falló la conexión segura con …»

- El certificado de CA debe ser el que firmó el certificado del servidor.
- La dirección que introdujiste debe coincidir con un nombre o una dirección IP del certificado del servidor.
- El servidor debe ejecutarse con `--tlsverify` y confiar en la CA de tu certificado de cliente.

### La clave del servidor cambió

El servidor SSH presentó una clave distinta de la que aceptaste. Si reinstalaste el servidor o cambiaste sus claves, elige **Confiar en la clave nueva**. Si no sabes por qué cambió, no te conectes: pregunta a quien administra el servidor. Consulta [Hosts SSH](/docs/hosts/ssh/#cuando-cambia-la-clave-del-servidor).

### Credenciales perdidas

La clave que cifra tus secretos falta en el llavero o ha cambiado: tras restablecer el llavero, con un perfil de usuario nuevo o al copiar los datos de Homeport a otro equipo (para eso, usa una [copia de seguridad](/docs/reference/data-and-backups/#hacer-una-copia-de-seguridad-en-homeport), o [Exportar e Importar](/docs/reference/security/#pasar-a-otro-equipo)). Homeport conserva el host; vuelve a introducir sus credenciales en la página del host:

- **TLS**: vuelve a cargar los tres archivos de certificado y elige **Guardar**.
- **SSH con una clave importada**: vuelve a pegar la clave privada y la frase de contraseña y elige **Guardar**.
- **SSH con una clave de Homeport**: elige **Regenerar clave** y luego autoriza la clave nueva en el servidor.

Los entornos de los stacks gestionados y sincronizados, los inicios de sesión de registros y las claves y tokens de los repositorios cifrados con la clave antigua también hay que volver a introducirlos, y hay que actualizar los clústeres desde su kubeconfig; [Si se pierden los secretos](/docs/reference/data-and-backups/#si-se-pierden-los-secretos) lista cada uno.

## Clústeres

### Acceso denegado: «… rechazó las credenciales de su kubeconfig»

El servidor de API respondió 401: el certificado o el token ha caducado o se ha revocado. Comprueba que `kubectl --context <context> get namespaces` sigue funcionando y luego usa **Actualizar desde el kubeconfig** en la página del clúster. Con un plugin exec, Homeport ya le pidió un token nuevo una vez.

### «… no está instalado en este equipo»

El [plugin exec](/docs/clusters/exec-plugins/) del clúster (`aws`, `gke-gcloud-auth-plugin`, `kubelogin`…) no se encontró en el `PATH` ni en sus lugares de instalación habituales. Instálalo, comprueba que se ejecuta en un terminal y luego **Volver a conectar**.

### «… necesita que inicies sesión, y no puede hacerlo dentro de Homeport»

El plugin exec quiere un navegador o un código de dispositivo. Ejecuta una vez `kubectl --context <context> get namespaces` en un terminal, inicia sesión allí y luego **Volver a conectar**.

### «… no pudo obtener credenciales para …»

El plugin exec se ejecutó y falló. **Detalles** muestra el final de lo que imprimió, a menudo un perfil caducado o una región equivocada. Homeport lo vuelve a intentar solo.

### «Su usuario inicia sesión con el auth provider gcp, que Kubernetes ya no admite»

Los auth providers `gcp` y `azure` se eliminaron de Kubernetes. Vuelve a obtener las credenciales con `gke-gcloud-auth-plugin` instalado (GKE), o ejecuta `kubelogin convert-kubeconfig` (AKS), y luego vuelve a añadir el contexto.

### “Tus credenciales no permiten «…» sobre … en …”

El RBAC del clúster no permite que el usuario de tu kubeconfig haga eso. Elige en el selector de arriba a la derecha un namespace que tengas permiso para ver, o pregunta al administrador del clúster. El mensaje incluye el comando `kubectl auth can-i` que lo comprueba. Las demás páginas siguen funcionando.

### Inaccesible: «… rechazó la conexión» o «… no respondió a tiempo»

No se puede llegar al servidor de API desde este equipo: el clúster está detenido (uno local), está detrás de una VPN o la dirección del servidor en el kubeconfig ha cambiado. Homeport sigue reintentándolo solo; comprueba `kubectl cluster-info` con el mismo contexto.

### «El certificado de … no está firmado por la CA de su kubeconfig»

kind, k3d y minikube crean certificados nuevos cada vez que vuelven a crear un clúster. Usa **Actualizar desde el kubeconfig** en la página del clúster.

### Nomad: «… no conoce el token ACL que envió Homeport»

El token se escribió mal, ha caducado o se ha borrado. Crea uno nuevo con la política de [Políticas de ACL](/docs/clusters/nomad/#políticas-acl) (`nomad acl token create -name=homeport -policy=homeport`) y pega su secret ID en los **Ajustes** del clúster. El clúster espera como **Acceso denegado** hasta que lo hagas.

### Nomad: «… se negó: al token ACL le falta un permiso que esto necesita»

Las políticas del token no conceden lo que necesita esta página o acción; el mensaje dice qué (leer jobs o logs, ejecutar o detener jobs, escalar, abrir un shell, cambiar nodos). Añade la capacidad a la política del token; [Políticas de ACL](/docs/clusters/nomad/#políticas-acl) lista lo que necesita cada cosa. Las demás páginas siguen funcionando.

### Nomad: «… no tiene líder: sus servidores no logran ponerse de acuerdo en uno»

Menos de la mayoría de los servidores del clúster están en ejecución o pueden comunicarse entre sí, así que Nomad no puede responder. Compruébalos con `nomad server members` en un servidor y recupera suficientes. Homeport sigue reintentándolo.

### Nomad: «… no tiene ningún job …»

El job se detuvo y se purgó, o lo eliminó el recolector de basura, o está en otro namespace distinto del elegido. Vuelve a abrir la página Jobs, con **Todos los namespaces** si hace falta.

### Nomad: «El task group … no puede ejecutar más de …» o «menos de …»

El bloque `scaling` del task group define un `min` y un `max`, y el número pedido está fuera de ellos. Cambia los límites en la especificación del job y vuelve a ejecutarlo para ir más allá.

### Nomad: «Nomad no escalará … mientras haya un deployment suyo en curso»

Ejecutar una versión nueva de un job de servicio inicia un deployment, y Nomad se niega a cambiar su número hasta que todas las allocations de la nueva versión estén saludables. Espera a que las allocations del job se estabilicen y vuelve a escalar.

### Nomad: «No se puede analizar la especificación del job, en la línea …, columna …»

El analizador de Nomad encontró un problema en ese lugar; **Detalles** tiene su mensaje. Los bloques `variable` necesitan un valor predeterminado, ya que Homeport no pasa archivos de variables. Una especificación JSON debe ser un job en el formato de la API de Nomad, tal como lo imprime `nomad job inspect`.

### Nomad: «Nomad no puede colocar todas las allocations de …»

El plan no encontró ningún nodo elegible con los recursos que pide el task group, o que cumpla sus restricciones; el plan dice cuáles. **Ejecutar de todos modos** deja en cola lo que no se puede colocar hasta que un nodo pueda aceptarlo; si no, reduce los recursos, cambia las restricciones o añade nodos.

### Nomad: «… cambió desde que se planificó, así que no se ejecutó»

Otra persona ejecutó o detuvo el job entre tu plan y **Ejecutar**, y Homeport no sobrescribe su cambio. Vuelve atrás, planifica de nuevo y revisa el nuevo diff antes de ejecutar.

### Nomad: «La task … de la allocation … no se está ejecutando»

Un shell necesita una task en ejecución: esta no ha empezado, ha terminado o ha fallado. Sus logs se pueden seguir leyendo hasta que el recolector de basura elimine la allocation.

### Nomad: «… no puede alcanzar el nodo en el que se ejecuta la allocation …»

Los logs y los shells pasan por los servidores hasta el nodo cliente, y los servidores no pueden alcanzarlo: el nodo está caído, desconectado o su agente se detuvo. Compruébalo en la página Nodes o con `nomad node status`.

### Helm: «La CLI helm no está instalada en este equipo»

Los [releases de Helm](/docs/clusters/helm/) necesitan `helm` 3 o posterior en este equipo. Instálalo (`brew install helm`, `winget install Helm.Helm` o `snap install helm --classic`) y vuelve a abrir la página Helm; **Ajustes → Este equipo** muestra el que encontró Homeport. Un `helm` 2 se indica como no compatible.

### Helm: «Todavía hay en curso otra instalación, actualización o reversión de este release»

El release está en `pending-…`: una instalación, actualización o reversión no terminó, a menudo porque se interrumpió. helm se niega a cambiarlo hasta que termine. Espera a que termine, o revierte el release a su última revisión desplegada desde su **Historial**.

## Usar Homeport

### Desplegar aparece atenuado: «La CLI docker compose no está instalada en este equipo»

Homeport necesita `docker compose` en este equipo para desplegar stacks. Instala Docker Desktop o la CLI de Docker con el plugin Compose, y luego reinicia Homeport. **Ajustes → Este equipo** muestra si se encuentra.

Si está instalado pero no se encuentra, puede que Homeport tenga un `PATH` distinto del de tu terminal, algo habitual cuando se inicia desde un menú del escritorio en macOS. Asegúrate de que `docker` está en una ubicación estándar como `/usr/local/bin`.

### El despliegue falla: «usa build:»

Un stack gestionado no tiene ninguna carpeta desde la que compilar. O bien [vincula la carpeta del proyecto](/docs/guides/stacks/#crear-un-stack-vinculado) en su lugar —un stack vinculado sí compila—, o compila y sube la imagen a un registro, y luego usa `image:`.

### Mi stack pasó a ser externo

Un [stack vinculado](/docs/guides/stacks/#cuando-la-carpeta-se-mueve-o-desaparece) aparece como externo, marcado con ⚠ **Sus archivos compose ya no están en este equipo**, cuando su carpeta se movió, se renombró o se borró, o lo hizo un archivo compose que contenía. No se pierde nada: vuelve a poner la carpeta donde estaba y el stack vuelve a ser vinculado en la siguiente actualización. Si la carpeta se movió para siempre, elige **Olvidar** en la pestaña **Definición** del stack y luego vuelve a vincularla desde su nuevo lugar con **Nuevo stack → Usar una carpeta de este equipo**.

Un stack iniciado desde una carpeta de otro equipo siempre es externo aquí, ya que este equipo no tiene esa ruta.

### «Modificado fuera de Homeport»

Los archivos compose de un stack vinculado se editaron fuera de Homeport —en tu editor, con `git pull`— desde la última vez que Homeport los leyó. Elige **Volver a cargar** para verlos tal como están. Si estabas editando en Homeport al mismo tiempo, el guardado se rechaza con **… cambió fuera de Homeport desde que lo abriste**, para que ningún cambio se pierda sin avisar: vuelve a cargar y rehaz tu edición, o elige **Sobrescribir** para conservar la tuya. Consulta [editar un stack vinculado](/docs/guides/stacks/#editar-un-stack-vinculado).

### Falla la descarga de una imagen privada

Añade un inicio de sesión para el registro de la imagen en **Ajustes → Registros**, y usa **Comprobar inicio de sesión** para confirmar que el registro lo acepta. GHCR, y las cuentas de Docker Hub con autenticación en dos pasos, necesitan un token de acceso en lugar de la contraseña. El diálogo de descarga indica qué inicio de sesión usará; si dice **de forma anónima**, el registro de la imagen no coincide con ningún inicio de sesión guardado: `ghcr.io/org/app` necesita un inicio de sesión para `ghcr.io`, y `org/app` uno para Docker Hub.

### Falla una compilación

El mensaje nombra el paso que falló, como **La compilación falló en el paso 3/5 (RUN make)**; **Detalles** contiene el final de la salida de la compilación. Homeport usa el compilador clásico del Engine, así que los Dockerfiles que necesitan BuildKit (`RUN --mount`, `--secret`, heredocs) fallan; compílalos desde un terminal. Una imagen `FROM` privada necesita un inicio de sesión para su registro en **Ajustes → Registros**.

### No aparece la marca de imagen más reciente

La marca solo aparece cuando se preguntó al registro y este respondió. Las imágenes compiladas localmente o descargadas por digest nunca la muestran. Si no se puede llegar a un registro o limita las peticiones (Docker Hub cuenta estas comprobaciones en su límite de descargas), Homeport escribe una línea en su [log](/docs/reference/security/#logs) y no muestra nada; además, las respuestas se guardan durante 6 horas, así que elige **Buscar imágenes más recientes** para volver a preguntar ahora. Un inicio de sesión de Docker Hub en **Ajustes → Registros** aumenta el límite.

### Buscar en Docker Hub muestra un error

La búsqueda se ejecuta en el host seleccionado, así que necesita que ese host tenga acceso a internet, y las etiquetas las lee este equipo desde `hub.docker.com`. Cualquiera de las dos cosas puede tener un límite de peticiones o estar bloqueada por un proxy. El diálogo sigue descargando cualquier referencia completa que escribas, como `postgres:17-alpine`.

### La pestaña Archivos dice que el contenedor no tiene las herramientas find/stat

Las imágenes mínimas y distroless no incluyen `find` ni `stat`, que Homeport usa para listar archivos. Usa `docker cp` con esos contenedores.

### El terminal no se abre

El contenedor debe estar en ejecución y tener `/bin/sh` o `/bin/bash`. Usa **Nuevo terminal** para ejecutar otro programa.

### Un volumen o una red no se puede eliminar

El Engine se niega mientras un contenedor, aunque esté detenido, siga usándolo. Elimina antes esos contenedores; la confirmación los lista.

### «Esto requiere Podman 5.8 o posterior»

La API de Podman lee, instala, edita y elimina [unidades Quadlet](/docs/guides/quadlet/) a partir de Podman 5.8, y las lista a partir de 5.7. Actualiza Podman en el host, o usa `podman quadlet` en un terminal allí. Iniciar y detener unidades funciona en cualquier Podman.

### Quadlet: Iniciar y Detener aparecen atenuados

Homeport inicia y detiene el servicio de una unidad con `systemctl`, que solo puede ejecutar por SSH o para un Podman en este equipo (en Linux). Accede al host por SSH en lugar de por TCP o TLS. Una unidad cuyo servicio muestra **aún no generado** no tiene nada que iniciar: el generador de systemd omitió su archivo.

### Quadlet: «El systemd del usuario no se está ejecutando en el host»

Las unidades de usuario (Podman sin root) necesitan la instancia de systemd del usuario, que solo se ejecuta mientras el usuario tiene una sesión iniciada, salvo que el lingering esté activado. Como ese usuario en el host, ejecuta `loginctl enable-linger "$USER"` y vuelve a intentarlo.

### Quadlet: «systemd no permitió que este usuario gestionara …»

Las unidades de sistema (Podman con root) solo las puede iniciar y detener un usuario con permiso para gestionarlas, como `root`, y Homeport nunca usa `sudo`. Conéctate como un usuario así, o usa Podman sin root, cuyas unidades son las del propio usuario.

### containerd: «no tiene permiso para usar containerd en /run/containerd/containerd.sock»

El socket de containerd pertenece a root. Da acceso a un grupo con `gid` bajo `[grpc]` en `/etc/containerd/config.toml`, añade tu usuario (o el usuario SSH) a ese grupo, reinicia containerd y vuelve a iniciar sesión; [la página de containerd](/docs/hosts/containerd/#permitir-que-tu-usuario-use-el-socket) tiene los comandos. Al containerd sin root no se puede acceder de ninguna forma: su socket solo existe dentro del namespace de RootlessKit.

### containerd: «La salida de este contenedor no se registra»

containerd no guarda logs; la herramienta que inició la tarea decide adónde va su salida. Los contenedores creados con `ctr` no suelen enviarla a ninguna parte. Los contenedores de nerdctl y de Kubernetes registran en archivos que Homeport lee; consulta [Logs](/docs/hosts/containerd/#logs).

### containerd: los logs o el terminal dicen que necesitan SSH

Necesitan los archivos de la máquina de containerd, a los que Homeport accede por SSH, o en este equipo para un socket local en Linux. Añade el host por SSH. Si el archivo de log no se puede leer, normalmente pertenece a root: conéctate como un usuario con permiso para leerlo.

### containerd: «no puede descargar imágenes para Homeport»

La descarga pasa por el servicio Transfer de containerd, estable a partir de containerd 2.0. Actualiza containerd, o descarga con `ctr images pull` o `nerdctl pull` en el host; la imagen aparece entonces en la página Imágenes.

### Incus: «no tiene permiso para usar Incus en /var/lib/incus/unix.socket»

El socket de administración de Incus es para el grupo `incus-admin`, y el `unix.socket.user` restringido, para el grupo `incus`. Añade tu usuario (o el usuario SSH) con `sudo usermod -aG incus-admin "$USER"`, y luego cierra la sesión y vuelve a iniciarla, o vuelve a conectarte por SSH. Consulta [los sockets de Incus](/docs/hosts/incus/#sus-sockets).

### Incus: «Homeport no pudo alcanzar Incus en …» al añadirlo con un token de confianza

Incus no escucha en la red hasta que se define `core.https_address`. En el servidor, ejecuta `incus config set core.https_address :8443`, comprueba que el cortafuegos deja pasar el puerto 8443 y crea un token nuevo: el token lista las direcciones en las que escuchaba Incus cuando se creó.

### Incus: «Incus rechazó el token de confianza»

Un token sirve una sola vez, y caduca. Crea uno nuevo en el servidor con `incus config trust add homeport` y pégalo antes de que caduque.

### Incus: «… presentó un certificado con la huella …, no la … de confianza»

El certificado del servidor no es el que aceptó Homeport: Incus se reinstaló, se renovó su certificado u otra cosa responde en esa dirección. Comprueba la huella en el servidor con `incus info`; si es la que se muestra, quita el host y vuelve a añadirlo con un token nuevo. Consulta [certificados fijados](/docs/hosts/incus/#el-certificado-del-servidor-queda-fijado).

### Incus: «Este host no puede ejecutar máquinas virtuales»

Incus necesita KVM (`/dev/kvm`) y QEMU para las máquinas virtuales. Activa la virtualización en el firmware del servidor (o la virtualización anidada para un servidor virtual), o crea un contenedor de sistema en su lugar.

### Incus: «El agente de Incus de la máquina virtual no se está ejecutando»

Exec en una máquina virtual pasa por el `incus-agent` que hay dentro, que las propias imágenes de Incus inician al arrancar. Espera a que termine el arranque, o usa la pestaña **Consola**, que no necesita agente.

### «Algo salió mal»

Una página de Homeport se encontró con un problema inesperado y muestra esto en lugar de su contenido; la barra lateral sigue funcionando. **Volver a cargar** suele recuperar la página, y **Volver al resumen** sale de ella. Si sigue pasando, copia el informe de **Detalles** en [una issue](https://github.com/DamienReichhart/Homeport/issues/new/choose), junto con el [archivo de diagnóstico](#exportar-el-diagnóstico-para-un-informe-de-error).

### «… no está conectado»

La acción necesitaba el host, pero se desconectó. Espera a que vuelva a conectarse, o elige **Reintentar ahora**.

## Exportar el diagnóstico para un informe de error

**Ajustes → Acerca de → Exportar diagnóstico…** crea un zip para adjuntar a una issue de GitHub. Homeport no lo envía a ninguna parte: guarda el archivo donde elijas, y compartirlo depende de ti.

El diálogo lista los archivos y muestra `summary.json` antes de que guardes:

- `summary.json`: la versión de Homeport y cómo se instaló, tu sistema operativo, la versión del esquema de la base de datos, si hay un llavero disponible, tus ajustes y, para cada host y clúster, su tipo de conexión, su Engine y su versión, su estado y el código de su último error, además de cuántos stacks de cada tipo tienes.
- `logs/`: los [archivos de log](/docs/reference/security/#logs) de Homeport, con las claves privadas y los certificados, las contraseñas, los tokens, las cabeceras `Authorization` y las credenciales de las URL sustituidos por `[redacted]`.

Los hosts y los clústeres aparecen como `host-1`, `cluster-1`… y sus nombres y direcciones se sustituyen de la misma forma en los logs, salvo que marques **Incluir nombres y direcciones**. Márcalo cuando el nombre o la dirección importen para el problema y no te importe compartirlos.

Los secretos guardados nunca se leen para el archivo. Quitar los secretos de los logs funciona reconociendo sus formas habituales, así que revisa los archivos antes de adjuntarlos a una issue pública si tus logs pueden citar algo poco habitual.
