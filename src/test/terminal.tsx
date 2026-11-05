import { useEffect, useRef, useState } from 'react';
import type { XTermHandle } from '@/shared/ui';

/** What a test can see of a fake terminal: its text without ANSI escapes, and its calls. */
export interface FakeTerminal {
  text: () => string;
  type: (data: string) => void;
  findNext: ReturnType<typeof vi.fn>;
  findPrevious: ReturnType<typeof vi.fn>;
  focus: ReturnType<typeof vi.fn>;
}

/** Every fake terminal rendered since the last `resetTerminals()`, in order. */
export const terminals: FakeTerminal[] = [];

export function resetTerminals() {
  terminals.length = 0;
}

// eslint-disable-next-line no-control-regex
const ANSI = /\x1b\[[0-9;]*m/g;

/**
 * Stands in for `XTerm` (xterm.js needs a canvas jsdom lacks): it calls
 * `onReady` with a terminal that renders what is written as plain text.
 * Use with `vi.mock('@/shared/ui', …)`, as the container tab tests do.
 */
export function FakeXTerm({
  onReady,
}: {
  onReady: (handle: XTermHandle) => (() => void) | void;
  readOnly?: boolean;
  className?: string;
}) {
  const [output, setOutput] = useState('');
  const ready = useRef(onReady);
  useEffect(() => {
    let written = '';
    const listeners: ((data: string) => void)[] = [];
    const fake: FakeTerminal = {
      text: () => written.replace(ANSI, '').replace(/\r\n/g, '\n'),
      type: (data) => listeners.forEach((l) => l(data)),
      findNext: vi.fn(),
      findPrevious: vi.fn(),
      focus: vi.fn(),
    };
    terminals.push(fake);
    const write = (data: string) => {
      written += data;
      setOutput(fake.text());
    };
    const terminal = {
      cols: 80,
      rows: 24,
      write,
      reset: () => write(''),
      clear: () => {
        written = '';
        setOutput('');
      },
      focus: fake.focus,
      scrollToBottom: () => {},
      onData: (listener: (data: string) => void) => {
        listeners.push(listener);
        return { dispose: () => listeners.splice(listeners.indexOf(listener), 1) };
      },
      onResize: () => ({ dispose: () => {} }),
    };
    const search = { findNext: fake.findNext, findPrevious: fake.findPrevious };
    return ready.current({
      terminal: terminal as unknown as XTermHandle['terminal'],
      search: search as unknown as XTermHandle['search'],
      fit: () => {},
    }) as (() => void) | undefined;
  }, []);
  return <pre data-testid="terminal">{output}</pre>;
}
