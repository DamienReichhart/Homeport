import { FitAddon } from '@xterm/addon-fit';
import { SearchAddon } from '@xterm/addon-search';
import { Terminal, type ITheme } from '@xterm/xterm';
import '@xterm/xterm/css/xterm.css';
import { useEffect, useRef } from 'react';
import { useResolvedTheme } from '@/shared/lib/theme';
import { cn } from '@/shared/lib/utils';

const LIGHT: ITheme = {
  background: '#ffffff',
  foreground: '#1f2328',
  cursor: '#1f2328',
  selectionBackground: '#b6d3ff',
};
const DARK: ITheme = {
  background: '#0d1117',
  foreground: '#e6edf3',
  cursor: '#e6edf3',
  selectionBackground: '#264f78',
};

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
    const term = new Terminal({
      convertEol: readOnly,
      cursorBlink: !readOnly,
      disableStdin: readOnly,
      fontFamily: 'ui-monospace, SFMono-Regular, Menlo, Consolas, monospace',
      fontSize: 12,
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
        resolved === 'dark' ? 'bg-[#0d1117]' : 'bg-white',
        className,
      )}
    >
      <div ref={host} className="h-full w-full" />
    </div>
  );
}
