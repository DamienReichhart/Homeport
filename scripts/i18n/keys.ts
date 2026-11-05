/**
 * The checks behind `bun run i18n:check` (ADR 0011): every key a source file
 * names exists in the English catalogue, every catalogue key is used, keys
 * sit in the namespace file named after their prefix, and no key is built at
 * runtime, which the checks couldn't follow.
 */

/** A catalogue namespace: its name and keys. */
export interface Namespace {
  name: string;
  keys: string[];
}

/** A source file's path and text. */
export interface Source {
  path: string;
  text: string;
}

const KEY = /(['"`])([a-z][a-zA-Z0-9]*(?:\.[a-zA-Z0-9]+)+)\1/g;
const DYNAMIC = /\b(?:t|tRich)\(\s*`[^`]*\$\{/g;
/** File names such as `images.tar` look like keys; they aren't. */
const FILE = /\.(tar|gz|tgz|xz|txt|json|ya?ml|ts|tsx|js|png|svg|log|conf)$/;

/** Problems found, one sentence each; empty when the catalogue and the code agree. */
export function checkKeys(namespaces: Namespace[], sources: Source[]): string[] {
  const problems: string[] = [];
  const names = new Set(namespaces.map((n) => n.name));
  const known = new Set<string>();
  for (const ns of namespaces) {
    for (const key of ns.keys) {
      if (!key.startsWith(`${ns.name}.`))
        problems.push(`${key} is in ${ns.name}.ts but doesn't start with "${ns.name}."`);
      known.add(key);
    }
  }
  const used = new Set<string>();
  for (const source of sources) {
    for (const match of source.text.matchAll(KEY)) {
      const key = match[2] ?? '';
      const prefix = key.split('.')[0] ?? '';
      if (!names.has(prefix) || FILE.test(key)) continue;
      used.add(key);
      if (!known.has(key)) problems.push(`${source.path} uses ${key}, which the catalogue lacks`);
    }
    for (const match of source.text.matchAll(DYNAMIC)) {
      problems.push(
        `${source.path} builds a key at runtime (${match[0]}…): use literal keys, for example in a map`,
      );
    }
  }
  for (const key of known) {
    if (!used.has(key)) problems.push(`${key} is in the catalogue but nothing uses it`);
  }
  return problems;
}
