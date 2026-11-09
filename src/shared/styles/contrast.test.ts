/**
 * Every colour pair the app and the website use, in both themes, against
 * WCAG 2.2 AA: 4.5:1 for text, 3:1 for what isn't text but must be seen —
 * input borders, focus rings, status shapes, chart lines (1.4.3, 1.4.11).
 * The pairs come from real usage (`text-muted-foreground` on a card…), not
 * every combination. Ratios are listed in docs/BRAND.md. axe can't measure
 * contrast in jsdom, so this is the check.
 */
import { readFileSync } from 'node:fs';
import { join } from 'node:path';

const root = join(__dirname, '..', '..', '..');
const app = readFileSync(join(root, 'src/shared/styles/global.css'), 'utf8');
const site = readFileSync(join(root, 'website/src/app/styles/theme.css'), 'utf8');

/** The custom properties declared in the first block opened by `selector`. */
function tokens(css: string, selector: string): Record<string, string> {
  const start = css.indexOf(selector);
  if (start < 0) throw new Error(`no ${selector} block`);
  const open = css.indexOf('{', start);
  const body = css.slice(open, css.indexOf('}', open));
  return Object.fromEntries(
    [...body.matchAll(/--([\w-]+):\s*([^;]+);/g)].map((m) => [m[1], m[2]?.trim() ?? '']),
  );
}

type Rgba = [number, number, number, number];

function parse(colour: string): Rgba {
  const hex = /^#([0-9a-f]{6})$/i.exec(colour);
  if (hex) {
    const n = parseInt(hex[1] ?? '', 16);
    return [(n >> 16) & 255, (n >> 8) & 255, n & 255, 1];
  }
  const rgb = /^rgb\((\d+) (\d+) (\d+) \/ ([\d.]+)(%?)\)$/.exec(colour);
  if (rgb) {
    const alpha = Number(rgb[4]) / (rgb[5] ? 100 : 1);
    return [Number(rgb[1]), Number(rgb[2]), Number(rgb[3]), alpha];
  }
  throw new Error(`can't read colour ${colour}`);
}

/** `fg` (possibly translucent, or at `opacity`) composed over the opaque `bg`. */
function over(fg: Rgba, bg: Rgba, opacity = 1): Rgba {
  const a = fg[3] * opacity;
  return [0, 1, 2].map((i) => (fg[i] ?? 0) * a + (bg[i] ?? 0) * (1 - a)).concat(1) as Rgba;
}

function luminance([r, g, b]: Rgba): number {
  const [lr, lg, lb] = [r, g, b].map((c) => {
    const s = c / 255;
    return s <= 0.03928 ? s / 12.92 : ((s + 0.055) / 1.055) ** 2.4;
  });
  return 0.2126 * (lr ?? 0) + 0.7152 * (lg ?? 0) + 0.0722 * (lb ?? 0);
}

export function ratio(a: Rgba, b: Rgba): number {
  const [hi, lo] = [luminance(a), luminance(b)].sort((x, y) => y - x);
  return ((hi ?? 0) + 0.05) / ((lo ?? 0) + 0.05);
}

const TEXT = 4.5;
const UI = 3;

/** [foreground token, background token, minimum, foreground opacity, background over] */
type Pair = [string, string, number, number?, string?];

const APP_PAIRS: Pair[] = [
  ['foreground', 'background', TEXT],
  ['card-foreground', 'card', TEXT],
  ['popover-foreground', 'popover', TEXT],
  ['muted-foreground', 'background', TEXT],
  ['muted-foreground', 'card', TEXT],
  ['muted-foreground', 'popover', TEXT],
  ['muted-foreground', 'muted', TEXT],
  ['muted-foreground', 'accent', TEXT],
  ['muted-foreground', 'sidebar', TEXT],
  ['primary-foreground', 'primary', TEXT],
  ['secondary-foreground', 'secondary', TEXT],
  ['accent-foreground', 'accent', TEXT],
  ['destructive', 'background', TEXT],
  ['destructive', 'card', TEXT],
  ['warning', 'background', TEXT],
  ['warning', 'card', TEXT],
  ['sidebar-foreground', 'sidebar', TEXT],
  ['sidebar-accent-foreground', 'sidebar-accent', TEXT],
  // A Stack's group row: bg-muted/30 over the card, with muted text on it.
  ['muted-foreground', 'muted', TEXT, 1, 'card'],
  // A control's only boundary, and the focus ring at the opacity it's drawn with.
  ['input', 'background', UI],
  ['input', 'card', UI],
  ['ring', 'background', UI, 0.5],
  ['ring', 'card', UI, 0.5],
  // Status shapes and the grey chart series.
  ['success', 'card', UI],
  ['destructive', 'card', UI],
  ['warning', 'card', UI],
  ['info', 'card', UI],
  ['chart-4', 'card', UI],
];

const SITE_PAIRS: Pair[] = [
  ['hp-text', 'hp-bg', TEXT],
  ['hp-text', 'hp-surface', TEXT],
  ['hp-text-muted', 'hp-bg', TEXT],
  ['hp-text-muted', 'hp-surface', TEXT],
  ['hp-text-faint', 'hp-bg', TEXT],
  ['hp-text-faint', 'hp-surface', TEXT],
  ['hp-text-faint', 'hp-bg-raised', TEXT],
  ['hp-text', 'hp-code-bg', TEXT],
  ['hp-on-primary', 'hp-primary', TEXT],
  ['hp-signal', 'hp-bg', TEXT],
  ['hp-board-text', 'hp-board', TEXT],
  ['hp-board-muted', 'hp-board', TEXT],
  ['hp-board-signal', 'hp-board', TEXT],
  // --hp-border-strong outlines buttons and panels whose text already
  // identifies them, so 1.4.11 doesn't ask it for 3:1.
];

/** Each failing pair with its ratio; empty when every pair passes. */
function failures(theme: Record<string, string>, pairs: Pair[]): string[] {
  const colour = (name: string) => {
    const value = theme[name];
    if (!value) throw new Error(`no --${name}`);
    return parse(value);
  };
  return pairs.flatMap(([fg, bg, min, opacity = 1, under]) => {
    const back = under ? over(colour(bg), colour(under), 0.3) : colour(bg);
    const front = over(colour(fg), back, opacity);
    const r = ratio(front, back);
    return r + 1e-9 < min ? [`--${fg} on --${bg}: ${r.toFixed(2)}:1, needs ${min}:1`] : [];
  });
}

describe('colour contrast (WCAG 2.2 AA)', () => {
  const light = tokens(app, ':root {');
  const dark = { ...light, ...tokens(app, '.dark {') };
  it('passes in the app, light theme', () => expect(failures(light, APP_PAIRS)).toEqual([]));
  it('passes in the app, dark theme', () => expect(failures(dark, APP_PAIRS)).toEqual([]));

  const siteDark = tokens(site, ':root,\n');
  it('passes on the website, dark theme', () => expect(failures(siteDark, SITE_PAIRS)).toEqual([]));
  const siteLight = { ...siteDark, ...tokens(site, ":root[data-theme='light'],\n") };
  it('passes on the website, light theme', () =>
    expect(failures(siteLight, SITE_PAIRS)).toEqual([]));

  it('catches a failing value', () =>
    expect(failures({ ...light, 'muted-foreground': '#9a9a94' }, APP_PAIRS)).not.toEqual([]));
});
