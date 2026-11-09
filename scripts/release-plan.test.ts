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

  it('names every download by system and processor, one row each', () => {
    const table = notes.split('\n').filter((l) => l.startsWith('| '));
    expect(table).toEqual([
      '| Your computer | File |',
      '| --- | --- |',
      '| Windows (Intel/AMD, x64) | `Homeport-0.3.0-windows-x64-setup.exe`, or `Homeport-0.3.0-windows-x64.msi` for managed and silent installs |',
      '| Windows (ARM, arm64) | `Homeport-0.3.0-windows-arm64-setup.exe` |',
      '| macOS (Apple silicon: M1 and later) | `Homeport-0.3.0-macos-arm64.dmg` |',
      '| macOS (Intel) | `Homeport-0.3.0-macos-x64.dmg` |',
      '| Linux (Intel/AMD, x64) | `Homeport-0.3.0-linux-x64.deb` (Debian, Ubuntu), `Homeport-0.3.0-linux-x64.rpm` (Fedora, RHEL, openSUSE) or `Homeport-0.3.0-linux-x64.AppImage` (any distribution) |',
      '| Linux (ARM, arm64) | `Homeport-0.3.0-linux-arm64.deb` (Debian, Ubuntu), `Homeport-0.3.0-linux-arm64.rpm` (Fedora, RHEL, openSUSE) or `Homeport-0.3.0-linux-arm64.AppImage` (any distribution) |',
    ]);
    expect(notes).toContain('[Homeport package repository]');
  });

  it('points at the download page first, and sets the updater files aside', () => {
    const download = notes.slice(notes.indexOf('## Download'));
    const which = download.indexOf(
      '**Which file do I need?** The [download page](https://damienreichhart.github.io/Homeport/download/)',
    );
    expect(which).toBeGreaterThan(0);
    expect(which).toBeLessThan(download.indexOf('| Your computer |'));
    expect(download).toContain(
      "Files ending in `.sig` and `-update.app.tar.gz` are used by the in-app updater; you don't need them.",
    );
    // No updater file is offered as a download.
    expect(download.replace(/Files ending in.*/, '')).not.toMatch(/app\.tar\.gz|\.sig/);
  });

  it('offers no MSI for a prerelease', () => {
    expect(releaseNotes({ version: '1.0.0-beta.1', commits })).not.toContain('.msi');
  });
});
