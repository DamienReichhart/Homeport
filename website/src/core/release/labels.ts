import type { Arch, Format, Os } from './model';

/** How each installer format is named by Tauri's bundler, and what it's for. */
export const FORMATS: Record<Format, { label: string; ext: string; hint: string }> = {
  nsis: { label: 'Installer', ext: '.exe', hint: 'Recommended. Installs for the current user.' },
  msi: { label: 'MSI package', ext: '.msi', hint: 'For managed and silent installs.' },
  dmg: { label: 'Disk image', ext: '.dmg', hint: 'Drag Homeport into Applications.' },
  deb: { label: 'Debian package', ext: '.deb', hint: 'Debian, Ubuntu, Linux Mint, Pop!_OS.' },
  rpm: { label: 'RPM package', ext: '.rpm', hint: 'Fedora, RHEL, Rocky, openSUSE.' },
  appimage: { label: 'AppImage', ext: '.AppImage', hint: 'Any distribution, no install step.' },
};

export function archLabel(os: Os, arch: Arch): string {
  if (os === 'macos') return arch === 'arm64' ? 'Apple silicon' : 'Intel';
  return arch === 'arm64' ? 'ARM64' : 'x64';
}
