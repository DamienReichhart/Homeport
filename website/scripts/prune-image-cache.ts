/**
 * Keeps Astro's image cache to what the last build used.
 *
 *   bun run build && bun scripts/prune-image-cache.ts
 *
 * Astro converts each screenshot to AVIF and WebP at several widths, and keeps
 * every result in `node_modules/.astro/assets/` to reuse it next time. A
 * result's name holds a hash of the source image's bytes
 * (`containers-dark.wZgUhDCz_1V0pIQ.avif`), so a screenshot whose content
 * changes gets a new name and is converted again, while the old result stays
 * behind. CI keeps this cache between runs; without pruning it would grow with
 * every recapture. This deletes every cached file the build in `dist/` didn't
 * use (and its `.json` metadata, which remote images have).
 */
import { readdirSync, rmSync, statSync } from 'node:fs';
import { join } from 'node:path';

const root = join(import.meta.dir, '..');

/** Every file name under `dir`, at any depth. */
export function fileNames(dir: string, out = new Set<string>()): Set<string> {
  for (const entry of readdirSync(dir)) {
    const path = join(dir, entry);
    if (statSync(path).isDirectory()) fileNames(path, out);
    else out.add(entry);
  }
  return out;
}

/** The cache entries the build didn't use: a result and its `.json` share a fate. */
export function unused(cached: string[], built: Set<string>): string[] {
  return cached.filter((name) => !built.has(name.replace(/\.json$/, '')));
}

if (import.meta.main) {
  const cache = join(root, 'node_modules', '.astro', 'assets');
  const dist = join(root, 'dist');
  let cached: string[] = [];
  try {
    cached = readdirSync(cache);
  } catch {
    console.log('No image cache to prune.');
    process.exit(0);
  }
  const stale = unused(cached, fileNames(dist));
  for (const name of stale) rmSync(join(cache, name), { force: true });
  console.log(`Image cache: kept ${cached.length - stale.length}, removed ${stale.length}.`);
}
