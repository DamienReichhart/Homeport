import type { Stack } from '@/shared/api';
import { takenOver } from './newLinked';

const stack = (name: string, kind: Stack['kind'], id: string | null = null) =>
  ({ name, kind, id }) as Stack;

describe('a new Linked Stack', () => {
  it('takes over a running External Stack of the same name', () => {
    const stacks = [stack('site', 'external'), stack('shop', 'managed', 'm1')];
    expect(takenOver(stacks, 'site')?.name).toBe('site');
    expect(takenOver(stacks, 'shop')).toBeUndefined();
    expect(takenOver(stacks, 'other')).toBeUndefined();
    expect(takenOver(undefined, 'site')).toBeUndefined();
  });

  it('never takes over a Stack Homeport already has a record of', () => {
    expect(takenOver([stack('moved', 'external', 'l1')], 'moved')).toBeUndefined();
  });
});
