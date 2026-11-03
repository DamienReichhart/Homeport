import type { LogLine } from '@/shared/api';

export const TAILS = [
  { value: '100', label: 'Last 100' },
  { value: '1000', label: 'Last 1 000' },
  { value: '10000', label: 'Last 10 000' },
  { value: 'all', label: 'Everything' },
];

export const SINCE = [
  { value: 'all', label: 'Any time' },
  { value: '300', label: 'Last 5 min' },
  { value: '3600', label: 'Last hour' },
  { value: '86400', label: 'Last 24 h' },
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
