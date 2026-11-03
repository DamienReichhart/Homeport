import { commandLines, type Hint } from '@/shared/api';
import { FitAddon } from '@xterm/addon-fit';
import { SearchAddon } from '@xterm/addon-search';
import { Terminal, type ITheme } from '@xterm/xterm';
import '@xterm/xterm/css/xterm.css';
import { useEffect, useRef } from 'react';
import { useResolvedTheme } from '@/shared/lib/theme';
import { cn } from '@/shared/lib/utils';

// Homeport's terminal palettes, derived from the brand hues (docs/BRAND.md).
// xterm.js needs hex, so these mirror the CSS tokens rather than reading them.
const LIGHT: ITheme = {
  background: '#ffffff',
  foreground: '#0f1926',
  cursor: '#006aa5',
  cursorAccent: '#ffffff',
  selectionBackground: '#c3e3f9',
  black: '#0f1926',
  red: '#c2272d',
  green: '#007e46',
  yellow: '#a06604',
  blue: '#0062a6',
  magenta: '#8c399e',
  cyan: '#00778d',
  white: '#989fa8',
  brightBlack: '#545f6c',
  brightRed: '#de3b3d',
  brightGreen: '#0e9254',
  brightYellow: '#b77610',
  brightBlue: '#0077c2',
  brightMagenta: '#a44bb8',
  brightCyan: '#008aa3',
  brightWhite: '#b9bec4',
};
const DARK: ITheme = {
  background: '#080e16',
  foreground: '#e2e9ee',
  cursor: '#f8842d',
  cursorAccent: '#080e16',
  selectionBackground: '#1f405b',
  black: '#212935',
  red: '#f66c6d',
  green: '#54cc8e',
  yellow: '#eec05b',
  blue: '#55aee8',
  magenta: '#c889d7',
  cyan: '#66cfe1',
  white: '#d2d8dd',
  brightBlack: '#677380',
  brightRed: '#ff8d8b',
  brightGreen: '#7ae4aa',
  brightYellow: '#fddb7c',
  brightBlue: '#80c7f8',
  brightMagenta: '#dda7ea',
  brightCyan: '#94e3f1',
  brightWhite: '#f6f9fb',
};

/** Search highlights in Beacon orange, for `SearchAddon.findNext`/`findPrevious`. */
export const SEARCH_DECORATIONS = {
  matchOverviewRuler: '#989fa8',
  activeMatchColorOverviewRuler: '#f8842d',
  matchBackground: '#f8842d55',
  activeMatchBackground: '#f8842d',
};

const FONT_SIZE = 12;
const MONO = "'Geist Mono Variable', ui-monospace, SFMono-Regular, Menlo, Consolas, monospace";
const MONO_FALLBACK = 'ui-monospace, SFMono-Regular, Menlo, Consolas, monospace';

export interface XTermHandle {
  terminal: Terminal;
  search: SearchAddon;
  fit: () => void;
}

/**
 * An xterm.js terminal that fills its container. `onReady` runs once with the
 * terminal and may return a cleanup; `onResize` reports the size in cells.
 */
export function XTerm({
  onReady,
  onResize,
  readOnly = false,
  className,
}: {
  onReady: (handle: XTermHandle) => (() => void) | void;
  onResize?: (cols: number, rows: number) => void;
  readOnly?: boolean;
  className?: string;
}) {
  const host = useRef<HTMLDivElement>(null);
  const terminal = useRef<Terminal | null>(null);
  const resolved = useResolvedTheme();
  const callbacks = useRef({ onReady, onResize });

  useEffect(() => {
    callbacks.current = { onReady, onResize };
  });

  useEffect(() => {
    const el = host.current;
    if (!el) return;
    // xterm measures its cells once, so start on the fallback until Geist Mono is in.
    const fonts = typeof document.fonts === 'undefined' ? undefined : document.fonts;
    const fontReady = fonts?.check(`${FONT_SIZE}px 'Geist Mono Variable'`) ?? true;
    const term = new Terminal({
      convertEol: readOnly,
      cursorBlink: !readOnly,
      disableStdin: readOnly,
      fontFamily: fontReady ? MONO : MONO_FALLBACK,
      fontSize: FONT_SIZE,
      scrollback: 50_000,
      allowProposedApi: true,
    });
    const fit = new FitAddon();
    const search = new SearchAddon();
    term.loadAddon(fit);
    term.loadAddon(search);
    term.open(el);
    terminal.current = term;
    const doFit = () => {
      try {
        fit.fit();
      } catch {
        // Not laid out yet.
      }
    };
    doFit();
    if (!fontReady) {
      void fonts?.load(`${FONT_SIZE}px 'Geist Mono Variable'`).then(() => {
        if (terminal.current !== term) return;
        term.options.fontFamily = MONO;
        doFit();
      });
    }
    const resize = term.onResize(({ cols, rows }) => callbacks.current.onResize?.(cols, rows));
    const observer = new ResizeObserver(doFit);
    observer.observe(el);
    const cleanup = callbacks.current.onReady({ terminal: term, search, fit: doFit });
    callbacks.current.onResize?.(term.cols, term.rows);
    return () => {
      cleanup?.();
      observer.disconnect();
      resize.dispose();
      term.dispose();
      terminal.current = null;
    };
  }, [readOnly]);

  useEffect(() => {
    if (terminal.current) terminal.current.options.theme = resolved === 'dark' ? DARK : LIGHT;
  }, [resolved]);

  return (
    <div
      className={cn(
        'h-full min-h-0 overflow-hidden rounded-md border p-2',
        resolved === 'dark' ? 'bg-[#080e16]' : 'bg-white',
        className,
      )}
    >
      <div ref={host} className="h-full w-full" />
    </div>
  );
}

/**
 * A failure as terminal text: the message in red, then the hint dimmed, so
 * output panes say what to do and not only what broke. Ends with a newline.
 */
export function terminalError(error: { message: string; hint: Hint | null }): string {
  const lines = [`\x1b[31m${error.message}\x1b[39m`];
  if (error.hint) {
    lines.push(`\x1b[2m${error.hint.text}\x1b[22m`);
    lines.push(...commandLines(error.hint).map((l) => `\x1b[2m${l}\x1b[22m`));
  }
  return `${lines.join('\r\n')}\r\n`;
}
