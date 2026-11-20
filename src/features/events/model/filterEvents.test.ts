import type { EngineEvent } from '@/shared/api';
import { actorName, filterEvents, isTrouble } from './filterEvents';

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

describe('isTrouble', () => {
  const died = (exitCode: string) => ({
    ...event('container', 'die', 'db'),
    attributes: { name: 'db', exitCode },
  });

  it('marks out-of-memory kills, failed exits and failing health checks', () => {
    expect(isTrouble(event('container', 'oom', 'db'))).toBe(true);
    expect(isTrouble(event('container', 'health_status: unhealthy', 'db'))).toBe(true);
    expect(isTrouble(died('137'))).toBe(true);
  });

  it('leaves clean exits and everything else quiet', () => {
    expect(isTrouble(died('0'))).toBe(false);
    expect(isTrouble(event('container', 'die', 'db'))).toBe(false);
    expect(isTrouble(event('container', 'health_status: healthy', 'db'))).toBe(false);
    expect(isTrouble(event('image', 'oom', 'nginx'))).toBe(false);
  });
});
