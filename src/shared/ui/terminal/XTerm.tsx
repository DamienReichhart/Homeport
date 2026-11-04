import { commandLines, type Hint } from '@/shared/api';
import { FitAddon } from '@xterm/addon-fit';
import { SearchAddon } from '@xterm/addon-search';
import { Terminal, type ITheme } from '@xterm/xterm';
import '@xterm/xterm/css/xterm.css';
import { useEffect, useRef } from 'react';
import { useResolvedTheme } from '@/shared/lib/theme';
import { cn } from '@/shared/lib/utils';

// Homeport's terminal palettes (docs/BRAND.md): Ink on white, Paper on near-black.
// The 16 ANSI colours belong to the programs that print them, so they stay
// distinct, tuned to the warm-grey neutrals; red is the brand's Signal red.
// xterm.js needs hex, so these mirror the CSS tokens rather than reading them.
const LIGHT: ITheme = {
  background: '#ffffff',
  foreground: '#141414',
  cursor: '#141414',
  cursorAccent: '#ffffff',
  selectionBackground: '#d9d9d2',
  black: '#141414',
  red: '#c8102e',
  green: '#2f7a3a',
  yellow: '#8f5a00',
  blue: '#1f5a96',
  magenta: '#8a3a8f',
  cyan: '#1f7380',
  white: '#8c8c86',
  brightBlack: '#5c5c57',
  brightRed: '#e1001a',
  brightGreen: '#3a8f47',
  brightYellow: '#a86b00',
  brightBlue: '#2b6cb0',
  brightMagenta: '#a34ba8',
  brightCyan: '#278a99',
  brightWhite: '#b5b5ae',
};
const DARK: ITheme = {
  background: '#0f0f0f',
  foreground: '#f7f7f4',
  cursor: '#f7f7f4',
  cursorAccent: '#0f0f0f',
  selectionBackground: '#3a3a37',
  black: '#262624',
  red: '#ff5c5c',
  green: '#6cc77d',
  yellow: '#f2a93b',
  blue: '#7aa7e0',
  magenta: '#cf8fd6',
  cyan: '#6cc4cf',
  white: '#d6d6d0',
  brightBlack: '#6e6e69',
  brightRed: '#ff8080',
  brightGreen: '#8fdc9d',
  brightYellow: '#ffc46b',
  brightBlue: '#9dc0ee',
  brightMagenta: '#e0aee6',
  brightCyan: '#93d9e2',
  brightWhite: '#ffffff',
};

/** Search highlights, for `SearchAddon.findNext`/`findPrevious`: grey matches, the current one amber. */
export const SEARCH_DECORATIONS = {
  matchOverviewRuler: '#8c8c86',
  activeMatchColorOverviewRuler: '#f2a93b',
  matchBackground: '#8c8c8666',
  activeMatchBackground: '#f2a93b',
};

const FONT_SIZE = 12;
const MONO = "'Fragment Mono', ui-monospace, SFMono-Regular, Menlo, Consolas, monospace";
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
    // xterm measures its cells once, so start on the fallback until Fragment Mono is in.
    const fonts = typeof document.fonts === 'undefined' ? undefined : document.fonts;
    const fontReady = fonts?.check(`${FONT_SIZE}px 'Fragment Mono'`) ?? true;
    const term = new Terminal({
      convertEol: readOnly,
      cursorBlink: !readOnly,
      disableStdin: readOnly,
      fontFamily: fontReady ? MONO : MONO_FALLBACK,
      fontSize: FONT_SIZE,
      // Fragment Mono has one weight; bold text shows in the bright colours instead of a faked bold.
      fontWeightBold: 'normal',
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
      void fonts?.load(`${FONT_SIZE}px 'Fragment Mono'`).then(() => {
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
        resolved === 'dark' ? 'bg-[#0f0f0f]' : 'bg-white',
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
