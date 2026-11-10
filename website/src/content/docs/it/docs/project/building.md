---
title: Compilare dal codice sorgente
description: Compila ed esegui Homeport dal suo codice sorgente.
---

Homeport è un’app [Tauri 2](https://tauri.app): un’interfaccia React e TypeScript sopra un nucleo Rust, che comunica con i Docker Engine tramite [bollard](https://github.com/fussybeaver/bollard) e memorizza i suoi dati in SQLite.

## Prerequisiti

- [Bun](https://bun.sh) 1.4 o successivo
- [Rust](https://www.rust-lang.org/tools/install), stabile (il file `rust-toolchain.toml` del repository sceglie i componenti)
- Le [dipendenze di sistema di Tauri](https://tauri.app/start/prerequisites/) per la tua piattaforma. Su Debian e Ubuntu:

  ```sh
  sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libsoup-3.0-dev librsvg2-dev \
    libayatana-appindicator3-dev libssl-dev
  ```

- Per le build di release, [cargo-about](https://github.com/EmbarkStudios/cargo-about), che genera le note di licenza di terze parti:

  ```sh
  cargo install cargo-about --locked --features cli
  ```

## Eseguilo

```sh
git clone https://github.com/DamienReichhart/Homeport.git
cd Homeport
bun install
bun run tauri:dev
```

`tauri:dev` avvia l’app con il ricaricamento a caldo dell’interfaccia; le modifiche a Rust la ricompilano e la riavviano.

## Compilare i programmi di installazione

```sh
bun run tauri:build
```

I programmi di installazione per la tua piattaforma vengono scritti in `target/release/bundle/`. La build genera prima le note di licenza, e si ferma se la licenza di una dipendenza non è stata esaminata in `about.toml`.

Le build locali non sono firmate per l’aggiornamento integrato nell’app e lasciano l’`.rpm` non firmato; solo il [workflow di release](https://github.com/DamienReichhart/Homeport/blob/main/docs/RELEASING.md) ha le chiavi di firma. Una copia che compili da te controlla comunque gli aggiornamenti, ma il suo **Installa e riavvia** installa solo versioni firmate dal workflow di release.

## Comandi utili

| Comando               | Cosa fa                                                                                                                                                                                                                             |
| --------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `bun run check`       | Controllo dei tipi, lint, controllo della formattazione e test del frontend                                                                                                                                                         |
| `bun run rust:lint`   | `cargo clippy` con gli avvisi trattati come errori                                                                                                                                                                                  |
| `bun run rust:test`   | Test unitari Rust                                                                                                                                                                                                                   |
| `bun run bindings`    | Rigenera i binding TypeScript per l’IPC                                                                                                                                                                                             |
| `bun run licenses`    | Genera le note di licenza di terze parti                                                                                                                                                                                            |
| `bun run screenshots` | Ricattura gli screenshot usati in questo sito, in ogni lingua (`--locale fr` per rifarne una), a 2880×1800, il doppio della densità di pixel. Installa prima oxipng (`cargo install oxipng`) perché vengano compressi senza perdita |

I test di integrazione comunicano con il Docker Engine locale e con la CLI Compose. Sono marcati `#[ignore = "needs a local Docker Engine"]` (o `"… and the compose CLI"`, e alcuni `"… access to Docker Hub"`), creano solo risorse chiamate `homeport-test-*` e le rimuovono al termine. Eseguili con un Engine e il plugin compose su questo computer:

```sh
cargo test --workspace --exclude homeport -- --ignored
```

Anche la CI li esegue, nel job `engine-tests` di `.github/workflows/ci.yml`, a ogni push su `dev`, a ogni pull request e a ogni release. Il job esegue poi `scripts/ci/engine-test-leftovers.sh`, che fallisce se un test ha lasciato un container, un’immagine, un volume o una rete `homeport-test-*`; eseguilo in locale dopo i test per controllare la stessa cosa.

## Questo sito

Il sito si trova in `website/` ed è costruito con [Astro](https://astro.build) e [Starlight](https://starlight.astro.build):

```sh
cd website
bun install
bun run dev      # http://localhost:4321/Homeport/
bun run build    # checks types and links, then builds into website/dist/
```

Viene pubblicato su GitHub Pages dal workflow **Pages** a ogni push su `main` che lo modifica, e dopo ogni release perché i link di download restino aggiornati.
