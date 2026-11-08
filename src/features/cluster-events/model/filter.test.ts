import type { EventView } from '@/shared/api';
import { byType } from './filter';

const event = (type: string, name: string) =>
  ({ type, name, object: { kind: 'Pod', namespace: 'shop', name: 'web' } }) as unknown as EventView;

describe('byType', () => {
  it('keeps warnings, normal events or both', () => {
    const list = [event('Warning', 'a'), event('Normal', 'b')];
    expect(byType(list, 'Warning').map((e) => e.name)).toEqual(['a']);
    expect(byType(list, 'Normal').map((e) => e.name)).toEqual(['b']);
    expect(byType(list, 'all')).toHaveLength(2);
  });
});
