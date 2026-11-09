---
title: Pods de Podman
description: Lista, inicia, detén y elimina pods de Podman, crea uno con sus puertos publicados y ejecuta contenedores en él.
---

Un **pod** es la forma que tiene Podman de agrupar contenedores: comparten una red (y su nombre de host e IPC), así que se alcanzan entre sí en `localhost`, y se inician y se detienen juntos. Cada pod tiene un **contenedor infra** que guarda lo que comparten y los puertos que publica el pod. Homeport muestra los pods en los hosts cuyo Engine es Podman, a través de la API propia de Podman en el socket que el host ya usa; no se instala nada en el host.

La página **Pods** aparece en la barra lateral solo mientras hay un host Podman seleccionado. En cualquier otro Engine compatible con Docker no está, y un enlace a ella explica por qué. Consulta [Tipos de Engine](/docs/hosts/#engine-kinds) para saber cómo distingue Homeport a Podman.

## La página Pods

| Columna          | Muestra                                                                                                                                |
| ---------------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| **Nombre**       | El nombre del pod, con su estado como un punto.                                                                                        |
| **Estado**       | **En ejecución**, **En ejecución parcial** (algunos contenedores detenidos), **En pausa**, **Detenido**, **Creado** o **Con errores**. |
| **Contenedores** | Cuántos de sus propios contenedores están en ejecución, sin contar el contenedor infra.                                                |
| **Redes**        | Las redes en las que está el pod.                                                                                                      |
| **Creado**       | Cuándo se creó.                                                                                                                        |

Cada fila tiene la acción que corresponde a su estado — **Iniciar** para un pod detenido, **Detener** para uno en ejecución — y **Eliminar**. Selecciona un pod para abrir su página. La lista se actualiza sola a medida que los contenedores del pod se inician y se detienen, ya sea desde Homeport o desde un terminal.

En la página **Contenedores**, un contenedor que se ejecuta en un pod muestra el nombre del pod junto al suyo; selecciona el nombre para abrir el pod.

## La página de un pod

La cabecera tiene **Ejecutar contenedor** y las acciones del pod: **Iniciar**, **Detener**, **Reiniciar**, **Pausar**, **Reanudar** y **Eliminar**, según lo permita su estado. Cada una actúa a la vez sobre todos los contenedores del pod, como hacen `podman pod start` y similares.

- **Contenedores** lista los contenedores del pod, con su contenedor infra al final. Selecciona uno para abrir su página de contenedor, con sus logs, su terminal y sus archivos.
- **Ajustes** muestra los puertos que publica el pod, los namespaces que comparten sus contenedores, sus redes, su nombre de host y sus etiquetas, y el comando `podman pod create` con el que se creó, si Podman lo conservó.

## Eliminar un pod

**Eliminar** pide confirmación e indica cuántos contenedores se eliminan con el pod. Un pod en ejecución se detiene primero: el botón dice **Detener y eliminar**. Los volúmenes del pod se conservan.

## Crear un pod

**Nuevo pod** (o `n` en la página Pods, o **Nuevo pod** en la paleta de comandos) pide:

- un **Nombre**: letras, dígitos, `_`, `.` y `-`, empezando por una letra o un dígito;
- los **Puertos** que publicar, del puerto del host al puerto del contenedor, como en el cuadro de diálogo Ejecutar. Deja vacío el puerto del host para que Podman elija uno libre;
- **Etiquetas**, una `key=value` por línea.

Los puertos pertenecen al pod: se publican en su contenedor infra y llegan a cualquier contenedor del pod que escuche en ese puerto. Se abre la página del nuevo pod, lista para ejecutar contenedores en él. Un pod no tiene contenedores propios hasta que ejecutas uno; Podman lo inicia con el primero.

## Ejecutar un contenedor en un pod

En un host Podman, el cuadro de diálogo **Ejecutar** tiene un campo **Pod**, y el botón **Ejecutar contenedor** de un pod abre el cuadro de diálogo con ese pod ya elegido. Un contenedor en un pod:

- comparte la red del pod, así que no puede publicar puertos ni unirse a otra red: el cuadro de diálogo oculta esos campos en cuanto se elige un pod;
- usa el nombre de host y los ajustes de DNS del pod, así que Homeport rechaza los suyos propios;
- conserva todo lo demás que ofrece el cuadro de diálogo Ejecutar: comando, entorno, montajes, política de reinicio, límites, capacidades, dispositivos, healthcheck y logging.

Duplicar y Editar nunca mueven un contenedor a un pod: Podman no puede mover un contenedor existente a uno.

## Qué necesita Homeport

Los pods funcionan en Podman 4.0 y posteriores, con cualquier tipo de conexión — socket local, SSH, TLS o TCP —, ya que pasan por el mismo socket que todo lo demás.
