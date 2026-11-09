---
title: Requisitos del sistema
description: Lo que Homeport necesita para funcionar, y las piezas opcionales que activan SSH, TLS, el despliegue de stacks, los clústeres de Kubernetes y Nomad, y Helm.
---

## Sistemas operativos

| Sistema | Versiones                                                                  | Arquitecturas        |
| ------- | -------------------------------------------------------------------------- | -------------------- |
| Windows | Windows 10 y 11                                                            | x64, ARM64           |
| macOS   | macOS 10.15 Catalina o posterior                                           | Apple silicon, Intel |
| Linux   | glibc 2.35 o posterior: Ubuntu 22.04, Debian 12, Fedora 36 y más recientes | x64, ARM64           |

Homeport usa la vista web del sistema para dibujar su ventana: WebView2 en Windows (incluido con Windows 10 y 11), WebKit en macOS y WebKitGTK en Linux (se instala como dependencia de los paquetes `.deb` y `.rpm`, y va incluido en la AppImage).

## Un Docker Engine que gestionar

Homeport no incluye un runtime de contenedores. Gestiona Docker Engines que ya se ejecutan en algún sitio:

- **En este equipo**: Docker Engine, Docker Desktop, Colima, OrbStack, Rancher Desktop o Podman. Homeport los encuentra por ti; consulta [Engines locales](/docs/hosts/local/).
- **En un servidor**: accesible por [SSH](/docs/hosts/ssh/), o por [TLS o TCP](/docs/hosts/tls-tcp/).

Funciona cualquier Engine que hable la API de Docker Engine, incluido el socket compatible con Docker de Podman.

También gestiona directamente [containerd](/docs/hosts/containerd/) 1.7 y posteriores, a través de su socket en este equipo o por SSH: el de k3s, el propio de Docker Engine o el servicio de containerd en un servidor. Para descargar imágenes ahí se necesita containerd 2.0 o posterior, y los logs y los terminales necesitan que se acceda al host por SSH, o un containerd en este equipo (Linux).

También gestiona [Incus](/docs/hosts/incus/) 6.0 y posteriores, a través de su socket en este equipo o por SSH, o por HTTPS con un token de confianza. Las máquinas virtuales necesitan virtualización por hardware (`/dev/kvm`) en el servidor de Incus, y un terminal en una de ellas necesita el `incus-agent` que ejecuta su imagen.

En un host de Podman, Homeport también usa la API propia de Podman: los [pods](/docs/guides/podman-pods/) necesitan Podman 4.0 o posterior, y las [unidades Quadlet](/docs/guides/quadlet/) se listan a partir de Podman 5.7 y se instalan, editan y eliminan a partir de 5.8. Iniciar y detener una unidad Quadlet necesita que se acceda al host por SSH, o un Podman en este equipo (Linux).

## Opcional: clústeres de Kubernetes

Para explorar [clústeres](/docs/clusters/), Homeport necesita un kubeconfig que llegue a ellos (el que usa `kubectl`, o uno que pegues) y acceso de red a sus servidores de API. Habla la API de Kubernetes por sí mismo, así que `kubectl` no hace falta.

- **Versiones de Kubernetes**: clústeres con Kubernetes 1.24 o posterior. Homeport usa solo API estables (`v1`, `apps/v1`, `batch/v1`, `networking.k8s.io/v1`), y está compilado contra la API 1.33.
- **Plugins exec**: un kubeconfig de la nube nombra un programa que se ejecuta en este equipo para obtener sus credenciales: la [AWS CLI](https://aws.amazon.com/cli/) (versión 2) para EKS, `gke-gcloud-auth-plugin` de la Google Cloud CLI para GKE, `kubelogin` para AKS. Instala el que nombre tu kubeconfig; consulta [plugins exec](/docs/clusters/exec-plugins/).

## Opcional: clústeres de Nomad

Para gestionar [clústeres de Nomad](/docs/clusters/nomad/), Homeport necesita acceso de red a la API HTTP de un agente de Nomad (puerto 4646 por defecto) y, cuando el clúster tiene las ACL activadas, un token; consulta [la política de ACL](/docs/clusters/nomad/#políticas-acl) que necesita. Habla la API de Nomad por sí mismo, así que la CLI `nomad` no hace falta, y no se instala nada en el clúster.

- **Versiones de Nomad**: Nomad 1.6 o posterior. **Editar y ejecutar** parte de la especificación enviada de un job, que Nomad conserva a partir de 1.6.

## Opcional: un llavero del sistema

Homeport cifra las claves SSH, los certificados TLS, las credenciales de kubeconfig de los clústeres, los entornos de los stacks, los inicios de sesión de los registros y las claves de despliegue y tokens de los repositorios con una clave que guarda en el llavero de tu sistema operativo:

- **macOS**: Llavero
- **Windows**: Administrador de credenciales
- **Linux**: un proveedor de Secret Service, como GNOME Keyring o KWallet

Sin llavero, Homeport muestra un aviso y sigue funcionando con sockets locales, TCP sin cifrar y SSH mediante **ssh-agent**. Los hosts TLS, los hosts SSH con claves guardadas, los clústeres, los entornos de los stacks, los inicios de sesión de los registros y los repositorios privados necesitan el llavero; sin él, las imágenes se descargan de forma anónima y solo se pueden añadir repositorios públicos. Consulta [seguridad y tus datos](/docs/reference/security/).

## Opcional: Docker Compose

Para desplegar [stacks gestionados, vinculados y sincronizados](/docs/guides/stacks/), Homeport ejecuta el comando `docker compose` en **este** equipo, no en el host. Instala la CLI de Docker con el plugin Compose (Docker Desktop lo incluye), o `docker-compose` v2 independiente.

Sin él, los stacks siguen apareciendo —los vinculados también— y se pueden iniciar, detener y eliminar; solo el despliegue necesita Compose. **Ajustes → Este equipo** muestra si Homeport lo encontró.

Para desplegar un stack en un [Swarm](/docs/guides/swarm/), Homeport ejecuta en cambio `docker stack deploy` en este equipo, así que necesita el propio comando `docker` (el plugin Compose no se usa). Docker Desktop lo tiene; en Linux, instala el paquete de la CLI de Docker de tu distribución (`docker-ce-cli`, `docker.io` o `docker`). No hace falta que se ejecute ningún Engine aquí.

## Opcional: git

Los [stacks sincronizados](/docs/guides/synced-stacks/) ejecutan el `git` instalado en **este** equipo para clonar y actualizar sus repositorios: Git for Windows, las herramientas de línea de comandos de Apple (`xcode-select --install`) o el paquete `git` de tu distribución. En Windows, Homeport lo dirige al OpenSSH del sistema para los repositorios SSH. Todas las demás funciones funcionan sin git; **Ajustes → Este equipo** muestra si Homeport lo encontró.

## Opcional: Helm

Para gestionar [releases de Helm](/docs/clusters/helm/) en un clúster, Homeport ejecuta el comando `helm`, versión 3 o posterior, en **este** equipo. Dirige helm al clúster a través de una conexión privada, así que no se escribe ninguna credencial del clúster en el disco, y helm usa sus propios repositorios de charts, como en un terminal. Instálalo con `brew install helm` (macOS), `winget install Helm.Helm`, Chocolatey o Scoop (Windows), o `snap install helm --classic`, el paquete de tu distribución o el script de [helm.sh](https://helm.sh/docs/intro/install/) (Linux). Todas las demás páginas del clúster funcionan sin él; **Ajustes → Este equipo** muestra si Homeport lo encontró.

## Opcional: una bandeja del sistema

Homeport pone un icono en la bandeja del sistema (la barra de menús en macOS, el área de notificación en Windows) que lista todos los hosts con su estado. Windows y macOS siempre tienen una. En Linux necesita la biblioteca **appindicator**: la AppImage la incluye, y los paquetes `.deb` y `.rpm` la recomiendan, así que `apt`, `dnf` y `zypper` la instalan salvo que hayas desactivado los paquetes recomendados. Si no, instala `libayatana-appindicator3-1` (Debian, Ubuntu, openSUSE) o `libayatana-appindicator-gtk3` (Fedora). Algunos escritorios ocultan los iconos de la bandeja: GNOME solo los muestra con una extensión como AppIndicator Support. Sin bandeja, Homeport funciona desde su ventana, y al cerrar la ventana se sale de la app.

## Qué necesita cada función

| Función                                                     | Llavero | CLI de Compose | git | helm |
| ----------------------------------------------------------- | :-----: | :------------: | :-: | :--: |
| Desplegar stacks en un Swarm                                |    —    | CLI de docker  |  —  |  —   |
| Hosts de socket local y TCP                                 |    —    |       —        |  —  |  —   |
| Hosts SSH con ssh-agent                                     |    —    |       —        |  —  |  —   |
| Hosts SSH con una clave de Homeport o importada             |   Sí    |       —        |  —  |  —   |
| Hosts TLS                                                   |   Sí    |       —        |  —  |  —   |
| Contenedores, imágenes, volúmenes, redes                    |    —    |       —        |  —  |  —   |
| Iniciar, detener y eliminar stacks                          |    —    |       —        |  —  |  —   |
| Desplegar stacks gestionados y vinculados                   |    —    |       Sí       |  —  |  —   |
| El entorno de un stack gestionado o sincronizado            |   Sí    |       —        |  —  |  —   |
| Inicios de sesión de registros (imágenes privadas, subidas) |   Sí    |       —        |  —  |  —   |
| Stacks sincronizados desde un repositorio público           |    —    |       Sí       | Sí  |  —   |
| Stacks sincronizados con clave de despliegue o token        |   Sí    |       Sí       | Sí  |  —   |
| Compilar una imagen desde una carpeta                       |    —    |       —        |  —  |  —   |
| Clústeres de Kubernetes                                     |   Sí    |       —        |  —  |  —   |
| Clústeres de Nomad                                          |   Sí    |       —        |  —  |  —   |
| Releases de Helm                                            |   Sí    |       —        |  —  |  Sí  |
