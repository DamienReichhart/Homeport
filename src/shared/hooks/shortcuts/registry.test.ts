import { ShortcutRegistry } from './registry';

const key = (k: string, init: KeyboardEventInit = {}, target: Element = document.body) => {
  const event = new KeyboardEvent('keydown', { key: k, bubbles: true, cancelable: true, ...init });
  Object.defineProperty(event, 'target', { value: target });
  return event;
};

describe('shortcut registry', () => {
  it('runs the latest match and lists each shortcut once', () => {
    const registry = new ShortcutRegistry();
    const ran: string[] = [];
    registry.register({ keys: 'n', label: 'New', group: 'App', handler: () => ran.push('app') });
    const off = registry.register({
      keys: 'n',
      label: 'Run container',
      group: 'This page',
      handler: () => ran.push('page'),
    });
    const event = key('n');
    expect(registry.dispatch(event)).toBe(true);
    expect(event.defaultPrevented).toBe(true);
    expect(ran).toEqual(['page']);
    expect(registry.list().map((s) => s.label)).toEqual(['New', 'Run container']);
    // Leaving the page takes its shortcut away.
    off();
    registry.dispatch(key('n'));
    expect(ran).toEqual(['page', 'app']);
    expect(registry.list().map((s) => s.label)).toEqual(['New']);
  });

  it('leaves typing in a field alone', () => {
    const registry = new ShortcutRegistry();
    let ran = 0;
    registry.register({ keys: '/', label: 'Search', group: 'Page', handler: () => ran++ });
    const input = document.createElement('input');
    document.body.append(input);
    const event = key('/', {}, input);
    expect(registry.dispatch(event)).toBe(false);
    expect(event.defaultPrevented).toBe(false);
    expect(ran).toBe(0);
  });

  it('skips disabled shortcuts', () => {
    const registry = new ShortcutRegistry();
    registry.register({
      keys: 'n',
      label: 'New',
      group: 'Page',
      handler: () => {},
      enabled: false,
    });
    expect(registry.dispatch(key('n'))).toBe(false);
    expect(registry.list()).toEqual([]);
  });
});
