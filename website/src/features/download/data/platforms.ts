import type { StringKey } from '@/core/i18n';
import type { Os } from '@/core/release';

export interface PlatformInfo {
  os: Os;
  title: string;
  icon: 'windows' | 'apple' | 'linux';
  /** The system it needs, as a key of the page's strings. */
  requirement: StringKey;
}

/** The download page's platforms, in the order they show when the visitor's isn't known. */
export const PLATFORMS: PlatformInfo[] = [
  { os: 'windows', title: 'Windows', icon: 'windows', requirement: 'download.windows.requirement' },
  { os: 'macos', title: 'macOS', icon: 'apple', requirement: 'download.macos.requirement' },
  { os: 'linux', title: 'Linux', icon: 'linux', requirement: 'download.linux.requirement' },
];
