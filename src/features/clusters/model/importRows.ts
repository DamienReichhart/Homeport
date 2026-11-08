import type { ContextChoice, KubeContext } from '@/shared/api';

/** One context of the import list, as the user edits it. */
export interface ContextRow {
  context: KubeContext;
  /** The name its Cluster gets; the context's own name by default. */
  name: string;
  picked: boolean;
}

/**
 * The rows for `contexts`: each named after its context, and the current
 * context picked when it can be imported and isn't added yet.
 */
export function initialRows(contexts: KubeContext[]): ContextRow[] {
  return contexts.map((context) => ({
    context,
    name: context.name,
    picked: context.current && importable(context),
  }));
}

/** A context with no problem that isn't a Cluster yet. */
export const importable = (c: KubeContext) => !c.problem && !c.added;

/** What to import: the picked rows, by context and trimmed name. */
export function choices(rows: ContextRow[]): ContextChoice[] {
  return rows
    .filter((r) => r.picked && importable(r.context))
    .map((r) => ({ context: r.context.name, name: r.name.trim() }));
}

/** Why the picked rows can't be imported as they are, if they can't. */
export function rowsProblem(
  rows: ContextRow[],
  taken: string[],
): { kind: 'nothing' } | { kind: 'empty' } | { kind: 'duplicate' | 'taken'; name: string } | null {
  const picked = choices(rows);
  if (picked.length === 0) return { kind: 'nothing' };
  if (picked.some((c) => c.name === '')) return { kind: 'empty' };
  const seen = new Set<string>();
  for (const { name } of picked) {
    const key = name.toLowerCase();
    if (seen.has(key)) return { kind: 'duplicate', name };
    seen.add(key);
  }
  const used = picked.find((c) => taken.some((t) => t.toLowerCase() === c.name.toLowerCase()));
  return used ? { kind: 'taken', name: used.name } : null;
}
