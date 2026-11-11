/**
 * The names of the files attached to a GitHub release:
 *
 *   Homeport-<version>-<os>-<arch>[-<variant>].<ext>
 *
 * with `<os>` one of windows, macos, linux and `<arch>` one of x64, arm64, so
 * every file says which system and processor it is for in the same words.
 * Tauri names its bundles in each format's own vocabulary (`amd64`, `aarch64`,
 * `x86_64`…), which releases up to 0.22 shipped as they were; `describeAsset`
 * reads both. Pure functions only; `release-assets.ts` copies the files.
 */

export type Os = 'windows' | 'macos' | 'linux';
export type Arch = 'x64' | 'arm64';
/** `app` is the zipped macOS app the in-app updater installs, not a download. */
export type Kind = 'nsis' | 'msi' | 'dmg' | 'deb' | 'rpm' | 'appimage' | 'app';

export interface AssetKind {
  os: Os;
  arch: Arch;
  kind: Kind;
}

export class AssetError extends Error {}

/** What follows `Homeport-<version>-<os>-<arch>` for each kind of file. */
const SUFFIX: Record<Kind, string> = {
  nsis: '-setup.exe',
  msi: '.msi',
  dmg: '.dmg',
  deb: '.deb',
  rpm: '.rpm',
  appimage: '.AppImage',
  app: '-update.app.tar.gz',
};

const OS_OF: Record<Kind, Os> = {
  nsis: 'windows',
  msi: 'windows',
  dmg: 'macos',
  app: 'macos',
  deb: 'linux',
  rpm: 'linux',
  appimage: 'linux',
};

/** The release name of a file, e.g. `Homeport-1.0.0-windows-x64-setup.exe`. */
export function assetName(version: string, { os, arch, kind }: AssetKind): string {
  return `Homeport-${version}-${os}-${arch}${SUFFIX[kind]}`;
}

const escape = (text: string) => text.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');

const NAMED = new RegExp(
  `^Homeport-.+-(windows|macos|linux)-(x64|arm64)(${Object.values(SUFFIX).map(escape).join('|')})$`,
);

/** Tauri's own names, which releases up to 0.22 carry. */
const TAURI: [RegExp, Arch, Kind][] = [
  [/_x64-setup\.exe$/, 'x64', 'nsis'],
  [/_arm64-setup\.exe$/, 'arm64', 'nsis'],
  [/_x64_[\w-]+\.msi$/, 'x64', 'msi'],
  [/_arm64_[\w-]+\.msi$/, 'arm64', 'msi'],
  [/_x64\.dmg$/, 'x64', 'dmg'],
  [/_aarch64\.dmg$/, 'arm64', 'dmg'],
  [/_x64\.app\.tar\.gz$/, 'x64', 'app'],
  [/_aarch64\.app\.tar\.gz$/, 'arm64', 'app'],
  [/_amd64\.deb$/, 'x64', 'deb'],
  [/_arm64\.deb$/, 'arm64', 'deb'],
  [/\.x86_64\.rpm$/, 'x64', 'rpm'],
  [/\.aarch64\.rpm$/, 'arm64', 'rpm'],
  [/_amd64\.AppImage$/, 'x64', 'appimage'],
  [/_aarch64\.AppImage$/, 'arm64', 'appimage'],
];

/**
 * The system, processor and kind of a release file, under either naming
 * scheme. `undefined` for anything else: signatures, checksums.
 */
export function describeAsset(name: string): AssetKind | undefined {
  const named = NAMED.exec(name);
  if (named) {
    const kind = (Object.keys(SUFFIX) as Kind[]).find((k) => SUFFIX[k] === named[3])!;
    const os = named[1] as Os;
    return os === OS_OF[kind] ? { os, arch: named[2] as Arch, kind } : undefined;
  }
  for (const [pattern, arch, kind] of TAURI) {
    if (pattern.test(name)) return { os: OS_OF[kind], arch, kind };
  }
  return undefined;
}

/** The system and processor a Rust target triple builds for. */
export function platformOf(target: string): { os: Os; arch: Arch } {
  const arch = target.startsWith('x86_64-')
    ? 'x64'
    : target.startsWith('aarch64-')
      ? 'arm64'
      : undefined;
  const os = target.includes('-windows-')
    ? 'windows'
    : target.endsWith('-apple-darwin')
      ? 'macos'
      : target.includes('-linux-')
        ? 'linux'
        : undefined;
  if (!arch || !os) throw new AssetError(`Unknown target ${target}`);
  return { os, arch };
}

/**
 * Where Tauri writes each bundle under `target/<triple>/release/bundle`, and
 * whether the in-app updater installs it, which takes its `.sig`.
 */
const BUNDLES: Record<string, { kind: Kind; dir: string; ends: string; updater: boolean }> = {
  deb: { kind: 'deb', dir: 'deb', ends: '.deb', updater: false },
  rpm: { kind: 'rpm', dir: 'rpm', ends: '.rpm', updater: false },
  appimage: { kind: 'appimage', dir: 'appimage', ends: '.AppImage', updater: true },
  msi: { kind: 'msi', dir: 'msi', ends: '.msi', updater: true },
  nsis: { kind: 'nsis', dir: 'nsis', ends: '-setup.exe', updater: true },
  dmg: { kind: 'dmg', dir: 'dmg', ends: '.dmg', updater: false },
  app: { kind: 'app', dir: 'macos', ends: '.app.tar.gz', updater: true },
};

export interface CollectInput {
  version: string;
  /** The Rust target triple that was built. */
  target: string;
  /** The bundles that were built, as given to `tauri build --bundles`. */
  bundles: readonly string[];
  /** Every file in the bundle directory's folders, relative to it, with `/` separators. */
  files: readonly string[];
}

export interface Copy {
  /** Relative to the bundle directory. */
  from: string;
  /** The file's name in the release. */
  to: string;
}

/**
 * The files a build attaches to the release, under their release names: one
 * per bundle, and its `.sig` when Tauri signed it. Fails when a bundle or an
 * updater signature is missing, when a bundle has several files, or when two
 * files would get the same name, rather than publish a release that lacks an
 * installer or can't update.
 */
export function collectAssets({ version, target, bundles, files }: CollectInput): Copy[] {
  const { os, arch } = platformOf(target);
  const present = new Set(files);
  const copies: Copy[] = [];
  for (const bundle of bundles) {
    const expected = BUNDLES[bundle];
    if (!expected) throw new AssetError(`Unknown bundle "${bundle}"`);
    const { kind, dir, ends, updater } = expected;
    if (OS_OF[kind] !== os) throw new AssetError(`${target} doesn't build ${bundle} bundles`);
    const to = assetName(version, { os, arch, kind });
    const found = files.filter((f) => f.startsWith(`${dir}/`) && f.endsWith(ends));
    if (!found.length) throw new AssetError(`No ${dir}/*${ends} was built for ${target}`);
    if (found.length > 1) throw new AssetError(`${found.join(' and ')} would both be named ${to}`);
    const from = found[0]!;
    copies.push({ from, to });
    if (present.has(`${from}.sig`)) copies.push({ from: `${from}.sig`, to: `${to}.sig` });
    else if (updater) throw new AssetError(`${from} has no .sig for the in-app updater`);
  }
  if (!copies.length) throw new AssetError(`No bundle to collect for ${target}`);
  const names = new Set<string>();
  for (const { from, to } of copies) {
    if (names.has(to)) throw new AssetError(`Two files would be named ${to} (one is ${from})`);
    names.add(to);
  }
  return copies;
}
