import { describe, expect, test } from 'bun:test';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { fingerprintOf } from './signing-key';

const key = readFileSync(join(import.meta.dir, '../../../../packaging/linux/homeport.asc'), 'utf8');

describe('fingerprintOf', () => {
  test('matches gpg for the key the packages ship', () => {
    // gpg --show-keys --with-fingerprint packaging/linux/homeport.asc
    expect(fingerprintOf(key)).toBe('41FA 6740 4722 C21D A7E7 4372 4612 C888 1F62 39EC');
  });

  test('reads a key with Windows line endings', () => {
    expect(fingerprintOf(key.replace(/\n/g, '\r\n'))).toBe(fingerprintOf(key));
  });

  test('rejects text that is not an armored public key', () => {
    expect(() => fingerprintOf('hello')).toThrow('Not an armored public key');
  });
});
