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

| Comando               | Qué hace                                                                                    |
| --------------------- | ------------------------------------------------------------------------------------------- |
| `bun run check`       | Comprobación de tipos, lint, comprobación de formato y pruebas del frontend                 |
| `bun run rust:lint`   | `cargo clippy` con los avisos tratados como errores                                         |
| `bun run rust:test`   | Pruebas unitarias de Rust                                                                   |
| `bun run bindings`    | Vuelve a generar los bindings de TypeScript para el IPC                                     |
| `bun run licenses`    | Genera los avisos de licencia de terceros                                                   |
| `bun run screenshots` | Vuelve a capturar las capturas de pantalla de este sitio web (`--locale fr` para un idioma) |

Las pruebas de integración hablan con el Docker Engine local y con la CLI de Compose. Están marcadas con `#[ignore = "needs a local Docker Engine"]` (o `"… and the compose CLI"`, y unas pocas con `"… access to Docker Hub"`), solo crean recursos llamados `homeport-test-*` y los eliminan después. Ejecútalas con un Engine y el plugin compose en este equipo:

```sh
cargo test --workspace --exclude homeport -- --ignored
```

La CI también las ejecuta, en el job `engine-tests` de `.github/workflows/ci.yml`, en cada push a `dev`, en cada pull request y en cada release. Después, el job ejecuta `scripts/ci/engine-test-leftovers.sh`, que falla si una prueba dejó atrás un contenedor, una imagen, un volumen o una red `homeport-test-*`; ejecútalo en local después de las pruebas para comprobar lo mismo.

## Este sitio web

El sitio está en `website/` y se compila con [Astro](https://astro.build) y [Starlight](https://starlight.astro.build):

```sh
cd website
bun install
bun run dev      # http://localhost:4321/Homeport/
bun run build    # checks types and links, then builds into website/dist/
```

Lo despliega en GitHub Pages el workflow **Pages** en cada push a `main` que lo modifica, y después de cada release, para que los enlaces de descarga estén siempre al día.
