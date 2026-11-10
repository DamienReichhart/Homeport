---
title: Compiler depuis les sources
description: Compiler et exécuter Homeport à partir de son code source.
---

Homeport est une application [Tauri 2](https://tauri.app) : une interface React et TypeScript au-dessus d’un cœur Rust, qui parle aux Docker Engines avec [bollard](https://github.com/fussybeaver/bollard) et enregistre ses données dans SQLite.

## Prérequis

- [Bun](https://bun.sh) 1.4 ou ultérieur
- [Rust](https://www.rust-lang.org/tools/install), stable (le `rust-toolchain.toml` du dépôt choisit les composants)
- Les [dépendances système de Tauri](https://tauri.app/start/prerequisites/) pour votre plateforme. Sous Debian et Ubuntu :

  ```sh
  sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libsoup-3.0-dev librsvg2-dev \
    libayatana-appindicator3-dev libssl-dev
  ```

- Pour les builds de release, [cargo-about](https://github.com/EmbarkStudios/cargo-about), qui génère les mentions de licence des dépendances tierces :

  ```sh
  cargo install cargo-about --locked --features cli
  ```

## L’exécuter

```sh
git clone https://github.com/DamienReichhart/Homeport.git
cd Homeport
bun install
bun run tauri:dev
```

`tauri:dev` démarre l’application avec le rechargement à chaud de l’interface ; les modifications Rust la recompilent et la redémarrent.

## Construire les programmes d’installation

```sh
bun run tauri:build
```

Les programmes d’installation pour votre plateforme sont écrits dans `target/release/bundle/`. Le build génère d’abord les mentions de licence, et s’arrête si la licence d’une dépendance n’a pas été examinée dans `about.toml`.

Les builds locaux ne sont pas signés pour le système de mise à jour intégré et laissent le `.rpm` non signé ; seul le [workflow de release](https://github.com/DamienReichhart/Homeport/blob/main/docs/RELEASING.md) dispose des clés de signature. Une copie que vous compilez vous-même recherche quand même les mises à jour, mais son **Installer et redémarrer** n’installe que des versions signées par le workflow de release.

## Commandes utiles

| Commande              | Ce qu’elle fait                                                                                                                                                                                                                                                       |
| --------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `bun run check`       | Vérification des types, lint, vérification du formatage et tests du frontend                                                                                                                                                                                          |
| `bun run rust:lint`   | `cargo clippy` avec les avertissements traités comme des erreurs                                                                                                                                                                                                      |
| `bun run rust:test`   | Tests unitaires Rust                                                                                                                                                                                                                                                  |
| `bun run bindings`    | Régénère les liaisons TypeScript de l’IPC                                                                                                                                                                                                                             |
| `bun run licenses`    | Génère les mentions de licence des dépendances tierces                                                                                                                                                                                                                |
| `bun run screenshots` | Refait les captures d’écran utilisées sur ce site web, dans chaque langue (`--locale fr` pour en refaire une seule), en 2880×1800, soit deux fois la densité de pixels. Installez d’abord oxipng (`cargo install oxipng`) pour qu’elles soient compressées sans perte |

Les tests d’intégration parlent au Docker Engine local et au CLI Compose. Ils sont marqués `#[ignore = "needs a local Docker Engine"]` (ou `"… and the compose CLI"`, et quelques-uns `"… access to Docker Hub"`), ne créent que des ressources nommées `homeport-test-*` et les suppriment ensuite. Lancez-les avec un moteur et le plugin compose sur cette machine :

```sh
cargo test --workspace --exclude homeport -- --ignored
```

La CI les exécute aussi, dans le job `engine-tests` de `.github/workflows/ci.yml`, à chaque push sur `dev`, à chaque pull request et à chaque release. Le job exécute ensuite `scripts/ci/engine-test-leftovers.sh`, qui échoue si un test a laissé derrière lui un conteneur, une image, un volume ou un réseau `homeport-test-*` ; exécutez-le en local après les tests pour faire la même vérification.

### Tests de bout en bout

`e2e/` pilote la vraie application — le backend Rust, la webview et les appels entre les deux — avec [WebdriverIO](https://webdriver.io) via `tauri-driver`, contre le Docker Engine local. Les scénarios : l’application démarre et se connecte au moteur local ; les journaux d’un conteneur défilent et il s’arrête depuis l’application ; une image est récupérée ; une stack gérée est déployée avec un environnement chiffré ; un redémarrage conserve les hôtes, la stack et son secret ; et un hôte SSH est ajouté avec une clé existante, sa clé de serveur approuvée, puis connecté. Ils trouvent les éléments par leur rôle, leur libellé et leur texte, jamais par une classe CSS.

Ils ne tournent que sous Linux : macOS n’a pas de WebDriver pour sa webview. En plus des [prérequis](#prerequisites) et d’un Docker Engine, ils ont besoin de `tauri-driver` (`cargo install tauri-driver --locked`), du pilote de WebKit, d’un écran virtuel et d’un trousseau : sous Debian et Ubuntu, `webkit2gtk-driver`, `xvfb`, `dbus-x11` et `gnome-keyring`.

```sh
bun tauri build --debug --no-bundle
cd e2e
bun install
./run.sh
```

Le build embarque le frontend, comme le fait une release, donc une commande absente de `commands::builder()`, une permission absente de `src-tauri/capabilities/` ou un style que bloque la Content Security Policy échoue ici. `run.sh` donne à chaque exécution un dossier de données neuf, son propre trousseau et son propre écran, pour qu’il ne touche jamais à votre propre Homeport. Passez les options de WebdriverIO après lui, par exemple `./run.sh --spec specs/ssh.e2e.ts`. Le scénario SSH construit une petite image de serveur SSH à partir de `e2e/support/sshd/` ; définissez `HOMEPORT_E2E_SSHD_IMAGE` pour en utiliser une construite à l’avance. Les conteneurs que démarrent les tests sont nommés `homeport-e2e-*` et supprimés ensuite.

Un échec laisse une capture d’écran et le journal de l’application dans `e2e/artifacts/`. La CI exécute les tests dans le job `e2e` de `.github/workflows/ci.yml`, à chaque push sur `dev` et à chaque pull request, et conserve ce dossier quand l’un d’eux échoue.

## Ce site web

Le site se trouve dans `website/` et est construit avec [Astro](https://astro.build) et [Starlight](https://starlight.astro.build) :

```sh
cd website
bun install
bun run dev      # http://localhost:4321/Homeport/
bun run build    # checks types and links, then builds into website/dist/
```

Il est déployé sur GitHub Pages par le workflow **Pages** à chaque push sur `main` qui le touche, et après chaque release pour que les liens de téléchargement restent à jour.
