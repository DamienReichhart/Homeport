import type { SwarmTask } from '@/shared/api';
import { arrangeTasks, isFailed, taskName } from './tasks';

function task(overrides: Partial<SwarmTask>): SwarmTask {
  return {
    id: 't',
    serviceId: 's',
    serviceName: 'web',
    slot: 1,
    nodeId: 'n1',
    nodeHostname: 'alpha',
    desiredState: 'running',
    state: 'running',
    since: '2026-11-08T10:00:00Z',
    error: null,
    message: null,
    image: 'nginx',
    containerId: 'c',
    exitCode: null,
    hostId: null,
    ...overrides,
  };
}

describe('tasks', () => {
  const tasks = [
    task({
      id: 'old',
      slot: 1,
      desiredState: 'shutdown',
      state: 'failed',
      since: '2026-11-08T09:00:00Z',
    }),
    task({ id: 'b2', slot: 2 }),
    task({ id: 'b1', slot: 1 }),
    task({ id: 'api', serviceName: 'api', slot: 10 }),
    task({ id: 'api9', serviceName: 'api', slot: 9 }),
  ];

  it('shows one row per slot without history', () => {
    expect(arrangeTasks(tasks, false).map((t) => t.id)).toEqual(['api9', 'api', 'b1', 'b2']);
  });

  it('adds the earlier attempts with history, newest first in their slot', () => {
    expect(arrangeTasks(tasks, true).map((t) => t.id)).toEqual(['api9', 'api', 'b1', 'old', 'b2']);
  });

  it('names replicas by slot and global tasks by node', () => {
    expect(taskName(task({ slot: 3 }))).toBe('web.3');
    expect(taskName(task({ slot: null, serviceName: 'agent' }))).toBe('agent@alpha');
    expect(isFailed(task({ state: 'rejected' }))).toBe(true);
  });
});
