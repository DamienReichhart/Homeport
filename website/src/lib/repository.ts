/**
 * The Linux package repositories and the commands that set them up, shared by
 * the download page and the docs. The repositories live on this site at a
 * permanent URL (ADR 0003), independent of where a preview build is served.
 */
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { createHash } from 'node:crypto';

export const REPOSITORY_URL = 'https://damienreichhart.github.io/Homeport';
export const KEY_URL = `${REPOSITORY_URL}/homeport.asc`;
export const INSTALL_SCRIPT_URL = `${REPOSITORY_URL}/install.sh`;
export const INSTALL_COMMAND = `curl -fsSL ${INSTALL_SCRIPT_URL} | sh`;

export interface ManualSetup {
  id: 'apt' | 'dnf' | 'zypper';
  label: string;
  distributions: string;
  commands: string;
}

/** What `install.sh` does, by hand: the same files at the same paths. */
export const MANUAL_SETUP: ManualSetup[] = [
  {
    id: 'apt',
    label: 'Debian, Ubuntu',
    distributions: 'Debian 12, Ubuntu 22.04, Linux Mint 21, Pop!_OS 22.04 or newer',
    commands: [
      'sudo install -d -m 0755 /etc/apt/keyrings',
      `curl -fsSL ${KEY_URL} | sudo tee /etc/apt/keyrings/homeport.asc > /dev/null`,
      `curl -fsSL ${REPOSITORY_URL}/apt/homeport.sources | sudo tee /etc/apt/sources.list.d/homeport.sources > /dev/null`,
      'sudo apt update',
      'sudo apt install homeport',
    ].join('\n'),
  },
  {
    id: 'dnf',
    label: 'Fedora, RHEL',
    distributions: 'Fedora 36, RHEL 10, Rocky Linux and AlmaLinux 10 or newer',
    commands: [
      `sudo rpm --import ${KEY_URL}`,
      `curl -fsSL ${REPOSITORY_URL}/rpm/homeport.repo | sudo tee /etc/yum.repos.d/homeport.repo > /dev/null`,
      'sudo dnf install homeport',
    ].join('\n'),
  },
  {
    id: 'zypper',
    label: 'openSUSE',
    distributions: 'openSUSE Tumbleweed and Leap 16',
    commands: [
      `sudo rpm --import ${KEY_URL}`,
      `curl -fsSL ${REPOSITORY_URL}/rpm/homeport.repo | sudo tee /etc/zypp/repos.d/homeport.repo > /dev/null`,
      'sudo zypper refresh',
      'sudo zypper install homeport',
    ].join('\n'),
  },
];

/** How each package manager updates Homeport, as the app shows it too. */
export const UPDATE_COMMANDS: Record<ManualSetup['id'], string> = {
  apt: 'sudo apt update && sudo apt install --only-upgrade homeport',
  dnf: 'sudo dnf upgrade --refresh homeport',
  zypper: 'sudo zypper refresh && sudo zypper update homeport',
};

/** Installers the in-app updater doesn't cover yet, each with its tracking issue. */
export const COMING_LATER: { name: string; what: string; issue: number }[] = [
  { name: 'Homebrew', what: 'brew install --cask homeport, on macOS', issue: 5 },
  { name: 'winget', what: 'winget install, on Windows', issue: 6 },
  { name: 'Scoop', what: 'a Homeport bucket, on Windows', issue: 7 },
  { name: 'AUR', what: 'homeport-bin, on Arch Linux', issue: 8 },
  { name: 'Flathub', what: 'a Flatpak, on any Linux distribution', issue: 9 },
  { name: 'Snap', what: 'the Snap Store, on Ubuntu and others', issue: 10 },
];

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
