---
title: Teclado
description: Usa Homeport sin el ratón — la paleta de comandos y la búsqueda en todos los hosts, atajos para cada página y cada host, y la hoja de ayuda que los lista.
---

Homeport se puede usar desde el teclado. Pulsa <kbd>Mod</kbd> <kbd>K</kbd> para abrir la [paleta de comandos](#paleta-de-comandos), y <kbd>?</kbd> en cualquier página para ver todos los atajos que funcionan en ella.

A continuación, **Mod** significa <kbd>⌘</kbd> en macOS y <kbd>Ctrl</kbd> en Windows y Linux; en macOS, <kbd>Alt</kbd> es <kbd>⌥</kbd>.

## Paleta de comandos

<kbd>Mod</kbd> <kbd>K</kbd> abre la paleta de comandos desde cualquier página. Escribe lo que buscas, muévete con <kbd>↑</kbd> y <kbd>↓</kbd>, y pulsa <kbd>Enter</kbd>; <kbd>Esc</kbd> la cierra y te devuelve adonde estabas. Lista:

- **Ir a** — cada página de la barra lateral: escribe `vol` y pulsa <kbd>Enter</kbd> para abrir Volúmenes.
- **Hosts** — cada host con su estado: escribe parte del nombre de un host para cambiar a él.
- **Acciones** — **Ejecutar un contenedor**, **Descargar una imagen**, **Nuevo stack**, **Limpiar el host seleccionado**, **Añadir host**, **Gestionar hosts**, **Buscar actualizaciones** y **Mostrar atajos de teclado**.
- **Ajustes** — **Tema: claro**, **Tema: oscuro** y **Tema: igual que el sistema**.

Las acciones que necesitan un host — ejecutar un contenedor, descargar una imagen, crear un stack, limpiar — solo aparecen mientras el host seleccionado está conectado.

### Buscar en todos los hosts

Escribe dos caracteres o más y la paleta busca también en los **contenedores**, **stacks**, **imágenes**, **volúmenes** y **redes** de todos los hosts conectados: «¿dónde se ejecuta `postgres`?» tiene respuesta sin cambiar de host. Busca en los nombres, los ID cortos y, en los contenedores, su imagen y su stack; los nombres que empiezan por lo que escribiste aparecen primero.

- Los resultados se agrupan por tipo, cinco por tipo hasta **Mostrar los…**. Cada uno muestra su host y, en los contenedores y stacks, si están en ejecución: _prod-server · en ejecución_.
- Al elegir uno, Homeport cambia a su host y lo abre: la página de un contenedor o de un stack, o la página Imágenes, Volúmenes o Redes filtrada por él.
- Los hosts que no están conectados se omiten, y la paleta indica cuántos: _2 hosts sin buscar (no conectados)_.

Homeport solo carga las listas cuando has escrito dos caracteres, y después las mantiene al día con los eventos en directo de cada host. No se busca en los logs, los archivos ni las definiciones de Compose. Cada elemento muestra su atajo, si tiene uno. Sin nada escrito, los elementos que usaste por última vez aparecen primero, en **Recientes**, hasta que cierras Homeport.

## Atajos

### En todas partes

| Teclas                                     | Hace                                                                                           |
| ------------------------------------------ | ---------------------------------------------------------------------------------------------- |
| <kbd>?</kbd>                               | Mostrar los atajos de teclado                                                                  |
| <kbd>Mod</kbd> <kbd>1</kbd> … <kbd>9</kbd> | Abrir las páginas de la barra lateral en orden: Resumen, Contenedores, Imágenes…               |
| <kbd>Mod</kbd> <kbd>,</kbd>                | Abrir Ajustes                                                                                  |
| <kbd>Alt</kbd> <kbd>↑</kbd> / <kbd>↓</kbd> | Seleccionar el host anterior o el siguiente, en el orden en que los lista la página Hosts      |
| <kbd>Mod</kbd> <kbd>R</kbd>                | Actualizar las listas del host seleccionado: contenedores, imágenes, volúmenes, redes, stacks… |

Pasa el puntero sobre una página de la barra lateral para ver sus teclas.

### En una página

| Teclas       | Hace                                                                                                                                                         |
| ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| <kbd>/</kbd> | Poner el foco en el campo de búsqueda de la página. <kbd>Esc</kbd> en un campo vacío lo abandona.                                                            |
| <kbd>N</kbd> | Iniciar la acción principal de la página: **Ejecutar contenedor**, **Descargar imagen**, **Crear volumen**, **Crear red**, **Nuevo stack** o **Añadir host** |

El botón de la acción principal de una página muestra <kbd>N</kbd> cuando pasas el puntero sobre él.

### Mientras escribes

Las teclas solas — <kbd>N</kbd>, <kbd>/</kbd>, <kbd>?</kbd> y las flechas con <kbd>Alt</kbd> — nunca se activan mientras escribes en un campo, en el editor de un stack o en un cuadro de diálogo: escriben el carácter. Los atajos con <kbd>Mod</kbd> también funcionan ahí. En el **Terminal** de un contenedor, todas las teclas van al contenedor y no se activa ningún atajo.
