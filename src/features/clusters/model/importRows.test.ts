import type { KubeContext } from '@/shared/api';
import { choices, initialRows, rowsProblem } from './importRows';

const context = (name: string, overrides: Partial<KubeContext> = {}): KubeContext => ({
  name,
  server: 'https://127.0.0.1:6443',
  user: name,
  namespace: null,
  auth: { method: 'token' },
  current: false,
  added: null,
  problem: null,
  ...overrides,
});

describe('import rows', () => {
  it('picks the current context and names each after its context', () => {
    const rows = initialRows([context('kind-dev', { current: true }), context('prod')]);
    expect(rows.map((r) => [r.name, r.picked])).toEqual([
      ['kind-dev', true],
      ['prod', false],
    ]);
  });

  it('doesn’t pick a current context that is already added or broken', () => {
    const added = context('a', { current: true, added: { id: 'c1', name: 'a' } });
    const broken = context('b', {
      current: true,
      problem: { code: 'validation', key: '', params: {}, message: 'x', hint: null, detail: null },
    });
    expect(initialRows([added]).map((r) => r.picked)).toEqual([false]);
    expect(initialRows([broken]).map((r) => r.picked)).toEqual([false]);
  });

  it('imports the picked rows under their trimmed names', () => {
    const rows = initialRows([context('kind-dev'), context('prod')]).map((r) => ({
      ...r,
      picked: true,
      name: r.name === 'prod' ? '  Production ' : r.name,
    }));
    expect(choices(rows)).toEqual([
      { context: 'kind-dev', name: 'kind-dev' },
      { context: 'prod', name: 'Production' },
    ]);
  });

  it('says what’s wrong with the picked rows', () => {
    const rows = initialRows([context('a'), context('b')]);
    expect(rowsProblem(rows, [])).toEqual({ kind: 'nothing' });
    const both = rows.map((r) => ({ ...r, picked: true }));
    expect(rowsProblem(both, [])).toBeNull();
    expect(rowsProblem(both, ['A'])).toEqual({ kind: 'taken', name: 'a' });
    expect(
      rowsProblem(
        both.map((r) => ({ ...r, name: 'same' })),
        [],
      ),
    ).toEqual({
      kind: 'duplicate',
      name: 'same',
    });
    expect(rowsProblem([{ ...both[0]!, name: ' ' }], [])).toEqual({ kind: 'empty' });
  });
});
