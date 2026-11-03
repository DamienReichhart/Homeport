/**
 * Plans a release for `.github/workflows/release.yml`.
 *
 *   bun scripts/release.ts plan [--notes <file>]
 *
 * Reads the version from tauri.conf.json, package.json, Cargo.toml and
 * Cargo.lock and fails unless they agree and that version has no `v*` tag yet
 * (see `release-plan.ts`), or while the public signing keys aren't committed.
 * Prints `version`, `tag` and `prerelease` as
 * `key=value` lines, also appended to `$GITHUB_OUTPUT` when set. `--notes`
 * writes the release notes, from the commits since the previous release.
 */
import { appendFileSync, existsSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import {
  checkSigningSetup,
  type Commit,
  planRelease,
  readCargoLockVersion,
  readCargoTomlVersion,
  readJsonVersion,
  ReleaseError,
  releaseNotes,
} from './release-plan';

const ROOT = join(import.meta.dir, '..');

function git(...args: string[]): { ok: boolean; out: string } {
  const proc = Bun.spawnSync(['git', ...args], { cwd: ROOT, stderr: 'pipe' });
  return { ok: proc.exitCode === 0, out: proc.stdout.toString().trim() };
}

function latestTag(): string | undefined {
  const { ok, out } = git('describe', '--tags', '--abbrev=0', '--match', 'v[0-9]*', 'HEAD');
  return ok && out ? out : undefined;
}

function commitsSince(tag: string | undefined): Commit[] {
  const { ok, out } = git('log', '--format=%H%x1f%s%x1f%b%x1e', tag ? `${tag}..HEAD` : 'HEAD');
  if (!ok) throw new Error('git log failed');
  return out
    .split('\x1e')
    .map((record) => record.trim())
    .filter(Boolean)
    .map((record) => {
      const [sha = '', subject = '', body = ''] = record.split('\x1f');
      return { sha, subject, body };
    });
}

function plan(args: string[]) {
  if (git('rev-parse', '--is-shallow-repository').out === 'true') {
    throw new ReleaseError(
      'The clone is shallow; check out with fetch-depth: 0 so tags and history are there.',
    );
  }
  const read = (file: string) => readFileSync(join(ROOT, file), 'utf8');
  const result = planRelease({
    versions: {
      'src-tauri/tauri.conf.json': readJsonVersion(read('src-tauri/tauri.conf.json')),
      'package.json': readJsonVersion(read('package.json')),
      'Cargo.toml': readCargoTomlVersion(read('Cargo.toml')),
      'Cargo.lock': readCargoLockVersion(read('Cargo.lock')),
    },
    tags: git('tag', '--list').out.split('\n'),
  });
  const tauri = JSON.parse(read('src-tauri/tauri.conf.json')) as {
    plugins?: { updater?: { pubkey?: string } };
  };
  checkSigningSetup({
    updaterPubkey: tauri.plugins?.updater?.pubkey,
    packageKey: existsSync(join(ROOT, 'packaging/linux/homeport.asc')),
  });

  const outputs = { version: result.version, tag: result.tag, prerelease: result.prerelease };
  const lines = Object.entries(outputs)
    .map(([key, value]) => `${key}=${value}`)
    .join('\n');
  if (process.env.GITHUB_OUTPUT) appendFileSync(process.env.GITHUB_OUTPUT, `${lines}\n`);
  console.log(lines);

  const notesIndex = args.indexOf('--notes');
  const notesFile = notesIndex >= 0 ? args[notesIndex + 1] : undefined;
  if (notesFile) {
    const previous = latestTag();
    writeFileSync(
      notesFile,
      releaseNotes({
        version: result.version,
        previous: previous?.slice(1),
        commits: commitsSince(previous),
        repository: process.env.GITHUB_REPOSITORY,
      }),
    );
  }
}

if (import.meta.main) {
  const [command, ...args] = process.argv.slice(2);
  try {
    if (command === 'plan') plan(args);
    else throw new ReleaseError('Usage: bun scripts/release.ts plan [--notes <file>]');
  } catch (error) {
    if (!(error instanceof ReleaseError)) throw error;
    // An error annotation shows on the run's summary page.
    console.error(
      process.env.GITHUB_ACTIONS
        ? `::error::${error.message.replaceAll('\n', '%0A')}`
        : error.message,
    );
    process.exit(1);
  }
}
