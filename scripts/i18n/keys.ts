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

const RUST_TEST_FILE = /(^|\/)(tests|[a-z_]+_tests)\.rs$|\/test_support\/|\/widgets\/gallery\//;
const RUST_CALL = /\b(?:t|rich)!\(\s*"([^"]+)"/g;
const RUST_CONST = /\bconst\s+[A-Z0-9_]+\s*:[^=]*&(?:'static\s+)?str[^=]*=\s*([^;]*);/g;
const RUST_STRING = /"([^"\\]*)"/g;

/**
 * The keys the iced app really reads: the first argument of a `t!` or
 * `rich!` call and the strings of a `const` of `&str` (a table of keys, as
 * the size units), in code that ships. A key in a comment, in a test or in
 * the widget gallery names a message without showing it, so it doesn't make
 * the message used.
 */
export function rustKeys(sources: Source[]): string[] {
  const keys = new Set<string>();
  for (const source of sources) {
    if (RUST_TEST_FILE.test(source.path)) continue;
    // Tests sit at the end of a file, after `#[cfg(test)]`.
    const shipped = (source.text.split(/^\s*#\[cfg\(test\)\]/m)[0] ?? '').replace(
      /^\s*\/\/.*$/gm,
      '',
    );
    for (const match of shipped.matchAll(RUST_CALL)) keys.add(match[1] ?? '');
    for (const table of shipped.matchAll(RUST_CONST)) {
      for (const match of (table[1] ?? '').matchAll(RUST_STRING)) keys.add(match[1] ?? '');
    }
  }
  return [...keys];
}
