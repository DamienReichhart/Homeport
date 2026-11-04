/**
 * Writes the winget manifests, the Scoop manifest and the AUR package for a
 * published release, for `.github/workflows/release.yml`:
 *
 *   bun scripts/channels.ts --version <x.y.z> --sums <SHA256SUMS.txt> --out <dir>
 *
 * creates `<dir>/winget/*.yaml`, `<dir>/scoop/homeport.json`,
 * `<dir>/aur/PKGBUILD` and `<dir>/aur/.SRCINFO` (see `channels-plan.ts`).
 */
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { parseArgs } from 'node:util';
import {
  aurPackage,
  ChannelError,
  parseSums,
  scoopManifest,
  WINGET_ID,
  wingetManifests,
} from './channels-plan';

function main() {
  const { values } = parseArgs({
    options: {
      version: { type: 'string' },
      sums: { type: 'string' },
      out: { type: 'string' },
    },
  });
  if (!values.version || !values.sums || !values.out) {
    console.error('usage: bun scripts/channels.ts --version <x.y.z> --sums <file> --out <dir>');
    process.exit(2);
  }
  const sums = parseSums(readFileSync(values.sums, 'utf8'));
  try {
    const winget = join(values.out, 'winget');
    mkdirSync(winget, { recursive: true });
    const w = wingetManifests(values.version, sums);
    writeFileSync(join(winget, `${WINGET_ID}.yaml`), w.version);
    writeFileSync(join(winget, `${WINGET_ID}.installer.yaml`), w.installer);
    writeFileSync(join(winget, `${WINGET_ID}.locale.en-US.yaml`), w.locale);
    const scoop = join(values.out, 'scoop');
    const aur = join(values.out, 'aur');
    mkdirSync(scoop, { recursive: true });
    mkdirSync(aur, { recursive: true });
    writeFileSync(join(scoop, 'homeport.json'), scoopManifest(values.version, sums));
    const pkg = aurPackage(values.version, sums);
    writeFileSync(join(aur, 'PKGBUILD'), pkg.pkgbuild);
    writeFileSync(join(aur, '.SRCINFO'), pkg.srcinfo);
    console.log(
      `Wrote the winget manifests, the Scoop manifest and the AUR package for ${values.version} to ${values.out}`,
    );
  } catch (e) {
    if (e instanceof ChannelError) {
      console.error(`::error::${e.message}`);
      process.exit(1);
    }
    throw e;
  }
}

if (import.meta.main) main();
