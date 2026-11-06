import type { EngineEvent } from '@/shared/api';
import { actorName, filterEvents } from './filterEvents';

const event = (kind: string, action: string, name: string): EngineEvent => ({
  time: 1_700_000_000,
  timeNano: 1_700_000_000_000_000_000,
  kind,
  action,
  actorId: 'sha256:0123456789abcdef',
  attributes: { name },
});

describe('filterEvents', () => {
  it('filters by type and search', () => {
    const list = [
      event('container', 'start', 'web'),
      event('image', 'pull', 'nginx'),
      event('container', 'die', 'db'),
    ];
    expect(filterEvents(list, 'container', '').map(actorName)).toEqual(['web', 'db']);
    expect(filterEvents(list, 'all', 'pul').map(actorName)).toEqual(['nginx']);
  });
});
