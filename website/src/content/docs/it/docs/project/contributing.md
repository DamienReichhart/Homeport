---
title: Contribuire
description: Come segnalare problemi, suggerire funzioni e contribuire con codice a Homeport.
---

Homeport è sviluppato apertamente su [github.com/DamienReichhart/Homeport](https://github.com/DamienReichhart/Homeport). I contributi di qualsiasi dimensione sono benvenuti.

## Segnalare un bug

[Apri una issue](https://github.com/DamienReichhart/Homeport/issues/new) con:

- la tua versione di Homeport e la piattaforma, da **Impostazioni** (per esempio `v0.2.0 · macos/aarch64`);
- il tipo di host (locale, SSH, TLS o TCP) e la versione del suo Engine;
- cosa hai fatto, cosa ti aspettavi e cosa è successo — il messaggio esatto è ciò che aiuta di più;
- qualsiasi elemento pertinente del [file di log](/docs/reference/security/#log).

Segnala i problemi di sicurezza in privato, tramite la [segnalazione delle vulnerabilità di GitHub](https://github.com/DamienReichhart/Homeport/security/advisories/new).

## Suggerire una funzione

Apri una issue che descriva il problema che vuoi risolvere, non solo la soluzione. Sapere come lavori con i tuoi Engine aiuta a trovarle la forma giusta.

## Contribuire con codice

1. Leggi [`docs/ARCHITECTURE.md`](https://github.com/DamienReichhart/Homeport/blob/main/docs/ARCHITECTURE.md). Le funzioni sono sezioni verticali autonome, il frontend chiama Rust solo tramite i binding IPC generati, e la logica Rust vive in servizi testati con test unitari senza Tauri.
2. Usa il vocabolario del progetto da [`GLOSSARY.md`](https://github.com/DamienReichhart/Homeport/blob/main/GLOSSARY.md) — Host, Stack, Managed, External, Adopt — nel codice, nel testo dell’interfaccia e nei commit. Le decisioni principali sono registrate in [`docs/adr`](https://github.com/DamienReichhart/Homeport/tree/main/docs/adr).
3. Segui la [guida del marchio](https://github.com/DamienReichhart/Homeport/blob/main/docs/BRAND.md) per i colori e il tono: semplice, specifico, calmo, con l’ortografia britannica.
4. Assicurati che questi passino:

   ```sh
   bun run check
   bun run rust:lint
   bun run rust:test
   ```

5. Scrivi i messaggi di commit secondo i [Conventional Commits](https://www.conventionalcommits.org) — `feat(stacks): …`, `fix(hosts): …`. Le note di rilascio vengono generate da essi.
6. Apri una pull request sul branch `dev`. La CI esegue i controlli del frontend e il controllo delle licenze su Linux, e `cargo clippy` e `cargo test` su Linux, macOS e Windows, così il codice dietro `#[cfg(windows)]` o `#[cfg(target_os = "macos")]` viene compilato prima di essere unito. Un test che richiede un Docker Engine è marcato `#[ignore = "needs a local Docker Engine"]`; un job Linux separato li esegue sull’Engine del runner.

Vedi [compilare dal codice sorgente](/docs/project/building/) per prepararti.

### Aggiungere testo all’app

Il testo dell’app vive in un catalogo, mai inline, così può essere tradotto ([ADR 0011](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0011-formatjs-catalogues-and-keyed-errors.md)). Per aggiungere una stringa:

1. Aggiungi una chiave al file della funzione in [`src/shared/i18n/en/`](https://github.com/DamienReichhart/Homeport/tree/main/src/shared/i18n/en) — `images.ts` per la funzione delle immagini, `ui.ts` per il kit dell’interfaccia, `common.ts` solo per le parole che significano la stessa cosa ovunque (Cancel, Close, Retry). Le chiavi iniziano con il nome del file e dicono a cosa serve il testo: `images.remove.title`, `images.column.size`.
2. Scrivi il valore come [messaggio ICU](https://formatjs.github.io/docs/core-concepts/icu-syntax/): `{name}` per un valore, `{count, plural, one {# image} other {# images}}` invece di costruire i plurali a mano, `<strong>…</strong>` per l’enfasi resa con `tRich`. Una chiave per frase; non incollare frammenti tra loro, perché l’ordine delle parole cambia tra le lingue.
3. Usala con `t('images.remove.title', { name })` da `@/shared/i18n` — nei componenti, nelle mutazioni e nei toast allo stesso modo. Le chiavi sono tipizzate, quindi un errore di battitura fa fallire il controllo dei tipi. Usa chiavi letterali: quando il testo dipende da un valore, associa il valore alle chiavi (`{ start: 'containers.done.start', stop: 'containers.done.stop' }`) invece di costruire `` `containers.done.${action}` ``.

`bun run lint` fallisce quando un componente scrive testo inline: testo JSX, prop lette dalle persone (`title`, `aria-label`, `placeholder`, `description`, `label`…), e il testo di toast e conferme. Le stringhe che si leggono come codice anziché come parole — `tcp`, `nginx:latest`, `/dev/dri`, `max-size=10m` — e i nomi propri come Docker o Podman passano. Per conservare di proposito un letterale, spiega il perché nella riga precedente: `// eslint-disable-next-line homeport/no-literal-ui-string -- a shell command`. `bun run check` fallisce anche per una chiave usata nel codice ma assente dal catalogo, una chiave che nulla usa, e una chiave costruita a runtime.

I messaggi di errore sono scritti in Rust: ogni crate dichiara le sue chiavi e i modelli in inglese nel suo blocco `messages!`, e `bun run bindings` li scrive in `src/shared/i18n/en/errors.json`. Non modificare quel file a mano.

## Tradurre Homeport

L’app e il sito parlano inglese, francese, spagnolo, tedesco, italiano, portoghese brasiliano, russo, cinese semplificato, giapponese, coreano, hindi e arabo. Ogni lingua usa una parola per ogni termine del prodotto: cercali in [`docs/i18n/`](https://github.com/DamienReichhart/Homeport/tree/main/docs/i18n) prima di tradurre, e aggiungi lì una riga quando ne scegli uno nuovo.

### Aggiungere o modificare una stringa

Qualunque sia la stringa, `bun run check` (e quindi la CI) fallisce finché ogni lingua non la contiene, con gli stessi `{arguments}` e tag, e con ogni forma plurale di cui quella lingua ha bisogno.

- **Testo dell’app**: aggiungi la chiave a `src/shared/i18n/en/<namespace>.ts` (vedi [Aggiungere testo all’app](#aggiungere-testo-allapp)), poi al file con lo stesso nome nella cartella di ogni altra lingua (`fr/`, `de/`, `zh-CN/`…). Traducila se puoi. Se non puoi, copia lì il testo inglese e segnalalo nella pull request, così un traduttore potrà occuparsene; non omettere mai la chiave.
- **Un messaggio di errore o un suggerimento**: dichiaralo nel blocco `messages!` del crate ed esegui `bun run bindings`, che scrive `en/errors.json`; poi aggiungi la chiave al file `errors.json` di ogni lingua. Il testo mostrato dal sistema operativo — il menu dell’area di notifica e le notifiche — funziona allo stesso modo con una chiave `native.` e `native.json`.
- **Testo del sito** (home, download, changelog, intestazione e piè di pagina): aggiungi la chiave al file giusto di `website/src/core/i18n/strings/en/` (`site`, `home`, `download` o `changelog`) e allo stesso file nella cartella di ogni altra lingua; `astro check` fallisce quando ne manca una. Un’etichetta della barra laterale della documentazione va in `website/src/app/config/sidebar/<code>.ts`.
- **Una pagina della documentazione**: la pagina inglese è in `website/src/content/docs/docs/`. Quando la modifichi, aggiorna la stessa pagina in ogni lingua, allo stesso percorso sotto `website/src/content/docs/<code>/docs/`, nella stessa pull request; una lingua di cui non puoi aggiornare la pagina mostra la sua traduzione precedente finché un traduttore non lo fa. Mantieni i titoli delle pagine tradotte nello stesso ordine e agli stessi livelli dell’inglese, e i link esattamente come li scrive l’inglese, ancore comprese: dopo `bun run build`, `bun scripts/localize-anchors.ts <code>` fa puntare ogni ancora al titolo tradotto, e il controllo dei link di una seconda build lo conferma.

Controlla una lingua con `bun run i18n:check --locale fr`.

### Aggiungere una lingua

File, più una riga per il sito:

1. **App**: copia `src/shared/i18n/en/` in `src/shared/i18n/<tag>/` (un tag BCP 47 come `pl` o `zh-TW`), scrivi il suo `meta.ts` (`{ name: 'Polski', dir: 'ltr' }`, il nome della lingua nella lingua stessa; `rtl` per una scrittura da destra a sinistra) e traduci ogni file, mantenendo ogni chiave e trasformando ogni `export default { … }` in `export default { … } satisfies Translation<typeof en>` come fanno le altre lingue. `bun run i18n:check --locale <tag>` elenca ciò che manca. Impostazioni elenca la nuova lingua, e la lingua del sistema operativo la sceglie, senza nessun’altra modifica.
2. **Sito**: aggiungi la lingua a `website/src/core/site/locales.ts`, poi le sue stringhe in una copia di `website/src/core/i18n/strings/en/` chiamata `<code>/` (registrata in `TABLES` in `core/i18n/index.ts`), le etichette della sua barra laterale in `website/src/app/config/sidebar/<code>.ts` (registrate in `sidebar.ts`; queste due righe sono l’unico codice di cui una lingua ha bisogno, perché quei file vengono caricati fuori da Vite, che è ciò che trova le cartelle), e la nota di Starlight per le pagine non tradotte in `website/src/content/i18n/<tag>.json`. Le sue pagine, gli URL, le voci del selettore di lingua, i link `hreflang` e le alternative della sitemap derivano dall’elenco. La sua documentazione va sotto `website/src/content/docs/<code>/docs/`: traduci lì ogni pagina (una mancante mostra l’inglese con una nota), poi esegui `bun scripts/localize-anchors.ts <code>` dopo una build.
3. **Screenshot**: `bun run screenshots` cattura ogni screenshot in ogni lingua, compresa la nuova, in `website/src/assets/screenshots/<tag>/`, così ogni pagina mostra l’app nella lingua di chi la legge. Il `bun test` del sito fallisce finché a una lingua manca uno screenshot che l’inglese ha.
4. **Termini**: aggiungi `docs/i18n/terms-<tag>.md`.

Se un test non vede una lingua che hai appena aggiunto, elimina `node_modules/.vitest-cache`: Vitest può conservare un vecchio elenco dei file del catalogo.

## Migliorare questa documentazione

Ogni pagina della documentazione ha in fondo un link **Modifica pagina**. Le pagine sono file Markdown in [`website/src/content/docs`](https://github.com/DamienReichhart/Homeport/tree/main/website/src/content/docs).

## Come avvengono le release

Ogni push su `main` con un nuovo numero di versione compila i programmi di installazione per ogni piattaforma, esegue l’intera suite di test e pubblica una release GitHub con checksum e provenienza della build. Il processo è descritto in [`docs/RELEASING.md`](https://github.com/DamienReichhart/Homeport/blob/main/docs/RELEASING.md).
