import { describe, expect, test } from 'bun:test';
import { classify, parseChecksums } from './model';

describe('classify', () => {
  // Every installer of a release, named by system and processor.
  test.each([
    ['Homeport-1.0.0-windows-x64-setup.exe', { os: 'windows', arch: 'x64', format: 'nsis' }],
    ['Homeport-1.0.0-windows-arm64-setup.exe', { os: 'windows', arch: 'arm64', format: 'nsis' }],
    ['Homeport-1.0.0-windows-x64.msi', { os: 'windows', arch: 'x64', format: 'msi' }],
    ['Homeport-1.0.0-macos-arm64.dmg', { os: 'macos', arch: 'arm64', format: 'dmg' }],
    ['Homeport-1.0.0-macos-x64.dmg', { os: 'macos', arch: 'x64', format: 'dmg' }],
    ['Homeport-1.0.0-linux-x64.deb', { os: 'linux', arch: 'x64', format: 'deb' }],
    ['Homeport-1.0.0-linux-arm64.deb', { os: 'linux', arch: 'arm64', format: 'deb' }],
    ['Homeport-1.0.0-linux-x64.rpm', { os: 'linux', arch: 'x64', format: 'rpm' }],
    ['Homeport-1.0.0-linux-arm64.rpm', { os: 'linux', arch: 'arm64', format: 'rpm' }],
    ['Homeport-1.0.0-linux-x64.AppImage', { os: 'linux', arch: 'x64', format: 'appimage' }],
    ['Homeport-1.0.0-linux-arm64.AppImage', { os: 'linux', arch: 'arm64', format: 'appimage' }],
    ['Homeport-1.0.0-beta.1-macos-arm64.dmg', { os: 'macos', arch: 'arm64', format: 'dmg' }],
    ['Homeport-1.0.0-beta.1-windows-x64-setup.exe', { os: 'windows', arch: 'x64', format: 'nsis' }],
  ])('recognises %s', (name, kind) => {
    expect(classify(name)).toEqual(kind as ReturnType<typeof classify>);
  });

  // Every installer of v0.8.0, as Tauri named the files up to 0.22.
  test.each([
    ['Homeport_0.8.0_x64-setup.exe', { os: 'windows', arch: 'x64', format: 'nsis' }],
    ['Homeport_0.8.0_arm64-setup.exe', { os: 'windows', arch: 'arm64', format: 'nsis' }],
    ['Homeport_0.8.0_x64_en-US.msi', { os: 'windows', arch: 'x64', format: 'msi' }],
    ['Homeport_0.8.0_aarch64.dmg', { os: 'macos', arch: 'arm64', format: 'dmg' }],
    ['Homeport_0.8.0_x64.dmg', { os: 'macos', arch: 'x64', format: 'dmg' }],
    ['Homeport_0.8.0_amd64.deb', { os: 'linux', arch: 'x64', format: 'deb' }],
    ['Homeport_0.8.0_arm64.deb', { os: 'linux', arch: 'arm64', format: 'deb' }],
    ['Homeport-0.8.0-1.x86_64.rpm', { os: 'linux', arch: 'x64', format: 'rpm' }],
    ['Homeport-0.8.0-1.aarch64.rpm', { os: 'linux', arch: 'arm64', format: 'rpm' }],
    ['Homeport_0.8.0_amd64.AppImage', { os: 'linux', arch: 'x64', format: 'appimage' }],
    ['Homeport_0.8.0_aarch64.AppImage', { os: 'linux', arch: 'arm64', format: 'appimage' }],
  ])('recognises %s, from an older release', (name, kind) => {
    expect(classify(name)).toEqual(kind as ReturnType<typeof classify>);
  });

  // The in-app updater's files are never offered as downloads.
  test.each([
    'Homeport-1.0.0-macos-arm64-update.app.tar.gz',
    'Homeport-1.0.0-macos-x64-update.app.tar.gz',
    'Homeport-1.0.0-macos-arm64-update.app.tar.gz.sig',
    'Homeport-1.0.0-windows-x64-setup.exe.sig',
    'Homeport-1.0.0-windows-arm64-setup.exe.sig',
    'Homeport-1.0.0-windows-x64.msi.sig',
    'Homeport-1.0.0-linux-x64.deb.sig',
    'Homeport-1.0.0-linux-arm64.rpm.sig',
    'Homeport-1.0.0-linux-x64.AppImage.sig',
    'Homeport_0.8.0_x64.app.tar.gz',
    'Homeport_0.8.0_aarch64.app.tar.gz',
    'Homeport_0.8.0_x64.app.tar.gz.sig',
    'Homeport_0.8.0_aarch64.app.tar.gz.sig',
    'Homeport_0.8.0_x64-setup.exe.sig',
    'Homeport_0.8.0_arm64-setup.exe.sig',
    'Homeport_0.8.0_x64_en-US.msi.sig',
    'Homeport_0.8.0_amd64.deb.sig',
    'Homeport_0.8.0_arm64.deb.sig',
    'Homeport-0.8.0-1.x86_64.rpm.sig',
    'Homeport-0.8.0-1.aarch64.rpm.sig',
    'Homeport_0.8.0_amd64.AppImage.sig',
    'Homeport_0.8.0_aarch64.AppImage.sig',
  ])('keeps the updater file %s out of the downloads', (name) => {
    expect(classify(name)).toBeUndefined();
  });

  test.each([
    'SHA256SUMS.txt',
    'latest.json',
    'Homeport.exe.sig',
    'Homeport-1.0.0-linux-x64.dmg',
    'Homeport-1.0.0-macos-arm64.zip',
  ])('ignores %s', (name) => {
    expect(classify(name)).toBeUndefined();
  });
});

describe('parseChecksums', () => {
  const a = 'a'.repeat(64);
  const b = 'B'.repeat(64);

  test('reads sha256sum lines, in text and binary mode', () => {
    const sums = parseChecksums(`${a}  first.dmg\n${b} *second.exe\n`);
    expect([...sums]).toEqual([
      ['first.dmg', a],
      ['second.exe', 'b'.repeat(64)],
    ]);
  });

  test('skips blank and malformed lines', () => {
    const sums = parseChecksums(
      `\n  \nnot a checksum\n${'c'.repeat(63)}  short.deb\n${a}  ok.deb\n`,
    );
    expect([...sums.keys()]).toEqual(['ok.deb']);
  });
});
