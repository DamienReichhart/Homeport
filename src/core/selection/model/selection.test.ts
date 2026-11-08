import { rememberedSelection, resolveSelection } from './selection';

const host = (id: string, usable = true) => ({ id, usable });

describe('rememberedSelection', () => {
  it('reads the selection saved by this version', () => {
    expect(
      rememberedSelection({ lastSelection: { kind: 'cluster', id: 'c1' }, lastHostId: 'h1' }),
    ).toEqual({ kind: 'cluster', id: 'c1' });
  });

  it('keeps the Host remembered by an older version', () => {
    expect(rememberedSelection({ lastHostId: 'h1' })).toEqual({ kind: 'host', id: 'h1' });
  });

  it('remembers nothing on a first launch', () => {
    expect(rememberedSelection({ lastHostId: null })).toBeNull();
  });
});

describe('resolveSelection', () => {
  it('keeps the chosen Host while it exists', () => {
    const chosen = { kind: 'host', id: 'h2' } as const;
    expect(resolveSelection(chosen, [host('h1'), host('h2')], [])).toBe(chosen);
  });

  it('keeps the chosen Cluster while the Clusters load', () => {
    const chosen = { kind: 'cluster', id: 'c1' } as const;
    expect(resolveSelection(chosen, [host('h1')], undefined)).toBe(chosen);
  });

  it('falls back to the first connected Host when the chosen one is gone', () => {
    expect(
      resolveSelection({ kind: 'host', id: 'gone' }, [host('h1', false), host('h2')], []),
    ).toEqual({ kind: 'host', id: 'h2' });
  });

  it('falls back to the first Host when none is connected', () => {
    expect(resolveSelection(null, [host('h1', false), host('h2', false)], [])).toEqual({
      kind: 'host',
      id: 'h1',
    });
  });

  it('falls back to a Cluster when there are no Hosts', () => {
    expect(
      resolveSelection({ kind: 'cluster', id: 'gone' }, [], [host('c1', false), host('c2')]),
    ).toEqual({ kind: 'cluster', id: 'c2' });
  });

  it('selects nothing before the Hosts load, or when there is nothing', () => {
    expect(resolveSelection(null, undefined, undefined)).toBeNull();
    expect(resolveSelection(null, [], [])).toBeNull();
  });
});
