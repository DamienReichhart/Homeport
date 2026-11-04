import {
  aurPackage,
  ChannelError,
  files,
  parseSums,
  scoopManifest,
  wingetManifests,
} from './channels-plan';

const hash = (c: string) => c.repeat(64);
const f = files('0.7.0');
const sums = parseSums(
  [
    `${hash('a')}  ${f.windowsX64}`,
    `${hash('b')}  ${f.windowsArm64}`,
    `${hash('c')} *${f.debAmd64}`,
    `${hash('d')}  ${f.debArm64}`,
    `${hash('e')}  Homeport_0.7.0_x64.dmg`,
    `${hash('f')}  ${f.msiX64}`,
    'not a sum line',
  ].join('\n'),
);

describe('winget, Scoop and AUR files', () => {
  it('writes winget manifests for the NSIS installers and the MSI', () => {
    const m = wingetManifests('0.7.0', sums);
    expect(m.version).toContain(
      'PackageIdentifier: DamienReichhart.Homeport\nPackageVersion: 0.7.0',
    );
    expect(m.version).toContain('ManifestType: version');
    expect(m.installer).toContain(
      `InstallerUrl: https://github.com/DamienReichhart/Homeport/releases/download/v0.7.0/${f.windowsArm64}`,
    );
    expect(m.installer).toContain(`InstallerSha256: ${hash('a').toUpperCase()}`);
    expect(m.installer).toContain('InstallerType: wix\n  Scope: machine');
    expect(m.locale).toContain('License: MIT');
    for (const file of [m.version, m.installer, m.locale]) expect(file.endsWith('\n')).toBe(true);
  });

  it('needs the MSI in the checksums for winget', () => {
    const noMsi = new Map([...sums].filter(([file]) => !file.endsWith('.msi')));
    expect(() => wingetManifests('0.7.0', noMsi)).toThrow(ChannelError);
  });

  it('reads SHA256SUMS.txt', () => {
    expect(sums.get(f.debAmd64)).toBe(hash('c'));
    expect(sums.size).toBe(6);
  });

  it('writes a Scoop manifest for both NSIS installers', () => {
    const manifest = JSON.parse(scoopManifest('0.7.0', sums));
    expect(manifest.version).toBe('0.7.0');
    expect(manifest.architecture['64bit']).toEqual({
      url: `https://github.com/DamienReichhart/Homeport/releases/download/v0.7.0/${f.windowsX64}`,
      hash: hash('a'),
    });
    expect(manifest.architecture.arm64.hash).toBe(hash('b'));
    expect(manifest.installer.args.at(-1)).toBe('/D=$dir');
    expect(manifest.autoupdate.architecture['64bit'].url).toContain(
      'v$version/Homeport_$version_x64-setup.exe',
    );
  });

  it('writes a PKGBUILD and .SRCINFO that agree', () => {
    const { pkgbuild, srcinfo } = aurPackage('0.7.0', sums);
    expect(pkgbuild).toContain('pkgver=0.7.0');
    expect(pkgbuild).toContain(`sha256sums_x86_64=('${hash('c')}')`);
    expect(pkgbuild).toContain(`sha256sums_aarch64=('${hash('d')}')`);
    expect(pkgbuild).toContain('rm -rf "$pkgdir/usr/share/homeport"');
    expect(srcinfo).toContain('\tpkgver = 0.7.0');
    expect(srcinfo).toContain(`\tsha256sums_x86_64 = ${hash('c')}`);
    expect(srcinfo).toContain(
      `\tsource_aarch64 = https://github.com/DamienReichhart/Homeport/releases/download/v0.7.0/${f.debArm64}`,
    );
    expect(srcinfo.trim().endsWith('pkgname = homeport-bin')).toBe(true);
  });

  it('refuses prereleases and missing files', () => {
    expect(() => scoopManifest('0.7.0-beta.1', sums)).toThrow(ChannelError);
    expect(() => aurPackage('0.8.0', sums)).toThrow(/no Homeport_0.8.0_amd64.deb/);
  });
});
