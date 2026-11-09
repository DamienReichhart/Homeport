---
title: Contribuer
description: Comment signaler des problèmes, proposer des fonctionnalités et contribuer au code de Homeport.
---

Homeport est développé ouvertement sur [github.com/DamienReichhart/Homeport](https://github.com/DamienReichhart/Homeport). Les contributions de toutes tailles sont les bienvenues.

## Signaler un bug

[Ouvrez une issue](https://github.com/DamienReichhart/Homeport/issues/new) avec :

- votre version de Homeport et votre plateforme, depuis **Paramètres** (par exemple `v0.2.0 · macos/aarch64`) ;
- le type d’hôte (local, SSH, TLS ou TCP) et la version de son moteur ;
- ce que vous avez fait, ce que vous attendiez, et ce qui s’est passé — le message exact aide le plus ;
- tout élément pertinent du [fichier journal](/docs/reference/security/#journaux).

Merci de signaler les problèmes de sécurité en privé, via le [signalement de vulnérabilités de GitHub](https://github.com/DamienReichhart/Homeport/security/advisories/new).

## Proposer une fonctionnalité

Ouvrez une issue qui décrit le problème que vous voulez résoudre, pas seulement la solution. Savoir comment vous travaillez avec vos moteurs aide à trouver la bonne forme.

## Contribuer au code

1. Lisez [`docs/ARCHITECTURE.md`](https://github.com/DamienReichhart/Homeport/blob/main/docs/ARCHITECTURE.md). Les fonctionnalités sont des tranches verticales autonomes, le frontend n’appelle Rust que via les liaisons IPC générées, et la logique Rust vit dans des services testés unitairement sans Tauri.
2. Employez le vocabulaire du projet tiré de [`GLOSSARY.md`](https://github.com/DamienReichhart/Homeport/blob/main/GLOSSARY.md) — Host, Stack, Managed, External, Adopt — dans le code, les textes de l’interface et les commits. Les principales décisions sont consignées dans [`docs/adr`](https://github.com/DamienReichhart/Homeport/tree/main/docs/adr).
3. Suivez la [charte graphique](https://github.com/DamienReichhart/Homeport/blob/main/docs/BRAND.md) pour les couleurs et le ton : simple, précis, posé, en orthographe britannique.
4. Assurez-vous que ces commandes passent :

   ```sh
   bun run check
   bun run rust:lint
   bun run rust:test
   ```

5. Rédigez les messages de commit selon les [Conventional Commits](https://www.conventionalcommits.org) — `feat(stacks): …`, `fix(hosts): …`. Les notes de version en sont générées.
6. Ouvrez une pull request vers la branche `dev`. La CI exécute les vérifications du frontend et la vérification des licences sous Linux, et `cargo clippy` et `cargo test` sous Linux, macOS et Windows, donc le code derrière `#[cfg(windows)]` ou `#[cfg(target_os = "macos")]` est compilé avant d’être fusionné. Un test qui a besoin d’un Docker Engine est marqué `#[ignore = "needs a local Docker Engine"]` ; un job Linux distinct les exécute sur le moteur du runner.

Voir [compiler depuis les sources](/docs/project/building/) pour vous installer.

### Ajouter du texte à l’application

Le texte de l’application vit dans un catalogue, jamais en dur, pour pouvoir être traduit ([ADR 0011](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0011-formatjs-catalogues-and-keyed-errors.md)). Pour ajouter une chaîne :

1. Ajoutez une clé au fichier de la fonctionnalité dans [`src/shared/i18n/en/`](https://github.com/DamienReichhart/Homeport/tree/main/src/shared/i18n/en) — `images.ts` pour la fonctionnalité des images, `ui.ts` pour le kit d’interface, `common.ts` seulement pour les mots qui veulent dire la même chose partout (Cancel, Close, Retry). Les clés commencent par le nom du fichier et disent à quoi sert le texte : `images.remove.title`, `images.column.size`.
2. Écrivez la valeur comme un [message ICU](https://formatjs.github.io/docs/core-concepts/icu-syntax/) : `{name}` pour une valeur, `{count, plural, one {# image} other {# images}}` au lieu de construire les pluriels à la main, `<strong>…</strong>` pour une mise en valeur rendue avec `tRich`. Une clé par phrase ; ne collez pas de fragments ensemble, parce que l’ordre des mots change d’une langue à l’autre.
3. Utilisez-la avec `t('images.remove.title', { name })` depuis `@/shared/i18n` — dans les composants, les mutations et les notifications. Les clés sont typées, donc une faute de frappe fait échouer la vérification des types. Utilisez des clés littérales : quand le texte dépend d’une valeur, associez la valeur à des clés (`{ start: 'containers.done.start', stop: 'containers.done.stop' }`) plutôt que de construire `` `containers.done.${action}` ``.

`bun run lint` échoue quand un composant écrit du texte en dur : texte JSX, props que les gens lisent (`title`, `aria-label`, `placeholder`, `description`, `label`…), et le texte des notifications et des confirmations. Les chaînes qui se lisent comme du code plutôt que comme des mots — `tcp`, `nginx:latest`, `/dev/dri`, `max-size=10m` — et les noms propres comme Docker ou Podman passent. Pour garder volontairement un littéral, dites pourquoi sur la ligne précédente : `// eslint-disable-next-line homeport/no-literal-ui-string -- a shell command`. `bun run check` échoue aussi pour une clé utilisée dans le code mais absente du catalogue, une clé que rien n’utilise, et une clé construite à l’exécution.

Les messages d’erreur sont écrits en Rust : chaque crate déclare ses clés et ses modèles anglais dans son bloc `messages!`, et `bun run bindings` les écrit dans `src/shared/i18n/en/errors.json`. Ne modifiez pas ce fichier à la main.

## Traduire Homeport

L’application et le site web parlent anglais, français, espagnol, allemand, italien, portugais du Brésil, russe, chinois simplifié, japonais, coréen, hindi et arabe. Chaque langue emploie un seul mot par terme du produit : cherchez-les dans [`docs/i18n/`](https://github.com/DamienReichhart/Homeport/tree/main/docs/i18n) avant de traduire, et ajoutez-y une ligne quand vous en choisissez un nouveau.

### Ajouter ou modifier une chaîne

Quelle que soit la chaîne, `bun run check` (et donc la CI) échoue tant que chaque langue ne l’a pas, avec les mêmes `{arguments}` et balises, et avec chaque forme plurielle dont cette langue a besoin.

- **Texte de l’application** : ajoutez la clé à `src/shared/i18n/en/<namespace>.ts` (voir [Ajouter du texte à l’application](#ajouter-du-texte-à-lapplication)), puis au fichier du même nom dans le dossier de chaque autre langue (`fr/`, `de/`, `zh-CN/`…). Traduisez-la si vous le pouvez. Sinon, copiez-y le texte anglais et dites-le dans la pull request, pour qu’un traducteur prenne le relais ; n’omettez jamais la clé.
- **Un message d’erreur ou une indication** : déclarez-le dans le bloc `messages!` de la crate et exécutez `bun run bindings`, qui écrit `en/errors.json` ; ajoutez ensuite la clé au `errors.json` de chaque langue. Le texte qu’affiche le système d’exploitation — le menu de la zone de notification et les notifications — fonctionne de la même façon, avec une clé `native.` et `native.json`.
- **Texte du site web** (accueil, téléchargement, journal des modifications, en-tête et pied de page) : ajoutez la clé au bon fichier de `website/src/core/i18n/strings/en/` (`site`, `home`, `download` ou `changelog`) et au même fichier du dossier de chaque autre langue ; `astro check` échoue quand l’un d’eux ne l’a pas. Un libellé de la barre latérale de la documentation va dans `website/src/app/config/sidebar/<code>.ts`.
- **Une page de documentation** : la page anglaise se trouve dans `website/src/content/docs/docs/`. Quand vous la modifiez, mettez à jour la même page dans chaque langue, au même chemin sous `website/src/content/docs/<code>/docs/`, dans la même pull request ; une langue dont vous ne pouvez pas mettre à jour la page affiche son ancienne traduction jusqu’à ce qu’un traducteur le fasse. Gardez les titres des pages traduites dans le même ordre et aux mêmes niveaux qu’en anglais, et les liens exactement tels que l’anglais les écrit, ancres comprises : après `bun run build`, `bun scripts/localize-anchors.ts <code>` fait pointer chaque ancre vers le titre traduit, et la vérification des liens d’un second build le confirme.

Vérifiez une langue avec `bun run i18n:check --locale fr`.

### Ajouter une langue

Des fichiers, plus une ligne pour le site web :

1. **Application** : copiez `src/shared/i18n/en/` vers `src/shared/i18n/<tag>/` (une étiquette BCP 47 comme `pl` ou `zh-TW`), écrivez son `meta.ts` (`{ name: 'Polski', dir: 'ltr' }`, le nom que la langue se donne elle-même ; `rtl` pour une écriture de droite à gauche) et traduisez chaque fichier, en gardant chaque clé et en transformant chaque `export default { … }` en `export default { … } satisfies Translation<typeof en>` comme le font les autres langues. `bun run i18n:check --locale <tag>` liste ce qui manque. Les paramètres listent la nouvelle langue, et la langue du système d’exploitation la sélectionne, sans autre changement.
2. **Site web** : ajoutez la langue à `website/src/core/site/locales.ts`, puis ses chaînes dans une copie de `website/src/core/i18n/strings/en/` nommée `<code>/` (enregistrée dans `TABLES` dans `core/i18n/index.ts`, la seule ligne de code dont une langue a besoin, puisque les tests de Bun ne peuvent pas découvrir les dossiers), ses libellés de barre latérale dans `website/src/app/config/sidebar/<code>.ts`, et la note de Starlight pour les pages non traduites dans `website/src/content/i18n/<tag>.json`. Ses pages, URL, entrées du sélecteur de langue, liens `hreflang` et alternatives du plan du site découlent de la liste. Sa documentation va sous `website/src/content/docs/<code>/docs/` : traduisez-y chaque page (une page manquante affiche l’anglais avec une note), puis lancez `bun scripts/localize-anchors.ts <code>` après un build.
3. **Captures d’écran** : `bun run screenshots --locale <tag>` capture chaque image dans la nouvelle langue dans `website/src/assets/screenshots/<tag>/` ; les pages qui n’en ont pas affichent la capture anglaise.
4. **Termes** : ajoutez `docs/i18n/terms-<tag>.md`.

Si un test ne voit pas une langue que vous venez d’ajouter, supprimez `node_modules/.vitest-cache` : Vitest peut garder une ancienne liste des fichiers de catalogue.

## Améliorer cette documentation

Chaque page de la documentation a un lien **Modifier cette page** en bas. Les pages sont des fichiers Markdown dans [`website/src/content/docs`](https://github.com/DamienReichhart/Homeport/tree/main/website/src/content/docs).

## Comment se font les releases

Chaque push sur `main` avec un nouveau numéro de version construit les programmes d’installation pour chaque plateforme, exécute toute la suite de tests, et publie une release GitHub avec les sommes de contrôle et la provenance de build. Le processus est décrit dans [`docs/RELEASING.md`](https://github.com/DamienReichhart/Homeport/blob/main/docs/RELEASING.md).
