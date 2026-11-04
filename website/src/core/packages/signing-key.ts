import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';

/**
 * The fingerprint of the repositories' signing key, read from the public key
 * the packages ship (`packaging/linux/homeport.asc`), as gpg prints it:
 * ten groups of four hex digits. `undefined` before the key is committed.
 */
export function signingKeyFingerprint(): string | undefined {
  let armored: string;
  try {
    // Builds run from website/.
    armored = readFileSync(join(process.cwd(), '../packaging/linux/homeport.asc'), 'utf8');
  } catch {
    return undefined;
  }
  return fingerprintOf(armored);
}

/** The v4 fingerprint of the first public key in an ASCII-armored key (RFC 4880, 12.2). */
export function fingerprintOf(armored: string): string {
  const lines = armored.split(/\r?\n/);
  const start = lines.findIndex((l) => l.startsWith('-----BEGIN PGP PUBLIC KEY BLOCK-----'));
  if (start < 0) throw new Error('Not an armored public key');
  const body: string[] = [];
  let inHeaders = true;
  for (const line of lines.slice(start + 1)) {
    if (inHeaders) {
      if (line.trim() === '') inHeaders = false;
      else if (!line.includes(':')) {
        inHeaders = false;
        body.push(line);
      }
      continue;
    }
    if (line.startsWith('=') || line.startsWith('-----')) break;
    body.push(line.trim());
  }
  const data = Buffer.from(body.join(''), 'base64');

  // The first packet must be a public key (tag 6), old or new format.
  const header = data[0]!;
  let tag: number;
  let length: number;
  let offset: number;
  if (header & 0x40) {
    tag = header & 0x3f;
    const first = data[1]!;
    if (first < 192) [length, offset] = [first, 2];
    else if (first < 224) [length, offset] = [((first - 192) << 8) + data[2]! + 192, 3];
    else [length, offset] = [data.readUInt32BE(2), 6];
  } else {
    tag = (header >> 2) & 0x0f;
    const type = header & 0x03;
    if (type === 0) [length, offset] = [data[1]!, 2];
    else if (type === 1) [length, offset] = [data.readUInt16BE(1), 3];
    else [length, offset] = [data.readUInt32BE(1), 5];
  }
  if (tag !== 6) throw new Error('The key block doesn’t start with a public key');
  const packet = data.subarray(offset, offset + length);
  if (packet[0] !== 4) throw new Error(`Unsupported key version ${packet[0]}`);

  const prefix = Buffer.from([0x99, (length >> 8) & 0xff, length & 0xff]);
  const hex = createHash('sha1').update(prefix).update(packet).digest('hex').toUpperCase();
  return hex.match(/.{4}/g)!.join(' ');
}
