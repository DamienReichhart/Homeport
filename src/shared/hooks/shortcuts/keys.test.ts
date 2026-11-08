import { firesAt, formatKeys, matches, parseKeys, targetOf, type KeyPress } from './keys';

const press = (key: string, mods: Partial<KeyPress> = {}): KeyPress => ({
  key,
  metaKey: false,
  ctrlKey: false,
  altKey: false,
  shiftKey: false,
  ...mods,
});

describe('shortcut keys', () => {
  it('reads Mod as ⌘ on macOS and Ctrl elsewhere', () => {
    const k = parseKeys('Mod+K');
    expect(matches(k, press('k', { metaKey: true }), true)).toBe(true);
    expect(matches(k, press('k', { ctrlKey: true }), true)).toBe(false);
    expect(matches(k, press('k', { ctrlKey: true }), false)).toBe(true);
    expect(matches(k, press('k', { metaKey: true }), false)).toBe(false);
    expect(matches(k, press('K', { ctrlKey: true, shiftKey: true }), false)).toBe(false);
  });

  it('needs exactly the modifiers it names', () => {
    const n = parseKeys('n');
    expect(matches(n, press('n'), false)).toBe(true);
    expect(matches(n, press('n', { ctrlKey: true }), false)).toBe(false);
    expect(matches(n, press('N', { shiftKey: true }), false)).toBe(false);
    const up = parseKeys('Alt+ArrowUp');
    expect(matches(up, press('ArrowUp', { altKey: true }), false)).toBe(true);
    expect(matches(up, press('ArrowUp'), false)).toBe(false);
  });

  it('matches symbols whatever Shift the layout needs', () => {
    expect(matches(parseKeys('?'), press('?', { shiftKey: true }), false)).toBe(true);
    expect(matches(parseKeys('/'), press('/'), false)).toBe(true);
    expect(matches(parseKeys('Mod+,'), press(',', { metaKey: true }), true)).toBe(true);
    expect(matches(parseKeys('Mod+1'), press('1', { ctrlKey: true }), false)).toBe(true);
  });

  it('shows the keys of each platform', () => {
    expect(formatKeys('Mod+K', true)).toEqual(['⌘', 'K']);
    expect(formatKeys('Mod+K', false)).toEqual(['Ctrl', 'K']);
    expect(formatKeys('Alt+ArrowDown', false)).toEqual(['Alt', '↓']);
    expect(formatKeys('Alt+ArrowDown', true)).toEqual(['⌥', '↓']);
    expect(formatKeys('?', false)).toEqual(['?']);
  });

  it('lets plain keys type in text fields, and keeps the terminal’s keys', () => {
    const n = parseKeys('n');
    const k = parseKeys('Mod+K');
    expect(firesAt(n, 'page')).toBe(true);
    expect(firesAt(n, 'text')).toBe(false);
    expect(firesAt(n, 'dialog')).toBe(false);
    expect(firesAt(k, 'text')).toBe(true);
    expect(firesAt(k, 'terminal')).toBe(false);
    expect(firesAt(parseKeys('Alt+ArrowUp'), 'text')).toBe(false);
  });

  it('tells where a key was pressed', () => {
    document.body.innerHTML = `
      <input id="field" />
      <div class="cm-editor"><div id="code" contenteditable="true"></div></div>
      <div class="xterm"><textarea id="term"></textarea></div>
      <div role="dialog"><button id="in-dialog"></button></div>
      <button id="plain"></button>`;
    const at = (id: string) => targetOf(document.getElementById(id));
    expect(at('field')).toBe('text');
    expect(at('code')).toBe('text');
    expect(at('term')).toBe('terminal');
    expect(at('in-dialog')).toBe('dialog');
    expect(at('plain')).toBe('page');
    expect(targetOf(document.body)).toBe('page');
  });
});
