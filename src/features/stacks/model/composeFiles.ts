import type { ComposeFile } from '@/shared/api';

/** The names offered for a new file, in order, before numbered ones. */
const SUGGESTED = ['compose.override.yaml', 'compose.prod.yaml', 'compose.dev.yaml'];

/** A name no file has yet: a common override name, else `compose.<n>.yaml`. */
export function freeName(files: ComposeFile[]): string {
  const taken = new Set(files.map((f) => f.name));
  const suggested = SUGGESTED.find((n) => !taken.has(n));
  if (suggested) return suggested;
  let n = 2;
  while (taken.has(`compose.${n}.yaml`)) n++;
  return `compose.${n}.yaml`;
}

/** Adds an empty override file at the end. */
export const addFile = (files: ComposeFile[]): ComposeFile[] => [
  ...files,
  { name: freeName(files), content: 'services:\n' },
];

/** Replaces the file at `index` with `change` applied. */
export const updateFile = (
  files: ComposeFile[],
  index: number,
  change: Partial<ComposeFile>,
): ComposeFile[] => files.map((f, i) => (i === index ? { ...f, ...change } : f));

/** Removes the file at `index`; the first file, the base, stays. */
export const removeFile = (files: ComposeFile[], index: number): ComposeFile[] =>
  index === 0 ? files : files.filter((_, i) => i !== index);

/** Moves the item at `index` by `delta` places, when that stays in the list. */
export function move<T>(items: T[], index: number, delta: number): T[] {
  const to = index + delta;
  if (to < 0 || to >= items.length) return items;
  const next = [...items];
  const [item] = next.splice(index, 1);
  if (item !== undefined) next.splice(to, 0, item);
  return next;
}

/** Whether two definitions have the same files, in the same order. */
export const sameFiles = (a: ComposeFile[], b: ComposeFile[]) =>
  a.length === b.length && a.every((f, i) => f.name === b[i]?.name && f.content === b[i]?.content);

/** The text of the files, in order, for the checks. */
export const contents = (files: ComposeFile[]) => files.map((f) => f.content);
