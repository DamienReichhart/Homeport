---
title: Contribuir
description: Cómo notificar problemas, sugerir funciones y contribuir con código a Homeport.
---

Homeport se desarrolla de forma abierta en [github.com/DamienReichhart/Homeport](https://github.com/DamienReichhart/Homeport). Las contribuciones de cualquier tamaño son bienvenidas.

## Notificar un error

[Abre una issue](https://github.com/DamienReichhart/Homeport/issues/new/choose) con:

- lo que hiciste, lo que esperabas y lo que pasó: el mensaje exacto es lo que más ayuda;
- el [archivo de diagnóstico](/docs/reference/troubleshooting/#exportar-el-diagnóstico-para-un-informe-de-error) de **Ajustes → Acerca de → Exportar diagnóstico…**: un solo zip con la versión y la plataforma de Homeport, los tipos de hosts, Engines y clústeres que usas, y sus logs sin secretos. Lees lo que contiene antes de guardarlo, y no se envía nada a ninguna parte.

Si Homeport no arranca, indica en su lugar su versión y tu sistema operativo, y lo que sea relevante del [archivo de log](/docs/reference/security/#logs).

Notifica los problemas de seguridad de forma privada, a través de la [notificación de vulnerabilidades de GitHub](https://github.com/DamienReichhart/Homeport/security/advisories/new).

## Sugerir una función

Abre una issue que describa el problema que quieres resolver, no solo la solución. Saber cómo trabajas con tus Engines ayuda a encontrarle la forma adecuada.

## Contribuir con código

1. Lee [`docs/ARCHITECTURE.md`](https://github.com/DamienReichhart/Homeport/blob/main/docs/ARCHITECTURE.md). Las funciones son cortes verticales autónomos, el frontend llama a Rust solo a través de los bindings IPC generados, y la lógica de Rust vive en servicios que se prueban unitariamente sin Tauri.
2. Usa el vocabulario del proyecto de [`GLOSSARY.md`](https://github.com/DamienReichhart/Homeport/blob/main/GLOSSARY.md) —Host, Stack, Managed, External, Adopt— en el código, en los textos de la interfaz y en los commits. Las decisiones principales se recogen en [`docs/adr`](https://github.com/DamienReichhart/Homeport/tree/main/docs/adr).
3. Sigue la [guía de marca](https://github.com/DamienReichhart/Homeport/blob/main/docs/BRAND.md) en cuanto a colores y tono: sencillo, concreto, sereno, en ortografía británica.
4. Asegúrate de que esto pasa:

   ```sh
   bun run check
   bun run rust:lint
   bun run rust:test
   ```

5. Cumple la promesa de los datos: cada versión de Homeport abre los datos de todas las versiones anteriores, hasta la 0.1.0, y los actualiza sin perder nada: hosts, stacks, clústeres, ajustes y secretos. Un cambio de esquema es una migración nueva, nunca una modificación de una ya publicada, y la CI lo comprueba con una base de datos de ejemplo de cada release. Consulta [Data compatibility](https://github.com/DamienReichhart/Homeport/blob/main/docs/RELEASING.md#data-compatibility).
6. Escribe los mensajes de commit como [Conventional Commits](https://www.conventionalcommits.org): `feat(stacks): …`, `fix(hosts): …`. Las notas de la versión se generan a partir de ellos.
7. Abre una pull request contra la rama `dev`. La CI ejecuta las comprobaciones del frontend y la comprobación de licencias en Linux, y `cargo clippy` y `cargo test` en Linux, macOS y Windows, así que el código bajo `#[cfg(windows)]` o `#[cfg(target_os = "macos")]` se compila antes de fusionarse. Una prueba que necesita un Docker Engine se marca con `#[ignore = "needs a local Docker Engine"]`; un job de Linux aparte las ejecuta contra el Engine del runner.

Consulta [compilar desde el código fuente](/docs/project/building/) para prepararte.

### Añadir texto a la app

El texto de la app vive en un catálogo, nunca en línea, para que se pueda traducir ([ADR 0011](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0011-formatjs-catalogues-and-keyed-errors.md)). Para añadir una cadena:

1. Añade una clave al archivo de la función en [`src/shared/i18n/en/`](https://github.com/DamienReichhart/Homeport/tree/main/src/shared/i18n/en): `images.ts` para la función de imágenes, `ui.ts` para el kit de interfaz, `common.ts` solo para palabras que significan lo mismo en todas partes (Cancel, Close, Retry). Las claves empiezan por el nombre del archivo y dicen para qué es el texto: `images.remove.title`, `images.column.size`.
2. Escribe el valor como un [mensaje ICU](https://formatjs.github.io/docs/core-concepts/icu-syntax/): `{name}` para un valor, `{count, plural, one {# image} other {# images}}` en lugar de construir los plurales a mano, `<strong>…</strong>` para el énfasis que se muestra con `tRich`. Una clave por frase; no pegues fragmentos, porque el orden de las palabras cambia de un idioma a otro.
3. Úsala con `t('images.remove.title', { name })` desde `@/shared/i18n`, igual en componentes, mutaciones y notificaciones emergentes. Las claves tienen tipo, así que un error tipográfico hace fallar la comprobación de tipos. Usa claves literales: cuando el texto depende de un valor, asigna el valor a claves (`{ start: 'containers.done.start', stop: 'containers.done.stop' }`) en lugar de construir `` `containers.done.${action}` ``.

`bun run lint` falla cuando un componente escribe texto en línea: texto JSX, props que leen las personas (`title`, `aria-label`, `placeholder`, `description`, `label`…) y el texto de las notificaciones emergentes y las confirmaciones. Las cadenas que se leen como código y no como palabras —`tcp`, `nginx:latest`, `/dev/dri`, `max-size=10m`— y los nombres propios como Docker o Podman pasan. Para mantener un literal a propósito, explica por qué en la línea anterior: `// eslint-disable-next-line homeport/no-literal-ui-string -- a shell command`. `bun run check` también falla con una clave usada en el código que falta en el catálogo, una clave que nada usa y una clave construida en tiempo de ejecución.

Los mensajes de error se escriben en Rust: cada crate declara sus claves y plantillas en inglés en su bloque `messages!`, y `bun run bindings` las escribe en `src/shared/i18n/en/errors.json`. No edites ese archivo a mano.

## Traducir Homeport

La app y el sitio web están en inglés, francés, español, alemán, italiano, portugués de Brasil, ruso, chino simplificado, japonés, coreano, hindi y árabe. Cada idioma usa una sola palabra por término del producto: búscalas en [`docs/i18n/`](https://github.com/DamienReichhart/Homeport/tree/main/docs/i18n) antes de traducir, y añade allí una fila cuando elijas una nueva.

### Añadir o cambiar una cadena

Sea cual sea la cadena, `bun run check` (y por tanto la CI) falla hasta que todos los idiomas la tienen, con los mismos `{arguments}` y etiquetas, y con todas las formas de plural que necesita ese idioma.

- **Texto de la app**: añade la clave a `src/shared/i18n/en/<namespace>.ts` (consulta [Añadir texto a la app](#añadir-texto-a-la-app)) y luego al archivo del mismo nombre en la carpeta de cada uno de los demás idiomas (`fr/`, `de/`, `zh-CN/`…). Tradúcela si puedes. Si no puedes, copia allí el texto en inglés y dilo en la pull request, para que un traductor pueda encargarse; nunca omitas la clave.
- **Un mensaje de error o una sugerencia**: decláralo en el bloque `messages!` del crate y ejecuta `bun run bindings`, que escribe `en/errors.json`; luego añade la clave al `errors.json` de cada idioma. El texto que muestra el sistema operativo —el menú de la bandeja y las notificaciones— funciona igual, con una clave `native.` y `native.json`.
- **Texto del sitio web** (inicio, descarga, registro de cambios, cabecera y pie de página): añade la clave al archivo correspondiente de `website/src/core/i18n/strings/en/` (`site`, `home`, `download` o `changelog`) y al mismo archivo de la carpeta de cada uno de los demás idiomas; `astro check` falla cuando falta en uno. Una etiqueta de la barra lateral de la documentación va en `website/src/app/config/sidebar/<code>.ts`.
- **Una página de la documentación**: la página en inglés está en `website/src/content/docs/docs/`. Cuando la cambies, actualiza la misma página en todos los idiomas, en la misma ruta bajo `website/src/content/docs/<code>/docs/`, en la misma pull request; un idioma cuya página no puedas actualizar muestra su traducción anterior hasta que un traductor lo haga. Mantén los encabezados de las páginas traducidas en el mismo orden y en los mismos niveles que en inglés, y los enlaces exactamente como los escribe el inglés, anclas incluidas: después de `bun run build`, `bun scripts/localize-anchors.ts <code>` apunta cada ancla al encabezado traducido, y la comprobación de enlaces de una segunda compilación lo confirma.

Comprueba un idioma con `bun run i18n:check --locale fr`.

### Añadir un idioma

Archivos, más una línea para el sitio web:

1. **App**: copia `src/shared/i18n/en/` a `src/shared/i18n/<tag>/` (una etiqueta BCP 47 como `pl` o `zh-TW`), escribe su `meta.ts` (`{ name: 'Polski', dir: 'ltr' }`, el nombre del idioma en el propio idioma; `rtl` para una escritura de derecha a izquierda) y traduce cada archivo, conservando cada clave y convirtiendo cada `export default { … }` en `export default { … } satisfies Translation<typeof en>`, como hacen los demás idiomas. `bun run i18n:check --locale <tag>` lista lo que falta. Ajustes lista el nuevo idioma, y el idioma del sistema operativo lo elige, sin ningún otro cambio.
2. **Sitio web**: añade el idioma a `website/src/core/site/locales.ts`, y luego sus cadenas en una copia de `website/src/core/i18n/strings/en/` llamada `<code>/` (registrada en `TABLES` en `core/i18n/index.ts`), sus etiquetas de la barra lateral en `website/src/app/config/sidebar/<code>.ts` (registradas en `sidebar.ts`; estas dos líneas son el único código que necesita un idioma, porque esos archivos se cargan fuera de Vite, que es lo que encuentra las carpetas) y la nota de Starlight para las páginas sin traducir en `website/src/content/i18n/<tag>.json`. Sus páginas, URL, entradas del selector de idioma, enlaces `hreflang` y alternativas del sitemap se derivan de la lista. Su documentación va en `website/src/content/docs/<code>/docs/`: traduce allí todas las páginas (una que falte muestra el inglés con una nota) y luego ejecuta `bun scripts/localize-anchors.ts <code>` después de una compilación.
3. **Capturas de pantalla**: `bun run screenshots` captura todas las imágenes en todos los idiomas, incluido el nuevo, en `website/src/assets/screenshots/<tag>/`, de modo que cada página muestra la app en el idioma de su lector. El `bun test` del sitio web falla mientras a un idioma le falte una captura que tiene el inglés.
4. **Términos**: añade `docs/i18n/terms-<tag>.md`.

Si una prueba no ve un idioma que acabas de añadir, borra `node_modules/.vitest-cache`: Vitest puede conservar una lista antigua de archivos de catálogo.

## Mejorar esta documentación

Cada página de la documentación tiene un enlace **Editar página** al final. Las páginas son archivos Markdown en [`website/src/content/docs`](https://github.com/DamienReichhart/Homeport/tree/main/website/src/content/docs).

## Cómo se publican las releases

Cada push a `main` con un número de versión nuevo compila instaladores para todas las plataformas, ejecuta todo el conjunto de pruebas y publica una release de GitHub con sumas de comprobación y procedencia de la compilación. El proceso se describe en [`docs/RELEASING.md`](https://github.com/DamienReichhart/Homeport/blob/main/docs/RELEASING.md).
