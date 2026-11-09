/**
 * Builds the package repositories and the update manifest for the website
 * (ADR 0003), from the latest stable GitHub releases:
 *
 *   bun scripts/repositories.ts --out website/dist [--public-key <file>] [--packages <dir>]
 *
 * Writes into `--out`:
 *   apt/                 APT repository (suite `stable`, component `main`), signed
 *   apt/homeport.sources the APT source the install script and the .deb add
 *   rpm/                 RPM repository for dnf and zypper, metadata signed
 *   rpm/homeport.repo    the repository file the install script and the .rpm add
 *   updates/latest.json  the in-app updater's manifest
 *   homeport.asc         the public signing key
 *   install.sh           the quick install script
 *
 * Signs with the secret key matching `--public-key` (default
 * `packaging/linux/homeport.asc`, which the packages ship), so gpg must hold it.
 * Needs `apt-ftparchive` (apt-utils), `createrepo_c` and `gpg`. `--packages`
 * uses the .deb, .rpm and .sig files in a directory as the only release, to
 * test without publishing one.
 */
import { copyFileSync, mkdirSync, readdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { basename, join, resolve } from 'node:path';
import {
  aptPoolName,
  type GitHubRelease,
  packagesOf,
  type Release,
  selectReleases,
  updateManifest,
} from './repositories-plan';

const ROOT = join(import.meta.dir, '..');
const PACKAGING = join(ROOT, 'packaging/linux');
const ARCHITECTURES = ['amd64', 'arm64'];

class RepositoryError extends Error {}

function option(args: string[], name: string): string | undefined {
  const index = args.indexOf(name);
  return index >= 0 ? args[index + 1] : undefined;
}

function run(cmd: string[], { cwd, stdout }: { cwd?: string; stdout?: string } = {}): string {
  const proc = Bun.spawnSync(cmd, { cwd, stdout: 'pipe', stderr: 'pipe' });
  if (proc.exitCode !== 0) {
    throw new RepositoryError(`${cmd.join(' ')} failed:\n${proc.stderr.toString()}`);
  }
  const out = proc.stdout.toString();
  if (stdout) writeFileSync(stdout, out);
  return out;
}

/** The fingerprint of the one key in an armored public key file. */
function fingerprint(publicKey: string): string {
  const lines = run(['gpg', '--batch', '--with-colons', '--show-keys', publicKey]).split('\n');
  const fprs = lines.filter((l) => l.startsWith('fpr:')).map((l) => l.split(':')[9]);
  const primary = fprs[0];
  if (!primary) throw new RepositoryError(`No key in ${publicKey}`);
  return primary;
}

function sign(fpr: string, file: string, mode: 'clearsign' | 'detach', output: string) {
  run([
    'gpg',
    '--batch',
    '--yes',
    '--armor',
    '--digest-algo',
    'SHA512',
    '--local-user',
    fpr,
    mode === 'clearsign' ? '--clearsign' : '--detach-sign',
    '--output',
    output,
    file,
  ]);
}

async function githubReleases(): Promise<GitHubRelease[]> {
  const repo = process.env.GITHUB_REPOSITORY ?? 'DamienReichhart/Homeport';
  const headers: Record<string, string> = {
    accept: 'application/vnd.github+json',
    'user-agent': 'homeport-repositories-build',
  };
  if (process.env.GITHUB_TOKEN) headers.authorization = `Bearer ${process.env.GITHUB_TOKEN}`;
  const url = `https://api.github.com/repos/${repo}/releases?per_page=100`;
  const response = await fetch(url, { headers });
  if (!response.ok)
    throw new RepositoryError(`GET ${url}: ${response.status} ${response.statusText}`);
  return (await response.json()) as GitHubRelease[];
}

/** Treats the files in `dir` as one release, for testing. */
function localRelease(dir: string): Release {
  const files = readdirSync(dir);
  // Tauri's own names (`Homeport_1.0.0_amd64.deb`) or a release's (`Homeport-1.0.0-linux-x64.deb`).
  const version = /Homeport[_-](\d+\.\d+\.\d+)[_-]/.exec(files.join(' '))?.[1] ?? '0.0.0';
  return {
    tag: `v${version}`,
    version,
    publishedAt: new Date().toISOString(),
    body: "## What's changed\n\nLocal test build.\n",
    assets: files.map((name) => ({ name, url: `file://${resolve(dir, name)}` })),
  };
}

async function download(url: string, file: string) {
  if (url.startsWith('file://')) return copyFileSync(url.slice('file://'.length), file);
  const response = await fetch(url);
  if (!response.ok)
    throw new RepositoryError(`GET ${url}: ${response.status} ${response.statusText}`);
  writeFileSync(file, new Uint8Array(await response.arrayBuffer()));
}

async function text(url: string): Promise<string> {
  if (url.startsWith('file://')) return readFileSync(url.slice('file://'.length), 'utf8');
  const response = await fetch(url);
  if (!response.ok)
    throw new RepositoryError(`GET ${url}: ${response.status} ${response.statusText}`);
  return response.text();
}

function buildApt(dir: string, fpr: string) {
  const dist = join(dir, 'dists/stable');
  for (const arch of ARCHITECTURES) {
    const binary = join(dist, `main/binary-${arch}`);
    mkdirSync(binary, { recursive: true });
    // Run from the repository root so each `Filename:` is relative to it.
    // `--arch` takes the files named `*_<arch>.deb`: see `aptPoolName`.
    run(['apt-ftparchive', '--arch', arch, 'packages', 'pool'], {
      cwd: dir,
      stdout: join(binary, 'Packages'),
    });
    run(['gzip', '-9nkf', join(binary, 'Packages')]);
  }
  const fields = {
    Origin: 'Homeport',
    Label: 'Homeport',
    Suite: 'stable',
    Codename: 'stable',
    Architectures: ARCHITECTURES.join(' '),
    Components: 'main',
    Description: 'Homeport, a desktop app for managing Docker Engines',
  };
  const release = run([
    'apt-ftparchive',
    ...Object.entries(fields).flatMap(([k, v]) => ['-o', `APT::FTPArchive::Release::${k}=${v}`]),
    'release',
    dist,
  ]);
  writeFileSync(join(dist, 'Release'), release);
  sign(fpr, join(dist, 'Release'), 'clearsign', join(dist, 'InRelease'));
  sign(fpr, join(dist, 'Release'), 'detach', join(dist, 'Release.gpg'));
}

function buildRpm(dir: string, fpr: string) {
  // gzip rather than zstd metadata, which older dnf and zypper can't read.
  run(['createrepo_c', '--general-compress-type=gz', dir]);
  const repomd = join(dir, 'repodata/repomd.xml');
  sign(fpr, repomd, 'detach', `${repomd}.asc`);
}

async function build(args: string[]) {
  const out = option(args, '--out');
  if (!out)
    throw new RepositoryError(
      'Usage: bun scripts/repositories.ts --out <dir> [--public-key <file>] [--packages <dir>]',
    );
  const publicKey = option(args, '--public-key') ?? join(PACKAGING, 'homeport.asc');
  const fpr = fingerprint(publicKey);
  run(['gpg', '--batch', '--list-secret-keys', fpr]); // fails early without the secret key

  const packagesDir = option(args, '--packages');
  const releases = packagesDir
    ? [localRelease(packagesDir)]
    : selectReleases(await githubReleases());
  if (!releases.length)
    console.warn('No release is built for the repositories yet: publishing them empty.');

  const apt = join(out, 'apt');
  const rpm = join(out, 'rpm');
  rmSync(apt, { recursive: true, force: true });
  rmSync(rpm, { recursive: true, force: true });
  rmSync(join(out, 'updates'), { recursive: true, force: true });
  const pool = join(apt, 'pool/main/h/homeport');
  const rpms = join(rpm, 'packages');
  mkdirSync(pool, { recursive: true });
  mkdirSync(rpms, { recursive: true });

  for (const release of releases) {
    const { deb, rpm: rpmFiles } = packagesOf(release);
    console.log(`${release.tag}: ${deb.length} .deb, ${rpmFiles.length} .rpm`);
    await Promise.all([
      ...deb.map((a) => download(a.url, join(pool, aptPoolName(release, a)))),
      ...rpmFiles.map((a) => download(a.url, join(rpms, a.name))),
    ]);
  }

  buildApt(apt, fpr);
  buildRpm(rpm, fpr);
  copyFileSync(join(PACKAGING, 'homeport.sources'), join(apt, 'homeport.sources'));
  copyFileSync(join(PACKAGING, 'homeport.repo'), join(rpm, 'homeport.repo'));
  copyFileSync(publicKey, join(out, 'homeport.asc'));
  copyFileSync(join(PACKAGING, 'install.sh'), join(out, 'install.sh'));

  const latest = releases[0];
  if (latest) {
    const names = new Set(latest.assets.map((a) => a.name));
    const signatures = new Map<string, string>();
    await Promise.all(
      latest.assets
        .filter((a) => names.has(`${a.name}.sig`))
        .map(async (a) => {
          const sig = latest.assets.find((s) => s.name === `${a.name}.sig`)!;
          signatures.set(a.name, await text(sig.url));
        }),
    );
    const manifest = updateManifest(latest, signatures);
    if (manifest) {
      mkdirSync(join(out, 'updates'), { recursive: true });
      writeFileSync(join(out, 'updates/latest.json'), `${JSON.stringify(manifest, null, 2)}\n`);
      console.log(
        `updates/latest.json: ${manifest.version}, ${Object.keys(manifest.platforms).join(', ')}`,
      );
    } else {
      console.warn(`${latest.tag} has no updater files: no updates/latest.json.`);
    }
  }
  console.log(
    `Signed with ${fpr}; wrote ${basename(apt)}/, ${basename(rpm)}/ and the key to ${out}`,
  );
}

if (import.meta.main) {
  try {
    await build(process.argv.slice(2));
  } catch (error) {
    if (!(error instanceof RepositoryError)) throw error;
    console.error(
      process.env.GITHUB_ACTIONS
        ? `::error::${error.message.replaceAll('\n', '%0A')}`
        : error.message,
    );
    process.exit(1);
  }
}
