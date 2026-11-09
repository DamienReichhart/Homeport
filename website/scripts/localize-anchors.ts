/**
 * Points a translated docs page's `#anchor` links at the translated headings.
 *
 *   bun run build && bun scripts/localize-anchors.ts <code>…
 *
 * Translators keep links exactly as English writes them (`/docs/hosts/ssh/#bastions`),
 * because a heading's id comes from its translated text, which another
 * translator may not have written yet. Translated pages keep English's headings
 * in the same order, so the n-th heading of a page has the n-th id in every
 * language: this reads both from the built site (`dist/`) and rewrites each
 * English anchor in `src/content/docs/<code>/docs/` to its translated id.
 * Run `bun run build` again afterwards; the link validator checks the result.
 */
import { existsSync, readdirSync, readFileSync, statSync, writeFileSync } from 'node:fs';
import { join, relative } from 'node:path';

const root = join(import.meta.dir, '..');
const dist = join(root, 'dist');
const content = join(root, 'src', 'content', 'docs');

/** A built page's heading ids, in order, without Starlight's own. */
export function headingIds(html: string): string[] {
  return [...html.matchAll(/<h[2-6][^>]*\sid="([^"]+)"/g)]
    .map((m) => m[1] ?? '')
    .filter((id) => !id.startsWith('starlight__') && id !== '_top');
}

/** English id → translated id, for one page; empty when the headings don't line up. */
export function anchorMap(english: string[], translated: string[]): Map<string, string> {
  if (english.length !== translated.length) return new Map();
  return new Map(english.map((id, i) => [id, translated[i] ?? id]));
}

/**
 * `text` with every link to `/docs/<page>/#<english id>` (and same-page
 * `#<english id>`, for `page`) pointing at the translated id instead.
 */
export function rewrite(
  text: string,
  page: string,
  maps: Map<string, Map<string, string>>,
): string {
  const swap = (target: string, anchor: string) =>
    maps.get(target)?.get(decodeURIComponent(anchor)) ?? anchor;
  return text
    .replace(
      /(\]\(|href=")(\/docs\/[^)"#\s]*)#([^)"\s]+)/g,
      (_, open: string, path: string, anchor: string) =>
        `${open}${path}#${swap(path.replace(/^\/|\/$/g, ''), anchor)}`,
    )
    .replace(
      /(\]\(|href=")#([^)"\s]+)/g,
      (_, open: string, anchor: string) => `${open}#${swap(page, anchor)}`,
    );
}

/** Every page of the docs, as `docs/hosts/ssh`, from the English sources. */
function pages(dir = join(content, 'docs'), out: string[] = []): string[] {
  for (const entry of readdirSync(dir)) {
    const path = join(dir, entry);
    if (statSync(path).isDirectory()) pages(path, out);
    else if (/\.mdx?$/.test(entry))
      out.push(
        relative(content, path)
          .replace(/\.mdx?$/, '')
          .replace(/\/index$/, ''),
      );
  }
  return out;
}

const read = (file: string) => (existsSync(file) ? readFileSync(file, 'utf8') : '');

if (import.meta.main) {
  const codes = process.argv.slice(2);
  if (codes.length === 0) throw new Error('usage: bun scripts/localize-anchors.ts <code>…');
  for (const code of codes) {
    const maps = new Map<string, Map<string, string>>();
    for (const page of pages()) {
      const english = headingIds(read(join(dist, page, 'index.html')));
      const translated = headingIds(read(join(dist, code, page, 'index.html')));
      const map = anchorMap(english, translated);
      if (english.length && !map.size)
        console.warn(
          `${code}/${page}: ${translated.length} headings, English has ${english.length}`,
        );
      maps.set(page, map);
    }
    let changed = 0;
    for (const page of pages()) {
      for (const ext of ['.md', '.mdx', '/index.md', '/index.mdx']) {
        const file = join(content, code, `${page}${ext}`);
        if (!existsSync(file)) continue;
        const before = readFileSync(file, 'utf8');
        const after = rewrite(before, page, maps);
        if (after !== before) {
          writeFileSync(file, after);
          changed++;
        }
      }
    }
    console.log(`${code}: anchors updated in ${changed} page(s)`);
  }
}
