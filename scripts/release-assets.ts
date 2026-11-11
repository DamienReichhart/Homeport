/**
 * Copies a build's installers and updater files under their release names
 * (see `release-assets-plan.ts`), for `.github/workflows/release.yml`:
 *
 *   bun scripts/release-assets.ts --version <x.y.z> --target <triple> \
 *     --bundles <deb,rpm,…> --from target/<triple>/release/bundle --out installers
 */
import { copyFileSync, mkdirSync, readdirSync } from 'node:fs';
import { join } from 'node:path';
import { parseArgs } from 'node:util';
import { AssetError, collectAssets } from './release-assets-plan';

function main() {
  const { values } = parseArgs({
    options: {
      version: { type: 'string' },
      target: { type: 'string' },
      bundles: { type: 'string' },
      from: { type: 'string' },
      out: { type: 'string' },
    },
  });
  const { version, target, bundles, from, out } = values;
  if (!version || !target || !bundles || !from || !out) {
    console.error(
      'usage: bun scripts/release-assets.ts --version <x.y.z> --target <triple> --bundles <a,b> --from <bundle dir> --out <dir>',
    );
    process.exit(2);
  }
  try {
    // Only the files directly in each bundle's folder: the .app and the AppDir are folders of their own.
    const files = readdirSync(from, { withFileTypes: true })
      .filter((dir) => dir.isDirectory())
      .flatMap((dir) =>
        readdirSync(join(from, dir.name), { withFileTypes: true })
          .filter((file) => file.isFile())
          .map((file) => `${dir.name}/${file.name}`),
      );
    const copies = collectAssets({
      version,
      target,
      bundles: bundles.split(',').filter(Boolean),
      files,
    });
    mkdirSync(out, { recursive: true });
    for (const copy of copies) {
      copyFileSync(join(from, copy.from), join(out, copy.to));
      console.log(`${copy.from} -> ${copy.to}`);
    }
  } catch (e) {
    if (!(e instanceof AssetError)) throw e;
    console.error(process.env.GITHUB_ACTIONS ? `::error::${e.message}` : e.message);
    process.exit(1);
  }
}

if (import.meta.main) main();
