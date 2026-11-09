---
title: Unidades Quadlet
description: Lista las unidades Quadlet de un host Podman con su estado de systemd, lee sus archivos, instálalas, edítalas y elimínalas, e inicia o detén sus servicios.
---

**Quadlet** es la forma en que Podman ejecuta servicios de larga duración bajo systemd. Una unidad Quadlet es un archivo — `web.container`, `shop.pod`, `cache.volume`, `lan.network`, `app.kube`, `base.image`, `api.build` — que el generador de systemd convierte en un servicio, `web.service`, que inicia el contenedor al arrancar y lo reinicia cuando se detiene. Podman con root guarda sus unidades en `/etc/containers/systemd/`; Podman sin root guarda las de cada usuario en `~/.config/containers/systemd/`.

La página **Quadlet** aparece en la barra lateral mientras hay un host Podman seleccionado. Pasa por la API propia de Podman en el socket que el host ya usa, y por `systemctl` para iniciar y detener, como decide el [ADR 0019](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0019-quadlet-units-through-podmans-api-and-systemctl.md).

## Qué permite cada Podman y cada conexión

La API de Podman fue incorporando la compatibilidad con Quadlet a lo largo de dos versiones, y nunca inicia ni detiene el servicio de una unidad. Así que lo que Homeport puede hacer depende del Podman del host y de cómo accede Homeport a él:

| Acción                                           | Podman                 | Conexión                        |
| ------------------------------------------------ | ---------------------- | ------------------------------- |
| Listar las unidades con su estado                | 5.7 y posteriores      | cualquiera                      |
| Listar las unidades a partir de sus contenedores | 4.0 a 5.6              | cualquiera                      |
| Leer el archivo de una unidad                    | 5.8 y posteriores      | cualquiera                      |
| Instalar, editar y eliminar unidades             | 5.8 y posteriores      | cualquiera                      |
| Iniciar, detener y reiniciar un servicio         | cualquiera con Quadlet | SSH, o un socket local en Linux |

En Podman anterior a 5.7, las unidades se encuentran a partir de la etiqueta `PODMAN_SYSTEMD_UNIT` que Podman pone en los contenedores que crean, así que una unidad solo aparece cuando ha creado un contenedor, con el nombre de su servicio. La página lo indica encima de la lista.

Iniciar y detener necesitan `systemctl`, que Homeport ejecuta donde se ejecuta Podman: en la sesión SSH de un host SSH, o en este equipo para un socket local en Linux. Un host TCP o TLS, o un socket local en macOS o Windows (donde Podman se ejecuta en una máquina virtual), puede hacer todo lo demás; los botones aparecen desactivados con el motivo.

## La página Quadlet

| Columna          | Muestra                                                                                                                                            |
| ---------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Unidad**       | El nombre de archivo de la unidad y la aplicación con la que se instaló.                                                                           |
| **Tipo**         | `container`, `pod`, `volume`, `network`, `kube`, `image`, `build` o `artifact`.                                                                    |
| **Servicio**     | El servicio de systemd que genera, o **aún no generado** hasta que systemd la ha cargado.                                                          |
| **Estado**       | **En ejecución**, **Iniciando**, **Deteniendo**, **Detenida**, **Con errores**, **No cargada por systemd** o **Plantilla**, según indique systemd. |
| **Contenedores** | Cuántos contenedores creó.                                                                                                                         |

Cada fila tiene **Iniciar** para una unidad detenida o **Detener** para una en ejecución. Selecciona una unidad para abrir su página. La lista se actualiza sola a medida que sus contenedores se inician y se detienen.

Una unidad **No cargada por systemd** está instalada, pero el generador de systemd la omitió: normalmente por un error en el archivo. En el host, `systemctl --user daemon-reload` (o `systemctl daemon-reload` para Podman con root) y `journalctl` indican entonces el motivo.

En la página **Contenedores**, un contenedor creado por una unidad muestra el servicio de la unidad junto a su nombre; selecciónalo para abrir la unidad.

## La página de una unidad

- La cabecera tiene **Iniciar**, **Detener** y **Reiniciar**, **Editar** y **Eliminar**.
- **Servicio** muestra la unidad de systemd, si es una unidad del sistema o una unidad de usuario (`systemctl --user`), dónde está su archivo en el host, y su aplicación y su pod cuando los tiene.
- **Archivo de la unidad** muestra el archivo tal como lo lee Podman, con resaltado.
- **Contenedores** lista los contenedores que creó, y cada uno abre su página de contenedor.

Detener el contenedor de una unidad desde la página Contenedores es luchar contra systemd, que lo vuelve a iniciar según `Restart=`. Detén la unidad en su lugar.

## Instalar una unidad

**Nueva unidad** (o `n` en la página Quadlet, o **Nueva unidad** en la paleta de comandos) abre un editor con un punto de partida para el tipo que elijas, o con un archivo que abras desde este equipo con **Abrir archivo…**. Ponle un nombre con su extensión, como `web.container`; el archivo necesita la sección de su tipo, `[Container]` para un `.container`.

**Añadir archivos…** coloca otros archivos junto a la unidad — un `Containerfile` para un `.build`, el YAML de un `.kube`, un archivo env — en una carpeta de aplicación con el nombre de la unidad, como hace `podman quadlet install`.

Podman escribe los archivos donde van las unidades de su usuario y recarga systemd, así que el servicio existe de inmediato. Instalar no lo inicia: elige **Iniciar** cuando esté. Una unidad cuyo nombre ya está ocupado se rechaza; edita esa en su lugar.

## Editar y eliminar

**Editar** abre el archivo de la unidad. **Guardar** lo escribe en su sitio y recarga systemd; los archivos instalados junto a ella se mantienen. Un servicio en ejecución conserva sus ajustes anteriores hasta que lo **reinicias**.

**Eliminar** borra el archivo de la unidad y recarga systemd. **Detener antes su servicio en ejecución** está activado por defecto; sin ello, Podman se niega a eliminar una unidad en ejecución. Los contenedores, volúmenes e imágenes que creó la unidad se conservan.

## Qué ejecuta Homeport en el host

Para **Iniciar**, **Detener** y **Reiniciar**, y nada más, Homeport ejecuta un comando en el servidor del host por SSH (o en este equipo para un socket local), con los permisos del usuario SSH:

```bash
systemctl --user start 'web.service'
```

`--user` está ahí para las unidades de un Podman sin root y se omite para las de Podman con root; `start` es `stop` o `restart` para esas acciones. Homeport comprueba el nombre del servicio y lo pone entre comillas, y nunca usa `sudo`. Por tanto:

- Las **unidades del sistema** (Podman con root) necesitan un usuario SSH con permiso para gestionarlas, como `root`.
- Las **unidades de usuario** (Podman sin root) necesitan la instancia de systemd del usuario. Para que se ejecute sin una sesión iniciada, y para que las unidades se inicien al arrancar, activa el lingering una vez, como ese usuario en el host:

```bash
loginctl enable-linger "$USER"
```

La [página de seguridad](/docs/reference/security/#what-homeport-runs-on-your-hosts) lista todos los comandos que Homeport ejecuta en un host.
