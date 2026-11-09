---
title: Teclado y accesibilidad
description: Usar Homeport con el teclado, con movimiento reducido y con un lector de pantalla, el contraste que cumple, los límites conocidos hoy y cómo notificar una barrera.
---

Homeport busca poder usarse solo con el teclado, con un lector de pantalla y con movimiento reducido, en cada uno de sus [idiomas](/docs/guides/settings/#appearance). Esta página explica qué funciona, qué no funciona todavía y cómo avisarnos cuando algo te impide avanzar.

## Moverse por la app

Todo en Homeport se alcanza con <kbd>Tab</kbd> y <kbd>Shift</kbd> <kbd>Tab</kbd>, y se usa con <kbd>Enter</kbd> o <kbd>Space</kbd>. Un anillo visible muestra dónde está el foco.

- **La barra lateral** lista todas las páginas; cada una es un enlace. <kbd>Mod</kbd> <kbd>1</kbd> a <kbd>9</kbd> las abren en orden, y la [paleta de comandos](/docs/guides/keyboard/#command-palette) (<kbd>Mod</kbd> <kbd>K</kbd>) llega a cualquier página, host o acción escribiendo.
- **Listas** (contenedores, imágenes, volúmenes, redes, stacks, hosts, Pods…): el nombre de cada fila es un botón o un enlace. Llega a él con <kbd>Tab</kbd> y pulsa <kbd>Enter</kbd> para abrir el elemento, como al hacer clic en la fila. Las casillas y el menú de la fila van antes y después.
- **Ordenación**: el encabezado de una columna ordenable es un botón. Pulsa <kbd>Enter</kbd> o <kbd>Space</kbd> para ordenar por ella, y otra vez para invertir el orden; los lectores de pantalla anuncian el orden.
- **Los stacks de la lista de contenedores** se abren y se cierran con el botón que hay antes de su nombre, que indica si está desplegado.
- **Los diálogos** toman el foco al abrirse, lo mantienen dentro mientras están abiertos y lo devuelven a lo que los abrió al cerrarse. <kbd>Esc</kbd> cierra un diálogo sin hacer nada.
- **Los menús, las listas desplegables y las pestañas** se recorren con las flechas, <kbd>Home</kbd> y <kbd>End</kbd>; <kbd>Enter</kbd> elige y <kbd>Esc</kbd> cierra.
- **Atajos**: <kbd>?</kbd> en cualquier página lista los que funcionan allí; consulta [Teclado](/docs/guides/keyboard/).

## Terminales, logs y el editor

El **Terminal** de un contenedor y el visor de logs reciben todas las teclas mientras tienen el foco, así que el programa de dentro también recibe <kbd>Tab</kbd>, <kbd>Esc</kbd> y las teclas con <kbd>Ctrl</kbd>. Sal de ellos con el ratón, o moviendo el foco con los comandos propios de tu lector de pantalla.

Los editores de compose, YAML, jobs y unidades son editores CodeMirror con nombre para los lectores de pantalla. En ellos, <kbd>Tab</kbd> sangra; pulsa <kbd>Esc</kbd> y luego <kbd>Tab</kbd> para sacar el foco del editor.

## Movimiento reducido

Cuando tu sistema operativo pide movimiento reducido (**Reducir movimiento** en macOS, **Efectos de animación** desactivado en Windows, **Reduce animation** en GNOME y KDE), Homeport solo usa fundidos: los diálogos, menús, listas desplegables e información sobre herramientas no se amplían ni se deslizan, nada parpadea, los botones no se mueven al pulsarlos y el cursor del terminal deja de parpadear. Los indicadores de carga siguen girando, más despacio, porque indican que hay trabajo en curso. El ajuste se sigue a medida que cambia, sin reiniciar.

## Lectores de pantalla

Cada página y los diálogos principales se comprueban con [axe](https://github.com/dequelabs/axe-core) en cada cambio: los botones, los campos y las regiones tienen nombre, los encabezados y las zonas de referencia están en orden, y nada usa ARIA de forma incorrecta. Homeport define el idioma de la página, así que un lector de pantalla lee cada idioma con su propia voz, y el árabe se muestra de derecha a izquierda.

La ventana de Homeport es una vista web: WebKitGTK en Linux, WKWebView en macOS y WebView2 en Windows. Hay en curso una auditoría práctica con Orca, VoiceOver y NVDA en cada uno ([#184](https://github.com/DamienReichhart/Homeport/issues/184)); hasta que termine, considera que la experiencia con lector de pantalla solo se ha probado con herramientas.

### Límites conocidos

- **Terminales y logs**: los lectores de pantalla todavía no leen el texto del terminal de un contenedor ni el del visor de logs. Copia la salida, o abre los logs en un terminal con `docker logs`.
- **Listas en directo**: las listas se actualizan en su sitio a medida que el Engine notifica cambios, sin anunciar cada cambio. Las notificaciones emergentes anuncian lo que hiciste.
- **Las gráficas** dibujan sus valores; el último valor de cada una figura también en el título de la gráfica, como texto.

## Contraste

El texto cumple WCAG 2.2 AA (4,5:1) tanto en el tema claro como en el oscuro, y lo mismo los bordes de los campos, el anillo de foco y las formas de estado (3:1). El estado nunca depende solo del color: cada uno tiene su propia forma. Las proporciones figuran en la guía de marca del proyecto.

## Notificar una barrera

Si algo te impide usar Homeport con tu teclado, lector de pantalla, ampliador u otra tecnología de apoyo, [abre una issue](https://github.com/DamienReichhart/Homeport/issues/new?labels=accessibility) con la etiqueta `accessibility`. Indica:

- qué intentabas hacer, y dónde (página, diálogo, tipo de host);
- tu sistema operativo y su versión, y la tecnología de apoyo y su versión;
- qué oíste o viste, y qué esperabas.

Las barreras se tratan como errores, no como peticiones de funciones.
