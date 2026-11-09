import { render } from '@testing-library/react';
import { mockReducedMotion } from '@/test/motion';

const created: { cursorBlink?: boolean }[] = [];
vi.mock('@xterm/xterm', () => ({
  Terminal: class {
    options: Record<string, unknown>;
    constructor(options: Record<string, unknown>) {
      this.options = { ...options };
      created.push(this.options);
    }
    loadAddon() {}
    open() {}
    onResize() {
      return { dispose() {} };
    }
    onData() {
      return { dispose() {} };
    }
    attachCustomKeyEventHandler() {}
    write() {}
    focus() {}
    dispose() {}
    get cols() {
      return 80;
    }
    get rows() {
      return 24;
    }
  },
}));
vi.mock('@xterm/addon-fit', () => ({
  FitAddon: class {
    fit() {}
  },
}));
vi.mock('@xterm/addon-search', () => ({ SearchAddon: class {} }));

const { XTerm } = await import('./XTerm');

describe('XTerm', () => {
  beforeEach(() => {
    created.length = 0;
  });

  it('blinks its cursor, unless the OS asks for reduced motion', () => {
    const motion = mockReducedMotion(false);
    const first = render(<XTerm onReady={() => {}} />);
    expect(created[0]?.cursorBlink).toBe(true);
    first.unmount();
    motion.set(true);
    render(<XTerm onReady={() => {}} />);
    expect(created[1]?.cursorBlink).toBe(false);
    motion.restore();
  });
});
