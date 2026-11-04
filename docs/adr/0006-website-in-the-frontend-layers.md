# The website uses the frontend's layers

The website (`website/`) grew as an Astro project laid out by file type: a flat `components/` mixing the site's header with download buttons and docs-only figures, a `lib/` holding a Markdown plugin, browser code, the GitHub releases client and the package repositories side by side, one 500-line global stylesheet, and pages of 650 to 750 lines that held their data, every section, a script and their styles. Changing the download page meant touching five folders, and nothing kept a page from growing.

The website now has the layers the app's frontend has (ADR 0004), under `website/src/`: `app` → `features` → `core` → `shared`, and a module only imports from the layers to its right. `app` is the shell: the layout, the header and footer, the Starlight overrides, the docs sidebar, the Markdown plugins and the global styles. `features` are the site's pages and the pieces only they use (`home`, `download`, `changelog`, `docs`), each with `ui/` for components (`sections/` for a page's sections), `model/` for logic, `data/` for what the page lists, and `index.ts` for what the rest of the site may use. `core` is what several features share about Homeport: the facts about the project (`site`), its releases on GitHub (`release`) and its Linux package repositories (`packages`). `shared` has no knowledge of Homeport: the icons, the screenshot frame and formatting helpers. Astro and Starlight fix `pages/` and `content/`, so routes stay in `pages/`, each a short file that puts a feature's sections in a layout, and the docs stay in `content/docs/`.

## Considered Options

- **Folders by type** (`components/{ui,layout,sections}`, `lib/{github,browser}`): the cheapest change and familiar to anyone who knows Astro, but a page's parts stay spread over several folders, and the site would follow a second set of rules beside the app's.
- **Content-driven pages** (page copy in content collections, rendered by generic blocks): scales to many marketing pages and several languages, but turns three hand-made pages into a page builder with blocks that take many options. Worth it only once the site has many similar pages or translations.

## Consequences

- A new page is a feature folder and a route in `pages/`; removing it is the reverse.
- Imports use `@/` for `website/src/`, so docs can import components without `../../../../`.
- Features import each other, and core modules are imported, only through their `index.ts`. Client scripts import a feature's `model/` files directly, since an `index.ts` that re-exports `.astro` components can't be bundled for the browser.
- ESLint (`website/eslint.config.js`) enforces the direction of imports and warns past 300 lines in a file, as it does for the app.
- The global stylesheet stays global and keeps its order (`app/styles/site/index.css` imports its parts). A section's own styles live in its component.
- Vite doesn't keep the global stylesheet ahead of a page's CSS: their order follows the module graph and changes when files move. A component rule that overrides a global one must outrank it (`.cta.card`, not `.cta` against `.card`) instead of relying on coming later. A cascade layer for the global styles was rejected: it would also let a component beat the global rules that are meant to win, such as the scroll reveal.
