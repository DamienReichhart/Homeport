---
title: Compilar desde el código fuente
description: Compila y ejecuta Homeport a partir de su código fuente.
---

Homeport es una app [Tauri 2](https://tauri.app): una interfaz de React y TypeScript sobre un núcleo en Rust, que habla con los Docker Engines mediante [bollard](https://github.com/fussybeaver/bollard) y guarda sus datos en SQLite.

## Requisitos previos

- [Bun](https://bun.sh) 1.4 o posterior
- [Rust](https://www.rust-lang.org/tools/install), estable (el `rust-toolchain.toml` del repositorio elige los componentes)
- Las [dependencias de sistema de Tauri](https://tauri.app/start/prerequisites/) para tu plataforma. En Debian y Ubuntu:

  ```sh
  sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libsoup-3.0-dev librsvg2-dev \
    libayatana-appindicator3-dev libssl-dev
  ```

- Para las compilaciones de release, [cargo-about](https://github.com/EmbarkStudios/cargo-about), que genera los avisos de licencia de terceros:

  ```sh
  cargo install cargo-about --locked --features cli
  ```

## Ejecutarlo

```sh
git clone https://github.com/DamienReichhart/Homeport.git
cd Homeport
bun install
bun run tauri:dev
```

`tauri:dev` inicia la app con recarga en caliente para la interfaz; los cambios en Rust la recompilan y la reinician.

## Compilar los instaladores

```sh
bun run tauri:build
```

Los instaladores para tu plataforma se escriben en `target/release/bundle/`. La compilación genera primero los avisos de licencia, y se detiene si la licencia de una dependencia no se ha revisado en `about.toml`.

Las compilaciones locales no están firmadas para el actualizador integrado y dejan el `.rpm` sin firmar; solo el [workflow de release](https://github.com/DamienReichhart/Homeport/blob/main/docs/RELEASING.md) tiene las claves de firma. Una copia que compiles tú mismo sigue buscando actualizaciones, pero su **Instalar y reiniciar** solo instala versiones firmadas por el workflow de release.

## Comandos útiles

| Comando               | Qué hace                                                                                                                                                                                                                                           |
| --------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `bun run check`       | Comprobación de tipos, lint, comprobación de formato y pruebas del frontend                                                                                                                                                                        |
| `bun run rust:lint`   | `cargo clippy` con los avisos tratados como errores                                                                                                                                                                                                |
| `bun run rust:test`   | Pruebas unitarias de Rust                                                                                                                                                                                                                          |
| `bun run bindings`    | Vuelve a generar los bindings de TypeScript para el IPC                                                                                                                                                                                            |
| `bun run licenses`    | Genera los avisos de licencia de terceros                                                                                                                                                                                                          |
| `bun run screenshots` | Vuelve a capturar las capturas de pantalla de este sitio web, en todos los idiomas (`--locale fr` para rehacer uno), a 2880×1800, el doble de densidad de píxeles. Instala antes oxipng (`cargo install oxipng`) para que se compriman sin pérdida |

Las pruebas de integración hablan con el Docker Engine local y con la CLI de Compose. Están marcadas con `#[ignore = "needs a local Docker Engine"]` (o `"… and the compose CLI"`, y unas pocas con `"… access to Docker Hub"`), solo crean recursos llamados `homeport-test-*` y los eliminan después. Ejecútalas con un Engine y el plugin compose en este equipo:

```sh
cargo test --workspace --exclude homeport -- --ignored
```

La CI también las ejecuta, en el job `engine-tests` de `.github/workflows/ci.yml`, en cada push a `dev`, en cada pull request y en cada release. Después, el job ejecuta `scripts/ci/engine-test-leftovers.sh`, que falla si una prueba dejó atrás un contenedor, una imagen, un volumen o una red `homeport-test-*`; ejecútalo en local después de las pruebas para comprobar lo mismo.

### Pruebas de extremo a extremo

`e2e/` controla la app real —el backend de Rust, la webview y las llamadas entre ambos— con [WebdriverIO](https://webdriver.io) a través de `tauri-driver`, contra el Docker Engine local. Los escenarios: la app arranca y se conecta al Engine local; los logs de un contenedor se transmiten y se detiene desde la app; se descarga una imagen; se despliega un stack gestionado con un entorno cifrado; un reinicio conserva los hosts, el stack y su secreto; y se añade un host SSH con una clave existente, se acepta la clave de su servidor y se conecta. Encuentran los elementos por su rol, su etiqueta y su texto, nunca por una clase CSS.

Solo se ejecutan en Linux: macOS no tiene WebDriver para su webview. Además de los [requisitos previos](#prerequisites) y un Docker Engine, necesitan `tauri-driver` (`cargo install tauri-driver --locked`), el driver de WebKit, una pantalla virtual y un llavero: en Debian y Ubuntu, `webkit2gtk-driver`, `xvfb`, `dbus-x11` y `gnome-keyring`.

```sh
bun tauri build --debug --no-bundle
cd e2e
bun install
./run.sh
```

La compilación incluye el frontend, como hace una release, así que un comando que falte en `commands::builder()`, un permiso que falte en `src-tauri/capabilities/` o un estilo que bloquee la Content Security Policy falla aquí. `run.sh` da a cada ejecución una carpeta de datos nueva, su propio llavero y su propia pantalla, así que nunca toca tu propio Homeport. Pasa las opciones de WebdriverIO después, por ejemplo `./run.sh --spec specs/ssh.e2e.ts`. El escenario SSH compila una pequeña imagen de servidor SSH a partir de `e2e/support/sshd/`; define `HOMEPORT_E2E_SSHD_IMAGE` para usar una compilada de antemano. Los contenedores que inician las pruebas se llaman `homeport-e2e-*` y se eliminan después.

Un fallo deja una captura de pantalla y el log de la app en `e2e/artifacts/`. La CI ejecuta las pruebas en el job `e2e` de `.github/workflows/ci.yml`, en cada push a `dev` y en cada pull request, y conserva esa carpeta cuando una falla.

## Este sitio web

El sitio está en `website/` y se compila con [Astro](https://astro.build) y [Starlight](https://starlight.astro.build):

```sh
cd website
bun install
bun run dev      # http://localhost:4321/Homeport/
bun run build    # checks types and links, then builds into website/dist/
```

Lo despliega en GitHub Pages el workflow **Pages** en cada push a `main` que lo modifica, y después de cada release, para que los enlaces de descarga estén siempre al día.
