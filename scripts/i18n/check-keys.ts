// `bun run i18n:check`: the English catalogue and the code name the same keys,
// and every other language has every English key with the same arguments.
// `bun run i18n:check --locale fr` checks one language only.
import { existsSync, readdirSync, readFileSync, statSync } from 'node:fs';
import { join, relative } from 'node:path';
import { checkKeys, type Namespace, type Source } from './keys';
import { compareMessages, type Messages } from './locales';

const root = join(import.meta.dir, '..', '..');
const i18n = join(root, 'src', 'shared', 'i18n');
const catalogue = join(i18n, 'en');
const only = process.argv.includes('--locale')
  ? process.argv[process.argv.indexOf('--locale') + 1]
  : undefined;

/** A namespace file's messages: a `.ts` default export or a generated `.json`. */
async function load(path: string): Promise<Messages> {
  if (path.endsWith('.json')) return JSON.parse(readFileSync(path, 'utf8')) as Messages;
  return ((await import(path)) as { default: Messages }).default;
}

/** The namespace files of a language folder, by file name. */
const files = (dir: string) =>
  readdirSync(dir).filter(
    (f) => /^[a-z]+\.(ts|json)$/.test(f) && !['index.ts', 'meta.ts'].includes(f),
  );

const english = new Map<string, Messages>();
for (const file of files(catalogue)) english.set(file, await load(join(catalogue, file)));

const problems: string[] = [];

if (!only) {
  const namespaces: Namespace[] = [];
  for (const [file, messages] of english) {
    const match = /^([a-z]+)\.ts$/.exec(file);
    if (match) namespaces.push({ name: match[1] ?? '', keys: Object.keys(messages) });
  }

  const walk = (dir: string, out: Source[], test: RegExp) => {
    for (const entry of readdirSync(dir)) {
      const path = join(dir, entry);
      if (statSync(path).isDirectory()) {
        if (path !== i18n && entry !== 'target') walk(path, out, test);
      } else if (test.test(entry) && !/\.test\.tsx?$/.test(entry)) {
        out.push({ path: relative(root, path), text: readFileSync(path, 'utf8') });
      }
    }
    return out;
  };
  const sources = walk(join(root, 'src'), [], /\.(ts|tsx)$/);
  // The iced app reads the same keys (`t!("ui.copy")`), some of them before
  // the React app does: a key only Rust names is used too. Whether Rust
  // names a key that exists is `app/src/i18n/check.rs`'s to say, so only
  // the keys of the catalogue count here.
  const rust = walk(join(root, 'app', 'src'), [], /\.rs$/)
    .map((source) => source.text)
    .join('\n');
  const named = namespaces.flatMap((ns) => ns.keys).filter((key) => rust.includes(`"${key}"`));
  sources.push({ path: 'app/src', text: named.map((key) => `'${key}'`).join('\n') });
  problems.push(...checkKeys(namespaces, sources));
}

const locales = readdirSync(i18n).filter(
  (d) => d !== 'en' && statSync(join(i18n, d)).isDirectory() && (!only || d === only),
);
if (only && locales.length === 0) problems.push(`src/shared/i18n/${only} doesn't exist`);
for (const locale of locales) {
  const dir = join(i18n, locale);
  if (!existsSync(join(dir, 'meta.ts'))) problems.push(`${locale}/meta.ts is missing`);
  for (const [file, messages] of english) {
    const path = join(dir, file);
    if (!existsSync(path)) {
      problems.push(`${locale}/${file} is missing (${Object.keys(messages).length} messages)`);
      continue;
    }
    problems.push(...compareMessages(locale, file, messages, await load(path)));
  }
  for (const file of files(dir)) {
    if (!english.has(file)) problems.push(`${locale}/${file} has no English file`);
  }
}

if (problems.length > 0) {
  console.error(problems.join('\n'));
  console.error(`\n${problems.length} catalogue problem(s); see ADR 0011.`);
  process.exit(1);
}
console.log(
  only
    ? `${only} has every English message.`
    : `The catalogues and the code agree (${english.size} namespaces, ${locales.length + 1} languages).`,
);
