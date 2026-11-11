/**
 * ICU MessageFormat (formatjs, ADR 0011) to Fluent, for the iced app's
 * catalogues: one message at a time, keeping its arguments, plurals, selects
 * and tags. Tags stay in the text (`<strong>…</strong>`); the iced lookup
 * splits them into spans. A plural's `#` becomes `NUM($count)`, the app's
 * function that writes a number the way the language does.
 */
import { parse, TYPE, type MessageFormatElement } from '@formatjs/icu-messageformat-parser';

/** `containers.empty.title` → `containers-empty-title`: Fluent ids have no dots. */
export function fluentId(key: string): string {
  if (!/^[a-zA-Z][a-zA-Z0-9_]*(\.[a-zA-Z0-9_]+)*$/.test(key))
    throw new Error(`"${key}" can't be made a Fluent id`);
  return key.replaceAll('.', '-');
}

const literal = (text: string) => `{"${text}"}`;

/** Text as Fluent takes it: braces and line breaks as string literals. */
function escapeText(text: string): string {
  return text.replace(/[{}]/g, (brace) => literal(brace)).replace(/\r?\n/g, literal('\\u000A'));
}

/** A pattern whose first and last spaces Fluent would otherwise trim. */
function keepEdges(pattern: string): string {
  if (pattern === '') return literal('');
  const leading = /^ +/.exec(pattern)?.[0] ?? '';
  const trailing = leading.length < pattern.length ? (/ +$/.exec(pattern)?.[0] ?? '') : '';
  const middle = pattern.slice(leading.length, pattern.length - trailing.length);
  return (leading && literal(leading)) + middle + (trailing && literal(trailing));
}

/** A variant key: a plural category, a select value or an exact number. */
function variantKey(option: string): string {
  const key = option.startsWith('=') ? option.slice(1) : option;
  if (!/^(-?[0-9]+(\.[0-9]+)?|[a-zA-Z][a-zA-Z0-9_-]*)$/.test(key))
    throw new Error(`"${option}" can't be a Fluent variant`);
  return key;
}

function select(
  argument: string,
  options: Record<string, { value: MessageFormatElement[] }>,
  depth: number,
  pound: string | undefined,
): string {
  const pad = '    '.repeat(depth + 1);
  // ICU prefers an exact `=N` wherever it's written; Fluent takes the first match.
  const names = Object.keys(options).sort(
    (a, b) => Number(b.startsWith('=')) - Number(a.startsWith('=')),
  );
  if (!names.includes('other')) throw new Error(`{${argument}} has no "other"`);
  const variants = names.map((name) => {
    const body = keepEdges(emit(options[name]?.value ?? [], depth + 1, pound));
    const marker = name === 'other' ? '   *' : '    ';
    return `${pad}${marker}[${variantKey(name)}] ${body}`;
  });
  return `{ $${argument} ->\n${variants.join('\n')}\n${pad}}`;
}

function emit(elements: MessageFormatElement[], depth: number, pound?: string): string {
  let out = '';
  for (const el of elements) {
    switch (el.type) {
      case TYPE.literal:
        out += escapeText(el.value);
        break;
      case TYPE.argument:
        out += `{ $${el.value} }`;
        break;
      case TYPE.pound:
        if (!pound) throw new Error('"#" outside a plural');
        // The count in the language's digits and grouping, as ICU's `#` is.
        out += `{ NUM($${pound}) }`;
        break;
      case TYPE.plural:
        if (el.pluralType === 'ordinal' || el.offset)
          throw new Error(`{${el.value}} uses an ordinal or an offset, which isn't converted`);
        out += select(el.value, el.options, depth, el.value);
        break;
      case TYPE.select:
        out += select(el.value, el.options, depth, pound);
        break;
      case TYPE.tag:
        out += `<${el.value}>${emit(el.children, depth, pound)}</${el.value}>`;
        break;
      default:
        throw new Error(`a ${TYPE[el.type]} argument isn't converted`);
    }
  }
  return out;
}

/** The Fluent pattern of an ICU message. */
export function fluentPattern(message: string): string {
  return keepEdges(emit(parse(message, { requiresOtherClause: true }), 0));
}

/** One `.ftl` file from one namespace's messages. */
export function fluentFile(header: string, messages: Record<string, string>): string {
  const lines = header.split('\n').map((line) => (line ? `### ${line}` : '###'));
  lines.push('');
  for (const [key, message] of Object.entries(messages)) {
    try {
      lines.push(`${fluentId(key)} = ${fluentPattern(message)}`);
    } catch (e) {
      throw new Error(`${key}: ${e instanceof Error ? e.message : String(e)}`, { cause: e });
    }
  }
  return lines.join('\n') + '\n';
}
