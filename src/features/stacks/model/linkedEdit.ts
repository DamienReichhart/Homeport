import type { LinkedFile, LinkedFiles } from '@/shared/api';

/** The edited text of each compose file, by path. */
export type Drafts = Record<string, string>;

/** The text a file shows: its draft, or what was read. */
export const textOf = (file: LinkedFile, drafts: Drafts) => drafts[file.path] ?? file.content;

/** The files whose draft differs from what the edit started from. */
export function dirtyFiles(base: LinkedFiles, drafts: Drafts): LinkedFile[] {
  return base.files.filter((f) => textOf(f, drafts) !== f.content);
}

/**
 * Whether the files on disk differ from what Homeport last read or wrote, or
 * from what this edit started from: either means saving could overwrite
 * someone else's change.
 */
export function changedOutside(base: LinkedFiles, current: LinkedFiles | undefined): boolean {
  if (!current) return base.changedOutside;
  if (current.changedOutside) return true;
  return current.files.some(
    (f) => base.files.find((b) => b.path === f.path)?.fingerprint !== f.fingerprint,
  );
}

/** "compose.prod.yaml" from its full path, on any system. */
export const fileName = (path: string) => path.split(/[\\/]/).pop() ?? path;
