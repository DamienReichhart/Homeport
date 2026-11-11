import {
  type AssetKind,
  AssetError,
  assetName,
  collectAssets,
  describeAsset,
  platformOf,
} from './release-assets-plan';

describe('assetName', () => {
  it('names every file by system and processor', () => {
    const name = (os: AssetKind['os'], arch: AssetKind['arch'], kind: AssetKind['kind']) =>
      assetName('1.0.0', { os, arch, kind });
    expect(name('windows', 'x64', 'nsis')).toBe('Homeport-1.0.0-windows-x64-setup.exe');
    expect(name('windows', 'arm64', 'nsis')).toBe('Homeport-1.0.0-windows-arm64-setup.exe');
    expect(name('windows', 'x64', 'msi')).toBe('Homeport-1.0.0-windows-x64.msi');
    expect(name('macos', 'arm64', 'dmg')).toBe('Homeport-1.0.0-macos-arm64.dmg');
    expect(name('macos', 'x64', 'dmg')).toBe('Homeport-1.0.0-macos-x64.dmg');
    expect(name('macos', 'arm64', 'app')).toBe('Homeport-1.0.0-macos-arm64-update.app.tar.gz');
    expect(name('linux', 'x64', 'deb')).toBe('Homeport-1.0.0-linux-x64.deb');
    expect(name('linux', 'arm64', 'rpm')).toBe('Homeport-1.0.0-linux-arm64.rpm');
    expect(name('linux', 'x64', 'appimage')).toBe('Homeport-1.0.0-linux-x64.AppImage');
  });
});

describe('describeAsset', () => {
  it.each([
    ['Homeport-1.0.0-windows-x64-setup.exe', 'windows', 'x64', 'nsis'],
    ['Homeport-1.0.0-windows-arm64-setup.exe', 'windows', 'arm64', 'nsis'],
    ['Homeport-1.0.0-windows-x64.msi', 'windows', 'x64', 'msi'],
    ['Homeport-1.0.0-macos-arm64.dmg', 'macos', 'arm64', 'dmg'],
    ['Homeport-1.0.0-macos-x64.dmg', 'macos', 'x64', 'dmg'],
    ['Homeport-1.0.0-macos-arm64-update.app.tar.gz', 'macos', 'arm64', 'app'],
    ['Homeport-1.0.0-macos-x64-update.app.tar.gz', 'macos', 'x64', 'app'],
    ['Homeport-1.0.0-linux-x64.deb', 'linux', 'x64', 'deb'],
    ['Homeport-1.0.0-linux-arm64.deb', 'linux', 'arm64', 'deb'],
    ['Homeport-1.0.0-linux-x64.rpm', 'linux', 'x64', 'rpm'],
    ['Homeport-1.0.0-linux-arm64.rpm', 'linux', 'arm64', 'rpm'],
    ['Homeport-1.0.0-linux-x64.AppImage', 'linux', 'x64', 'appimage'],
    ['Homeport-1.0.0-linux-arm64.AppImage', 'linux', 'arm64', 'appimage'],
    ['Homeport-1.0.0-beta.1-linux-arm64.AppImage', 'linux', 'arm64', 'appimage'],
  ])('reads %s', (name, os, arch, kind) => {
    expect(describeAsset(name)).toEqual({ os, arch, kind });
  });

  it.each([
    ['Homeport_0.8.0_x64-setup.exe', 'windows', 'x64', 'nsis'],
    ['Homeport_0.8.0_arm64-setup.exe', 'windows', 'arm64', 'nsis'],
    ['Homeport_0.8.0_x64_en-US.msi', 'windows', 'x64', 'msi'],
    ['Homeport_0.8.0_aarch64.dmg', 'macos', 'arm64', 'dmg'],
    ['Homeport_0.8.0_x64.dmg', 'macos', 'x64', 'dmg'],
    ['Homeport_0.8.0_amd64.deb', 'linux', 'x64', 'deb'],
    ['Homeport_0.8.0_arm64.deb', 'linux', 'arm64', 'deb'],
    ['Homeport-0.8.0-1.x86_64.rpm', 'linux', 'x64', 'rpm'],
    ['Homeport-0.8.0-1.aarch64.rpm', 'linux', 'arm64', 'rpm'],
    ['Homeport_0.8.0_amd64.AppImage', 'linux', 'x64', 'appimage'],
    ['Homeport_0.8.0_aarch64.AppImage', 'linux', 'arm64', 'appimage'],
    ['Homeport_0.8.0_x64.app.tar.gz', 'macos', 'x64', 'app'],
    ['Homeport_0.8.0_aarch64.app.tar.gz', 'macos', 'arm64', 'app'],
  ])('reads %s, as Tauri named it up to 0.22', (name, os, arch, kind) => {
    expect(describeAsset(name)).toEqual({ os, arch, kind });
  });

  it.each([
    'SHA256SUMS.txt',
    'Homeport_0.8.0_x64-setup.exe.sig',
    'Homeport-1.0.0-linux-x64.AppImage.sig',
    'Homeport-1.0.0-macos-arm64-update.app.tar.gz.sig',
    'Homeport-1.0.0-linux-x64.dmg',
    'Homeport-1.0.0-windows-x64.zip',
  ])('ignores %s', (name) => {
    expect(describeAsset(name)).toBeUndefined();
  });
});

describe('platformOf', () => {
  it('reads the system and processor of each release target', () => {
    expect(platformOf('x86_64-unknown-linux-gnu')).toEqual({ os: 'linux', arch: 'x64' });
    expect(platformOf('aarch64-unknown-linux-gnu')).toEqual({ os: 'linux', arch: 'arm64' });
    expect(platformOf('x86_64-pc-windows-msvc')).toEqual({ os: 'windows', arch: 'x64' });
    expect(platformOf('aarch64-pc-windows-msvc')).toEqual({ os: 'windows', arch: 'arm64' });
    expect(platformOf('aarch64-apple-darwin')).toEqual({ os: 'macos', arch: 'arm64' });
    expect(platformOf('x86_64-apple-darwin')).toEqual({ os: 'macos', arch: 'x64' });
  });

  it('refuses a target it doesn’t know', () => {
    expect(() => platformOf('riscv64gc-unknown-linux-gnu')).toThrow(AssetError);
    expect(() => platformOf('x86_64-unknown-freebsd')).toThrow(AssetError);
  });
});

describe('collectAssets', () => {
  const linux = [
    'deb/Homeport_1.0.0_amd64.deb',
    'deb/Homeport_1.0.0_amd64.deb.sig',
    'rpm/Homeport-1.0.0-1.x86_64.rpm',
    'rpm/Homeport-1.0.0-1.x86_64.rpm.sig',
    'appimage/Homeport_1.0.0_amd64.AppImage',
    'appimage/Homeport_1.0.0_amd64.AppImage.sig',
  ];
  const collect = (target: string, bundles: string, files: string[], version = '1.0.0') =>
    collectAssets({ version, target, bundles: bundles.split(','), files });

  it('renames the Linux packages and their signatures', () => {
    expect(collect('x86_64-unknown-linux-gnu', 'deb,rpm,appimage', linux)).toEqual([
      { from: 'deb/Homeport_1.0.0_amd64.deb', to: 'Homeport-1.0.0-linux-x64.deb' },
      { from: 'deb/Homeport_1.0.0_amd64.deb.sig', to: 'Homeport-1.0.0-linux-x64.deb.sig' },
      { from: 'rpm/Homeport-1.0.0-1.x86_64.rpm', to: 'Homeport-1.0.0-linux-x64.rpm' },
      { from: 'rpm/Homeport-1.0.0-1.x86_64.rpm.sig', to: 'Homeport-1.0.0-linux-x64.rpm.sig' },
      { from: 'appimage/Homeport_1.0.0_amd64.AppImage', to: 'Homeport-1.0.0-linux-x64.AppImage' },
      {
        from: 'appimage/Homeport_1.0.0_amd64.AppImage.sig',
        to: 'Homeport-1.0.0-linux-x64.AppImage.sig',
      },
    ]);
  });

  it('takes the Linux packages without a signature, which only the updater’s files need', () => {
    const files = linux.filter((f) => !/\.(deb|rpm)\.sig$/.test(f));
    expect(collect('x86_64-unknown-linux-gnu', 'deb,rpm,appimage', files).map((c) => c.to)).toEqual(
      [
        'Homeport-1.0.0-linux-x64.deb',
        'Homeport-1.0.0-linux-x64.rpm',
        'Homeport-1.0.0-linux-x64.AppImage',
        'Homeport-1.0.0-linux-x64.AppImage.sig',
      ],
    );
  });

  it('renames the Windows installers', () => {
    const files = [
      'msi/Homeport_1.0.0_x64_en-US.msi',
      'msi/Homeport_1.0.0_x64_en-US.msi.sig',
      'nsis/Homeport_1.0.0_x64-setup.exe',
      'nsis/Homeport_1.0.0_x64-setup.exe.sig',
    ];
    expect(collect('x86_64-pc-windows-msvc', 'msi,nsis', files).map((c) => c.to)).toEqual([
      'Homeport-1.0.0-windows-x64.msi',
      'Homeport-1.0.0-windows-x64.msi.sig',
      'Homeport-1.0.0-windows-x64-setup.exe',
      'Homeport-1.0.0-windows-x64-setup.exe.sig',
    ]);
    const arm = ['nsis/Homeport_1.0.0_arm64-setup.exe', 'nsis/Homeport_1.0.0_arm64-setup.exe.sig'];
    expect(collect('aarch64-pc-windows-msvc', 'nsis', arm).map((c) => c.to)).toEqual([
      'Homeport-1.0.0-windows-arm64-setup.exe',
      'Homeport-1.0.0-windows-arm64-setup.exe.sig',
    ]);
  });

  it('names the zipped macOS app as an updater file, apart from the .dmg', () => {
    const files = [
      'dmg/Homeport_1.0.0_aarch64.dmg',
      'macos/Homeport.app.tar.gz',
      'macos/Homeport.app.tar.gz.sig',
    ];
    expect(collect('aarch64-apple-darwin', 'app,dmg', files)).toEqual([
      { from: 'macos/Homeport.app.tar.gz', to: 'Homeport-1.0.0-macos-arm64-update.app.tar.gz' },
      {
        from: 'macos/Homeport.app.tar.gz.sig',
        to: 'Homeport-1.0.0-macos-arm64-update.app.tar.gz.sig',
      },
      { from: 'dmg/Homeport_1.0.0_aarch64.dmg', to: 'Homeport-1.0.0-macos-arm64.dmg' },
    ]);
    expect(
      collect('x86_64-apple-darwin', 'app,dmg', [
        'dmg/Homeport_1.0.0_x64.dmg',
        'macos/Homeport.app.tar.gz',
        'macos/Homeport.app.tar.gz.sig',
      ]).map((c) => c.to),
    ).toEqual([
      'Homeport-1.0.0-macos-x64-update.app.tar.gz',
      'Homeport-1.0.0-macos-x64-update.app.tar.gz.sig',
      'Homeport-1.0.0-macos-x64.dmg',
    ]);
  });

  it('keeps a prerelease’s version as it is', () => {
    const files = [
      'nsis/Homeport_1.0.0-beta.1_x64-setup.exe',
      'nsis/Homeport_1.0.0-beta.1_x64-setup.exe.sig',
    ];
    const [exe] = collect('x86_64-pc-windows-msvc', 'nsis', files, '1.0.0-beta.1');
    expect(exe!.to).toBe('Homeport-1.0.0-beta.1-windows-x64-setup.exe');
    expect(describeAsset(exe!.to)).toEqual({ os: 'windows', arch: 'x64', kind: 'nsis' });
  });

  it('fails when a bundle wasn’t built', () => {
    const files = linux.filter((f) => !f.startsWith('rpm/'));
    expect(() => collect('x86_64-unknown-linux-gnu', 'deb,rpm,appimage', files)).toThrow(
      /No rpm\/\*\.rpm was built/,
    );
    expect(() => collect('aarch64-apple-darwin', 'app,dmg', ['dmg/Homeport.dmg'])).toThrow(
      /No macos\/\*\.app\.tar\.gz/,
    );
  });

  it('fails when an updater file has no signature', () => {
    const files = linux.filter((f) => f !== 'appimage/Homeport_1.0.0_amd64.AppImage.sig');
    expect(() => collect('x86_64-unknown-linux-gnu', 'deb,rpm,appimage', files)).toThrow(
      /AppImage has no \.sig/,
    );
  });

  it('fails when two files would get the same name', () => {
    const files = [
      'msi/Homeport_1.0.0_x64_en-US.msi',
      'msi/Homeport_1.0.0_x64_en-US.msi.sig',
      'msi/Homeport_1.0.0_x64_fr-FR.msi',
      'msi/Homeport_1.0.0_x64_fr-FR.msi.sig',
    ];
    expect(() => collect('x86_64-pc-windows-msvc', 'msi', files)).toThrow(
      /would both be named Homeport-1\.0\.0-windows-x64\.msi/,
    );
    expect(() => collect('x86_64-unknown-linux-gnu', 'deb,deb', linux)).toThrow(
      /Two files would be named Homeport-1\.0\.0-linux-x64\.deb/,
    );
  });

  it('fails on a bundle the target can’t build, or doesn’t know', () => {
    expect(() => collect('x86_64-unknown-linux-gnu', 'dmg', linux)).toThrow(AssetError);
    expect(() => collect('x86_64-unknown-linux-gnu', 'snap', linux)).toThrow(/Unknown bundle/);
  });
});
