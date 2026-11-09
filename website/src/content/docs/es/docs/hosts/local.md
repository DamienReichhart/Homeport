---
title: Engines locales
description: Conéctate a Docker Engine, Docker Desktop, Colima, OrbStack, Rancher Desktop o Podman en este equipo.
---

Un host de **Socket local** habla con un Engine de este equipo a través de su socket Unix, o de una tubería con nombre en Windows. No necesita credenciales ni llavero.

## Detección automática

Homeport busca Engines en ejecución en los lugares habituales y lista los que responden en **Encontrados en este equipo**, en la pantalla de bienvenida y en la página **Añadir host**. Elige **Añadir** junto a uno para crear un host con su nombre.

| Runtime                  | macOS y Linux                                                                                                       | Windows                           |
| ------------------------ | ------------------------------------------------------------------------------------------------------------------- | --------------------------------- |
| Docker Engine            | `/var/run/docker.sock`                                                                                              | —                                 |
| Docker Engine (sin root) | `$XDG_RUNTIME_DIR/docker.sock`                                                                                      | —                                 |
| Docker Desktop           | `~/.docker/desktop/docker.sock`, `~/.docker/run/docker.sock`                                                        | `//./pipe/docker_engine`          |
| Colima                   | `~/.colima/default/docker.sock`                                                                                     | —                                 |
| OrbStack                 | `~/.orbstack/run/docker.sock`                                                                                       | —                                 |
| Rancher Desktop          | `~/.rd/docker.sock`                                                                                                 | —                                 |
| Podman                   | `$XDG_RUNTIME_DIR/podman/podman.sock`, `/run/podman/podman.sock`; [máquinas en macOS](#máquinas-de-podman-en-macos) | `//./pipe/podman-machine-default` |

Cada candidato se comprueba pidiéndole al Engine su versión, así que solo se listan los Engines en ejecución. Cuando varias rutas llevan al mismo socket —en macOS, `/var/run/docker.sock` suele apuntar a Docker Desktop, OrbStack o Rancher Desktop—, el Engine se lista una sola vez, con el primer nombre encontrado.

### Máquinas de Podman en macOS

En macOS, Podman ejecuta su Engine en una VM (`podman machine`) cuyo socket cambia con cada máquina. Homeport le pide a la CLI `podman` el socket de cada máquina en ejecución (`podman machine list` y `podman machine inspect`) y lista cada una como **Podman (_nombre de la máquina_)**, por ejemplo **Podman (podman-machine-default)**. Las máquinas detenidas no se listan; inicia una con `podman machine start` y vuelve a abrir **Añadir host**.

Si Homeport no puede ejecutar `podman` —no está en su PATH ni en `/opt/homebrew/bin` o `/usr/local/bin`—, busca en los lugares donde Podman pone los sockets: `$TMPDIR/podman/<machine>-api.sock` para Podman 5, y `~/.local/share/containers/podman/machine/…/podman.sock` para Podman 4. Cuando `podman-mac-helper` ha enlazado `/var/run/docker.sock` a una máquina, esa máquina se lista una sola vez, con su nombre de Podman.

## Añadir un socket a mano

Si tu Engine no se detecta —un perfil de Colima distinto de `default`, una máquina de Podman en Windows con un nombre distinto del predeterminado o una ruta personalizada—, añádelo tú mismo:

1. Elige **Añadir host…** en el selector de hosts.
2. Selecciona **Socket local**.
3. Introduce un **Nombre** y la **Ruta del socket**: un socket Unix como `/run/user/1000/docker.sock`, o una tubería con nombre como `//./pipe/docker_engine` en Windows.
4. Elige **Añadir host**.

Para encontrar el socket que usa tu CLI de Docker, ejecuta:

```sh
docker context inspect --format '{{.Endpoints.docker.Host}}'
```

Para una máquina de Podman:

```sh
podman machine inspect --format '{{.ConnectionInfo.PodmanSocket.Path}}'
```

## Permisos en Linux

El socket predeterminado `/var/run/docker.sock` pertenece al grupo `docker`. Si tu usuario no está en él, el host aparece como **Inaccesible** con _Permission denied_. Añádete y vuelve a iniciar sesión:

```sh
sudo usermod -aG docker $USER
```

Pertenecer al grupo `docker` equivale a ser root en esa máquina; Docker sin root o Podman lo evitan, si eso te importa.
