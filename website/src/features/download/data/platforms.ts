import type { Os } from '@/core/release';

export interface PlatformInfo {
  os: Os;
  title: string;
  icon: 'windows' | 'apple' | 'linux';
  requirement: string;
}

/** The download page's platforms, in the order they show when the visitor's isn't known. */
export const PLATFORMS: PlatformInfo[] = [
  { os: 'windows', title: 'Windows', icon: 'windows', requirement: 'Windows 10 or 11' },
  { os: 'macos', title: 'macOS', icon: 'apple', requirement: 'macOS 10.15 or later' },
  {
    os: 'linux',
    title: 'Linux',
    icon: 'linux',
    requirement: 'Ubuntu 22.04, Debian 12, Fedora 36 or newer',
  },
];
