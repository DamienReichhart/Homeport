import {
  compareVersions,
  type GitHubRelease,
  packagesOf,
  selectReleases,
  updateManifest,
  updateNotes,
  updaterTargets,
} from './repositories-plan';

const release = (tag: string, extra: Partial<GitHubRelease> = {}): GitHubRelease => ({
  tag_name: tag,
  body: null,
  draft: false,
  prerelease: false,
  published_at: '2026-11-03T12:00:00Z',
  assets: [],
  ...extra,
});

const asset = (name: string) => ({
  name,
  browser_download_url: `https://github.com/o/r/releases/download/v1.0.0/${name}`,
});

describe('compareVersions', () => {
  it('orders by major, minor, then patch, numerically', () => {
    expect(compareVersions('0.10.0', '0.9.9')).toBeGreaterThan(0);
    expect(compareVersions('v1.2.3', '1.2.3')).toBe(0);
    expect(compareVersions('1.0.0', '1.0.1')).toBeLessThan(0);
  });

  it('rejects prereleases', () => {
    expect(() => compareVersions('1.0.0-beta.1', '1.0.0')).toThrow(/stable/);
  });
});

describe('selectReleases', () => {
  it('keeps the newest stable releases since the first repository release', () => {
    const releases = [
      release('v0.2.0'),
      release('v0.3.0'),
      release('v0.10.0'),
      release('v0.4.0-beta.1', { prerelease: true }),
      release('v0.5.0', { draft: true }),
      release('v0.4.0'),
    ];
    expect(selectReleases(releases).map((r) => r.version)).toEqual(['0.10.0', '0.4.0', '0.3.0']);
  });

  it('keeps at most `keep` releases', () => {
    const releases = ['v1.0.0', 'v1.1.0', 'v1.2.0'].map((tag) => release(tag));
    expect(selectReleases(releases, { keep: 2 }).map((r) => r.tag)).toEqual(['v1.2.0', 'v1.1.0']);
  });

  it('is empty before the first repository release', () => {
    expect(selectReleases([release('v0.2.0'), release('v0.1.0')])).toEqual([]);
  });
});

describe('packagesOf', () => {
  it('picks the .deb and .rpm files', () => {
    const [r] = selectReleases([
      release('v1.0.0', {
        assets: [
          asset('Homeport_1.0.0_amd64.deb'),
          asset('Homeport_1.0.0_amd64.deb.sig'),
          asset('Homeport-1.0.0-1.aarch64.rpm'),
          asset('Homeport_1.0.0_amd64.AppImage'),
        ],
      }),
    ]);
    const { deb, rpm } = packagesOf(r!);
    expect(deb.map((a) => a.name)).toEqual(['Homeport_1.0.0_amd64.deb']);
    expect(rpm.map((a) => a.name)).toEqual(['Homeport-1.0.0-1.aarch64.rpm']);
  });
});

describe('updaterTargets', () => {
  it('maps each updater file to its targets', () => {
    expect(updaterTargets('Homeport_1.0.0_amd64.AppImage')).toEqual([
      'linux-x86_64',
      'linux-x86_64-appimage',
    ]);
    expect(updaterTargets('Homeport_1.0.0_aarch64.AppImage')).toEqual([
      'linux-aarch64',
      'linux-aarch64-appimage',
    ]);
    expect(updaterTargets('Homeport_1.0.0_x64-setup.exe')).toEqual([
      'windows-x86_64',
      'windows-x86_64-nsis',
    ]);
    expect(updaterTargets('Homeport_1.0.0_arm64-setup.exe')).toEqual([
      'windows-aarch64',
      'windows-aarch64-nsis',
    ]);
    expect(updaterTargets('Homeport_1.0.0_x64_en-US.msi')).toEqual(['windows-x86_64-msi']);
    expect(updaterTargets('Homeport_1.0.0_aarch64.app.tar.gz')).toEqual([
      'darwin-aarch64',
      'darwin-aarch64-app',
    ]);
    expect(updaterTargets('Homeport_1.0.0_x64.app.tar.gz')).toEqual([
      'darwin-x86_64',
      'darwin-x86_64-app',
    ]);
  });

  it('ignores files the updater doesn’t install', () => {
    for (const name of [
      'Homeport_1.0.0_amd64.deb',
      'Homeport-1.0.0-1.x86_64.rpm',
      'Homeport_1.0.0_aarch64.dmg',
      'Homeport_1.0.0_amd64.AppImage.sig',
      'SHA256SUMS.txt',
    ]) {
      expect(updaterTargets(name)).toBeUndefined();
    }
  });
});

describe('updateNotes', () => {
  it('keeps what changed and drops the download table', () => {
    const body =
      "## What's changed\n\n### Features\n\n- **updates:** in-app updates\n\n## Download\n\n| a | b |";
    expect(updateNotes(body)).toBe('### Features\n\n- **updates:** in-app updates');
  });

  it('keeps notes without sections as they are', () => {
    expect(updateNotes(' Maintenance release. ')).toBe('Maintenance release.');
  });
});

describe('updateManifest', () => {
  const [r] = selectReleases([
    release('v1.0.0', {
      body: "## What's changed\n\nFaster.\n\n## Download\n",
      assets: [
        asset('Homeport_1.0.0_amd64.AppImage'),
        asset('Homeport_1.0.0_amd64.AppImage.sig'),
        asset('Homeport_1.0.0_x64-setup.exe'),
        asset('Homeport_1.0.0_x64_en-US.msi'),
        asset('Homeport_1.0.0_amd64.deb'),
      ],
    }),
  ]);

  it('lists every signed updater file under its targets', () => {
    const manifest = updateManifest(
      r!,
      new Map([
        ['Homeport_1.0.0_amd64.AppImage', 'sig-appimage\n'],
        ['Homeport_1.0.0_x64-setup.exe', 'sig-nsis'],
        ['Homeport_1.0.0_amd64.deb', 'sig-deb'],
      ]),
    );
    expect(manifest).toEqual({
      version: '1.0.0',
      notes: 'Faster.',
      pub_date: '2026-11-03T12:00:00Z',
      platforms: {
        'linux-x86_64': {
          url: expect.stringMatching(/amd64\.AppImage$/),
          signature: 'sig-appimage',
        },
        'linux-x86_64-appimage': {
          url: expect.stringMatching(/amd64\.AppImage$/),
          signature: 'sig-appimage',
        },
        'windows-x86_64': { url: expect.stringMatching(/x64-setup\.exe$/), signature: 'sig-nsis' },
        'windows-x86_64-nsis': {
          url: expect.stringMatching(/x64-setup\.exe$/),
          signature: 'sig-nsis',
        },
      },
    });
  });

  it('is undefined for a release without updater files', () => {
    expect(updateManifest(r!, new Map())).toBeUndefined();
  });
});
