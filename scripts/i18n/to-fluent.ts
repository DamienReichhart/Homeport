// `bun run i18n:fluent`: writes the iced app's Fluent catalogues
// (`app/i18n/<language>/<namespace>.ftl`) from the formatjs ones
// (`src/shared/i18n`), which stay the source until the React app is removed.
// `--check` fails when a file on disk isn't what would be written.
//
// It also writes `app/tests/fixtures/i18n-golden.json`: what formatjs renders
// for every message with a plural or a select, with sample arguments, so the
// Rust test can hold the Fluent catalogues to the same output.
import {
  existsSync,
  mkdirSync,
  readdirSync,
  readFileSync,
  statSync,
  unlinkSync,
  writeFileSync,
} from 'node:fs';
import { dirname, join } from 'node:path';
import { IntlMessageFormat } from 'intl-messageformat';
import { parse, TYPE, type MessageFormatElement } from '@formatjs/icu-messageformat-parser';
import { fluentFile } from './fluent/convert';

type Messages = Record<string, string>;
interface Meta {
  name: string;
  dir: 'ltr' | 'rtl';
}
/** One rendering: `[key, arguments, text]`. */
type Sample = [string, Record<string, string | number>, string];

const root = join(import.meta.dir, '..', '..');
const source = join(root, 'src', 'shared', 'i18n');
const target = join(root, 'app', 'i18n');
const golden = join(root, 'app', 'tests', 'fixtures', 'i18n-golden.json');
const check = process.argv.includes('--check');

async function load<T>(path: string): Promise<T> {
  if (path.endsWith('.json')) return JSON.parse(readFileSync(path, 'utf8')) as T;
  return ((await import(path)) as { default: T }).default;
}

const namespaces = (dir: string) =>
  readdirSync(dir)
    .filter((f) => /^[a-z]+\.(ts|json)$/.test(f) && !['index.ts', 'meta.ts'].includes(f))
    .sort();

/** The counts that reach each plural category of `locale`, and zero and one. */
function counts(locale: string): number[] {
  const rules = new Intl.PluralRules(locale);
  const first = new Map<string, number>();
  for (let n = 0; n <= 200; n++) if (!first.has(rules.select(n))) first.set(rules.select(n), n);
  return [...new Set([0, 1, ...first.values()])].sort((a, b) => a - b);
}

interface Choice {
  name: string;
  values: (string | number)[];
}

/** What a message can be given: each plural's counts, each select's options. */
function choices(elements: MessageFormatElement[], locale: string, found: Choice[]): Choice[] {
  const add = (name: string, values: (string | number)[]) => {
    const known = found.find((c) => c.name === name);
    if (known) known.values = [...new Set([...known.values, ...values])];
    else found.push({ name, values });
  };
  for (const el of elements) {
    if (el.type === TYPE.argument) add(el.value, []);
    else if (el.type === TYPE.plural) {
      const exact = Object.keys(el.options).flatMap((o) =>
        o.startsWith('=') ? [+o.slice(1)] : [],
      );
      add(el.value, [...counts(locale), ...exact]);
    } else if (el.type === TYPE.select) {
      add(el.value, [...Object.keys(el.options).filter((o) => o !== 'other'), 'somethingElse']);
    }
    if (el.type === TYPE.plural || el.type === TYPE.select)
      for (const option of Object.values(el.options)) choices(option.value, locale, found);
    if (el.type === TYPE.tag) choices(el.children, locale, found);
  }
  return found;
}

/** Renderings of `message`: each value of each choice once, the others at their first. */
function samples(locale: string, key: string, message: string): Sample[] {
  const ast = parse(message, { requiresOtherClause: true });
  const found = choices(ast, locale, []);
  // Text that converts to itself needs no witness; anything with an
  // argument, a tag, a quote or an edge the converter rewrites does.
  if (found.length === 0 && !/['{}<>\n\r]|^\s|\s$/.test(message)) return [];
  const base = Object.fromEntries(found.map((c) => [c.name, c.values[0] ?? `<${c.name}>`]));
  // Tags render as themselves, as the Fluent text keeps them.
  const tags = new Set<string>();
  const collect = (elements: MessageFormatElement[]) => {
    for (const el of elements) {
      if (el.type === TYPE.tag) {
        tags.add(el.value);
        collect(el.children);
      }
      if (el.type === TYPE.plural || el.type === TYPE.select)
        for (const option of Object.values(el.options)) collect(option.value);
    }
  };
  collect(ast);
  const tagValues = Object.fromEntries(
    [...tags].map((tag) => [tag, (chunks: string[]) => `<${tag}>${chunks.join('')}</${tag}>`]),
  );
  const format = new IntlMessageFormat(ast, locale);
  if (!found.some((c) => c.values.length > 0))
    return [[key, base, String(format.format({ ...base, ...tagValues }))]];
  const out: Sample[] = [];
  const seen = new Set<string>();
  for (const choice of found) {
    for (const value of choice.values) {
      const args = { ...base, [choice.name]: value };
      const id = JSON.stringify(args);
      if (seen.has(id)) continue;
      seen.add(id);
      out.push([key, args, String(format.format({ ...args, ...tagValues }))]);
    }
  }
  return out;
}

const stale: string[] = [];
/** Every file this run writes, to tell the ones a removed namespace left. */
const written = new Set<string>();
function write(path: string, text: string) {
  written.add(path);
  if (check) {
    if (!existsSync(path) || readFileSync(path, 'utf8') !== text)
      stale.push(path.slice(root.length + 1));
    return;
  }
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, text);
}

const locales = readdirSync(source)
  .filter((d) => statSync(join(source, d)).isDirectory())
  .sort();
const rendered: Record<string, Sample[]> = {};
let total = 0;
for (const locale of locales) {
  const meta = await load<Meta>(join(source, locale, 'meta.ts'));
  write(
    join(target, locale, 'meta.ftl'),
    fluentFile(`What the app needs to know about this language (${locale}).`, {
      'meta.name': meta.name,
      'meta.dir': meta.dir,
    }),
  );
  rendered[locale] = [];
  for (const file of namespaces(join(source, locale))) {
    const messages = await load<Messages>(join(source, locale, file));
    const namespace = file.replace(/\.(ts|json)$/, '');
    const header = [
      `The \`${namespace}\` namespace, in ${meta.name} (${locale}).`,
      '',
      `Written by \`bun run i18n:fluent\` from src/shared/i18n/${locale}/${file}:`,
      'change the text there, then run it again.',
    ].join('\n');
    try {
      write(join(target, locale, `${namespace}.ftl`), fluentFile(header, messages));
    } catch (e) {
      throw new Error(`${locale}/${file}: ${e instanceof Error ? e.message : String(e)}`, {
        cause: e,
      });
    }
    for (const [key, message] of Object.entries(messages))
      rendered[locale]?.push(...samples(locale, key, message));
    total += Object.keys(messages).length;
  }
}
write(golden, JSON.stringify(rendered) + '\n');

// A namespace or a language removed from the source leaves its files here,
// which the app would still build in.
const walk = (dir: string): string[] =>
  existsSync(dir)
    ? readdirSync(dir).flatMap((entry) => {
        const path = join(dir, entry);
        return statSync(path).isDirectory() ? walk(path) : [path];
      })
    : [];
for (const path of walk(target)) {
  if (written.has(path)) continue;
  if (check) stale.push(`${path.slice(root.length + 1)} (no longer written: remove it)`);
  else unlinkSync(path);
}

if (stale.length > 0) {
  console.error(stale.join('\n'));
  console.error(`\n${stale.length} Fluent file(s) are out of date; run \`bun run i18n:fluent\`.`);
  process.exit(1);
}
console.log(
  `${check ? 'The Fluent catalogues are up to date' : 'Wrote the Fluent catalogues'} (${locales.length} languages, ${total} messages).`,
);
