/**
 * Writes `src-tauri/licenses/THIRD_PARTY_LICENSES.txt`: the license of every Rust
 * crate and npm package that ends up in Homeport, grouped by license text.
 *
 * Rust crates come from `cargo about` (configured by `about.toml`); npm packages
 * are the production dependency tree of `package.json`. Both are checked against
 * `about.toml > accepted`, so a dependency under any other license fails the run.
 *
 * Runs before every `tauri build` (`build.beforeBuildCommand`); the file is
 * bundled as a resource and shown in Settings.
 */
import { existsSync, mkdirSync, readdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join, relative } from 'node:path';
import spdx from 'spdx-license-list/full';
import { isAccepted } from './spdx';

const ROOT = join(import.meta.dir, '..');
const OUTPUT = join(ROOT, 'src-tauri/licenses/THIRD_PARTY_LICENSES.txt');
const ABOUT = Bun.TOML.parse(readFileSync(join(ROOT, 'about.toml'), 'utf8')) as {
  accepted: string[];
};
const RULE = '='.repeat(80);

interface Package {
  name: string;
  version: string;
  ecosystem: 'crate' | 'npm';
  repository?: string;
}

interface License {
  /** Display name, e.g. "MIT License" or "(MIT OR Apache-2.0)". */
  name: string;
  text: string;
  usedBy: Package[];
}

class LicenseError extends Error {}

// --- Rust ----------------------------------------------------------------------

interface AboutOutput {
  licenses: {
    name: string;
    text: string;
    used_by: {
      crate: { name: string; version: string; source: string | null; repository: string | null };
    }[];
  }[];
}

async function rustLicenses(): Promise<License[]> {
  const proc = Bun.spawn(
    ['cargo', 'about', 'generate', '--format', 'json', '--workspace', '--locked', '--fail'],
    { cwd: ROOT, stdout: 'pipe', stderr: 'inherit' },
  );
  const [stdout, code] = await Promise.all([new Response(proc.stdout).text(), proc.exited]);
  if (code !== 0) {
    throw new LicenseError(
      'cargo about failed (see above). Install it with `cargo install cargo-about --locked --features cli`; ' +
        'if a crate uses a license missing from about.toml > accepted, review it before adding it.',
    );
  }
  const about = JSON.parse(stdout) as AboutOutput;
  return about.licenses.map((license) => ({
    name: license.name,
    text: license.text,
    usedBy: license.used_by
      // Workspace members (no source) are Homeport itself.
      .filter(({ crate }) => crate.source !== null)
      .map(({ crate }) => ({
        name: crate.name,
        version: crate.version,
        ecosystem: 'crate' as const,
        repository: crate.repository ?? undefined,
      })),
  }));
}

// --- npm -----------------------------------------------------------------------

interface Manifest {
  name: string;
  version: string;
  license?: string | { type: string };
  licenses?: { type: string }[];
  author?: string | { name: string };
  repository?: string | { url: string };
  dependencies?: Record<string, string>;
  optionalDependencies?: Record<string, string>;
}

const LICENSE_FILE = /^(licen[sc]e|copying|notice)([-._].*)?$/i;

/** Finds `name` the way Node does: in `node_modules` of `from` or any ancestor. */
function resolvePackage(name: string, from: string): string | undefined {
  for (let dir = from; ; dir = dirname(dir)) {
    const candidate = join(dir, 'node_modules', name);
    if (existsSync(join(candidate, 'package.json'))) return candidate;
    if (dir === ROOT || dirname(dir) === dir) return undefined;
  }
}

function declaredLicense(manifest: Manifest): string | undefined {
  if (typeof manifest.license === 'string') return manifest.license;
  if (manifest.license) return manifest.license.type;
  if (manifest.licenses?.length) return manifest.licenses.map((l) => l.type).join(' OR ');
  return undefined;
}

/**
 * The SPDX text of a single license, for packages that ship no license file,
 * with the package author as copyright holder.
 */
function canonicalText(expression: string, manifest: Manifest): string | undefined {
  const text = spdx[expression]?.licenseText;
  if (!text) return undefined;
  const author = typeof manifest.author === 'string' ? manifest.author : manifest.author?.name;
  return text.replace(/^Copyright .*<.*>.*$/m, author ? `Copyright (c) ${author}` : '');
}

function repositoryUrl(manifest: Manifest): string | undefined {
  const url =
    typeof manifest.repository === 'string' ? manifest.repository : manifest.repository?.url;
  return url
    ?.replace(/^git\+/, '')
    .replace(/\.git$/, '')
    .replace(/^github:/, 'https://github.com/');
}

function npmLicenses(): License[] {
  const root = JSON.parse(readFileSync(join(ROOT, 'package.json'), 'utf8')) as Manifest;
  const seen = new Set<string>();
  const byText = new Map<string, License>();
  const problems: string[] = [];

  const visit = (deps: Record<string, string> | undefined, from: string, optional: boolean) => {
    for (const name of Object.keys(deps ?? {})) {
      const dir = resolvePackage(name, from);
      if (!dir) {
        // Optional dependencies for other platforms are legitimately absent.
        if (!optional)
          problems.push(
            `${name} (required by ${relative(ROOT, from) || 'package.json'}) is not installed`,
          );
        continue;
      }
      if (seen.has(dir)) continue;
      seen.add(dir);

      const manifest = JSON.parse(readFileSync(join(dir, 'package.json'), 'utf8')) as Manifest;
      const id = `${manifest.name}@${manifest.version}`;
      const expression = declaredLicense(manifest);
      const files = readdirSync(dir, { withFileTypes: true })
        .filter((entry) => entry.isFile() && LICENSE_FILE.test(entry.name))
        .map((entry) => entry.name)
        .sort();
      if (!expression) problems.push(`${id} declares no license`);
      else if (!isAccepted(expression, ABOUT.accepted))
        problems.push(`${id} is ${expression}, which about.toml does not accept`);
      else {
        const text =
          files.length > 0
            ? files.map((file) => readFileSync(join(dir, file), 'utf8').trim()).join('\n\n')
            : canonicalText(expression, manifest);
        if (!text) problems.push(`${id} ships no license file`);
        else {
          const license = byText.get(text) ?? {
            name: spdx[expression]?.name ?? expression,
            text,
            usedBy: [],
          };
          license.usedBy.push({
            name: manifest.name,
            version: manifest.version,
            ecosystem: 'npm',
            repository: repositoryUrl(manifest),
          });
          byText.set(text, license);
        }
      }

      visit(manifest.dependencies, dir, false);
      visit(manifest.optionalDependencies, dir, true);
    }
  };
  visit(root.dependencies, ROOT, false);

  if (problems.length > 0) {
    throw new LicenseError(
      `npm packages need attention:\n${problems.map((p) => `  - ${p}`).join('\n')}`,
    );
  }
  return [...byText.values()];
}

// --- Output --------------------------------------------------------------------

function render(licenses: License[]): string {
  // The same text can come from both ecosystems; list it once.
  const byText = new Map<string, License>();
  for (const license of licenses) {
    const text = license.text.trim();
    const existing = byText.get(text);
    if (existing) existing.usedBy.push(...license.usedBy);
    else byText.set(text, { ...license, text, usedBy: [...license.usedBy] });
  }

  // Code-point order, so the file is the same whatever the machine's locale.
  const compare = (a: string, b: string) => (a < b ? -1 : a > b ? 1 : 0);
  const sorted = [...byText.values()].filter((license) => license.usedBy.length > 0);
  for (const license of sorted) {
    license.usedBy.sort((a, b) => compare(a.name, b.name) || compare(a.version, b.version));
  }
  sorted.sort((a, b) => compare(a.name, b.name) || compare(a.usedBy[0]!.name, b.usedBy[0]!.name));

  const count = (ecosystem: Package['ecosystem']) =>
    sorted.flatMap((license) => license.usedBy).filter((pkg) => pkg.ecosystem === ecosystem).length;

  const sections = sorted.map((license) => {
    const packages = license.usedBy.map(
      (pkg) =>
        `  ${pkg.name} ${pkg.version} (${pkg.ecosystem})${pkg.repository ? ` ${pkg.repository}` : ''}`,
    );
    return [RULE, license.name, '', 'Used by:', ...packages, '', license.text, ''].join('\n');
  });

  return [
    'Third-party software in Homeport',
    '',
    'Homeport includes the open-source software listed below. Each list of packages',
    'is followed by the license they are distributed under.',
    '',
    `${count('crate')} Rust crates and ${count('npm')} npm packages.`,
    'Generated by `bun run licenses`; do not edit.',
    '',
    ...sections,
  ].join('\n');
}

if (import.meta.main) {
  try {
    const [rust, npm] = await Promise.all([rustLicenses(), Promise.resolve().then(npmLicenses)]);
    mkdirSync(dirname(OUTPUT), { recursive: true });
    writeFileSync(OUTPUT, render([...rust, ...npm]));
    console.log(`Wrote ${relative(ROOT, OUTPUT)}`);
  } catch (error) {
    if (!(error instanceof LicenseError)) throw error;
    console.error(error.message);
    process.exit(1);
  }
}
