import type { ContainerdSnapshot } from '@/shared/api';
import { removable, shortKey, snapshotTree } from './tree';

const snap = (key: string, parent: string, size: number, over: Partial<ContainerdSnapshot> = {}) =>
  ({
    key,
    parent,
    kind: 'committed',
    size,
    inodes: 1,
    createdAt: null,
    labels: {},
    users: [],
    children: 0,
    ...over,
  }) as ContainerdSnapshot;

describe('the snapshots tree', () => {
  it('lists each base layer, then what’s built on it, largest first', () => {
    const rows = snapshotTree([
      snap('web', 'top', 4_000, { kind: 'active' }),
      snap('base', '', 10_000, { children: 1 }),
      snap('top', 'base', 2_000, { children: 2 }),
      snap('db', 'top', 9_000, { kind: 'active' }),
      snap('lost', 'gone', 1_000),
    ]);
    expect(rows.map((r) => [r.snapshot.key, r.depth])).toEqual([
      ['base', 0],
      ['top', 1],
      ['db', 2],
      ['web', 2],
      ['lost', 0],
    ]);
  });

  it('only offers removing what nothing uses or builds on', () => {
    expect(removable(snap('orphan', 'base', 1))).toBe(true);
    expect(removable(snap('base', '', 1, { children: 1 }))).toBe(false);
    expect(removable(snap('web', 'top', 1, { users: [{ kind: 'container', id: 'web' }] }))).toBe(
      false,
    );
  });

  it('shortens chain IDs only', () => {
    expect(shortKey(`sha256:${'7'.repeat(64)}`)).toBe('sha256:777777777777');
    expect(shortKey('web')).toBe('web');
  });
});
