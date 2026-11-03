import {
  checkSigningSetup,
  type Commit,
  isPrerelease,
  parseCommit,
  planRelease,
  readCargoLockVersion,
  readCargoTomlVersion,
  readJsonVersion,
  releaseNotes,
} from './release-plan';

let n = 0;
const commit = (subject: string, body = ''): Commit => ({
  sha: `${(++n).toString(16).padStart(7, '0')}${'0'.repeat(33)}`,
  subject,
  body,
});

const agreeing = (version: string) => ({
  'tauri.conf.json': version,
  'package.json': version,
  'Cargo.toml': version,
  'Cargo.lock': version,
});

describe('parseCommit', () => {
  it('reads type, scope and description', () => {
    expect(parseCommit(commit('feat(stacks): deploy over SSH'))).toMatchObject({
      type: 'feat',
      scope: 'stacks',
      description: 'deploy over SSH',
      breaking: false,
    });
  });

  it.each([
    ['feat!: drop TCP Hosts', ''],
    ['fix(db)!: rename the settings table', ''],
    ['refactor: move secrets', 'Details.\n\nBREAKING CHANGE: secrets need re-entry'],
    ['refactor: move secrets', 'BREAKING-CHANGE: secrets need re-entry'],
  ])('marks %s as breaking', (subject, body) => {
    expect(parseCommit(commit(subject, body))?.breaking).toBe(true);
  });

  it.each(["Merge branch 'dev'", 'Update README', 'feat:missing space'])(
    'ignores %s',
    (subject) => {
      expect(parseCommit(commit(subject))).toBeUndefined();
    },
  );
});

describe('isPrerelease', () => {
  it.each([
    ['1.2.3', false],
    ['0.1.0', false],
    ['1.0.0-beta.1', true],
    ['2.0.0-rc', true],
  ])('%s → %s', (version, expected) => {
    expect(isPrerelease(version)).toBe(expected);
  });

  it.each(['v1.2.3', '1.2', '01.2.3', '1.2.3-', '1.2.3+build', ''])('rejects %s', (version) => {
    expect(() => isPrerelease(version)).toThrow(/semantic version/);
  });
});

describe('planRelease', () => {
  it('releases the version the files declare', () => {
    expect(planRelease({ versions: agreeing('0.2.0'), tags: ['v0.1.0'] })).toEqual({
      version: '0.2.0',
      tag: 'v0.2.0',
      prerelease: false,
    });
  });

  it('marks a version with a suffix as a prerelease', () => {
    expect(planRelease({ versions: agreeing('1.0.0-beta.1'), tags: [] }).prerelease).toBe(true);
  });

  it('fails when the version is already released', () => {
    expect(() => planRelease({ versions: agreeing('0.1.0'), tags: ['v0.1.0'] })).toThrow(
      /v0\.1\.0 is already released/,
    );
  });

  it('fails when the files disagree, naming each one', () => {
    expect(() =>
      planRelease({ versions: { ...agreeing('0.2.0'), 'Cargo.lock': '0.1.0' }, tags: [] }),
    ).toThrow(/disagree:[\s\S]*Cargo\.lock: 0\.1\.0/);
  });

  it('fails when a file has no version', () => {
    expect(() =>
      planRelease({ versions: { ...agreeing('0.2.0'), 'Cargo.toml': undefined }, tags: [] }),
    ).toThrow(/No version found in Cargo\.toml/);
  });
});

describe('checkSigningSetup', () => {
  it('passes once both public keys are committed', () => {
    expect(() =>
      checkSigningSetup({ updaterPubkey: 'dW50cnVzdGVk', packageKey: true }),
    ).not.toThrow();
  });

  it('names each missing key and how to create them', () => {
    expect(() => checkSigningSetup({ updaterPubkey: '', packageKey: false })).toThrow(
      /updater public key.*and the package signing key.*setup-signing\.sh/,
    );
    expect(() => checkSigningSetup({ updaterPubkey: undefined, packageKey: true })).toThrow(
      /^Missing the updater public key/,
    );
  });
});

describe('reading versions', () => {
  it('reads the top-level JSON version only', () => {
    expect(readJsonVersion('{ "deps": { "x": { "version": "1" } }, "version": "0.3.0" }')).toBe(
      '0.3.0',
    );
    expect(readJsonVersion('{ "deps": { "version": "1" } }')).toBeUndefined();
  });

  it('reads the workspace package version', () => {
    const toml =
      '[workspace]\nmembers = ["src-tauri"]\n\n[workspace.package]\nedition = "2021"\nversion = "0.1.0"\n';
    expect(readCargoTomlVersion(toml)).toBe('0.1.0');
    expect(readCargoTomlVersion('[package]\nversion = "0.1.0"\n')).toBeUndefined();
  });

  it('reads the homeport entry of Cargo.lock', () => {
    const lock =
      '[[package]]\nname = "home"\nversion = "0.5.9"\n\n[[package]]\nname = "homeport"\nversion = "0.1.0"\ndependencies = []\n';
    expect(readCargoLockVersion(lock)).toBe('0.1.0');
  });
});

describe('releaseNotes', () => {
  const commits = [
    commit('feat(hosts): add Podman sockets'),
    commit('fix: keep the window size'),
    commit('feat!: require Docker 24'),
    commit('build: update tauri'),
    commit('docs: tidy README'),
  ];

  const notes = releaseNotes({
    version: '0.3.0',
    previous: '0.2.1',
    commits,
    repository: 'owner/homeport',
  });

  it('groups commits, listing a breaking feature once', () => {
    expect(notes).toMatch(
      /### Breaking changes\n\n- require Docker 24 .*\n\n### Features\n\n- \*\*hosts:\*\* add Podman sockets .*\n\n### Fixes\n\n- keep the window size .*\n\n### Other changes\n\n- update tauri /,
    );
    expect(notes).not.toContain('tidy README');
  });

  it('links commits and the full diff', () => {
    expect(notes).toContain(
      `([${commits[0]!.sha.slice(0, 7)}](https://github.com/owner/homeport/commit/${commits[0]!.sha}))`,
    );
    expect(notes).toContain('https://github.com/owner/homeport/compare/v0.2.1...v0.3.0');
  });

  it('says so when nothing user-facing changed', () => {
    expect(releaseNotes({ version: '0.3.1', commits: [commit('docs: a')] })).toContain(
      'Maintenance release.',
    );
  });

  it('names every download', () => {
    expect(notes).toContain('`Homeport_0.3.0_aarch64.dmg`');
    expect(notes).toContain('`Homeport-0.3.0-1.x86_64.rpm`');
    expect(notes).toContain('`Homeport_0.3.0_x64_en-US.msi`');
    expect(notes).toContain('[Homeport package repository]');
  });

  it('offers no MSI for a prerelease', () => {
    expect(releaseNotes({ version: '1.0.0-beta.1', commits })).not.toContain('.msi');
  });
});
