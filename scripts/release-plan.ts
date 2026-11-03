/**
 * Checks that the version in the repo files can be released, and writes the
 * release notes from the Conventional Commits since the previous release.
 * Pure functions only; `release.ts` feeds them from git and the repo files.
 */

export class ReleaseError extends Error {}

export interface Commit {
  sha: string;
  subject: string;
  body: string;
}

export interface ParsedCommit {
  sha: string;
  type: string;
  scope?: string;
  description: string;
  breaking: boolean;
}

const HEADER = /^(\w+)(?:\(([^)]+)\))?(!)?: (.+)$/;
const BREAKING_FOOTER = /^BREAKING[ -]CHANGE: /m;

/** `undefined` for a subject that isn't a Conventional Commit (merge commits, for instance). */
export function parseCommit(commit: Commit): ParsedCommit | undefined {
  const match = HEADER.exec(commit.subject.trim());
  if (!match) return undefined;
  const [, type, scope, bang, description] = match;
  return {
    sha: commit.sha,
    type: type!.toLowerCase(),
    scope,
    description: description!,
    breaking: bang === '!' || BREAKING_FOOTER.test(commit.body),
  };
}

// --- Versions ------------------------------------------------------------------

const SEMVER =
  /^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[A-Za-z-][0-9A-Za-z-]*)(?:\.(?:0|[1-9]\d*|\d*[A-Za-z-][0-9A-Za-z-]*))*))?$/;

/** Whether `version` has a prerelease suffix (`1.0.0-beta.1`). Throws on anything but `x.y.z[-pre]`. */
export function isPrerelease(version: string): boolean {
  const match = SEMVER.exec(version);
  if (!match) throw new ReleaseError(`"${version}" is not a semantic version (x.y.z or x.y.z-pre)`);
  return match[4] !== undefined;
}

/** The top-level `"version"` of `package.json` or `tauri.conf.json`. */
export function readJsonVersion(text: string): string | undefined {
  const version = (JSON.parse(text) as { version?: unknown }).version;
  return typeof version === 'string' ? version : undefined;
}

/** `[workspace.package] version` in the root `Cargo.toml`. */
export function readCargoTomlVersion(text: string): string | undefined {
  return /\[workspace\.package\][^[]*?^version\s*=\s*"([^"]*)"/m.exec(text)?.[1];
}

/** The version of the `homeport` package in `Cargo.lock`. */
export function readCargoLockVersion(text: string): string | undefined {
  return /\[\[package\]\]\r?\nname = "homeport"\r?\nversion = "([^"]*)"/.exec(text)?.[1];
}

export interface PlanInput {
  /** The version each repo file declares, by file name. */
  versions: Record<string, string | undefined>;
  /** Every tag in the repo. */
  tags: readonly string[];
}

export interface Plan {
  version: string;
  tag: string;
  prerelease: boolean;
}

/**
 * Releases the version the repo files declare. Fails when they disagree, or
 * when that version was already released: bumping it is a deliberate step.
 */
export function planRelease({ versions, tags }: PlanInput): Plan {
  const missing = Object.keys(versions).filter((file) => !versions[file]);
  if (missing.length) throw new ReleaseError(`No version found in ${missing.join(', ')}`);

  const distinct = new Set(Object.values(versions));
  if (distinct.size > 1) {
    const list = Object.entries(versions).map(([file, version]) => `  ${file}: ${version}`);
    throw new ReleaseError(
      `The version files disagree:\n${list.join('\n')}\nSet the same version in all of them (run \`cargo check\` to update Cargo.lock).`,
    );
  }

  const version = [...distinct][0]!;
  const prerelease = isPrerelease(version);
  const tag = `v${version}`;
  if (tags.includes(tag)) {
    throw new ReleaseError(
      `${tag} is already released. Bump the version in tauri.conf.json, package.json and Cargo.toml (then \`cargo check\`) to release again.`,
    );
  }
  return { version, tag, prerelease };
}

// --- Notes ---------------------------------------------------------------------

/** Commit types the notes list. The rest (docs, ci, chore, test, style) don't change what users get. */
const LISTED = new Set(['feat', 'fix', 'perf', 'refactor', 'revert', 'build']);

const SECTIONS: { title: string; matches: (c: ParsedCommit) => boolean }[] = [
  { title: 'Breaking changes', matches: (c) => c.breaking },
  { title: 'Features', matches: (c) => c.type === 'feat' },
  { title: 'Fixes', matches: (c) => c.type === 'fix' },
  { title: 'Performance', matches: (c) => c.type === 'perf' },
  { title: 'Other changes', matches: (c) => LISTED.has(c.type) },
];

export interface NotesInput {
  version: string;
  /** The previous release's version, if any. */
  previous?: string;
  commits: readonly Commit[];
  /** `owner/name`, to link commits and the full diff. */
  repository?: string;
}

/** Markdown for the GitHub release: what changed, then which file to download. */
export function releaseNotes({ version, previous, commits, repository }: NotesInput): string {
  const base = repository ? `https://github.com/${repository}` : undefined;
  const line = (c: ParsedCommit) => {
    const short = c.sha.slice(0, 7);
    const ref = base ? `[${short}](${base}/commit/${c.sha})` : short;
    return `- ${c.scope ? `**${c.scope}:** ` : ''}${c.description} (${ref})`;
  };

  const parsed = commits.map(parseCommit).filter((c): c is ParsedCommit => c !== undefined);
  const listed = new Set<ParsedCommit>();
  const changes: string[] = [];
  for (const section of SECTIONS) {
    const entries = parsed.filter((c) => !listed.has(c) && section.matches(c));
    if (!entries.length) continue;
    entries.forEach((c) => listed.add(c));
    changes.push(`### ${section.title}`, '', ...entries.map(line), '');
  }
  if (!changes.length) changes.push('Maintenance release.', '');

  const v = version;
  // WiX only takes numeric prerelease identifiers, so prereleases ship no MSI.
  const msi = isPrerelease(v) ? '' : ` or \`Homeport_${v}_x64_en-US.msi\``;
  return [
    "## What's changed",
    '',
    ...changes,
    '## Download',
    '',
    '| System | File |',
    '| --- | --- |',
    `| Windows (x64) | \`Homeport_${v}_x64-setup.exe\`${msi} |`,
    `| Windows (ARM) | \`Homeport_${v}_arm64-setup.exe\` |`,
    `| macOS (Apple silicon) | \`Homeport_${v}_aarch64.dmg\` |`,
    `| macOS (Intel) | \`Homeport_${v}_x64.dmg\` |`,
    `| Debian, Ubuntu (x64 / ARM) | \`Homeport_${v}_amd64.deb\` / \`Homeport_${v}_arm64.deb\` |`,
    `| Fedora, RHEL, openSUSE (x64 / ARM) | \`Homeport-${v}-1.x86_64.rpm\` / \`Homeport-${v}-1.aarch64.rpm\` |`,
    `| Any Linux (x64 / ARM) | \`Homeport_${v}_amd64.AppImage\` / \`Homeport_${v}_aarch64.AppImage\` |`,
    '',
    'Check a download against `SHA256SUMS.txt`, or verify where it was built with',
    `\`gh attestation verify <file> --repo ${repository ?? '<owner>/<repo>'}\`.`,
    ...(base && previous ? ['', `**Full changelog:** ${base}/compare/v${previous}...v${v}`] : []),
    '',
  ].join('\n');
}
