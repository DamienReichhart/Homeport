import type { Stack } from '@/shared/api';
import type { ServiceFreshness } from './freshness';
import { imagesToUpdate, updateAction } from './update';

const stack = (kind: Stack['kind'], state: Stack['state'] = 'running') =>
  ({ kind, state, id: kind === 'external' ? null : 's1' }) as Stack;
const newer: ServiceFreshness[] = [
  { service: 'web', freshness: 'newer', images: ['nginx:1.27'] },
  { service: 'proxy', freshness: 'newer', images: ['nginx:1.27'] },
];

describe('updating a Stack', () => {
  it('offers Update to deployed Managed and Linked Stacks with newer images', () => {
    expect(updateAction(stack('managed'), newer)).toBe('update');
    expect(updateAction(stack('linked'), newer)).toBe('update');
    expect(updateAction(stack('managed'), [])).toBe('none');
    expect(updateAction(stack('managed', 'notDeployed'), newer)).toBe('none');
  });

  it('asks to adopt an External Stack first', () => {
    expect(updateAction(stack('external'), newer)).toBe('adoptFirst');
  });

  it('pulls each image once', () => {
    expect(imagesToUpdate(newer)).toEqual(['nginx:1.27']);
  });
});
