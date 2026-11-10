---
title: Aus dem Quellcode bauen
description: Homeport aus seinem Quellcode bauen und ausführen.
---

Homeport ist eine [Tauri-2](https://tauri.app)-App: eine Oberfläche in React und TypeScript über einem Rust-Kern, der mit [bollard](https://github.com/fussybeaver/bollard) mit Docker Engines spricht und seine Daten in SQLite speichert.

## Voraussetzungen

- [Bun](https://bun.sh) 1.4 oder neuer
- [Rust](https://www.rust-lang.org/tools/install), stabil (die `rust-toolchain.toml` des Repositorys wählt die Komponenten)
- Die [Systemabhängigkeiten von Tauri](https://tauri.app/start/prerequisites/) für Ihre Plattform. Unter Debian und Ubuntu:

  ```sh
  sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libsoup-3.0-dev librsvg2-dev \
    libayatana-appindicator3-dev libssl-dev
  ```

- Für Release-Builds [cargo-about](https://github.com/EmbarkStudios/cargo-about), das die Lizenzhinweise für Drittanbieter erzeugt:

  ```sh
  cargo install cargo-about --locked --features cli
  ```

## Ausführen

```sh
git clone https://github.com/DamienReichhart/Homeport.git
cd Homeport
bun install
bun run tauri:dev
```

`tauri:dev` startet die App mit Hot Reload für die Oberfläche; Rust-Änderungen bauen sie neu und starten sie neu.

## Installationsprogramme bauen

```sh
bun run tauri:build
```

Die Installationsprogramme für Ihre Plattform werden nach `target/release/bundle/` geschrieben. Der Build erzeugt zuerst die Lizenzhinweise und bricht ab, wenn die Lizenz einer Abhängigkeit nicht in `about.toml` geprüft wurde.

Lokale Builds sind nicht für den In-App-Updater signiert und lassen die `.rpm` unsigniert; nur der [Release-Workflow](https://github.com/DamienReichhart/Homeport/blob/main/docs/RELEASING.md) hat die Signaturschlüssel. Eine selbst gebaute Kopie sucht trotzdem nach Updates, aber ihr **Installieren und neu starten** installiert nur Versionen, die vom Release-Workflow signiert wurden.

## Nützliche Befehle

| Befehl                | Was er tut                                                                                                                                                                                                                                                     |
| --------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `bun run check`       | Typprüfung, Lint, Formatprüfung und Frontend-Tests                                                                                                                                                                                                             |
| `bun run rust:lint`   | `cargo clippy` mit Warnungen als Fehler                                                                                                                                                                                                                        |
| `bun run rust:test`   | Rust-Unit-Tests                                                                                                                                                                                                                                                |
| `bun run bindings`    | Die TypeScript-Bindings für die IPC neu erzeugen                                                                                                                                                                                                               |
| `bun run licenses`    | Die Lizenzhinweise für Drittanbieter erzeugen                                                                                                                                                                                                                  |
| `bun run screenshots` | Die auf dieser Website verwendeten Screenshots in jeder Sprache neu aufnehmen (`--locale fr`, um eine neu aufzunehmen), in 2880×1800, der doppelten Pixeldichte. Vorher oxipng installieren (`cargo install oxipng`), damit sie verlustfrei komprimiert werden |

Integrationstests sprechen mit der lokalen Docker Engine und der Compose-CLI. Sie sind mit `#[ignore = "needs a local Docker Engine"]` markiert (oder `"… and the compose CLI"`, und einige mit `"… access to Docker Hub"`), erstellen nur Ressourcen namens `homeport-test-*` und entfernen sie danach. Führen Sie sie mit einer Engine und dem Compose-Plugin auf diesem Rechner aus:

```sh
cargo test --workspace --exclude homeport -- --ignored
```

Die CI führt sie ebenfalls aus, im Job `engine-tests` von `.github/workflows/ci.yml`, bei jedem Push auf `dev`, jedem Pull Request und jedem Release. Der Job führt dann `scripts/ci/engine-test-leftovers.sh` aus, das fehlschlägt, wenn ein Test einen `homeport-test-*`-Container, ein Image, ein Volume oder ein Netzwerk zurückgelassen hat; führen Sie es nach den Tests lokal aus, um dasselbe zu prüfen.

## Diese Website

Die Website liegt in `website/` und wird mit [Astro](https://astro.build) und [Starlight](https://starlight.astro.build) gebaut:

```sh
cd website
bun install
bun run dev      # http://localhost:4321/Homeport/
bun run build    # checks types and links, then builds into website/dist/
```

Sie wird vom Workflow **Pages** bei jedem Push auf `main`, der sie betrifft, und nach jedem Release auf GitHub Pages bereitgestellt, damit die Download-Links aktuell bleiben.
