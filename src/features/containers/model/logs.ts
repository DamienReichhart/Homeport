import type { LogLine } from '@/shared/api';
import type { MessageKey } from '@/shared/i18n';

/** How many lines to show, with the key of each choice's label. */
export const TAILS: { value: string; label: MessageKey }[] = [
  { value: '100', label: 'containers.logs.tail.100' },
  { value: '1000', label: 'containers.logs.tail.1000' },
  { value: '10000', label: 'containers.logs.tail.10000' },
  { value: 'all', label: 'containers.logs.tail.all' },
];

/** How far back to show lines, in seconds, with the key of each choice's label. */
export const SINCE: { value: string; label: MessageKey }[] = [
  { value: 'all', label: 'containers.logs.since.all' },
  { value: '300', label: 'containers.logs.since.300' },
  { value: '3600', label: 'containers.logs.since.3600' },
  { value: '86400', label: 'containers.logs.since.86400' },
];

/** How many lines a viewer keeps for downloading. */
export const MAX_KEPT_LINES = 100_000;

/** ANSI colours cycled across sources in a multi-container stream. */
const SOURCE_COLOURS = [36, 35, 33, 32, 34, 31];

function colourFor(label: string, labels: string[]): number {
  return SOURCE_COLOURS[labels.indexOf(label) % SOURCE_COLOURS.length] ?? 36;
}

/** Puts a dimmed timestamp and a coloured source label in front of a line. */
export function renderLine(line: LogLine, labels: string[], timestamps: boolean): string {
  let text = line.text;
  let prefix = '';
  if (timestamps) {
    const space = text.indexOf(' ');
    if (space > 0) {
      prefix += `\x1b[2m${text.slice(0, space)}\x1b[22m `;
      text = text.slice(space + 1);
    }
  }
  if (line.source) {
    const width = Math.max(...labels.map((l) => l.length));
    prefix += `\x1b[${colourFor(line.source, labels)}m${line.source.padEnd(width)} |\x1b[39m `;
  }
  return prefix + text;
}
