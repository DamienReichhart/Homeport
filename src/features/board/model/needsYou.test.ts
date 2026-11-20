import type { Container } from '@/shared/api';
import type { BoardRow } from './board';
import { exitCode, needsYou, stackUpdates } from './needsYou';

const row = (
  name: string,
  state: string,
  status: string,
  health: string | null = null,
): BoardRow => ({
  key: `h/${name}`,
  hostId: 'h',
  hostName: 'prod',
  container: { id: name, name, state, status, health } as Container,
});

describe('needs you', () => {
  it('reads the exit code from the Engine status', () => {
    expect(exitCode('Exited (137) 3 hours ago')).toBe(137);
    expect(exitCode('Up 2 days')).toBeNull();
  });

  it('lists what needs attention, red first, and nothing healthy', () => {
    const needs = needsYou({
      hosts: [
        { id: 'a', name: 'a', unreachable: null },
        { id: 'b', name: 'b', unreachable: 'timed out' },
      ],
      clusters: [],
      rows: [
        row('ok', 'running', 'Up 2 days (healthy)', 'healthy'),
        row('crashed', 'exited', 'Exited (1) 2 hours ago'),
        row('stopped', 'exited', 'Exited (0) 2 hours ago'),
        row('sigterm', 'exited', 'Exited (143) 2 hours ago'),
        row('sick', 'running', 'Up 1 hour (unhealthy)', 'unhealthy'),
        row('looping', 'restarting', 'Restarting (1) 5 seconds ago'),
      ],
    });
    expect(needs.map((n) => [n.key, n.tone])).toEqual([
      ['host/b', 'danger'],
      ['h/sick', 'danger'],
      ['h/crashed', 'warning'],
      ['h/looping', 'warning'],
    ]);
  });

  it('lists unready workloads, red when none of their Pods is ready', () => {
    const workload = (name: string, ready: number) =>
      ({ kind: 'deployment', namespace: 'shop', name, ready, desired: 2 }) as never;
    const needs = needsYou({
      hosts: [],
      clusters: [],
      rows: [],
      unready: [
        {
          clusterId: 'c',
          clusterName: 'eks',
          workloads: [workload('api', 1), workload('search', 0)],
        },
      ],
    });
    expect(needs.map((n) => [n.key, n.tone])).toEqual([
      ['workload/c/deployment/shop/search', 'danger'],
      ['workload/c/deployment/shop/api', 'warning'],
    ]);
  });

  it('counts the containers with newer images per Stack and Host', () => {
    const newer = [row('a', 'running', 'Up'), row('b', 'running', 'Up'), row('c', 'running', 'Up')];
    newer[0]!.container.stack = 'shop';
    newer[1]!.container.stack = 'shop';
    newer[2]!.container.stack = null;
    expect(stackUpdates(newer)).toEqual([
      { hostId: 'h', hostName: 'prod', stack: 'shop', containers: 2 },
    ]);
  });
});
