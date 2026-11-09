---
title: Mitwirken
description: Wie Sie Probleme melden, Funktionen vorschlagen und Code zu Homeport beitragen.
---

Homeport wird öffentlich unter [github.com/DamienReichhart/Homeport](https://github.com/DamienReichhart/Homeport) entwickelt. Beiträge jeder Größe sind willkommen.

## Einen Fehler melden

[Eröffnen Sie ein Issue](https://github.com/DamienReichhart/Homeport/issues/new) mit:

- Ihrer Homeport-Version und Plattform aus den **Einstellungen** (zum Beispiel `v0.2.0 · macos/aarch64`);
- der Art des Hosts (lokal, SSH, TLS oder TCP) und der Version seiner Engine;
- dem, was Sie getan haben, was Sie erwartet haben und was passiert ist – die genaue Meldung hilft am meisten;
- allem Relevanten aus der [Logdatei](/docs/reference/security/#logs).

Bitte melden Sie Sicherheitsprobleme vertraulich über die [Meldung von Sicherheitslücken auf GitHub](https://github.com/DamienReichhart/Homeport/security/advisories/new).

## Eine Funktion vorschlagen

Eröffnen Sie ein Issue, das das Problem beschreibt, das Sie gelöst haben möchten, nicht nur die Lösung. Zu wissen, wie Sie mit Ihren Engines arbeiten, hilft, die richtige Form dafür zu finden.

## Code beitragen

1. Lesen Sie [`docs/ARCHITECTURE.md`](https://github.com/DamienReichhart/Homeport/blob/main/docs/ARCHITECTURE.md). Funktionen sind in sich geschlossene vertikale Schnitte, das Frontend ruft Rust nur über die erzeugten IPC-Bindings auf, und die Rust-Logik liegt in Services, die ohne Tauri per Unit-Test geprüft werden.
2. Verwenden Sie das Vokabular des Projekts aus [`GLOSSARY.md`](https://github.com/DamienReichhart/Homeport/blob/main/GLOSSARY.md) – Host, Stack, Managed, External, Adopt – in Code, UI-Texten und Commits. Die wichtigsten Entscheidungen sind in [`docs/adr`](https://github.com/DamienReichhart/Homeport/tree/main/docs/adr) festgehalten.
3. Folgen Sie dem [Markenleitfaden](https://github.com/DamienReichhart/Homeport/blob/main/docs/BRAND.md) für Farben und Tonfall: schlicht, konkret, ruhig, in britischer Schreibweise.
4. Stellen Sie sicher, dass diese erfolgreich durchlaufen:

   ```sh
   bun run check
   bun run rust:lint
   bun run rust:test
   ```

5. Schreiben Sie Commit-Nachrichten als [Conventional Commits](https://www.conventionalcommits.org) – `feat(stacks): …`, `fix(hosts): …`. Die Release Notes werden daraus erzeugt.
6. Eröffnen Sie einen Pull Request gegen den Branch `dev`. Die CI führt die Frontend-Prüfungen und die Lizenzprüfung unter Linux aus sowie `cargo clippy` und `cargo test` unter Linux, macOS und Windows, sodass Code hinter `#[cfg(windows)]` oder `#[cfg(target_os = "macos")]` kompiliert wird, bevor er gemergt wird. Ein Test, der eine Docker Engine braucht, ist mit `#[ignore = "needs a local Docker Engine"]` markiert; ein separater Linux-Job führt diese gegen die Engine des Runners aus.

Siehe [Aus dem Quellcode bauen](/docs/project/building/), um alles einzurichten.

### Text zur App hinzufügen

Die Texte der App liegen in einem Katalog, nie inline, damit sie übersetzt werden können ([ADR 0011](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0011-formatjs-catalogues-and-keyed-errors.md)). Um einen String hinzuzufügen:

1. Fügen Sie einen Schlüssel zur Datei der Funktion in [`src/shared/i18n/en/`](https://github.com/DamienReichhart/Homeport/tree/main/src/shared/i18n/en) hinzu – `images.ts` für die Images-Funktion, `ui.ts` für das UI-Kit, `common.ts` nur für Wörter, die überall dasselbe bedeuten (Cancel, Close, Retry). Schlüssel beginnen mit dem Namen der Datei und sagen, wofür der Text ist: `images.remove.title`, `images.column.size`.
2. Schreiben Sie den Wert als [ICU-Nachricht](https://formatjs.github.io/docs/core-concepts/icu-syntax/): `{name}` für einen Wert, `{count, plural, one {# image} other {# images}}`, statt Plurale von Hand zu bauen, `<strong>…</strong>` für Hervorhebungen, die mit `tRich` gerendert werden. Ein Schlüssel pro Satz; kleben Sie keine Bruchstücke zusammen, denn die Wortstellung ändert sich zwischen Sprachen.
3. Verwenden Sie ihn mit `t('images.remove.title', { name })` aus `@/shared/i18n` – in Komponenten, Mutationen und Toasts gleichermaßen. Schlüssel sind typisiert, daher lässt ein Tippfehler die Typprüfung fehlschlagen. Verwenden Sie literale Schlüssel: Wenn der Text von einem Wert abhängt, ordnen Sie den Wert Schlüsseln zu (`{ start: 'containers.done.start', stop: 'containers.done.stop' }`), statt `` `containers.done.${action}` `` zu bauen.

`bun run lint` schlägt fehl, wenn eine Komponente Text inline schreibt: JSX-Text, Props, die Menschen lesen (`title`, `aria-label`, `placeholder`, `description`, `label`…), und den Text von Toasts und Bestätigungen. Strings, die sich wie Code statt wie Wörter lesen – `tcp`, `nginx:latest`, `/dev/dri`, `max-size=10m` – und Eigennamen wie Docker oder Podman gehen durch. Um ein Literal absichtlich zu behalten, sagen Sie in der Zeile davor, warum: `// eslint-disable-next-line homeport/no-literal-ui-string -- a shell command`. `bun run check` schlägt außerdem bei einem Schlüssel fehl, der im Code verwendet wird, aber im Katalog fehlt, bei einem Schlüssel, den nichts verwendet, und bei einem zur Laufzeit gebauten Schlüssel.

Fehlermeldungen werden in Rust geschrieben: Jede Crate deklariert ihre Schlüssel und englischen Vorlagen in ihrem `messages!`-Block, und `bun run bindings` schreibt sie nach `src/shared/i18n/en/errors.json`. Bearbeiten Sie diese Datei nicht von Hand.

## Homeport übersetzen

Die App und die Website sprechen Englisch, Französisch, Spanisch, Deutsch, Italienisch, brasilianisches Portugiesisch, Russisch, vereinfachtes Chinesisch, Japanisch, Koreanisch, Hindi und Arabisch. Jede Sprache verwendet ein Wort pro Produktbegriff: Schlagen Sie sie vor dem Übersetzen in [`docs/i18n/`](https://github.com/DamienReichhart/Homeport/tree/main/docs/i18n) nach, und fügen Sie dort eine Zeile hinzu, wenn Sie einen neuen wählen.

### Einen String hinzufügen oder ändern

Welcher String es auch ist, `bun run check` (und damit die CI) schlägt fehl, bis jede Sprache ihn hat, mit denselben `{arguments}` und Tags und mit jeder Pluralform, die diese Sprache braucht.

- **App-Text**: Fügen Sie den Schlüssel zu `src/shared/i18n/en/<namespace>.ts` hinzu (siehe [Text zur App hinzufügen](#text-zur-app-hinzufügen)) und dann zur gleichnamigen Datei in jedem anderen Sprachordner (`fr/`, `de/`, `zh-CN/`…). Übersetzen Sie ihn, wenn Sie können. Wenn nicht, kopieren Sie den englischen Text dorthin und sagen Sie es im Pull Request, damit ein Übersetzer nachziehen kann; lassen Sie den Schlüssel nie weg.
- **Eine Fehlermeldung oder ein Hinweis**: Deklarieren Sie sie im `messages!`-Block der Crate und führen Sie `bun run bindings` aus, das `en/errors.json` schreibt; fügen Sie den Schlüssel dann zur `errors.json` jeder Sprache hinzu. Text, den das Betriebssystem anzeigt – das Menü im Infobereich und Benachrichtigungen –, funktioniert genauso mit einem `native.`-Schlüssel und `native.json`.
- **Website-Text** (Startseite, Download, Änderungen, Kopf- und Fußzeile): Fügen Sie den Schlüssel zur richtigen Datei von `website/src/core/i18n/strings/en/` hinzu (`site`, `home`, `download` oder `changelog`) und zur selben Datei im Ordner jeder anderen Sprache; `astro check` schlägt fehl, wenn er in einer fehlt. Eine Beschriftung der Docs-Seitenleiste gehört in `website/src/app/config/sidebar/<code>.ts`.
- **Eine Docs-Seite**: Die englische Seite liegt in `website/src/content/docs/docs/`. Wenn Sie sie ändern, aktualisieren Sie im selben Pull Request dieselbe Seite in jeder Sprache unter demselben Pfad in `website/src/content/docs/<code>/docs/`; eine Sprache, deren Seite Sie nicht aktualisieren können, zeigt ihre ältere Übersetzung, bis jemand beim Übersetzen das nachholt. Halten Sie die Überschriften übersetzter Seiten in derselben Reihenfolge und auf denselben Ebenen wie im Englischen, und Links genau so, wie das Englische sie schreibt, Anker eingeschlossen: Nach `bun run build` richtet `bun scripts/localize-anchors.ts <code>` jeden Anker auf die übersetzte Überschrift, und die Linkprüfung eines zweiten Builds bestätigt das.

Prüfen Sie eine Sprache mit `bun run i18n:check --locale fr`.

### Eine Sprache hinzufügen

Dateien, plus eine Zeile für die Website:

1. **App**: Kopieren Sie `src/shared/i18n/en/` nach `src/shared/i18n/<tag>/` (ein BCP-47-Tag wie `pl` oder `zh-TW`), schreiben Sie seine `meta.ts` (`{ name: 'Polski', dir: 'ltr' }`, der eigene Name der Sprache für sich selbst; `rtl` für eine Schrift von rechts nach links) und übersetzen Sie jede Datei, wobei Sie jeden Schlüssel behalten und jedes `export default { … }` in `export default { … } satisfies Translation<typeof en>` umwandeln, wie es die anderen Sprachen tun. `bun run i18n:check --locale <tag>` listet auf, was fehlt. Die Einstellungen listen die neue Sprache auf, und die Sprache des Betriebssystems wählt sie aus, ohne weitere Änderung.
2. **Website**: Fügen Sie die Sprache zu `website/src/core/site/locales.ts` hinzu, dann ihre Strings in einer Kopie von `website/src/core/i18n/strings/en/` namens `<code>/` (registriert in `TABLES` in `core/i18n/index.ts`), ihre Seitenleisten-Beschriftungen in `website/src/app/config/sidebar/<code>.ts` (registriert in `sidebar.ts`; diese zwei Zeilen sind der einzige Code, den eine Sprache braucht, weil diese Dateien außerhalb von Vite geladen werden, das sonst die Ordner findet) und den Hinweis von Starlight für unübersetzte Seiten in `website/src/content/i18n/<tag>.json`. Ihre Seiten, URLs, Einträge im Sprachumschalter, `hreflang`-Links und Sitemap-Alternativen ergeben sich aus der Liste. Ihre Docs gehören unter `website/src/content/docs/<code>/docs/`: Übersetzen Sie dort jede Seite (eine fehlende zeigt Englisch mit einem Hinweis) und führen Sie dann nach einem Build `bun scripts/localize-anchors.ts <code>` aus.
3. **Screenshots**: `bun run screenshots` nimmt jeden Screenshot in jeder Sprache, die neue eingeschlossen, in `website/src/assets/screenshots/<tag>/` auf, sodass jede Seite die App in der Sprache ihrer Leser zeigt. `bun test` der Website schlägt fehl, solange einer Sprache ein Screenshot fehlt, den es auf Englisch gibt.
4. **Begriffe**: Fügen Sie `docs/i18n/terms-<tag>.md` hinzu.

Wenn ein Test eine gerade hinzugefügte Sprache nicht sieht, löschen Sie `node_modules/.vitest-cache`: Vitest kann eine alte Liste der Katalogdateien behalten.

## Diese Dokumentation verbessern

Jede Docs-Seite hat unten einen Link **Seite bearbeiten**. Die Seiten sind Markdown-Dateien in [`website/src/content/docs`](https://github.com/DamienReichhart/Homeport/tree/main/website/src/content/docs).

## Wie Releases entstehen

Jeder Push auf `main` mit einer neuen Versionsnummer baut Installationsprogramme für jede Plattform, führt die vollständige Testsuite aus und veröffentlicht ein GitHub-Release mit Prüfsummen und Build-Provenienz. Der Ablauf ist in [`docs/RELEASING.md`](https://github.com/DamienReichhart/Homeport/blob/main/docs/RELEASING.md) beschrieben.
