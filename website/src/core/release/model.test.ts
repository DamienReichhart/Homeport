import { describe, expect, test } from 'bun:test';
import { classify, parseChecksums } from './model';

describe('classify', () => {
  test.each([
    ['Homeport_0.4.0_x64-setup.exe', { os: 'windows', arch: 'x64', format: 'nsis' }],
    ['Homeport_0.4.0_arm64-setup.exe', { os: 'windows', arch: 'arm64', format: 'nsis' }],
    ['Homeport_0.4.0_x64_en-US.msi', { os: 'windows', arch: 'x64', format: 'msi' }],
    ['Homeport_0.4.0_aarch64.dmg', { os: 'macos', arch: 'arm64', format: 'dmg' }],
    ['Homeport_0.4.0_x64.dmg', { os: 'macos', arch: 'x64', format: 'dmg' }],
    ['Homeport_0.4.0_amd64.deb', { os: 'linux', arch: 'x64', format: 'deb' }],
    ['Homeport_0.4.0_arm64.deb', { os: 'linux', arch: 'arm64', format: 'deb' }],
    ['Homeport-0.4.0-1.aarch64.rpm', { os: 'linux', arch: 'arm64', format: 'rpm' }],
    ['Homeport_0.4.0_amd64.AppImage', { os: 'linux', arch: 'x64', format: 'appimage' }],
  ])('recognises %s', (name, kind) => {
    expect(classify(name)).toEqual(kind as ReturnType<typeof classify>);
  });

  test.each(['SHA256SUMS.txt', 'latest.json', 'Homeport_0.4.0_x64.app.tar.gz', 'Homeport.exe.sig'])(
    'ignores %s',
    (name) => {
      expect(classify(name)).toBeUndefined();
    },
  );
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
