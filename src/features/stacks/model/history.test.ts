import type { DeploySummary } from '@/shared/api';
import { currentDeploy, outcomeOf } from './history';

const deploy = (
  id: string,
  success: boolean | null,
  finished = success !== null,
): DeploySummary => ({
  id,
  stackId: 's1',
  kind: 'managed',
  action: 'up',
  startedAt: '2026-11-05T10:00:00.000Z',
  finishedAt: finished ? '2026-11-05T10:00:05.000Z' : null,
  success,
  exitCode: success === null ? null : success ? 0 : 1,
  hasEnv: false,
});

describe('a Stack’s deploy history', () => {
  it('tells how each deploy ended', () => {
    expect(outcomeOf(deploy('a', true))).toBe('succeeded');
    expect(outcomeOf(deploy('b', false))).toBe('failed');
    expect(outcomeOf(deploy('c', null))).toBe('unfinished');
  });

  it('marks the newest success as what is deployed', () => {
    // Newest first, as the backend lists them.
    const list = [
      deploy('cancelled', null),
      deploy('failed', false),
      deploy('ok', true),
      deploy('older', true),
    ];
    expect(currentDeploy(list)).toBe('ok');
    expect(currentDeploy([deploy('failed', false)])).toBeNull();
  });
});
