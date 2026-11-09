/**
 * The cross-language half of `bun run i18n:check` (ADR 0011): every
 * language's catalogue has exactly the English keys, each message parses as
 * ICU, keeps the English message's arguments and tags, and gives every plural
 * the categories its language needs (Russian `few` and `many`, Arabic `zero`
 * and `two`…), so a new English string can't ship untranslated by mistake.
 */
import { parse, TYPE, type MessageFormatElement } from '@formatjs/icu-messageformat-parser';

/** One namespace file of one language: its messages by key. */
export type Messages = Record<string, string>;

/** What a message names: arguments (plurals and selects included) and tags. */
export interface Shape {
  args: Set<string>;
  tags: Set<string>;
  /** The categories of each plural, by its argument. */
  plurals: Map<string, Set<string>>;
}

function walk(elements: MessageFormatElement[], shape: Shape) {
  for (const el of elements) {
    switch (el.type) {
      case TYPE.argument:
      case TYPE.number:
      case TYPE.date:
      case TYPE.time:
        shape.args.add(el.value);
        break;
      case TYPE.select:
      case TYPE.plural: {
        shape.args.add(el.value);
        if (el.type === TYPE.plural && el.pluralType !== 'ordinal') {
          const categories = shape.plurals.get(el.value) ?? new Set<string>();
          for (const option of Object.keys(el.options))
            if (!option.startsWith('=')) categories.add(option);
          shape.plurals.set(el.value, categories);
        }
        for (const option of Object.values(el.options)) walk(option.value, shape);
        break;
      }
      case TYPE.tag:
        shape.tags.add(el.value);
        walk(el.children, shape);
        break;
      default:
        break;
    }
  }
}

/** The arguments, tags and plurals of `message`; throws on ICU it can't parse. */
export function shape(message: string): Shape {
  const result: Shape = { args: new Set(), tags: new Set(), plurals: new Map() };
  walk(parse(message, { requiresOtherClause: true }), result);
  return result;
}

/**
 * The plural categories counts up to 1000 fall in: French's `many` only
 * covers millions, which falls back to `other` without harm.
 */
export function categories(locale: string): string[] {
  const rules = new Intl.PluralRules(locale);
  const found = new Set<string>();
  for (let n = 0; n <= 1000; n++) found.add(rules.select(n));
  return [...found];
}

const sameSet = (a: Set<string>, b: Set<string>) =>
  a.size === b.size && [...a].every((x) => b.has(x));
const list = (s: Set<string>) => [...s].sort().join(', ') || 'none';

/**
 * Problems in `translated` (the `locale` file `file`) against `english`, one
 * sentence each; empty when the translation is complete.
 */
export function compareMessages(
  locale: string,
  file: string,
  english: Messages,
  translated: Messages,
): string[] {
  const problems: string[] = [];
  const where = `${locale}/${file}`;
  const needed = categories(locale);
  for (const [key, source] of Object.entries(english)) {
    const text = translated[key];
    if (text === undefined) {
      problems.push(`${where} lacks ${key} (English: ${JSON.stringify(source)})`);
      continue;
    }
    let theirs: Shape;
    try {
      theirs = shape(text);
    } catch (e) {
      problems.push(`${where}: ${key} isn't valid ICU (${(e as Error).message})`);
      continue;
    }
    const ours = shape(source);
    if (!sameSet(ours.args, theirs.args))
      problems.push(
        `${where}: ${key} names {${list(theirs.args)}}, English names {${list(ours.args)}}`,
      );
    if (!sameSet(ours.tags, theirs.tags))
      problems.push(
        `${where}: ${key} has tags <${list(theirs.tags)}>, English has <${list(ours.tags)}>`,
      );
    for (const [arg, categories] of theirs.plurals) {
      const missing = needed.filter((c) => !categories.has(c));
      if (missing.length > 0)
        problems.push(
          `${where}: ${key}'s plural {${arg}} lacks ${missing.join(', ')} for ${locale}`,
        );
    }
  }
  for (const key of Object.keys(translated)) {
    if (!(key in english)) problems.push(`${where} has ${key}, which English doesn't`);
  }
  return problems;
}
