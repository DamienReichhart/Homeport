import type { Format, Installer, Os, Release } from '@/core/release';

/** One row per format, with a file per architecture: Apple silicon first on macOS, x64 elsewhere. */
export function formatsFor(
  release: Release | undefined,
  os: Os,
): { format: Format; files: Installer[] }[] {
  const rows = new Map<Format, Installer[]>();
  for (const file of release?.installers.filter((i) => i.os === os) ?? []) {
    rows.set(file.format, [...(rows.get(file.format) ?? []), file]);
  }
  return [...rows].map(([format, files]) => ({
    format,
    files: files.sort((a, b) => {
      const first = os === 'macos' ? 'arm64' : 'x64';
      return a.arch === first ? -1 : b.arch === first ? 1 : 0;
    }),
  }));
}
