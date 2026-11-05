import type { ComposeFile } from '@/shared/api';
import { addFile, freeName, move, removeFile, sameFiles, updateFile } from './composeFiles';

const f = (name: string, content = ''): ComposeFile => ({ name, content });

describe('a Stack’s compose files', () => {
  it('suggests a free override name', () => {
    expect(freeName([f('compose.yaml')])).toBe('compose.override.yaml');
    expect(
      freeName([
        f('compose.yaml'),
        f('compose.override.yaml'),
        f('compose.prod.yaml'),
        f('compose.dev.yaml'),
      ]),
    ).toBe('compose.2.yaml');
    expect(addFile([f('compose.yaml')]).map((x) => x.name)).toEqual([
      'compose.yaml',
      'compose.override.yaml',
    ]);
  });

  it('keeps the base file', () => {
    const files = [f('compose.yaml'), f('compose.prod.yaml')];
    expect(removeFile(files, 0)).toBe(files);
    expect(removeFile(files, 1)).toEqual([f('compose.yaml')]);
  });

  it('renames, edits and reorders', () => {
    const files = [f('a.yaml'), f('b.yaml'), f('c.yaml')];
    expect(updateFile(files, 1, { name: 'x.yaml' })[1]).toEqual(f('x.yaml'));
    expect(move(files, 2, -1).map((x) => x.name)).toEqual(['a.yaml', 'c.yaml', 'b.yaml']);
    expect(move(files, 0, -1)).toBe(files);
    expect(move(files, 2, 1)).toBe(files);
  });

  it('compares definitions file by file, in order', () => {
    expect(sameFiles([f('a.yaml', '1')], [f('a.yaml', '1')])).toBe(true);
    expect(sameFiles([f('a.yaml'), f('b.yaml')], [f('b.yaml'), f('a.yaml')])).toBe(false);
    expect(sameFiles([f('a.yaml', '1')], [f('a.yaml', '2')])).toBe(false);
  });
});
