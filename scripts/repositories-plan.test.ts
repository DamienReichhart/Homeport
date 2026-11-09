import {
  aptPoolName,
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

  it('picks them under the names releases carry since they say the system and processor', () => {
    const [r] = selectReleases([
      release('v1.0.0', {
        assets: [
          asset('Homeport-1.0.0-linux-x64.deb'),
          asset('Homeport-1.0.0-linux-arm64.deb'),
          asset('Homeport-1.0.0-linux-x64.deb.sig'),
          asset('Homeport-1.0.0-linux-x64.rpm'),
          asset('Homeport-1.0.0-linux-arm64.rpm'),
          asset('Homeport-1.0.0-linux-x64.rpm.sig'),
          asset('Homeport-1.0.0-linux-x64.AppImage'),
        ],
      }),
    ]);
    const { deb, rpm } = packagesOf(r!);
    expect(deb.map((a) => a.name)).toEqual([
      'Homeport-1.0.0-linux-x64.deb',
      'Homeport-1.0.0-linux-arm64.deb',
    ]);
    expect(rpm.map((a) => a.name)).toEqual([
      'Homeport-1.0.0-linux-x64.rpm',
      'Homeport-1.0.0-linux-arm64.rpm',
    ]);
  });
});

describe('aptPoolName', () => {
  const [r] = selectReleases([release('v1.0.0')]);
  const pooled = (name: string) => aptPoolName(r!, { name, url: '' });

  // apt-ftparchive --arch amd64 only indexes *_amd64.deb.
  it('names a .deb by its Debian architecture, as apt-ftparchive selects them', () => {
    expect(pooled('Homeport-1.0.0-linux-x64.deb')).toBe('Homeport_1.0.0_amd64.deb');
    expect(pooled('Homeport-1.0.0-linux-arm64.deb')).toBe('Homeport_1.0.0_arm64.deb');
  });

  it('keeps the name older releases gave it', () => {
    expect(pooled('Homeport_1.0.0_amd64.deb')).toBe('Homeport_1.0.0_amd64.deb');
    expect(pooled('Homeport_1.0.0_arm64.deb')).toBe('Homeport_1.0.0_arm64.deb');
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

  it('maps the files named by system and processor to the same targets', () => {
    expect(updaterTargets('Homeport-1.0.0-linux-x64.AppImage')).toEqual([
      'linux-x86_64',
      'linux-x86_64-appimage',
    ]);
    expect(updaterTargets('Homeport-1.0.0-linux-arm64.AppImage')).toEqual([
      'linux-aarch64',
      'linux-aarch64-appimage',
    ]);
    expect(updaterTargets('Homeport-1.0.0-windows-x64-setup.exe')).toEqual([
      'windows-x86_64',
      'windows-x86_64-nsis',
    ]);
    expect(updaterTargets('Homeport-1.0.0-windows-arm64-setup.exe')).toEqual([
      'windows-aarch64',
      'windows-aarch64-nsis',
    ]);
    expect(updaterTargets('Homeport-1.0.0-windows-x64.msi')).toEqual(['windows-x86_64-msi']);
    expect(updaterTargets('Homeport-1.0.0-macos-arm64-update.app.tar.gz')).toEqual([
      'darwin-aarch64',
      'darwin-aarch64-app',
    ]);
    expect(updaterTargets('Homeport-1.0.0-macos-x64-update.app.tar.gz')).toEqual([
      'darwin-x86_64',
      'darwin-x86_64-app',
    ]);
  });

  it('ignores files the updater doesn’t install', () => {
    for (const name of [
      'Homeport-1.0.0-linux-x64.deb',
      'Homeport-1.0.0-linux-arm64.rpm',
      'Homeport-1.0.0-macos-arm64.dmg',
      'Homeport-1.0.0-linux-x64.AppImage.sig',
      'Homeport-1.0.0-macos-arm64-update.app.tar.gz.sig',
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

  // Installed copies of every version read the same manifest: a release named
  // either way must fill the targets they ask for.
  it('gives a release the same targets whichever way its files are named', () => {
    const manifestOf = (names: string[]) => {
      const [named] = selectReleases([release('v1.0.0', { assets: names.map(asset) })]);
      return updateManifest(named!, new Map(names.map((name) => [name, `sig of ${name}`])))!;
    };
    const tauri = manifestOf([
      'Homeport_1.0.0_x64-setup.exe',
      'Homeport_1.0.0_arm64-setup.exe',
      'Homeport_1.0.0_x64_en-US.msi',
      'Homeport_1.0.0_aarch64.dmg',
      'Homeport_1.0.0_x64.dmg',
      'Homeport_1.0.0_amd64.deb',
      'Homeport_1.0.0_arm64.deb',
      'Homeport-1.0.0-1.x86_64.rpm',
      'Homeport-1.0.0-1.aarch64.rpm',
      'Homeport_1.0.0_amd64.AppImage',
      'Homeport_1.0.0_aarch64.AppImage',
      'Homeport_1.0.0_x64.app.tar.gz',
      'Homeport_1.0.0_aarch64.app.tar.gz',
    ]);
    const named = manifestOf([
      'Homeport-1.0.0-windows-x64-setup.exe',
      'Homeport-1.0.0-windows-arm64-setup.exe',
      'Homeport-1.0.0-windows-x64.msi',
      'Homeport-1.0.0-macos-arm64.dmg',
      'Homeport-1.0.0-macos-x64.dmg',
      'Homeport-1.0.0-linux-x64.deb',
      'Homeport-1.0.0-linux-arm64.deb',
      'Homeport-1.0.0-linux-x64.rpm',
      'Homeport-1.0.0-linux-arm64.rpm',
      'Homeport-1.0.0-linux-x64.AppImage',
      'Homeport-1.0.0-linux-arm64.AppImage',
      'Homeport-1.0.0-macos-x64-update.app.tar.gz',
      'Homeport-1.0.0-macos-arm64-update.app.tar.gz',
    ]);
    const file = (url: string) => url.slice(url.lastIndexOf('/') + 1);
    const files = (manifest: typeof named) =>
      Object.fromEntries(Object.entries(manifest.platforms).map(([t, p]) => [t, file(p.url)]));
    expect(files(tauri)).toEqual({
      'darwin-aarch64': 'Homeport_1.0.0_aarch64.app.tar.gz',
      'darwin-aarch64-app': 'Homeport_1.0.0_aarch64.app.tar.gz',
      'darwin-x86_64': 'Homeport_1.0.0_x64.app.tar.gz',
      'darwin-x86_64-app': 'Homeport_1.0.0_x64.app.tar.gz',
      'linux-aarch64': 'Homeport_1.0.0_aarch64.AppImage',
      'linux-aarch64-appimage': 'Homeport_1.0.0_aarch64.AppImage',
      'linux-x86_64': 'Homeport_1.0.0_amd64.AppImage',
      'linux-x86_64-appimage': 'Homeport_1.0.0_amd64.AppImage',
      'windows-aarch64': 'Homeport_1.0.0_arm64-setup.exe',
      'windows-aarch64-nsis': 'Homeport_1.0.0_arm64-setup.exe',
      'windows-x86_64': 'Homeport_1.0.0_x64-setup.exe',
      'windows-x86_64-msi': 'Homeport_1.0.0_x64_en-US.msi',
      'windows-x86_64-nsis': 'Homeport_1.0.0_x64-setup.exe',
    });
    expect(files(named)).toEqual({
      'darwin-aarch64': 'Homeport-1.0.0-macos-arm64-update.app.tar.gz',
      'darwin-aarch64-app': 'Homeport-1.0.0-macos-arm64-update.app.tar.gz',
      'darwin-x86_64': 'Homeport-1.0.0-macos-x64-update.app.tar.gz',
      'darwin-x86_64-app': 'Homeport-1.0.0-macos-x64-update.app.tar.gz',
      'linux-aarch64': 'Homeport-1.0.0-linux-arm64.AppImage',
      'linux-aarch64-appimage': 'Homeport-1.0.0-linux-arm64.AppImage',
      'linux-x86_64': 'Homeport-1.0.0-linux-x64.AppImage',
      'linux-x86_64-appimage': 'Homeport-1.0.0-linux-x64.AppImage',
      'windows-aarch64': 'Homeport-1.0.0-windows-arm64-setup.exe',
      'windows-aarch64-nsis': 'Homeport-1.0.0-windows-arm64-setup.exe',
      'windows-x86_64': 'Homeport-1.0.0-windows-x64-setup.exe',
      'windows-x86_64-msi': 'Homeport-1.0.0-windows-x64.msi',
      'windows-x86_64-nsis': 'Homeport-1.0.0-windows-x64-setup.exe',
    });
  });
});
