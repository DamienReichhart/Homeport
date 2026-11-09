---
title: containerd
description: Conéctate a containerd en este equipo o por SSH, elige un namespace, ejecuta las tareas de sus contenedores, lee sus logs, abre un shell y gestiona imágenes e instantáneas.
---

**containerd** funciona por debajo de k3s, nerdctl, BuildKit y muchos servidores que no tienen Docker. Homeport se conecta a él directamente, a través de su propia API gRPC, como decide el [ADR 0020](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0020-containerd-over-grpc.md). Un host de containerd tiene sus propias páginas en la barra lateral: **Contenedores**, **Imágenes**, **Instantáneas** y **Namespaces**. Las páginas de Docker, los stacks y el Tablero no se ofrecen en él.

## Añadir un host de containerd

Elige **containerd** en el campo **Engine** del formulario. Solo accede a containerd a través de su socket, así que la conexión es **Socket local** o **SSH**; TCP y TLS aparecen atenuados, porque containerd no sirve su API por la red.

| containerd                       | Socket                                                                         |
| -------------------------------- | ------------------------------------------------------------------------------ |
| El servicio propio de containerd | `/run/containerd/containerd.sock`                                              |
| k3s                              | `/run/k3s/containerd/containerd.sock`                                          |
| El containerd de Docker Engine   | `/run/containerd/containerd.sock`, con sus contenedores en el namespace `moby` |

En Linux, **Encontrados en este equipo** lista el containerd de este equipo y el de k3s cuando responden. Un socket que todavía no puedes usar se lista junto con cómo solucionarlo.

### Permitir que tu usuario use el socket

El socket de containerd pertenece a root. Para usarlo como otro usuario, en local o por SSH, da acceso a un grupo en `/etc/containerd/config.toml` y añade el usuario a ese grupo:

```bash
sudo groupadd -f containerd
sudo usermod -aG containerd "$USER"
getent group containerd | cut -d: -f3
```

Pon el número que se muestra bajo `[grpc]` en `/etc/containerd/config.toml`, reinicia containerd y vuelve a iniciar sesión:

```toml
[grpc]
  gid = 1001
```

```bash
sudo systemctl restart containerd
```

**containerd sin root** (`containerd-rootless.sh`) guarda su socket dentro del namespace de RootlessKit, donde nada de fuera puede alcanzarlo, ni siquiera Homeport. Homeport lo indica cuando lo encuentra; usa en su lugar el servicio de sistema de containerd.

### Qué permite cada versión y conexión

Homeport se conecta a containerd **1.7** y posteriores. La descarga pasa por el servicio Transfer de containerd, estable a partir de **2.0**.

| Acción                                                    | containerd        | Conexión                        |
| --------------------------------------------------------- | ----------------- | ------------------------------- |
| Namespaces, contenedores y tareas, imágenes, instantáneas | 1.7 y posteriores | socket local o SSH              |
| Descargar una imagen                                      | 2.0 y posteriores | socket local o SSH              |
| Logs y un terminal                                        | 1.7 y posteriores | SSH, o un socket local en Linux |

Un containerd demasiado antiguo para conectarse deja el host como **Mal configurado**, indicando su versión; uno demasiado antiguo para descargar lo dice cuando descargas.

## Namespaces

Cada objeto de containerd vive en un **namespace**: nerdctl usa `default`, Kubernetes `k8s.io`, Docker `moby`, BuildKit `buildkit`. Las páginas muestran un namespace a la vez, elegido bajo el selector de hosts, que lista cada uno con cuántos contenedores e imágenes contiene. Homeport recuerda la elección para cada host entre sesiones. Hasta que eliges uno, un host muestra `default`, o el único namespace que contiene algo cuando `default` está vacío, así que un host de k3s se abre en `k8s.io`.

La página **Namespaces** crea namespaces vacíos y elimina los vacíos. containerd se niega a eliminar un namespace que todavía contiene algo, y Homeport dice qué contiene: elimina antes sus contenedores, imágenes e instantáneas.

## Contenedores y tareas

En containerd, un **contenedor** son solo metadatos: su imagen, su spec, su instantánea. Lo que se ejecuta es su **tarea**. La página **Contenedores** lista los contenedores del namespace con su tarea: **En ejecución**, **En pausa**, **Detenido** con su código de salida, o **Sin tarea**. Cada uno indica qué herramienta lo creó —**Kubernetes** (con su Pod), **nerdctl**, **Docker** o **ctr u otro**—, y los contenedores pause de los Pods se ocultan salvo que pidas verlos.

| Acción           | Qué hace Homeport                                                                                                                                                                                                                                                                                       |
| ---------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Iniciar**      | Crea la tarea sobre la instantánea del contenedor, con los logs donde los guarda nerdctl, y la inicia. Una tarea detenida se sustituye, como exige `ctr task start`.                                                                                                                                    |
| **Detener**      | Envía la señal de parada de la imagen (SIGTERM salvo que nombre otra) y luego SIGKILL al cabo de 10 segundos. Se notifica una tarea que siga viva después de eso.                                                                                                                                       |
| **Pausar**       | Congela la tarea; **Reanudar** la descongela.                                                                                                                                                                                                                                                           |
| **Enviar señal** | Envía cualquier señal, por nombre (`SIGHUP`) o número, al proceso principal de la tarea.                                                                                                                                                                                                                |
| **Eliminar**     | Pregunta primero; mata una tarea en ejecución y elimina la tarea y el contenedor, y su instantánea con los archivos que cambió, salvo que desmarques **Eliminar también**: una instantánea conservada se marca con `containerd.io/gc.root`, para que el recolector de basura de containerd la conserve. |

La página de un contenedor muestra su runtime, su instantánea, sus etiquetas y su spec OCI. Homeport no crea contenedores en containerd: containerd no tiene red propia, que la añaden nerdctl y CNI.

El **kubelet** gestiona los contenedores de Kubernetes: los vuelve a iniciar tras una parada y los vuelve a crear tras una eliminación. Sus páginas lo indican; cambia el Pod en su clúster en su lugar. Eliminar desde aquí un contenedor de nerdctl deja los registros propios de nerdctl sobre él (su nombre y su red), como haría `ctr`; `nerdctl rm` es la forma más limpia para esos.

## Logs

containerd no guarda logs: la herramienta que inicia una tarea decide adónde va su salida. La pestaña **Logs** la lee donde esa herramienta la escribió:

| Creado por             | Adónde va su salida                                                                              | Qué muestra Homeport                                       |
| ---------------------- | ------------------------------------------------------------------------------------------------ | ---------------------------------------------------------- |
| nerdctl                | un archivo de log JSON en la carpeta de estado de nerdctl, indicado por sus etiquetas `nerdctl/` | ese archivo, en seguimiento                                |
| Kubernetes (`k8s.io`)  | `/var/log/pods/<pod>/<container>/N.log`, indicado en el registro que CRI guarda de él            | ese archivo, en seguimiento                                |
| `ctr` y otros clientes | adonde lo dirigieran, normalmente a ninguna parte                                                | **La salida de este contenedor no se registra**, y por qué |

Los contenedores de nerdctl que registran con otro driver (journald, fluentd) lo nombran en su lugar. Homeport lee el archivo con `tail` en la máquina del host (consulta [más abajo](#lo-que-se-ejecuta-en-el-host)); los archivos de log suelen pertenecer a root, así que el usuario SSH debe tener permiso para leerlos.

## Un terminal en un contenedor

La pestaña **Exec** abre un shell en un contenedor en ejecución —bash si la imagen lo tiene, sh si no— o el comando que indiques, como un usuario numérico (`uid` o `uid:gid`) y en la carpeta que elijas. **Volver a abrir** inicia uno nuevo con lo que escribiste.

containerd recibe la entrada y la salida de un proceso como rutas en su máquina. Así que Homeport ejecuta allí un script corto que crea dos FIFO en una carpeta temporal privada y copia el terminal a través de ellos; containerd inicia el proceso sobre esos FIFO, a partir de la spec de proceso del propio contenedor. Cerrar la pestaña mata el proceso y elimina la carpeta.

## Imágenes

La página **Imágenes** lista las imágenes del namespace con sus plataformas, los contenedores creados a partir de ellas y su tamaño para la plataforma del host, tal como lo cuenta `ctr images ls`.

**Descargar imagen** obtiene una imagen a través del propio containerd y luego la desempaqueta para la plataforma del host en su snapshotter predeterminado, para que `nerdctl run` pueda usarla al momento. Indica una referencia completa (`ghcr.io/home/app:1.2`) o una corta (`nginx`, que pasa a ser `docker.io/library/nginx:latest`). La imagen de un registro privado se descarga con su inicio de sesión de **Ajustes → Registros**, que se envía a containerd solo cuando el registro lo pide y nunca se guarda en el host.

Una imagen a partir de la cual se creó un contenedor del namespace no se puede eliminar; elimina antes los contenedores. containerd libera el contenido de una imagen en cuanto nada lo retiene.

## Instantáneas

Las instantáneas son donde se va el disco de containerd: las capas de las imágenes desempaquetadas y la capa modificable de cada contenedor. La página **Instantáneas** muestra las de un snapshotter (`overlayfs` primero, o cualquier otro que haya cargado el host) como un árbol por padre, de mayor a menor, con su uso tal como lo cuenta `ctr snapshot usage` y el total arriba.

Cada una indica qué la usa: un contenedor, por su instantánea, o una imagen, por sus capas. Las instantáneas que nada usa y sobre las que nada se construye —como una conservada al eliminar aquí su contenedor— se pueden mostrar solas con **Solo sin usar**, y eliminar. Las demás no se pueden eliminar desde aquí. Una instantánea a la que nada hace referencia y que nada marca como raíz, como la que deja `ctr containers rm --keep-snapshot`, no dura: el recolector de basura de containerd la elimina en su siguiente ejecución.

## Lo que se ejecuta en el host

Todo lo anterior pasa por la API de containerd, salvo los logs y los terminales, que necesitan los archivos de la máquina de containerd. Para ellos, Homeport ejecuta estos comandos, por la sesión SSH que ya tiene abierta, o en este equipo para un socket local en Linux, con los permisos del usuario SSH y nunca con `sudo`:

| Para        | Comando                                                                                                                           |
| ----------- | --------------------------------------------------------------------------------------------------------------------------------- |
| Logs        | `tail -n <lines> [-F] -- '<log file>'`; para un contenedor de Kubernetes sin registro de CRI, antes un `ls` de su carpeta de logs |
| Un terminal | `mktemp -d`, `mkfifo -m 600` para dos FIFO en esa carpeta, `cat` a través de ellos, y `rm -rf` de la carpeta                      |

Un socket local en macOS o Windows no puede ejecutarlos, y entonces los logs y los terminales indican que necesitan SSH. La [página de seguridad](/docs/reference/security/#lo-que-homeport-ejecuta-en-tus-hosts) lista todos los comandos que Homeport ejecuta en un host.

## Eventos

Las páginas se actualizan a partir de los propios eventos de containerd, en todos los namespaces: una tarea que se inicia o termina, un contenedor o una imagen que se crea o se elimina, una instantánea confirmada. containerd no guarda historial de ellos, así que lo que pasó mientras Homeport estaba cerrado no se recupera.
